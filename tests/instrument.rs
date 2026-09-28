use lovers_lagoon::{
    backend::{self, Backend, Request, ResponseFuture, TokenFuture},
    config::{BackendConfig, Experiment, FixtureResponse, call_seed, fingerprint},
    observation::Observation,
    protocol::{Phase, PrivateState, PublicView, Status, normalize},
    record::Record,
    replay, runner,
};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

fn config() -> Experiment {
    serde_json::from_str(include_str!("../examples/fixture-experiment.json")).unwrap()
}

fn exit_config() -> Experiment {
    let mut config = config();
    config.pairing = lovers_lagoon::config::PairingProtocol::MatchedExit;
    config.rounds = 3;
    let mut fourth = config.agents[0].clone();
    fourth.handle = "d".into();
    config.agents.push(fourth);
    let aliases: BTreeMap<_, _> = config
        .agents
        .iter()
        .map(|agent| (agent.handle.clone(), config.alias(&agent.handle)))
        .collect();
    for agent in &mut config.agents {
        agent.initial_private.thought = Some(format!("preserve-{}", agent.handle));
        let partner = match agent.handle.as_str() {
            "a" => "b",
            "b" => "a",
            "c" => "d",
            _ => "c",
        };
        let nomination_round = if matches!(agent.handle.as_str(), "a" | "b") {
            0
        } else {
            1
        };
        agent.backend = BackendConfig::Fixture {responses:(0..3).flat_map(|round|[
            FixtureResponse {round,phase:Phase::Communication{step:0},delay_ms:0,body:Some("{}".into()),transport_failure:false},
            FixtureResponse {round,phase:Phase::Selection,delay_ms:0,body:Some(serde_json::json!({
                "partner":if round==nomination_round {Some(&aliases[partner])} else {None},
                "consent":{"state":"decline"}
            }).to_string()),transport_failure:false}
        ]).collect()};
    }
    config
}

#[tokio::test]
async fn matched_exit_replay_and_counts() {
    let config = exit_config();
    let session = runner::run(&config).await.unwrap();
    assert_eq!(session.calls.len(), 12);
    assert_eq!(session.batches.len(), 2);
    assert!(session.active_handles.is_empty());
    assert_eq!(session.private_states.len(), 4);
    assert!(session.public.pairs.iter().all(|pair| pair.retired));
    for agent in &config.agents {
        assert_eq!(
            session.private_states[&agent.handle].thought.as_deref(),
            Some(format!("preserve-{}", agent.handle).as_str())
        );
    }
    for call in session
        .calls
        .iter()
        .filter(|call| call.observation.round == 1)
    {
        assert!(matches!(call.handle.as_str(), "c" | "d"));
        assert_eq!(call.observation.handles.len(), 1);
        assert_eq!(call.observation.public.pairs.len(), 1);
        if call.observation.phase == Phase::Selection {
            assert_eq!(call.observation.response_format()["json_schema"]["schema"]["properties"]["partner"]["enum"].as_array().unwrap().len(),2);
        }
    }
    let record = Record::from_session(config, session, 0, 1);
    assert_eq!(record.report.total.decision_slots, 6);
    assert_eq!(
        record
            .report
            .per_round
            .iter()
            .map(|r| r.counts.decision_slots)
            .collect::<Vec<_>>(),
        vec![4, 2]
    );
    assert_eq!(record.report.unique_paired_participants, 4);
    assert_eq!(record.report.unique_paired_fraction_initial, 1.0);
    assert_eq!(record.report.total.blocked_pairs, 2);
    replay::reconstruct(&record).unwrap();
}

#[tokio::test]
async fn matched_exit_singleton_and_empty() {
    let mut config = config();
    config.pairing = lovers_lagoon::config::PairingProtocol::MatchedExit;
    let session = runner::run(&config).await.unwrap();
    assert_eq!(session.active_handles.len(), 1);
    assert_eq!(session.calls.len(), config.agents.len() * 2);
    let record = Record::from_session(config.clone(), session, 0, 1);
    replay::reconstruct(&record).unwrap();
    config.agents.truncate(1);
    let session = runner::run(&config).await.unwrap();
    assert!(session.calls.is_empty() && session.selections.is_empty());
    let record = Record::from_session(config, session, 0, 1);
    assert_eq!(record.report.total.decision_slots, 0);
    assert_eq!(record.report.total.paired_fraction_of_decision_slots, None);
    assert_eq!(record.report.total.voluntary_abstentions, 0);
    replay::reconstruct(&record).unwrap();
    let config = exit_config();
    let session = runner::run(&config).await.unwrap();
    assert!(session.active_handles.is_empty());
    assert_eq!(session.selections.len(), 2);
}

#[tokio::test]
async fn matched_exit_record_tampering() {
    let config = exit_config();
    let session = runner::run(&config).await.unwrap();
    let record = Record::from_session(config, session, 0, 1);
    let mut changed = record.clone();
    changed.session.calls.pop();
    assert!(replay::reconstruct(&changed).is_err());
    let mut changed = record.clone();
    changed.session.calls.push(changed.session.calls[0].clone());
    assert!(replay::reconstruct(&changed).is_err());
    let mut changed = record.clone();
    changed.session.public.pairs[0].retired = false;
    assert!(replay::reconstruct(&changed).is_err());
    let mut changed = record;
    changed.session.active_handles.insert("a".into());
    assert!(replay::reconstruct(&changed).is_err());
}

struct SeedProbe(Arc<Mutex<Vec<(String, u64)>>>);
impl Backend for SeedProbe {
    fn count_tokens(&self, request: Request) -> TokenFuture {
        Box::pin(async move { Ok(backend::fixture_tokens(&request.observation)) })
    }
    fn respond(&self, request: Request) -> ResponseFuture {
        let seen = self.0.clone();
        Box::pin(async move {
            seen.lock()
                .unwrap()
                .push((request.observation.owner, request.seed));
            Ok(r#"{"partner":null,"consent":{"state":"defer"}}"#.into())
        })
    }
}

#[tokio::test]
async fn call_seed_receipts_and_tampering() {
    // Independently calculated with Python hashlib over the documented UTF-8 tuple.
    assert_eq!(
        call_seed(9, 0, &Phase::Communication { step: 0 }, "a"),
        3361834622662243581
    );
    let mut config = config();
    config.communication_steps = 0;
    for agent in &mut config.agents {
        agent.backend = BackendConfig::Fixture { responses: vec![] };
    }
    config.decoding.temperature = 0.7;
    let seen = Arc::new(Mutex::new(Vec::new()));
    let backends = config
        .agents
        .iter()
        .map(|agent| {
            (
                agent.handle.clone(),
                Arc::new(SeedProbe(seen.clone())) as Arc<dyn Backend>,
            )
        })
        .collect();
    let session = runner::run_with(&config, backends).await.unwrap();
    for call in &session.calls {
        assert!(
            seen.lock()
                .unwrap()
                .contains(&(call.observation.owner.clone(), call.seed))
        );
        assert_eq!(
            call.seed,
            call_seed(
                config.seed,
                call.observation.round,
                &call.observation.phase,
                &call.handle
            )
        );
    }
    let unique: std::collections::BTreeSet<_> =
        session.calls.iter().map(|call| call.seed).collect();
    assert_eq!(unique.len(), session.calls.len());
    let phases = [
        Phase::Communication { step: 0 },
        Phase::Communication { step: 1 },
        Phase::Selection,
    ];
    let phase_seeds: std::collections::BTreeSet<_> = phases
        .iter()
        .map(|phase| call_seed(9, 0, phase, "a"))
        .collect();
    assert_eq!(phase_seeds.len(), phases.len());
    let mut record = Record::from_session(config, session, 0, 1);
    replay::reconstruct(&record).unwrap();
    record.session.calls[0].seed ^= 1;
    assert!(replay::reconstruct(&record).is_err());
}

#[test]
fn configured_unicode_limits_and_retention() {
    let mut config = config();
    config.string_limits.communication.public_message_chars = 256;
    config.string_limits.communication.private_note_chars = 100;
    config.string_limits.selection.private_note_chars = 20;
    config.validate().unwrap();
    let obs = Observation::new(
        &config,
        "a",
        &PrivateState::default(),
        &PublicView::default(),
        0,
        Phase::Communication { step: 0 },
    );
    assert_eq!(
        obs.response_format()["json_schema"]["schema"]["properties"]["public_message"]["maxLength"],
        256
    );
    let body = serde_json::json!({"public_message":"🙂".repeat(256),"private_update":{"thought":"x".repeat(100)}});
    let outcome = normalize(&body.to_string(), &obs, &config);
    assert_eq!(outcome.status, Status::Valid);
    let too_long = serde_json::json!({"public_message":"🙂".repeat(257)});
    assert_eq!(
        normalize(&too_long.to_string(), &obs, &config).status,
        Status::OversizedResponse
    );
    let state = outcome.private_update.unwrap();
    let selection = Observation::new(
        &config,
        "a",
        &state,
        &PublicView::default(),
        0,
        Phase::Selection,
    );
    assert_eq!(
        normalize(r#"{"partner":null}"#, &selection, &config).status,
        Status::Abstained
    );
    let invalid = serde_json::json!({"partner":null,"private_update":{"thought":"x".repeat(21)}});
    assert_eq!(
        normalize(&invalid.to_string(), &selection, &config).status,
        Status::OversizedResponse
    );
    let old_hash = fingerprint(&obs.response_format());
    config.string_limits.communication.public_message_chars = 255;
    let changed = Observation::new(
        &config,
        "a",
        &PrivateState::default(),
        &PublicView::default(),
        0,
        Phase::Communication { step: 0 },
    );
    assert_ne!(old_hash, fingerprint(&changed.response_format()));
    config.string_limits.selection.private_note_chars = 0;
    assert!(config.validate().is_err());
}

#[tokio::test]
async fn peer_ledger_privacy_and_replay() {
    let mut config = config();
    config.peer_memory = true;
    let peer = config.alias("b");
    let owner = config.alias("a");
    let body = serde_json::json!({"private_update":{"peers":{peer.clone():{"trust":8,"note":"LEDGER_PRIVATE_CANARY"}}}}).to_string();
    config.agents[0].backend = BackendConfig::Fixture {
        responses: vec![FixtureResponse {
            round: 0,
            phase: Phase::Communication { step: 0 },
            delay_ms: 0,
            body: Some(body.clone()),
            transport_failure: false,
        }],
    };
    let session = runner::run(&config).await.unwrap();
    assert_eq!(
        session.private_states["a"].peers.as_ref().unwrap()[&peer].trust,
        8
    );
    assert!(
        !serde_json::to_string(&session.public)
            .unwrap()
            .contains("LEDGER_PRIVATE_CANARY")
    );
    assert!(
        session
            .calls
            .iter()
            .filter(|call| call.handle != "a")
            .all(|call| !call
                .observation
                .messages()
                .to_string()
                .contains("LEDGER_PRIVATE_CANARY"))
    );
    let obs = &session
        .calls
        .iter()
        .find(|call| call.handle == "a" && call.observation.phase == Phase::Selection)
        .unwrap()
        .observation;
    assert!(obs.messages().to_string().contains("LEDGER_PRIVATE_CANARY"));
    let schema = obs.response_format();
    let peer_schema = &schema["json_schema"]["schema"]["properties"]["private_update"]["anyOf"][1]
        ["properties"]["peers"];
    assert_eq!(
        peer_schema["properties"][&peer]["properties"]["trust"]["maximum"],
        10
    );
    assert!(peer_schema["properties"].get(&owner).is_none());
    for (alias, trust, note) in [
        (owner, 8, "ok".into()),
        (peer.clone(), 11, "ok".into()),
        (peer.clone(), 8, "x".repeat(65)),
        ("unknown".into(), 8, "ok".into()),
    ] {
        let invalid = serde_json::json!({"partner":null,"private_update":{"peers":{alias:{"trust":trust,"note":note}}}});
        assert_eq!(
            normalize(&invalid.to_string(), obs, &config).status,
            Status::InvalidResponse
        );
    }
    let cleared = normalize(
        r#"{"partner":null,"private_update":{"peers":{}}}"#,
        obs,
        &config,
    );
    let mut state = obs.private_state.clone();
    state.update(cleared.private_update.as_ref().unwrap());
    assert_eq!(state.peers, Some(BTreeMap::new()));
    let record = Record::from_session(config.clone(), session, 0, 1);
    replay::reconstruct(&record).unwrap();
    config.peer_memory = false;
    let obs = Observation::new(
        &config,
        "a",
        &PrivateState::default(),
        &PublicView::default(),
        0,
        Phase::Communication { step: 0 },
    );
    assert_eq!(
        normalize(&body, &obs, &config).status,
        Status::InvalidResponse
    );
}
