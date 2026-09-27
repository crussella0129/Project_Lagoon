use crate::{
    config::fingerprint,
    protocol::{Phase, Status},
    record::{Record, RecordError, SCHEMA_VERSION},
    runner::{Reply, Session, outcome_from_reply},
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
            for (call, observation) in calls.iter().zip(observations) {
                if call.handle != observation.owner || call.observation != observation {
                    return Err(fail());
                }
                let over_context = serde_json::to_vec(&observation).map_err(|_| fail())?.len()
                    > config.max_context_bytes;
                match &call.reply {
                    Reply::Response { body }
                        if body.len() > config.max_response_bytes || over_context =>
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
                        ) || (*status == Status::ContextLimit) != over_context =>
                    {
                        return Err(fail());
                    }
                    _ => {}
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
        for (index, agent) in config.agents.iter_mut().enumerate() {
            agent.initial_private.thought = Some(format!("PRIVATE_CANARY_{index}"));
            let partner = match index {
                0 => Some("b"),
                1 => Some("a"),
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
        for kind in 0..9 {
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
                _ => record.session.calls[1] = record.session.calls[0].clone(),
            }
            assert!(reconstruct(&record).is_err(), "kind {kind}");
        }
    }
}
