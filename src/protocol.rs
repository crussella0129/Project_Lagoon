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
