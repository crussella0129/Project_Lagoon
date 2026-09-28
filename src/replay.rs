use crate::{
    config::{BackendConfig, fingerprint},
    protocol::{Phase, Status},
    record::{Record, RecordError, SCHEMA_VERSION},
    runner::{Reply, Session, fit_bytes, outcome_from_reply, tokens_fit},
};

/// Structural consistency checking, not authentication against the trusted operator.
/// Replay has no backend argument and never initializes one.
pub fn reconstruct(record: &Record) -> Result<Session, RecordError> {
    let fail = || RecordError("unsupported or inconsistent experiment record");
    if record.schema_version != SCHEMA_VERSION
        || record.harness_version != env!("CARGO_PKG_VERSION")
        || record.finished_unix_ms < record.started_unix_ms
    {
        return Err(fail());
    }
    let config = &record.config;
    config.validate().map_err(|_| fail())?;
    if fingerprint(config) != record.config_fingerprint {
        return Err(fail());
    }
    let count =
        (config.rounds as usize) * (config.communication_steps as usize + 1) * config.agents.len();
    if record.session.calls.len() != count {
        return Err(fail());
    }
    let mut session = Session::initial(config);
    let mut cursor = 0;
    for round in 0..config.rounds {
        for phase in (0..config.communication_steps)
            .map(|step| Phase::Communication { step })
            .chain(std::iter::once(Phase::Selection))
        {
            let observations = session.observations(config, round, phase.clone());
            let calls = &record.session.calls[cursor..cursor + observations.len()];
            for (call, mut observation) in calls.iter().zip(observations) {
                if config.alias(&call.handle) != observation.owner
                    || call.seed
                        != crate::config::call_seed(config.seed, round, &phase, &call.handle)
                    || !config.agents.iter().any(|a| a.handle == call.handle)
                {
                    return Err(fail());
                }
                let fixture = matches!(
                    config
                        .agents
                        .iter()
                        .find(|a| a.handle == call.handle)
                        .unwrap()
                        .backend,
                    BackendConfig::Fixture { .. }
                );
                let mut fits = false;
                let mut over_context = !fit_bytes(config, &mut observation);
                for (index, trial) in call.token_trials.iter().enumerate() {
                    if over_context
                        || fits
                        || trial.prompt_fingerprint != fingerprint(&observation.messages())
                        || (fixture && trial.count != crate::backend::fixture_tokens(&observation))
                    {
                        return Err(fail());
                    }
                    fits = tokens_fit(config, &trial.count);
                    if !fits {
                        if !observation.drop_oldest_event() {
                            over_context = true;
                        } else {
                            over_context = !fit_bytes(config, &mut observation);
                        }
                    }
                    if (fits || over_context) && index + 1 != call.token_trials.len() {
                        return Err(fail());
                    }
                }
                if call.observation != observation
                    || call.response_schema_fingerprint
                        != fingerprint(&observation.response_format())
                {
                    return Err(fail());
                }
                match &call.reply {
                    Reply::Response {
                        body,
                        finish_reason,
                    } if body.len() > config.max_response_bytes
                        || over_context
                        || !fits
                        || finish_reason
                            .as_ref()
                            .is_some_and(|r| !crate::backend::valid_finish_reason(r)) =>
                    {
                        return Err(fail());
                    }
                    Reply::Failure { status }
                        if !matches!(
                            status,
                            Status::ContextLimit
                                | Status::OversizedResponse
                                | Status::Timeout
                                | Status::TransportFailure
                                | Status::HttpFailure
                                | Status::MalformedContent
                                | Status::TokenizationFailure
                        ) || (*status == Status::ContextLimit) != over_context =>
                    {
                        return Err(fail());
                    }
                    _ => {}
                }
                if let Reply::Failure { status } = &call.reply
                    && ((*status == Status::TokenizationFailure && fits)
                        || (!fits
                            && !over_context
                            && !matches!(
                                status,
                                Status::TokenizationFailure
                                    | Status::Timeout
                                    | Status::TransportFailure
                            )))
                {
                    return Err(fail());
                }
                if call.outcome != outcome_from_reply(&call.reply, &observation, config) {
                    return Err(fail());
                }
            }
            cursor += calls.len();
            session.publish(config, calls.to_vec(), round, &phase);
        }
    }
    if session != record.session || crate::report::describe(config, &session) != record.report {
        return Err(fail());
    }
    Ok(session)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::{BackendConfig, FixtureResponse, tests::config},
        record::{self, Record},
        runner,
    };

    pub(crate) async fn recorded() -> Record {
        let mut config = config();
        let aliases = [config.alias("b"), config.alias("a")];
        for (index, agent) in config.agents.iter_mut().enumerate() {
            agent.initial_private.thought = Some(format!("PRIVATE_CANARY_{index}"));
            let partner = match index {
                0 => Some(aliases[0].as_str()),
                1 => Some(aliases[1].as_str()),
                _ => None,
            };
            agent.backend=BackendConfig::Fixture {responses:(0..2).flat_map(|round|[
                FixtureResponse {round,phase:Phase::Communication {step:0},delay_ms:0,transport_failure:false,body:Some(r#"{"public_message":"hello"}"#.into())},
                FixtureResponse {round,phase:Phase::Selection,delay_ms:0,transport_failure:false,body:Some(serde_json::json!({"partner":partner,"consent":{"state":"decline"}}).to_string())},
            ]).collect()};
        }
        let session = runner::run(&config).await.unwrap();
        Record::from_session(config, session, 100, 200)
    }

    #[tokio::test]
    async fn operator_record_and_public_projection_are_separate() {
        let record = recorded().await;
        let root = tempfile::tempdir().unwrap();
        let output = root.path().join("run");
        record::save(&record, &output).unwrap();
        let operator = std::fs::read_to_string(output.join("operator-record.json")).unwrap();
        let public = std::fs::read_to_string(output.join("public.json")).unwrap();
        assert!(operator.contains("PRIVATE_CANARY_1"));
        for hidden in [
            "PRIVATE_CANARY",
            "ballot",
            "backend",
            "checkpoint",
            "consent",
        ] {
            assert!(!public.contains(hidden));
        }
        assert!(public.contains("hello"));
        assert!(public.contains("pair"));
        assert!(record::save(&record, &output).is_err());
        assert_eq!(
            std::fs::read_to_string(output.join("operator-record.json")).unwrap(),
            operator
        );
    }

    #[tokio::test]
    async fn replay_matches_recording_without_backend_calls() {
        let record = recorded().await;
        let replay = reconstruct(&record).unwrap();
        assert_eq!(replay, record.session);
        let root = tempfile::tempdir().unwrap();
        let original = root.path().join("original");
        let copy = root.path().join("replay");
        record::save(&record, &original).unwrap();
        let loaded: Record = record::read_json(&original.join("operator-record.json")).unwrap();
        assert_eq!(loaded.started_unix_ms, 100);
        assert_eq!(loaded.finished_unix_ms, 200);
        record::save(&loaded, &copy).unwrap();
        for file in [
            "operator-record.json",
            "public.json",
            "report.json",
            "merge-requests.json",
        ] {
            assert_eq!(
                std::fs::read(original.join(file)).unwrap(),
                std::fs::read(copy.join(file)).unwrap()
            );
        }
    }

    #[tokio::test]
    async fn unsupported_or_inconsistent_records_are_rejected() {
        let original = recorded().await;
        for kind in 0..13 {
            let mut record = original.clone();
            match kind {
                0 => record.schema_version = 99,
                1 => {
                    record.session.calls.pop();
                }
                2 => record.session.calls[0].handle = "other".into(),
                3 => record.session.calls[0].observation.phase = Phase::Selection,
                4 => record.session.public.pairs.clear(),
                5 => record.session.calls[0].outcome.status = Status::Abstained,
                6 => record.config.recipes[0].density = Some(0.1),
                7 => record.report.total.admitted_children = 1,
                8 => record.session.calls[1] = record.session.calls[0].clone(),
                9 => record.session.calls[0].token_trials[0].count.prompt_tokens += 1,
                10 => record.session.calls[0].observation.handles.reverse(),
                11 => record.session.calls[0].response_schema_fingerprint = "fake".into(),
                _ => record.session.calls[0].observation.omitted_public_events += 1,
            }
            assert!(reconstruct(&record).is_err(), "kind {kind}");
        }
    }
}
