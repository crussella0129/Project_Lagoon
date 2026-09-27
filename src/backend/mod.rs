use crate::{
    config::{BackendConfig, ConfigError, Decoding},
    observation::Observation,
    protocol::Status,
};
use std::{future::Future, pin::Pin, sync::Arc};
pub mod fixture;

#[derive(Clone, Debug)]
pub struct Request {
    pub observation: Observation,
    pub decoding: Decoding,
    pub seed: u64,
    pub max_response_bytes: usize,
}

pub type ResponseFuture = Pin<Box<dyn Future<Output = Result<String, Status>> + Send>>;

pub trait Backend: Send + Sync {
    fn respond(&self, request: Request) -> ResponseFuture;
}

pub fn configured(config: &BackendConfig) -> Result<Arc<dyn Backend>, ConfigError> {
    match config {
        BackendConfig::Fixture { responses } => Ok(Arc::new(fixture::Fixture {
            responses: responses.clone(),
        })),
        BackendConfig::LocalHttp { .. } => Err(ConfigError("local adapter not yet configured")),
    }
}
