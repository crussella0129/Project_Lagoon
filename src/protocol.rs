use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// JSON Schema maxLength counts Unicode characters, independently of UTF-8 bytes.
pub const PUBLIC_MESSAGE_MAX_CHARS: usize = 128;
pub const PRIVATE_NOTE_MAX_CHARS: usize = 64;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrivateState {
    pub feeling: Option<String>,
    pub learned_preference: Option<String>,
    pub thought: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peers: Option<BTreeMap<String, PeerMemory>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeerMemory {
    pub trust: u8,
    pub note: String,
}

impl PrivateState {
    pub fn update(&mut self, update: &Self) {
        if let Some(peers) = &update.peers {
            self.peers = Some(peers.clone());
        }
        for (current, incoming) in [
            (&mut self.feeling, &update.feeling),
            (&mut self.learned_preference, &update.learned_preference),
            (&mut self.thought, &update.thought),
        ] {
            if let Some(value) = incoming {
                *current = Some(value.clone());
            }
        }
    }

    pub fn byte_len(&self) -> usize {
        let notes: usize = [&self.feeling, &self.learned_preference, &self.thought]
            .into_iter()
            .flatten()
            .map(String::len)
            .sum();
        notes
            + self.peers.as_ref().map_or(0, |peers| {
                serde_json::to_vec(peers).expect("bounded peer map").len()
            })
    }

    pub fn within_note_limits(&self) -> bool {
        self.within_note_limit(PRIVATE_NOTE_MAX_CHARS)
    }

    pub fn within_note_limit(&self, limit: usize) -> bool {
        [&self.feeling, &self.learned_preference, &self.thought]
            .into_iter()
            .flatten()
            .all(|s| s.chars().count() <= limit)
    }

    pub fn valid_peers(&self, config: &crate::config::Experiment, owner: &str) -> bool {
        self.peers.as_ref().is_none_or(|peers| {
            config.peer_memory
                && peers.len() < config.agents.len()
                && peers.iter().all(|(alias, entry)| {
                    entry.trust <= 10
                        && entry.note.chars().count() <= PRIVATE_NOTE_MAX_CHARS
                        && config.agents.iter().any(|agent| {
                            agent.handle != owner && config.alias(&agent.handle) == *alias
                        })
                })
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum Consent {
    Agree {
        plan_id: String,
        fingerprint: String,
    },
    Decline,
    Defer,
    Missing,
    Invalid,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ballot {
    pub partner: Option<String>,
    pub consent: Consent,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Phase {
    Communication { step: u32 },
    Selection,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Valid,
    Abstained,
    InvalidResponse,
    InvalidTarget,
    OversizedResponse,
    ContextLimit,
    Timeout,
    TransportFailure,
    HttpFailure,
    MalformedContent,
    TokenizationFailure,
    GenerationLimit,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicMessage {
    pub round: u32,
    pub step: u32,
    pub author: String,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pair {
    pub agents: [String; 2],
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicView {
    pub messages: Vec<PublicMessage>,
    pub pairs: Vec<PublicPair>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicPair {
    pub round: u32,
    pub pair: Pair,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Outcome {
    pub status: Status,
    pub public_message: Option<String>,
    pub private_update: Option<PrivateState>,
    pub ballot: Option<Ballot>,
}

impl Outcome {
    pub fn failure(status: Status) -> Self {
        Self {
            status,
            public_message: None,
            private_update: None,
            ballot: None,
        }
    }
}

/// Selection consent is parsed separately: invalid consent cannot erase a valid nomination.
pub fn normalize(
    body: &str,
    observation: &crate::observation::Observation,
    config: &crate::config::Experiment,
) -> Outcome {
    if body.len() > config.max_response_bytes {
        return Outcome::failure(Status::OversizedResponse);
    }
    let Ok(serde_json::Value::Object(mut fields)) = serde_json::from_str(json_body(body)) else {
        return Outcome::failure(Status::InvalidResponse);
    };
    let private_update = match fields.remove("private_update") {
        None | Some(serde_json::Value::Null) => None,
        Some(value) => match serde_json::from_value::<PrivateState>(value) {
            Ok(value) => Some(value),
            Err(_) => return Outcome::failure(Status::InvalidResponse),
        },
    };
    let mut proposed = observation.private_state.clone();
    if let Some(update) = &private_update {
        proposed.update(update);
    }
    if private_update.as_ref().is_some_and(|update| {
        !update.valid_peers(
            config,
            &config
                .handle_for_alias(&observation.owner)
                .expect("known owner"),
        )
    }) {
        return Outcome::failure(Status::InvalidResponse);
    }
    if proposed.byte_len() > config.max_private_bytes
        || private_update.as_ref().is_some_and(|update| {
            !update.within_note_limit(observation.response_limits.private_note_chars)
        })
    {
        return Outcome::failure(Status::OversizedResponse);
    }
    match observation.phase {
        Phase::Communication { .. } => {
            let public_message = match fields.remove("public_message") {
                None | Some(serde_json::Value::Null) => None,
                Some(serde_json::Value::String(value)) => Some(value),
                _ => return Outcome::failure(Status::InvalidResponse),
            };
            if !fields.is_empty() {
                return Outcome::failure(Status::InvalidResponse);
            }
            if public_message.as_ref().is_some_and(|s| {
                s.chars().count() > observation.response_limits.public_message_chars
            }) {
                return Outcome::failure(Status::OversizedResponse);
            }
            Outcome {
                status: Status::Valid,
                public_message,
                private_update,
                ballot: None,
            }
        }
        Phase::Selection => {
            let partner = match fields.remove("partner") {
                Some(serde_json::Value::Null) => None,
                Some(serde_json::Value::String(value)) => Some(value),
                _ => return Outcome::failure(Status::InvalidResponse),
            };
            if partner.as_ref().is_some_and(|value| {
                value == &observation.owner || !observation.handles.contains(value)
            }) {
                return Outcome::failure(Status::InvalidTarget);
            }
            let consent = match fields.remove("consent") {
                None => Consent::Missing,
                Some(mut value) => {
                    // The immutable observation binds the receipt. An obsolete copied hash
                    // is ignored, never interpreted as a different authorization.
                    if value.get("state").and_then(|v| v.as_str()) == Some("agree")
                        && let Some(fields) = value.as_object_mut()
                    {
                        fields.remove("fingerprint");
                        if let Some(card) = fields
                            .get("plan_id")
                            .and_then(|v| v.as_str())
                            .and_then(|id| observation.plans.iter().find(|p| p.plan.id == id))
                        {
                            fields
                                .insert("fingerprint".into(), serde_json::json!(card.fingerprint));
                        }
                    }
                    match serde_json::from_value::<Consent>(value) {
                        Ok(Consent::Agree {
                            plan_id,
                            fingerprint,
                        }) => {
                            if observation
                                .plans
                                .iter()
                                .any(|p| p.plan.id == plan_id && p.fingerprint == fingerprint)
                            {
                                Consent::Agree {
                                    plan_id,
                                    fingerprint,
                                }
                            } else {
                                Consent::Invalid
                            }
                        }
                        Ok(Consent::Decline) => Consent::Decline,
                        Ok(Consent::Defer) => Consent::Defer,
                        _ => Consent::Invalid,
                    }
                }
            };
            if !fields.is_empty() {
                return Outcome::failure(Status::InvalidResponse);
            }
            Outcome {
                status: if partner.is_some() {
                    Status::Valid
                } else {
                    Status::Abstained
                },
                public_message: None,
                private_update,
                ballot: Some(Ballot {
                    partner: partner.and_then(|alias| config.handle_for_alias(&alias)),
                    consent,
                }),
            }
        }
    }
}

/// Accept one complete JSON fence only. Never extract a guess from surrounding prose.
pub fn json_body(body: &str) -> &str {
    let trimmed = body.trim();
    trimmed
        .strip_prefix("```json\n")
        .or_else(|| trimmed.strip_prefix("```json\r\n"))
        .or_else(|| trimmed.strip_prefix("```\n"))
        .or_else(|| trimmed.strip_prefix("```\r\n"))
        .and_then(|s| s.strip_suffix("```"))
        .map(str::trim)
        .unwrap_or(trimmed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::tests::config, observation::Observation};

    #[test]
    fn unicode_bounds_accept_limit_and_reject_one_extra_character() {
        let config = config();
        let observation = Observation::new(
            &config,
            "a",
            &PrivateState::default(),
            &PublicView::default(),
            0,
            Phase::Communication { step: 0 },
        );
        let body = |public: usize, private: usize| {
            serde_json::json!({
                "public_message": "🙂".repeat(public),
                "private_update": {"thought": "🙂".repeat(private)}
            })
            .to_string()
        };
        assert_eq!(
            normalize(
                &body(PUBLIC_MESSAGE_MAX_CHARS, PRIVATE_NOTE_MAX_CHARS),
                &observation,
                &config
            )
            .status,
            Status::Valid
        );
        for (public, private) in [
            (PUBLIC_MESSAGE_MAX_CHARS + 1, 0),
            (0, PRIVATE_NOTE_MAX_CHARS + 1),
        ] {
            let outcome = normalize(&body(public, private), &observation, &config);
            assert_eq!(outcome, Outcome::failure(Status::OversizedResponse));
        }
    }

    #[test]
    fn maximally_escaped_bounded_communication_fits_example_byte_guard() {
        let config = config();
        let observation = Observation::new(
            &config,
            "a",
            &PrivateState::default(),
            &PublicView::default(),
            0,
            Phase::Communication { step: 0 },
        );
        let public = r"\ud83d\ude00".repeat(PUBLIC_MESSAGE_MAX_CHARS);
        let private = r"\ud83d\ude00".repeat(PRIVATE_NOTE_MAX_CHARS);
        let body = format!(
            r#"{{"public_message":"{public}","private_update":{{"feeling":"{private}","learned_preference":"{private}","thought":"{private}"}}}}"#
        );
        assert!(body.len() <= 4096);
        assert_eq!(
            normalize(&body, &observation, &config).status,
            Status::Valid
        );
        // This bounds canonical text bytes, not tokenizer output or arbitrary whitespace.
    }
}
