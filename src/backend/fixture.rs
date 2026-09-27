use super::{Backend, Request, ResponseFuture};
use crate::{config::FixtureResponse, protocol::Status};

pub struct Fixture {
    pub responses: Vec<FixtureResponse>,
}

impl Backend for Fixture {
    fn respond(&self, request: Request) -> ResponseFuture {
        let response = self
            .responses
            .iter()
            .find(|r| r.round == request.observation.round && r.phase == request.observation.phase)
            .cloned();
        Box::pin(async move {
            let response = response.ok_or(Status::TransportFailure)?;
            tokio::time::sleep(std::time::Duration::from_millis(response.delay_ms)).await;
            if response.transport_failure {
                return Err(Status::TransportFailure);
            }
            let body = response.body.ok_or(Status::MalformedContent)?;
            if body.len() > request.max_response_bytes {
                return Err(Status::OversizedResponse);
            }
            Ok(body)
        })
    }
}
