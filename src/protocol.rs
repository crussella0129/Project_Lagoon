use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrivateState {
    pub feeling: Option<String>,
    pub learned_preference: Option<String>,
    pub thought: Option<String>,
}

impl PrivateState {
    pub fn update(&mut self, update: &Self) {
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
        [&self.feeling, &self.learned_preference, &self.thought]
            .into_iter()
            .flatten()
            .map(String::len)
            .sum()
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
    let Ok(serde_json::Value::Object(mut fields)) = serde_json::from_str(body) else {
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
    if proposed.byte_len() > config.max_private_bytes {
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
                Some(value) => match serde_json::from_value::<Consent>(value) {
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
                },
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
                ballot: Some(Ballot { partner, consent }),
            }
        }
    }
}
