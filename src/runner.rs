use crate::{
    backend::{self, Backend, Request, TokenCount},
    config::{ConfigError, Experiment, fingerprint},
    observation::Observation,
    protocol::{Outcome, Phase, PrivateState, PublicMessage, PublicView, Status, normalize},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    sync::Arc,
    time::{Duration, Instant},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Reply {
    Response { body: String },
    Failure { status: Status },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Call {
    pub handle: String,
    pub observation: Observation,
    pub elapsed_ms: u64,
    pub token_trials: Vec<TokenTrial>,
    pub response_schema_fingerprint: String,
    pub reply: Reply,
    pub outcome: Outcome,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenTrial {
    pub prompt_fingerprint: String,
    pub count: TokenCount,
}

pub fn tokens_fit(config: &Experiment, count: &TokenCount) -> bool {
    let capacity = count
        .model_capacity
        .unwrap_or(config.memory.context_window_tokens)
        .min(config.memory.context_window_tokens);
    count.prompt_tokens <= config.memory.max_prompt_tokens
        && count.prompt_tokens as u64
            + config.decoding.max_tokens as u64
            + config.memory.safety_margin_tokens as u64
            <= capacity as u64
}

pub fn fit_bytes(config: &Experiment, observation: &mut Observation) -> bool {
    while observation.prompt_bytes() > config.max_context_bytes {
        if !observation.drop_oldest_event() {
            return false;
        }
    }
    true
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SelectionRound {
    pub round: u32,
    pub outcomes: BTreeMap<String, Outcome>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Session {
    pub calls: Vec<Call>,
    pub public: PublicView,
    pub private_states: BTreeMap<String, PrivateState>,
    pub selections: Vec<SelectionRound>,
    pub batches: Vec<crate::merge_request::BatchDecision>,
}

impl Session {
    pub fn initial(config: &Experiment) -> Self {
        Self {
            calls: vec![],
            public: PublicView::default(),
            private_states: config
                .agents
                .iter()
                .map(|a| (a.handle.clone(), a.initial_private.clone()))
                .collect(),
            selections: vec![],
            batches: vec![],
        }
    }

    pub fn observations(&self, config: &Experiment, round: u32, phase: Phase) -> Vec<Observation> {
        self.private_states
            .iter()
            .map(|(owner, state)| {
                Observation::new(config, owner, state, &self.public, round, phase.clone())
            })
            .collect()
    }

    /// Called only after all responses in this phase have finished.
    pub fn publish(&mut self, config: &Experiment, calls: Vec<Call>, round: u32, phase: &Phase) {
        let mut selections = BTreeMap::new();
        for call in &calls {
            if let Some(update) = &call.outcome.private_update {
                self.private_states
                    .get_mut(&call.handle)
                    .expect("known owner")
                    .update(update);
            }
            if let Phase::Communication { step } = phase {
                if let Some(text) = &call.outcome.public_message {
                    self.public.messages.push(PublicMessage {
                        round,
                        step: *step,
                        author: config.alias(&call.handle),
                        text: text.clone(),
                    });
                }
            } else {
                selections.insert(call.handle.clone(), call.outcome.clone());
            }
        }
        if *phase == Phase::Selection {
            let selection = SelectionRound {
                round,
                outcomes: selections,
            };
            let batches = crate::merge_request::resolve_round(config, &selection);
            self.public
                .pairs
                .extend(batches.iter().map(|batch| crate::protocol::PublicPair {
                    round,
                    pair: crate::protocol::Pair {
                        agents: batch.pair.agents.clone().map(|h| config.alias(&h)),
                    },
                }));
            self.batches.extend(batches);
            self.selections.push(selection);
        }
        self.calls.extend(calls);
    }
}

pub async fn run(config: &Experiment) -> Result<Session, ConfigError> {
    config.validate()?;
    let backends = config
        .agents
        .iter()
        .map(|a| Ok((a.handle.clone(), backend::configured(&a.backend)?)))
        .collect::<Result<BTreeMap<_, _>, ConfigError>>()?;
    run_with(config, backends).await
}

pub async fn run_with(
    config: &Experiment,
    backends: BTreeMap<String, Arc<dyn Backend>>,
) -> Result<Session, ConfigError> {
    config.validate()?;
    if backends.len() != config.agents.len()
        || config
            .agents
            .iter()
            .any(|a| !backends.contains_key(&a.handle))
    {
        return Err(ConfigError(
            "backend handles must exactly match configured agents",
        ));
    }
    let mut session = Session::initial(config);
    for round in 0..config.rounds {
        let phases = (0..config.communication_steps)
            .map(|step| Phase::Communication { step })
            .chain(std::iter::once(Phase::Selection));
        for phase in phases {
            let observations = session.observations(config, round, phase.clone());
            let permits = Arc::new(tokio::sync::Semaphore::new(config.workers));
            let mut pending = Vec::new();
            for mut observation in observations {
                let handle = config
                    .handle_for_alias(&observation.owner)
                    .expect("known owner");
                let backend = backends[&handle].clone();
                let permits = permits.clone();
                let limits = config.clone();
                let mut fallback = observation.clone();
                fit_bytes(config, &mut fallback);
                let task = tokio::spawn(async move {
                    let _permit = permits
                        .acquire_owned()
                        .await
                        .expect("open worker semaphore");
                    let start = Instant::now();
                    let deadline =
                        tokio::time::Instant::now() + Duration::from_millis(limits.timeout_ms);
                    let mut trials = Vec::new();
                    let failure = loop {
                        if !fit_bytes(&limits, &mut observation) {
                            break Some(Status::ContextLimit);
                        }
                        let request = Request {
                            observation: observation.clone(),
                            decoding: limits.decoding.clone(),
                            seed: limits.seed,
                            max_response_bytes: limits.max_response_bytes,
                        };
                        let count =
                            match tokio::time::timeout_at(deadline, backend.count_tokens(request))
                                .await
                            {
                                Err(_) => break Some(Status::Timeout),
                                Ok(Err(_)) => break Some(Status::TokenizationFailure),
                                Ok(Ok(count)) => count,
                            };
                        let fits = tokens_fit(&limits, &count);
                        trials.push(TokenTrial {
                            prompt_fingerprint: fingerprint(&observation.messages()),
                            count,
                        });
                        if fits {
                            break None;
                        }
                        if !observation.drop_oldest_event() {
                            break Some(Status::ContextLimit);
                        }
                    };
                    let reply = if let Some(status) = failure {
                        Reply::Failure { status }
                    } else {
                        let request = Request {
                            observation: observation.clone(),
                            decoding: limits.decoding.clone(),
                            seed: limits.seed,
                            max_response_bytes: limits.max_response_bytes,
                        };
                        match tokio::time::timeout_at(deadline, backend.respond(request)).await {
                            Err(_) => Reply::Failure {
                                status: Status::Timeout,
                            },
                            Ok(Err(status)) => Reply::Failure { status },
                            Ok(Ok(body)) if body.len() > limits.max_response_bytes => {
                                Reply::Failure {
                                    status: Status::OversizedResponse,
                                }
                            }
                            Ok(Ok(body)) => Reply::Response { body },
                        }
                    };
                    (
                        observation,
                        trials,
                        reply,
                        start.elapsed().as_millis().min(u64::MAX as u128) as u64,
                    )
                });
                pending.push((handle, fallback, task));
            }
            let mut calls = Vec::new();
            for (handle, fallback, task) in pending {
                let (observation, token_trials, reply, elapsed_ms) = task.await.unwrap_or((
                    fallback,
                    Vec::new(),
                    Reply::Failure {
                        status: Status::TransportFailure,
                    },
                    0,
                ));
                let outcome = outcome_from_reply(&reply, &observation, config);
                calls.push(Call {
                    handle,
                    response_schema_fingerprint: fingerprint(&observation.response_format()),
                    observation,
                    elapsed_ms,
                    token_trials,
                    reply,
                    outcome,
                });
            }
            session.publish(config, calls, round, &phase);
        }
    }
    Ok(session)
}

pub fn outcome_from_reply(
    reply: &Reply,
    observation: &Observation,
    config: &Experiment,
) -> Outcome {
    match reply {
        Reply::Response { body } => normalize(body, observation, config),
        Reply::Failure { status } => Outcome::failure(status.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        backend::ResponseFuture,
        config::{BackendConfig, FixtureResponse, tests::config},
    };
    use std::sync::{
        Mutex,
        atomic::{AtomicUsize, Ordering},
    };

    struct Probe {
        active: Arc<AtomicUsize>,
        peak: Arc<AtomicUsize>,
        seen: Arc<Mutex<Vec<Observation>>>,
        delay: u64,
    }
    impl Backend for Probe {
        fn count_tokens(&self, request: Request) -> crate::backend::TokenFuture {
            Box::pin(async move { Ok(crate::backend::fixture_tokens(&request.observation)) })
        }
        fn respond(&self, request: Request) -> ResponseFuture {
            let (active, peak, seen, delay) = (
                self.active.clone(),
                self.peak.clone(),
                self.seen.clone(),
                self.delay,
            );
            Box::pin(async move {
                let count = active.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(count, Ordering::SeqCst);
                seen.lock().unwrap().push(request.observation.clone());
                tokio::time::sleep(Duration::from_millis(delay)).await;
                active.fetch_sub(1, Ordering::SeqCst);
                Ok(if request.observation.phase == Phase::Selection {
                    r#"{"partner":null,"consent":{"state":"defer"}}"#.into()
                } else {
                    serde_json::json!({"public_message":request.observation.owner}).to_string()
                })
            })
        }
    }

    async fn probe_run(delays: [u64; 3]) -> (Session, usize) {
        let mut config = config();
        config.rounds = 1;
        let active = Arc::new(AtomicUsize::new(0));
        let peak = Arc::new(AtomicUsize::new(0));
        let seen = Arc::new(Mutex::new(Vec::new()));
        let backends = config
            .agents
            .iter()
            .zip(delays)
            .map(|(a, delay)| {
                (
                    a.handle.clone(),
                    Arc::new(Probe {
                        active: active.clone(),
                        peak: peak.clone(),
                        seen: seen.clone(),
                        delay,
                    }) as Arc<dyn Backend>,
                )
            })
            .collect();
        let session = run_with(&config, backends).await.unwrap();
        assert_eq!(active.load(Ordering::SeqCst), 0);
        (session, peak.load(Ordering::SeqCst))
    }

    #[tokio::test]
    async fn reordered_workers_share_frozen_snapshot() {
        let (a, _) = probe_run([20, 1, 10]).await;
        let (b, _) = probe_run([1, 20, 10]).await;
        assert_eq!(a.public, b.public);
        assert_eq!(a.selections, b.selections);
        for (x, y) in a.calls.iter().zip(&b.calls) {
            assert_eq!(x.observation, y.observation);
        }
        assert!(
            a.calls[..3]
                .iter()
                .all(|c| c.observation.public.messages.is_empty())
        );
        assert!(
            a.calls[3..]
                .iter()
                .all(|c| c.observation.public.messages.len() == 3)
        );
    }

    #[tokio::test]
    async fn concurrency_cap_and_ballot_barrier() {
        let (session, peak) = probe_run([15, 15, 15]).await;
        assert_eq!(peak, 2);
        let selections = &session.calls[3..];
        assert!(
            selections
                .windows(2)
                .all(|c| c[0].observation.public == c[1].observation.public)
        );
        assert_eq!(session.calls.len(), 6);
    }

    #[tokio::test(start_paused = true)]
    async fn invalid_timeout_failure_are_not_abstention() {
        let mut config = config();
        config.rounds = 1;
        config.communication_steps = 0;
        let cases = [
            (
                Some(r#"{"partner":null}"#.into()),
                0,
                false,
                Status::Abstained,
            ),
            (
                Some(r#"{"partner":"a"}"#.into()),
                0,
                false,
                Status::InvalidTarget,
            ),
            (
                Some(r#"{"partner":"absent"}"#.into()),
                0,
                false,
                Status::InvalidTarget,
            ),
            (Some("bad JSON".into()), 0, false, Status::InvalidResponse),
            (Some("x".repeat(4097)), 0, false, Status::OversizedResponse),
            (
                Some(r#"{"partner":null}"#.into()),
                101,
                false,
                Status::Timeout,
            ),
            (None, 0, true, Status::TransportFailure),
            (
                Some(r#"{"consent":{"state":"decline"}}"#.into()),
                0,
                false,
                Status::InvalidResponse,
            ),
        ];
        for (body, delay_ms, transport_failure, expected) in cases {
            config.agents[0].backend = BackendConfig::Fixture {
                responses: vec![FixtureResponse {
                    round: 0,
                    phase: Phase::Selection,
                    delay_ms,
                    body,
                    transport_failure,
                }],
            };
            let session = run(&config).await.unwrap();
            assert_eq!(session.calls[0].outcome.status, expected);
            assert_eq!(session.calls.len(), 3);
        }
        config.agents[0].backend=BackendConfig::Fixture {responses:vec![FixtureResponse {round:0,phase:Phase::Selection,delay_ms:0,transport_failure:false,body:Some(serde_json::json!({"partner":config.alias("b"),"consent":{"state":"agree","plan_id":"wrong","fingerprint":"fake"}}).to_string())}]};
        let session = run(&config).await.unwrap();
        assert_eq!(session.calls[0].outcome.status, Status::Valid);
        assert_eq!(
            session.calls[0].outcome.ballot.as_ref().unwrap().consent,
            crate::protocol::Consent::Invalid
        );
    }

    #[tokio::test(start_paused = true)]
    async fn all_abstain_and_all_fail_runs_terminate() {
        let mut config = config();
        let failed = run(&config).await.unwrap();
        assert_eq!(failed.calls.len(), 12);
        assert_eq!(failed.private_states.len(), 3);
        for a in &mut config.agents {
            a.backend = BackendConfig::Fixture {
                responses: (0..2)
                    .flat_map(|round| {
                        [
                            FixtureResponse {
                                round,
                                phase: Phase::Communication { step: 0 },
                                delay_ms: 0,
                                body: Some("{}".into()),
                                transport_failure: false,
                            },
                            FixtureResponse {
                                round,
                                phase: Phase::Selection,
                                delay_ms: 0,
                                body: Some(r#"{"partner":null}"#.into()),
                                transport_failure: false,
                            },
                        ]
                    })
                    .collect(),
            };
        }
        let session = run(&config).await.unwrap();
        assert_eq!(session.selections.len(), 2);
        assert!(
            session
                .selections
                .iter()
                .flat_map(|r| r.outcomes.values())
                .all(|o| o.status == Status::Abstained)
        );
        config.max_context_bytes = 1024;
        assert!(
            run(&config)
                .await
                .unwrap()
                .calls
                .iter()
                .all(|c| c.outcome.status == Status::ContextLimit)
        );
    }

    #[tokio::test]
    async fn invalid_configuration_makes_zero_backend_calls() {
        struct Counter(Arc<AtomicUsize>);
        impl Backend for Counter {
            fn count_tokens(&self, request: Request) -> crate::backend::TokenFuture {
                Box::pin(async move { Ok(crate::backend::fixture_tokens(&request.observation)) })
            }
            fn respond(&self, _: Request) -> ResponseFuture {
                self.0.fetch_add(1, Ordering::SeqCst);
                Box::pin(async { Ok("{}".into()) })
            }
        }
        let calls = Arc::new(AtomicUsize::new(0));
        for case in 0..6 {
            let mut config = config();
            match case {
                0 => config.workers = 0,
                1 => config.agents[1].handle = "a".into(),
                2 => config.rounds = 0,
                3 => {
                    config.agents[0].backend = BackendConfig::LocalHttp {
                        endpoint: "http://example.com/".into(),
                        model: "x".into(),
                        tokenizer: crate::config::TokenizerConfig::Vllm {
                            endpoint: "http://127.0.0.1/tokenize".into(),
                        },
                    }
                }
                4 => config.recipes[0].density = Some(2.0),
                _ => config.plans[0].children.clear(),
            }
            let backends = config
                .agents
                .iter()
                .map(|a| {
                    (
                        a.handle.clone(),
                        Arc::new(Counter(calls.clone())) as Arc<dyn Backend>,
                    )
                })
                .collect();
            assert!(run_with(&config, backends).await.is_err());
            assert_eq!(calls.load(Ordering::SeqCst), 0);
        }
    }

    struct BudgetProbe {
        mode: u8,
        generations: Arc<AtomicUsize>,
    }
    impl Backend for BudgetProbe {
        fn count_tokens(&self, request: Request) -> crate::backend::TokenFuture {
            let mode = self.mode;
            Box::pin(async move {
                if mode == 1 {
                    return Err(Status::TokenizationFailure);
                }
                if mode == 2 {
                    tokio::time::sleep(Duration::from_millis(101)).await;
                }
                Ok(TokenCount {
                    prompt_tokens: if mode == 3 {
                        4000
                    } else {
                        2000 + request.observation.event_count() as u32 * 1000
                    },
                    model_capacity: Some(4096),
                })
            })
        }
        fn respond(&self, request: Request) -> ResponseFuture {
            self.generations.fetch_add(1, Ordering::SeqCst);
            Box::pin(async move {
                Ok(if request.observation.phase == Phase::Selection {
                    r#"{"partner":null,"consent":{"state":"defer"}}"#.into()
                } else {
                    r#"{"public_message":"hello","private_update":{"learned_preference":"persistent"}}"#.into()
                })
            })
        }
    }

    #[tokio::test(start_paused = true)]
    async fn token_budget_trims_history_and_replays_without_tokenizer() {
        let mut config = config();
        config.rounds = 6;
        config.communication_steps = 2;
        config.memory.max_prompt_tokens = 3500;
        config.memory.context_window_tokens = 4096;
        for agent in &mut config.agents {
            agent.backend = BackendConfig::LocalHttp {
                endpoint: "http://127.0.0.1:1/chat".into(),
                model: "test".into(),
                tokenizer: crate::config::TokenizerConfig::Vllm {
                    endpoint: "http://127.0.0.1:1/tokenize".into(),
                },
            };
        }
        for mode in 0..4 {
            let generations = Arc::new(AtomicUsize::new(0));
            let backends = config
                .agents
                .iter()
                .map(|a| {
                    (
                        a.handle.clone(),
                        Arc::new(BudgetProbe {
                            mode,
                            generations: generations.clone(),
                        }) as Arc<dyn Backend>,
                    )
                })
                .collect();
            let session = run_with(&config, backends).await.unwrap();
            if mode == 0 {
                assert_eq!(generations.load(Ordering::SeqCst), 54);
                assert_eq!(session.public.messages.len(), 36);
                assert!(
                    session
                        .calls
                        .iter()
                        .all(|c| c.observation.event_count() <= 1)
                );
                assert!(session.calls.iter().any(|c| c.token_trials.len() > 1));
                assert!(
                    session
                        .private_states
                        .values()
                        .all(|s| s.learned_preference.as_deref() == Some("persistent"))
                );
            } else {
                assert_eq!(generations.load(Ordering::SeqCst), 0);
                let status = match mode {
                    1 => Status::TokenizationFailure,
                    2 => Status::Timeout,
                    _ => Status::ContextLimit,
                };
                assert!(session.calls.iter().all(|c| c.outcome.status == status));
            }
            let record = crate::record::Record::from_session(config.clone(), session, 1, 2);
            crate::replay::reconstruct(&record).unwrap();
        }
    }
}
