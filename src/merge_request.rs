use crate::{
    config::{Checkpoint, Experiment, MergeMetadata, Mode, Recipe, ReproductionPlan},
    matching::reciprocal_pairs,
    protocol::{Consent, Pair},
    runner::SelectionRound,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestStatus {
    PendingUnverified,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockReason {
    MissingConsent,
    InvalidConsent,
    Declined,
    Deferred,
    PlanDisagreement,
    MissingCheckpoint,
    MissingMergeMetadata,
    BaseMismatch,
    ArchitectureMismatch,
    TensorLayoutMismatch,
    TokenizerMismatch,
    LicenseMismatch,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChildRequest {
    pub child_index: usize,
    pub parents: [Option<Checkpoint>; 2],
    pub base: Option<Checkpoint>,
    pub recipe: Recipe,
    pub seed: u64,
    pub status: RequestStatus,
    pub blocked_reasons: Vec<BlockReason>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchDecision {
    pub round: u32,
    pub pair: Pair,
    pub mode: Mode,
    pub consents: [Consent; 2],
    pub parents: [Option<Checkpoint>; 2],
    pub declared_metadata: [Option<MergeMetadata>; 2],
    pub plan: Option<ReproductionPlan>,
    pub artifacts_verified: bool,
    pub executed: bool,
    pub plan_id: Option<String>,
    pub plan_fingerprint: Option<String>,
    pub max_compute_seconds: Option<u64>,
    pub max_output_bytes: Option<u64>,
    pub status: RequestStatus,
    pub blocked_reasons: Vec<BlockReason>,
    pub children: Vec<ChildRequest>,
}

pub fn resolve_round(config: &Experiment, selection: &SelectionRound) -> Vec<BatchDecision> {
    reciprocal_pairs(&selection.outcomes)
        .into_iter()
        .map(|pair| {
            let consent: Vec<_> = pair
                .agents
                .iter()
                .map(|a| {
                    &selection.outcomes[a]
                        .ballot
                        .as_ref()
                        .expect("reciprocal ballot")
                        .consent
                })
                .collect();
            let mut reasons = Vec::new();
            for c in &consent {
                let reason = match c {
                    Consent::Missing => Some(BlockReason::MissingConsent),
                    Consent::Invalid => Some(BlockReason::InvalidConsent),
                    Consent::Decline => Some(BlockReason::Declined),
                    Consent::Defer => Some(BlockReason::Deferred),
                    Consent::Agree { .. } => None,
                };
                if let Some(reason) = reason
                    && !reasons.contains(&reason)
                {
                    reasons.push(reason);
                }
            }
            let plan = match (consent[0], consent[1]) {
                (
                    Consent::Agree {
                        plan_id: a,
                        fingerprint: x,
                    },
                    Consent::Agree {
                        plan_id: b,
                        fingerprint: y,
                    },
                ) if a == b && x == y => config
                    .plans
                    .iter()
                    .find(|p| &p.id == a && config.plan_fingerprint(p) == *x),
                (Consent::Agree { .. }, Consent::Agree { .. }) => {
                    reasons.push(BlockReason::PlanDisagreement);
                    None
                }
                _ => None,
            };
            if reasons.is_empty() && plan.is_none() {
                reasons.push(BlockReason::InvalidConsent);
            }
            let parents: Vec<_> = pair
                .agents
                .iter()
                .map(|handle| {
                    config
                        .agents
                        .iter()
                        .find(|a| &a.handle == handle)
                        .expect("known parent")
                })
                .collect();
            let mut base = None;
            if plan.is_some() {
                if parents.iter().any(|a| a.checkpoint.is_none()) {
                    reasons.push(BlockReason::MissingCheckpoint);
                }
                match (&parents[0].merge_metadata, &parents[1].merge_metadata) {
                    (Some(a), Some(b)) => {
                        if a.base != b.base {
                            reasons.push(BlockReason::BaseMismatch);
                        } else {
                            base = Some(a.base.clone());
                        }
                        for (same, reason) in [
                            (
                                a.architecture == b.architecture,
                                BlockReason::ArchitectureMismatch,
                            ),
                            (
                                a.tensor_layout == b.tensor_layout,
                                BlockReason::TensorLayoutMismatch,
                            ),
                            (a.tokenizer == b.tokenizer, BlockReason::TokenizerMismatch),
                            (a.license == b.license, BlockReason::LicenseMismatch),
                        ] {
                            if !same {
                                reasons.push(reason);
                            }
                        }
                    }
                    _ => reasons.push(BlockReason::MissingMergeMetadata),
                }
            }
            let status = if reasons.is_empty() {
                RequestStatus::PendingUnverified
            } else {
                RequestStatus::Blocked
            };
            BatchDecision {
                round: selection.round,
                pair,
                mode: config.mode.clone(),
                consents: [consent[0].clone(), consent[1].clone()],
                parents: [parents[0].checkpoint.clone(), parents[1].checkpoint.clone()],
                declared_metadata: [
                    parents[0].merge_metadata.clone(),
                    parents[1].merge_metadata.clone(),
                ],
                plan: plan.cloned(),
                artifacts_verified: false,
                executed: false,
                plan_id: plan.map(|p| p.id.clone()),
                plan_fingerprint: plan.map(|p| config.plan_fingerprint(p)),
                max_compute_seconds: plan.map(|p| p.max_compute_seconds),
                max_output_bytes: plan.map(|p| p.max_output_bytes),
                children: plan
                    .map(|p| {
                        p.children
                            .iter()
                            .enumerate()
                            .map(|(index, child)| ChildRequest {
                                child_index: index,
                                parents: [
                                    parents[0].checkpoint.clone(),
                                    parents[1].checkpoint.clone(),
                                ],
                                base: base.clone(),
                                recipe: config
                                    .recipes
                                    .iter()
                                    .find(|r| r.id == child.recipe_id)
                                    .expect("validated recipe")
                                    .clone(),
                                seed: child.seed,
                                status: status.clone(),
                                blocked_reasons: reasons.clone(),
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                status,
                blocked_reasons: reasons,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::{ChildRecipe, MergeMetadata, Method, tests::config},
        protocol::{Ballot, Outcome, Status},
    };

    fn paired(config: &Experiment) -> SelectionRound {
        SelectionRound {
            round: 0,
            outcomes: [("a", "b"), ("b", "a")]
                .into_iter()
                .map(|(a, b)| {
                    (
                        a.into(),
                        Outcome {
                            status: Status::Valid,
                            public_message: None,
                            private_update: None,
                            ballot: Some(Ballot {
                                partner: Some(b.into()),
                                consent: Consent::Agree {
                                    plan_id: config.plans[0].id.clone(),
                                    fingerprint: config.plan_fingerprint(&config.plans[0]),
                                },
                            }),
                        },
                    )
                })
                .collect(),
        }
    }

    fn compatible() -> Experiment {
        let mut value = config();
        for agent in &mut value.agents {
            agent.checkpoint = Some(Checkpoint {
                model: format!("fixture/{}", agent.handle),
                revision: "a".repeat(40),
            });
            agent.merge_metadata = Some(MergeMetadata {
                base: Checkpoint {
                    model: "fixture/base".into(),
                    revision: "b".repeat(40),
                },
                architecture: "fixture".into(),
                tensor_layout: "signature".into(),
                tokenizer: "signature".into(),
                license: "Apache-2.0".into(),
            });
        }
        value
    }

    #[test]
    fn incomplete_or_incompatible_metadata_blocks_requests() {
        let config = config();
        let decisions = resolve_round(&config, &paired(&config));
        assert_eq!(
            decisions[0].blocked_reasons,
            vec![
                BlockReason::MissingCheckpoint,
                BlockReason::MissingMergeMetadata
            ]
        );
        for (index, reason) in [
            BlockReason::BaseMismatch,
            BlockReason::ArchitectureMismatch,
            BlockReason::TensorLayoutMismatch,
            BlockReason::TokenizerMismatch,
            BlockReason::LicenseMismatch,
        ]
        .into_iter()
        .enumerate()
        {
            let mut config = compatible();
            let meta = config.agents[1].merge_metadata.as_mut().unwrap();
            match index {
                0 => meta.base.revision = "c".repeat(40),
                1 => meta.architecture = "different".into(),
                2 => meta.tensor_layout = "different".into(),
                3 => meta.tokenizer = "different".into(),
                _ => meta.license = "different".into(),
            }
            let decisions = resolve_round(&config, &paired(&config));
            assert_eq!(decisions[0].status, RequestStatus::Blocked);
            assert_eq!(decisions[0].blocked_reasons, vec![reason]);
            assert_eq!(decisions[0].children[0].status, RequestStatus::Blocked);
        }
    }

    #[test]
    fn compatible_metadata_creates_unverified_pending_request() {
        let config = compatible();
        let decisions = resolve_round(&config, &paired(&config));
        assert_eq!(decisions[0].status, RequestStatus::PendingUnverified);
        assert!(!decisions[0].artifacts_verified);
        assert!(!decisions[0].executed);
        assert_eq!(decisions[0].plan, Some(config.plans[0].clone()));
        assert_eq!(decisions[0].mode, config.mode);
        assert!(
            decisions[0]
                .consents
                .iter()
                .all(|c| matches!(c, Consent::Agree { .. }))
        );
        assert_eq!(
            decisions[0].children[0].parents[0],
            config.agents[0].checkpoint
        );
        assert_eq!(
            decisions[0].children[0].base,
            Some(
                config.agents[0]
                    .merge_metadata
                    .as_ref()
                    .unwrap()
                    .base
                    .clone()
            )
        );
        let json = serde_json::to_string(&decisions).unwrap();
        for forbidden in ["execution_timestamp", "output_checkpoint", "admitted_child"] {
            assert!(!json.contains(forbidden));
        }
    }

    #[test]
    fn mutual_plan_consent_is_required_without_fallback() {
        let config = compatible();
        for (consent, reason) in [
            (Consent::Decline, BlockReason::Declined),
            (Consent::Defer, BlockReason::Deferred),
            (Consent::Missing, BlockReason::MissingConsent),
            (Consent::Invalid, BlockReason::InvalidConsent),
            (
                Consent::Agree {
                    plan_id: "other".into(),
                    fingerprint: "other".into(),
                },
                BlockReason::PlanDisagreement,
            ),
        ] {
            let mut selection = paired(&config);
            selection
                .outcomes
                .get_mut("b")
                .unwrap()
                .ballot
                .as_mut()
                .unwrap()
                .consent = consent;
            let decisions = resolve_round(&config, &selection);
            assert_eq!(decisions.len(), 1);
            assert_eq!(decisions[0].blocked_reasons, vec![reason]);
            assert!(decisions[0].children.is_empty());
            assert_eq!(decisions[0].parents[0], config.agents[0].checkpoint);
            assert_eq!(
                decisions[0].consents[1],
                selection.outcomes["b"].ballot.as_ref().unwrap().consent
            );
            assert!(!decisions[0].artifacts_verified && !decisions[0].executed);
        }
        let mut selection = paired(&config);
        selection
            .outcomes
            .get_mut("b")
            .unwrap()
            .ballot
            .as_mut()
            .unwrap()
            .consent = Consent::Agree {
            plan_id: config.plans[0].id.clone(),
            fingerprint: "changed payload".into(),
        };
        assert_eq!(
            resolve_round(&config, &selection)[0].blocked_reasons,
            vec![BlockReason::PlanDisagreement]
        );
    }

    #[test]
    fn sibling_requests_preserve_parents_count_and_recipes() {
        let mut config = compatible();
        config.max_siblings = 2;
        let mut recipe = config.recipes[0].clone();
        recipe.id = "dare".into();
        recipe.method = Method::DareTies;
        config.recipes.push(recipe);
        config.plans[0].children.push(ChildRecipe {
            recipe_id: "dare".into(),
            seed: 42,
        });
        config.validate().unwrap();
        let decisions = resolve_round(&config, &paired(&config));
        let children = &decisions[0].children;
        assert_eq!(decisions.len(), 1);
        assert_eq!(children.len(), 2);
        assert_eq!(children[0].parents, children[1].parents);
        assert_eq!(children[0].base, children[1].base);
        assert_eq!(children[0].recipe.method, Method::Ties);
        assert_eq!(children[1].recipe.method, Method::DareTies);
        assert_eq!(children[1].seed, 42);
        let mut session = crate::runner::Session::initial(&config);
        session.selections.push(paired(&config));
        session.batches = decisions;
        let report = crate::report::describe(&config, &session);
        assert_eq!(report.total.social_pairs, 1);
        assert_eq!(report.total.agreed_plan_batches, 1);
        assert_eq!(report.total.pending_batches, 1);
        assert_eq!(report.total.pending_child_requests, 2);
        assert_eq!(report.total.executed_merges, 0);
        assert_eq!(report.total.admitted_children, 0);
    }

    #[tokio::test]
    async fn abstainers_and_unmatched_agents_keep_memory() {
        use crate::config::{BackendConfig, FixtureResponse};
        use crate::protocol::Phase;
        let mut config = config();
        for (index, agent) in config.agents.iter_mut().enumerate() {
            agent.initial_private.thought = Some(format!("private-{index}"));
            let partner = match index {
                0 => Some("p-c1bc7d533fa54f3f"),
                1 => Some("p-809a715ff182bbd5"),
                _ => None,
            };
            agent.backend = BackendConfig::Fixture {
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
                                body: Some(serde_json::json!({"partner":partner}).to_string()),
                                transport_failure: false,
                            },
                        ]
                    })
                    .collect(),
            };
        }
        let session = crate::runner::run(&config).await.unwrap();
        assert_eq!(session.private_states.len(), 3);
        for agent in &config.agents {
            assert_eq!(session.private_states[&agent.handle], agent.initial_private);
        }
        assert_eq!(session.public.pairs.len(), 2);
        assert_eq!(
            session.batches[0].blocked_reasons,
            vec![BlockReason::MissingConsent]
        );
        assert!(
            session.calls[6..9]
                .iter()
                .all(|c| c.observation.public.pairs.len() == 1)
        );
        assert_eq!(
            session.selections[0].outcomes["c"].status,
            Status::Abstained
        );
    }
}
