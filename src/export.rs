//! Replay-checked research tables. Ballots remain operator/research data.
use crate::{
    config::fingerprint,
    protocol::Phase,
    record::{BoundedBuffer, Record, RecordError, save_artifacts},
    runner::Reply,
};
use sha2::{Digest, Sha256};
use std::{io::Write, path::Path};

struct Csv(BoundedBuffer);
impl Csv {
    fn new(header: &[&str]) -> Result<Self, RecordError> {
        let mut table = Self(BoundedBuffer(Vec::new()));
        table.row(header.iter().copied())?;
        Ok(table)
    }

    fn row(
        &mut self,
        fields: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<(), RecordError> {
        // Quote every field; double embedded quotes. Preserve commas, CR/LF and UTF-8.
        let result = (|| -> std::io::Result<()> {
            for (index, field) in fields.into_iter().enumerate() {
                if index > 0 {
                    self.0.write_all(b",")?;
                }
                self.0.write_all(b"\"")?;
                for part in field.as_ref().split_inclusive('"') {
                    self.0.write_all(part.as_bytes())?;
                    if part.ends_with('"') {
                        self.0.write_all(b"\"")?;
                    }
                }
                self.0.write_all(b"\"")?;
            }
            self.0.write_all(b"\r\n")
        })();
        result.map_err(|_| RecordError("CSV artifact exceeds 64 MiB limit"))
    }

    fn bytes(self) -> Vec<u8> {
        self.0.0
    }
}

fn status(value: &crate::protocol::Status) -> String {
    serde_json::to_value(value)
        .expect("status serializes")
        .as_str()
        .expect("status string")
        .into()
}

pub fn save(record: &Record, output: &Path) -> Result<serde_json::Value, RecordError> {
    crate::replay::reconstruct(record)?;
    let id = fingerprint(record);
    let seed = record.config.seed.to_string();
    let mut choices = Csv::new(&[
        "record_id",
        "run_seed",
        "round",
        "chooser",
        "candidate",
        "shown_position",
        "eligible_peers",
        "is_outside",
        "chosen",
        "status",
    ])?;
    let mut messages = Csv::new(&["record_id", "round", "step", "author", "text"])?;
    let mut events = Csv::new(&[
        "record_id",
        "round",
        "kind",
        "participant_a",
        "participant_b",
        "retired",
    ])?;
    let mut outcomes = Csv::new(&[
        "record_id",
        "run_seed",
        "round",
        "phase",
        "step",
        "owner",
        "call_seed",
        "status",
        "finish_reason",
        "elapsed_ms",
        "eligible_peers",
        "partner",
        "consent",
    ])?;
    // No social profile implementation exists yet. Header-only is explicit missingness.
    let profiles = Csv::new(&[
        "record_id",
        "round",
        "participant",
        "attribute",
        "state",
        "value",
        "visible_from_round",
    ])?;
    let mut choice_sets = 0;
    let mut choice_rows = 0;
    for call in &record.session.calls {
        let observation = &call.observation;
        let round = observation.round.to_string();
        let state = status(&call.outcome.status);
        let eligible = observation.handles.len().to_string();
        let ballot = call.outcome.ballot.as_ref();
        let partner_alias = ballot
            .and_then(|b| b.partner.as_ref())
            .map(|h| record.config.alias(h));
        let (phase, step) = match observation.phase {
            Phase::Communication { step } => ("communication", step.to_string()),
            Phase::Selection => ("selection", String::new()),
        };
        let finish = match &call.reply {
            Reply::Response { finish_reason, .. } => finish_reason.as_deref().unwrap_or(""),
            Reply::Failure { .. } => "",
        };
        let consent = ballot
            .map(|b| {
                serde_json::to_value(&b.consent).expect("consent serializes")["state"]
                    .as_str()
                    .expect("consent state")
                    .to_owned()
            })
            .unwrap_or_default();
        outcomes.row([
            id.as_str(),
            &seed,
            &round,
            phase,
            &step,
            &observation.owner,
            &call.seed.to_string(),
            &state,
            finish,
            &call.elapsed_ms.to_string(),
            &eligible,
            partner_alias.as_deref().unwrap_or(""),
            &consent,
        ])?;
        if observation.phase != Phase::Selection {
            continue;
        }
        choice_sets += 1;
        for (index, candidate) in observation
            .handles
            .iter()
            .map(Some)
            .chain(std::iter::once(None))
            .enumerate()
        {
            let outside = candidate.is_none();
            let chosen = ballot
                .map(|_| u8::from(partner_alias.as_ref() == candidate).to_string())
                .unwrap_or_default();
            let position = if outside {
                String::new()
            } else {
                (index + 1).to_string()
            };
            choices.row([
                id.as_str(),
                &seed,
                &round,
                &observation.owner,
                candidate.map(String::as_str).unwrap_or("__outside__"),
                &position,
                &eligible,
                if outside { "1" } else { "0" },
                &chosen,
                &state,
            ])?;
            choice_rows += 1;
        }
    }
    for message in &record.session.public.messages {
        messages.row([
            id.as_str(),
            &message.round.to_string(),
            &message.step.to_string(),
            &message.author,
            &message.text,
        ])?;
    }
    for pair in &record.session.public.pairs {
        events.row([
            id.as_str(),
            &pair.round.to_string(),
            "reciprocal_pair",
            &pair.pair.agents[0],
            &pair.pair.agents[1],
            if pair.retired { "1" } else { "0" },
        ])?;
    }
    let mut artifacts = vec![
        ("choices.csv", choices.bytes()),
        ("messages.csv", messages.bytes()),
        ("events.csv", events.bytes()),
        ("outcomes.csv", outcomes.bytes()),
        ("profiles.csv", profiles.bytes()),
    ];
    let digests: std::collections::BTreeMap<_, _> = artifacts
        .iter()
        .map(|(name, bytes)| {
            (
                *name,
                Sha256::digest(bytes)
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect::<String>(),
            )
        })
        .collect();
    let manifest = serde_json::json!({
        "export_version":1, "record_id":id, "config_fingerprint":record.config_fingerprint,
        "record_schema_version":record.schema_version,"harness_version":record.harness_version,
        "run_seed":record.config.seed,"pairing":record.config.pairing,
        "choice_sets":choice_sets,"choice_rows":choice_rows,"profile_rows":0,
        "profiles_available":false,"audience":"trusted_operator_research",
        "outside_candidate":"__outside__","positions":"one_based; empty for outside option",
        "missing_choice":"technical failure; never an abstention",
        "private_notes_exported":false, "sha256":digests
    });
    artifacts.push((
        "manifest.json",
        serde_json::to_vec_pretty(&manifest).expect("manifest serializes"),
    ));
    save_artifacts(output, artifacts)?;
    Ok(manifest)
}
