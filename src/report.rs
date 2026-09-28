use crate::{
    config::{Experiment, Mode},
    merge_request::RequestStatus,
    protocol::{Consent, Status},
    runner::Session,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Counts {
    pub decision_slots: u64,
    pub valid_nominations: u64,
    pub nonreciprocal_nominations: u64,
    pub voluntary_abstentions: u64,
    pub failed_decisions: u64,
    pub social_pairs: u64,
    pub paired_fraction_of_decision_slots: Option<f64>,
    pub agreed_plan_batches: u64,
    pub pending_batches: u64,
    pub blocked_batches: u64,
    pub blocked_pairs: u64,
    pub consented_child_jobs: u64,
    pub pending_child_requests: u64,
    pub blocked_child_jobs: u64,
    pub executed_merges: u64,
    pub admitted_children: u64,
    pub selection_status_counts: BTreeMap<String, u64>,
    pub consent_counts: BTreeMap<String, u64>,
    pub blocked_reason_counts: BTreeMap<String, u64>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoundReport {
    pub round: u32,
    pub counts: Counts,
    pub baseline: Baseline,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Report {
    pub initial_population: usize,
    pub pairing: crate::config::PairingProtocol,
    pub remaining_eligible: Vec<String>,
    pub unique_paired_participants: usize,
    pub unique_paired_fraction_initial: f64,
    pub configured_rounds: u32,
    pub mode: Mode,
    pub offered_plan_ids_in_order: Vec<String>,
    pub total: Counts,
    pub per_round: Vec<RoundReport>,
    pub all_call_status_counts: BTreeMap<String, u64>,
    pub finish_reason_counts: BTreeMap<String, u64>,
    pub fenced_json_responses: u64,
    pub presentation: Vec<PresentationCounts>,
    pub baseline: Baseline,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PresentationCounts {
    pub round: u32,
    pub kind: String,
    pub id: String,
    pub position: usize,
    pub exposures: u64,
    pub choices: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Baseline {
    pub expected_pairs_per_round: f64,
    pub expected_matched_fraction_per_round: Option<f64>,
    pub assumptions: String,
}

pub fn random_baseline(population: usize) -> Baseline {
    let expected = if population < 2 {
        0.0
    } else {
        population as f64 / (2.0 * (population - 1) as f64)
    };
    Baseline {expected_pairs_per_round:expected,expected_matched_fraction_per_round:if population==0{None}else{Some(2.0*expected/population as f64)},assumptions:"Independent uniform nominations of one other agent; no abstention; reciprocal pairs only. Descriptive reference, not a causal control.".into()}
}

fn key<T: Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .expect("enum")
        .as_str()
        .expect("string enum")
        .into()
}
fn count(map: &mut BTreeMap<String, u64>, name: String) {
    *map.entry(name).or_default() += 1;
}

pub fn describe(config: &Experiment, session: &Session) -> Report {
    let mut total = Counts::default();
    let mut per_round = Vec::new();
    for selection in &session.selections {
        let mut counts = Counts {
            decision_slots: selection.outcomes.len() as u64,
            ..Default::default()
        };
        for outcome in selection.outcomes.values() {
            count(&mut counts.selection_status_counts, key(&outcome.status));
            match outcome.status {
                Status::Valid => counts.valid_nominations += 1,
                Status::Abstained => counts.voluntary_abstentions += 1,
                _ => counts.failed_decisions += 1,
            }
            if let Some(ballot) = &outcome.ballot {
                let name = match ballot.consent {
                    Consent::Agree { .. } => "agree",
                    Consent::Decline => "decline",
                    Consent::Defer => "defer",
                    Consent::Missing => "missing",
                    Consent::Invalid => "invalid",
                };
                count(&mut counts.consent_counts, name.into());
            }
        }
        for batch in session
            .batches
            .iter()
            .filter(|b| b.round == selection.round)
        {
            counts.social_pairs += 1;
            if batch.plan_id.is_some() {
                counts.agreed_plan_batches += 1;
                if batch.status == RequestStatus::Blocked {
                    counts.blocked_batches += 1;
                }
            }
            if batch.status == RequestStatus::PendingUnverified {
                counts.pending_batches += 1;
            } else {
                counts.blocked_pairs += 1;
            }
            counts.consented_child_jobs += batch.children.len() as u64;
            for child in &batch.children {
                if child.status == RequestStatus::PendingUnverified {
                    counts.pending_child_requests += 1;
                } else {
                    counts.blocked_child_jobs += 1;
                }
            }
            for reason in &batch.blocked_reasons {
                count(&mut counts.blocked_reason_counts, key(reason));
            }
        }
        counts.nonreciprocal_nominations = counts.valid_nominations - 2 * counts.social_pairs;
        counts.paired_fraction_of_decision_slots = (counts.decision_slots > 0)
            .then(|| 2.0 * counts.social_pairs as f64 / counts.decision_slots as f64);
        total.decision_slots += counts.decision_slots;
        total.valid_nominations += counts.valid_nominations;
        total.nonreciprocal_nominations += counts.nonreciprocal_nominations;
        total.voluntary_abstentions += counts.voluntary_abstentions;
        total.failed_decisions += counts.failed_decisions;
        total.social_pairs += counts.social_pairs;
        total.agreed_plan_batches += counts.agreed_plan_batches;
        total.pending_batches += counts.pending_batches;
        total.blocked_batches += counts.blocked_batches;
        total.blocked_pairs += counts.blocked_pairs;
        total.consented_child_jobs += counts.consented_child_jobs;
        total.pending_child_requests += counts.pending_child_requests;
        total.blocked_child_jobs += counts.blocked_child_jobs;
        for (target, source) in [
            (
                &mut total.selection_status_counts,
                &counts.selection_status_counts,
            ),
            (&mut total.consent_counts, &counts.consent_counts),
            (
                &mut total.blocked_reason_counts,
                &counts.blocked_reason_counts,
            ),
        ] {
            for (key, value) in source {
                *target.entry(key.clone()).or_default() += value;
            }
        }
        per_round.push(RoundReport {
            round: selection.round,
            baseline: random_baseline(counts.decision_slots as usize),
            counts,
        });
    }
    total.paired_fraction_of_decision_slots = if total.decision_slots == 0 {
        None
    } else {
        Some(2.0 * total.social_pairs as f64 / total.decision_slots as f64)
    };
    let mut all_call_status_counts = BTreeMap::new();
    let mut finish_reason_counts = BTreeMap::new();
    let mut presentation: BTreeMap<(u32, String, String, usize), PresentationCounts> =
        BTreeMap::new();
    let mut fenced_json_responses = 0;
    for call in &session.calls {
        count(&mut all_call_status_counts, key(&call.outcome.status));
        if let crate::runner::Reply::Response {
            body,
            finish_reason,
        } = &call.reply
        {
            count(
                &mut finish_reason_counts,
                finish_reason.as_deref().unwrap_or("unreported").into(),
            );
            if crate::protocol::json_body(body) != body.trim() {
                fenced_json_responses += 1;
            }
        }
        if call.observation.phase != crate::protocol::Phase::Selection {
            continue;
        }
        let ballot = call.outcome.ballot.as_ref();
        let partner = ballot
            .and_then(|b| b.partner.as_deref())
            .map(|h| config.alias(h));
        let plan = ballot.and_then(|b| match &b.consent {
            Consent::Agree { plan_id, .. } => Some(plan_id.as_str()),
            _ => None,
        });
        for (kind, ids, selected) in [
            (
                "partner",
                call.observation.handles.clone(),
                partner.as_deref(),
            ),
            (
                "plan",
                call.observation
                    .plans
                    .iter()
                    .map(|p| p.plan.id.clone())
                    .collect(),
                plan,
            ),
        ] {
            for (position, id) in ids.into_iter().enumerate() {
                let entry = presentation
                    .entry((call.observation.round, kind.into(), id.clone(), position))
                    .or_insert_with(|| PresentationCounts {
                        round: call.observation.round,
                        kind: kind.into(),
                        id: id.clone(),
                        position,
                        ..Default::default()
                    });
                entry.exposures += 1;
                entry.choices += u64::from(selected == Some(id.as_str()));
            }
        }
    }
    let unique_paired_participants = session
        .batches
        .iter()
        .flat_map(|batch| batch.pair.agents.iter())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    Report {
        initial_population: config.agents.len(),
        pairing: config.pairing,
        remaining_eligible: session
            .active_handles
            .iter()
            .map(|handle| config.alias(handle))
            .collect(),
        unique_paired_participants,
        unique_paired_fraction_initial: unique_paired_participants as f64
            / config.agents.len() as f64,
        configured_rounds: config.rounds,
        mode: config.mode.clone(),
        offered_plan_ids_in_order: config.plans.iter().map(|p| p.id.clone()).collect(),
        total,
        per_round,
        all_call_status_counts,
        finish_reason_counts,
        fenced_json_responses,
        presentation: presentation.into_values().collect(),
        baseline: random_baseline(config.agents.len()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::{BackendConfig, FixtureResponse, tests::config},
        matching::reciprocal_pairs,
        protocol::{Ballot, Outcome, Phase},
    };

    #[tokio::test]
    async fn reports_keep_denominators_and_failure_categories() {
        let mut config = config();
        config.rounds = 1;
        config.communication_steps = 0;
        for (a, body) in config.agents.iter_mut().zip([
            r#"{"partner":"p-c1bc7d533fa54f3f","consent":{"state":"decline"}}"#,
            r#"{"partner":"p-809a715ff182bbd5","consent":{"state":"defer"}}"#,
            r#"{"partner":null}"#,
        ]) {
            a.backend = BackendConfig::Fixture {
                responses: vec![FixtureResponse {
                    round: 0,
                    phase: Phase::Selection,
                    delay_ms: 0,
                    body: Some(body.into()),
                    transport_failure: false,
                }],
            };
        }
        let session = crate::runner::run(&config).await.unwrap();
        let report = describe(&config, &session);
        assert_eq!(report.total.decision_slots, 3);
        assert_eq!(report.total.social_pairs, 1);
        assert_eq!(
            report.total.paired_fraction_of_decision_slots,
            Some(2.0 / 3.0)
        );
        assert_eq!(report.total.voluntary_abstentions, 1);
        assert_eq!(report.total.failed_decisions, 0);
        assert_eq!(report.total.nonreciprocal_nominations, 0);
        assert_eq!(report.total.consent_counts["decline"], 1);
        assert_eq!(report.total.consent_counts["defer"], 1);
        assert_eq!(report.total.blocked_pairs, 1);
        assert_eq!(report.total.pending_child_requests, 0);
        if let BackendConfig::Fixture { responses } = &mut config.agents[2].backend {
            responses[0].body =
                Some(r#"{"partner":"p-809a715ff182bbd5","consent":{"state":"defer"}}"#.into());
        }
        let nonreciprocal = describe(&config, &crate::runner::run(&config).await.unwrap());
        assert_eq!(nonreciprocal.total.valid_nominations, 3);
        assert_eq!(nonreciprocal.total.nonreciprocal_nominations, 1);
        assert_eq!(nonreciprocal.total.voluntary_abstentions, 0);
        let fingerprint = config.plan_fingerprint(&config.plans[0]);
        for (a, partner) in config.agents[..2]
            .iter_mut()
            .zip(["p-c1bc7d533fa54f3f", "p-809a715ff182bbd5"])
        {
            if let BackendConfig::Fixture { responses } = &mut a.backend {
                responses[0].body=Some(serde_json::json!({"partner":partner,"consent":{"state":"agree","plan_id":"single","fingerprint":fingerprint}}).to_string());
            }
        }
        let metadata_blocked = describe(&config, &crate::runner::run(&config).await.unwrap());
        assert_eq!(metadata_blocked.total.agreed_plan_batches, 1);
        assert_eq!(metadata_blocked.total.blocked_batches, 1);
        assert_eq!(metadata_blocked.total.consented_child_jobs, 1);
        assert_eq!(metadata_blocked.total.blocked_child_jobs, 1);
        assert_eq!(metadata_blocked.total.pending_child_requests, 0);
        for a in &mut config.agents {
            a.backend = BackendConfig::Fixture { responses: vec![] };
        }
        let failed = describe(&config, &crate::runner::run(&config).await.unwrap());
        assert_eq!(failed.total.failed_decisions, 3);
        assert_eq!(failed.total.voluntary_abstentions, 0);
        assert_eq!(failed.total.selection_status_counts["transport_failure"], 3);
        assert_eq!(failed.total.executed_merges, 0);
        assert_eq!(failed.total.admitted_children, 0);
        assert!(!serde_json::to_string(&failed).unwrap().contains("NaN"));
    }

    #[test]
    fn random_nomination_baseline_matches_enumerated_cases() {
        for n in 2..=4usize {
            let cases = (n - 1).pow(n as u32);
            let mut pair_sum = 0;
            for mut code in 0..cases {
                let outcomes = (0..n)
                    .map(|a| {
                        let options: Vec<_> = (0..n).filter(|b| *b != a).collect();
                        let b = options[code % (n - 1)];
                        code /= n - 1;
                        (
                            a.to_string(),
                            Outcome {
                                status: Status::Valid,
                                public_message: None,
                                private_update: None,
                                ballot: Some(Ballot {
                                    partner: Some(b.to_string()),
                                    consent: Consent::Missing,
                                }),
                            },
                        )
                    })
                    .collect();
                pair_sum += reciprocal_pairs(&outcomes).len();
            }
            let observed = 2.0 * pair_sum as f64 / (cases * n) as f64;
            assert!(
                (observed
                    - random_baseline(n)
                        .expected_matched_fraction_per_round
                        .unwrap())
                .abs()
                    < 1e-12
            );
        }
        assert_eq!(random_baseline(0).expected_matched_fraction_per_round, None);
        assert_eq!(
            random_baseline(1).expected_matched_fraction_per_round,
            Some(0.0)
        );
    }
}
