use crate::{
    config::{BackendConfig, ConfigError, Decoding},
    observation::Observation,
    protocol::Status,
};
use std::{future::Future, pin::Pin, sync::Arc};
pub mod fixture;
pub mod local_http;

#[derive(Clone, Debug)]
pub struct Request {
    pub observation: Observation,
    pub decoding: Decoding,
    pub seed: u64,
    pub max_response_bytes: usize,
}

#[derive(Clone, Debug)]
pub struct Generation {
    pub body: String,
    pub finish_reason: Option<String>,
}

impl From<String> for Generation {
    fn from(body: String) -> Self {
        Self {
            body,
            finish_reason: None,
        }
    }
}

impl From<&str> for Generation {
    fn from(body: &str) -> Self {
        body.to_owned().into()
    }
}

pub fn valid_finish_reason(reason: &str) -> bool {
    !reason.is_empty() && reason.len() <= 64 && !reason.chars().any(char::is_control)
}

pub type ResponseFuture = Pin<Box<dyn Future<Output = Result<Generation, Status>> + Send>>;
pub type TokenFuture = Pin<Box<dyn Future<Output = Result<TokenCount, Status>> + Send>>;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenCount {
    pub prompt_tokens: u32,
    pub model_capacity: Option<u32>,
}

/// Deterministic fixture accounting, deliberately not described as a model tokenizer.
pub fn fixture_tokens(observation: &Observation) -> TokenCount {
    TokenCount {
        prompt_tokens: observation.prompt_bytes() as u32 + 16,
        model_capacity: None,
    }
}

pub trait Backend: Send + Sync {
    fn respond(&self, request: Request) -> ResponseFuture;
    fn count_tokens(&self, request: Request) -> TokenFuture;
}

pub fn configured(config: &BackendConfig) -> Result<Arc<dyn Backend>, ConfigError> {
    match config {
        BackendConfig::Fixture { responses } => Ok(Arc::new(fixture::Fixture {
            responses: responses.clone(),
        })),
        BackendConfig::LocalHttp {
            endpoint,
            model,
            tokenizer,
        } => Ok(Arc::new(local_http::LocalHttp::new(
            endpoint, model, tokenizer,
        )?)),
    }
}
