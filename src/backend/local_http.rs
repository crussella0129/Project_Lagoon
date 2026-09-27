use super::{
    Backend, Generation, Request, ResponseFuture, TokenCount, TokenFuture, valid_finish_reason,
};
use crate::{
    config::{ConfigError, TokenizerConfig, validate_endpoint},
    protocol::Status,
};

pub struct LocalHttp {
    client: reqwest::Client,
    endpoint: reqwest::Url,
    model: String,
    tokenizer: TokenizerConfig,
}

impl LocalHttp {
    pub fn new(
        endpoint: &str,
        model: &str,
        tokenizer: &TokenizerConfig,
    ) -> Result<Self, ConfigError> {
        let endpoint = validate_endpoint(endpoint)?;
        match tokenizer {
            TokenizerConfig::Vllm { endpoint } => {
                validate_endpoint(endpoint)?;
            }
            TokenizerConfig::LlamaCpp {
                template_endpoint,
                tokenize_endpoint,
            } => {
                validate_endpoint(template_endpoint)?;
                validate_endpoint(tokenize_endpoint)?;
            }
        }
        let client = reqwest::Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| ConfigError("cannot create local HTTP client"))?;
        Ok(Self {
            client,
            endpoint,
            model: model.into(),
            tokenizer: tokenizer.clone(),
        })
    }
}

impl Backend for LocalHttp {
    fn count_tokens(&self, request: Request) -> TokenFuture {
        let (client, tokenizer, model) = (
            self.client.clone(),
            self.tokenizer.clone(),
            self.model.clone(),
        );
        Box::pin(async move {
            let parse: Result<TokenCount, Status> = async {
                match tokenizer {
                    TokenizerConfig::Vllm { endpoint } => {
                        let value = bounded_post(&client, &endpoint, &serde_json::json!({
                            "model":model,"messages":request.observation.messages(),
                            "add_generation_prompt":true,"add_special_tokens":false
                        }), 16_777_216).await?;
                        let prompt_tokens = value.get("count").and_then(|v| v.as_u64())
                            .and_then(|n| u32::try_from(n).ok()).ok_or(Status::MalformedContent)?;
                        let model_capacity = value.get("max_model_len").and_then(|v| v.as_u64())
                            .and_then(|n| u32::try_from(n).ok()).filter(|n| *n > 0)
                            .ok_or(Status::MalformedContent)?;
                        Ok(TokenCount { prompt_tokens, model_capacity:Some(model_capacity) })
                    }
                    TokenizerConfig::LlamaCpp { template_endpoint, tokenize_endpoint } => {
                        let value = bounded_post(&client, &template_endpoint, &serde_json::json!({
                            "messages":request.observation.messages(), "add_generation_prompt":true
                        }), 8_388_608).await?;
                        let prompt = value.get("prompt").and_then(|v| v.as_str()).ok_or(Status::MalformedContent)?;
                        let value = bounded_post(&client, &tokenize_endpoint, &serde_json::json!({
                            "content":prompt,"add_special":true,"parse_special":true,"with_pieces":false
                        }), 16_777_216).await?;
                        let tokens = value.get("tokens").and_then(|v| v.as_array())
                            .filter(|a| a.iter().all(|v| v.as_i64().is_some())).ok_or(Status::MalformedContent)?;
                        Ok(TokenCount { prompt_tokens: tokens.len() as u32, model_capacity: None })
                    }
                }
            }.await;
            parse.map_err(|_| Status::TokenizationFailure)
        })
    }

    fn respond(&self, request: Request) -> ResponseFuture {
        let (client, endpoint, model) = (
            self.client.clone(),
            self.endpoint.clone(),
            self.model.clone(),
        );
        Box::pin(async move {
            let payload = serde_json::json!({
                "model":model, "temperature":request.decoding.temperature,
                "max_tokens":request.decoding.max_tokens, "seed":request.seed,
                "stream":false,"n":1,
                "messages":request.observation.messages(),
                "response_format":request.observation.response_format(),
            });
            let mut response = client
                .post(endpoint)
                .json(&payload)
                .send()
                .await
                .map_err(|_| Status::TransportFailure)?;
            if !response.status().is_success() {
                return Err(Status::HttpFailure);
            }
            // JSON escaping can expand a content byte sixfold; the envelope is bounded too.
            let wire_limit = request.max_response_bytes * 6 + 8192;
            if response
                .content_length()
                .is_some_and(|n| n > wire_limit as u64)
            {
                return Err(Status::OversizedResponse);
            }
            let mut bytes = Vec::new();
            while let Some(chunk) = response
                .chunk()
                .await
                .map_err(|_| Status::TransportFailure)?
            {
                if chunk.len() > wire_limit.saturating_sub(bytes.len()) {
                    return Err(Status::OversizedResponse);
                }
                bytes.extend_from_slice(&chunk);
            }
            let value: serde_json::Value =
                serde_json::from_slice(&bytes).map_err(|_| Status::MalformedContent)?;
            let choices = value
                .get("choices")
                .and_then(|v| v.as_array())
                .filter(|v| v.len() == 1)
                .ok_or(Status::MalformedContent)?;
            let finish_reason = match choices[0].get("finish_reason") {
                None | Some(serde_json::Value::Null) => None,
                Some(serde_json::Value::String(reason)) if valid_finish_reason(reason) => {
                    Some(reason.clone())
                }
                _ => return Err(Status::MalformedContent),
            };
            let content = choices[0]
                .get("message")
                .and_then(|v| v.get("content"))
                .ok_or(Status::MalformedContent)?;
            let content = match content {
                serde_json::Value::String(content) => content.as_str(),
                serde_json::Value::Null if finish_reason.as_deref() == Some("length") => "",
                _ => return Err(Status::MalformedContent),
            };
            if content.len() > request.max_response_bytes {
                return Err(Status::OversizedResponse);
            }
            Ok(Generation {
                body: content.into(),
                finish_reason,
            })
        })
    }
}

async fn bounded_post(
    client: &reqwest::Client,
    endpoint: &str,
    payload: &serde_json::Value,
    limit: usize,
) -> Result<serde_json::Value, Status> {
    let mut response = client
        .post(endpoint)
        .json(payload)
        .send()
        .await
        .map_err(|_| Status::TransportFailure)?;
    if !response.status().is_success() {
        return Err(Status::HttpFailure);
    }
    if response.content_length().is_some_and(|n| n > limit as u64) {
        return Err(Status::OversizedResponse);
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response
        .chunk()
        .await
        .map_err(|_| Status::TransportFailure)?
    {
        if chunk.len() > limit.saturating_sub(bytes.len()) {
            return Err(Status::OversizedResponse);
        }
        bytes.extend_from_slice(&chunk);
    }
    serde_json::from_slice(&bytes).map_err(|_| Status::MalformedContent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        config::{BackendConfig, tests::config},
        runner,
    };
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
    };

    async fn server(
        response: String,
        delay: u64,
    ) -> (String, tokio::task::JoinHandle<serde_json::Value>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!(
            "http://{}/v1/chat/completions",
            listener.local_addr().unwrap()
        );
        let task = tokio::spawn(async move {
            let mut actual = None;
            for attempt in 0..2 {
                let (mut socket, _) = listener.accept().await.unwrap();
                let mut bytes = Vec::new();
                let mut buf = [0; 4096];
                let (offset, length) = loop {
                    let n = socket.read(&mut buf).await.unwrap();
                    assert!(n > 0);
                    bytes.extend_from_slice(&buf[..n]);
                    assert!(bytes.len() < 1_000_000);
                    if let Some(offset) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                        let header = String::from_utf8_lossy(&bytes[..offset]);
                        let length = header
                            .lines()
                            .find_map(|l| {
                                l.to_ascii_lowercase()
                                    .strip_prefix("content-length:")
                                    .map(|n| n.trim().parse::<usize>().unwrap())
                            })
                            .unwrap();
                        break (offset + 4, length);
                    }
                };
                while bytes.len() < offset + length {
                    let n = socket.read(&mut buf).await.unwrap();
                    assert!(n > 0);
                    bytes.extend_from_slice(&buf[..n]);
                }
                let request = serde_json::from_slice(&bytes[offset..offset + length]).unwrap();
                if attempt == 0 {
                    let body = r#"{"count":500,"max_model_len":8192}"#;
                    socket
                        .write_all(super::tests::response("200 OK", body, "").as_bytes())
                        .await
                        .unwrap();
                    continue;
                }
                tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                let _ = socket.write_all(response.as_bytes()).await;
                actual = Some(request);
            }
            actual.unwrap()
        });
        (endpoint, task)
    }

    fn response(status: &str, body: &str, extra: &str) -> String {
        format!(
            "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n{extra}\r\n{body}",
            body.len()
        )
    }

    #[tokio::test]
    async fn local_server_receives_only_owner_projection() {
        let body=serde_json::json!({"choices":[{"finish_reason":"stop","message":{"content":r#"{"partner":null,"consent":{"state":"defer"},"private_update":{"thought":"owner reply"}}"#}}]}).to_string();
        let (endpoint, server) = server(response("200 OK", &body, ""), 0).await;
        let mut config = config();
        config.rounds = 1;
        config.communication_steps = 0;
        config.timeout_ms = 1000;
        config.agents[0].initial_private.thought = Some("OWNER_CANARY".into());
        config.agents[1].initial_private.thought = Some("PEER_CANARY".into());
        config.agents[0].backend = BackendConfig::LocalHttp {
            tokenizer: TokenizerConfig::Vllm {
                endpoint: endpoint.clone(),
            },
            endpoint,
            model: "configured-local".into(),
        };
        let session = runner::run(&config).await.unwrap();
        let request = server.await.unwrap();
        assert_eq!(request["model"], "configured-local");
        assert_eq!(request["max_tokens"], config.decoding.max_tokens);
        assert_eq!(request["seed"], config.seed);
        assert_eq!(request["messages"][0]["role"], "system");
        let payload = request.to_string();
        assert!(payload.contains("OWNER_CANARY"));
        assert!(!payload.contains("PEER_CANARY"));
        assert_eq!(session.calls[0].outcome.status, Status::Abstained);
        assert!(
            matches!(&session.calls[0].reply, crate::runner::Reply::Response { finish_reason: Some(reason), .. } if reason == "stop")
        );
        assert_eq!(
            session.private_states["a"].thought.as_deref(),
            Some("owner reply")
        );
        assert!(session.public.messages.is_empty());
    }

    #[tokio::test]
    async fn token_limit_receipts_preserve_partial_text_and_replay_without_publishing() {
        for content in [
            serde_json::json!("{\"partner\":"),
            serde_json::json!(r#"{"partner":null,"private_update":{"thought":"do not apply"}}"#),
            serde_json::Value::Null,
        ] {
            let body = serde_json::json!({"choices":[{"finish_reason":"length","message":{"content":content}}]}).to_string();
            let (endpoint, task) = server(response("200 OK", &body, ""), 0).await;
            let mut config = config();
            config.rounds = 1;
            config.communication_steps = 0;
            config.timeout_ms = 1000;
            config.agents[0].backend = BackendConfig::LocalHttp {
                tokenizer: TokenizerConfig::Vllm {
                    endpoint: endpoint.clone(),
                },
                endpoint,
                model: "local".into(),
            };
            let session = runner::run(&config).await.unwrap();
            task.await.unwrap();
            assert_eq!(
                session.calls[0].outcome,
                crate::protocol::Outcome::failure(Status::GenerationLimit)
            );
            assert!(session.public.messages.is_empty());
            assert_eq!(session.private_states["a"].thought, None);
            assert!(
                matches!(&session.calls[0].reply, crate::runner::Reply::Response { body, finish_reason: Some(reason) } if body == content.as_str().unwrap_or("") && reason == "length")
            );
            let record = crate::record::Record::from_session(config, session, 100, 200);
            assert_eq!(record.report.all_call_status_counts["generation_limit"], 1);
            assert_eq!(record.report.finish_reason_counts["length"], 1);
            let root = tempfile::tempdir().unwrap();
            crate::record::save(&record, &root.path().join("run")).unwrap();
            let loaded: crate::record::Record =
                crate::record::read_json(&root.path().join("run/operator-record.json")).unwrap();
            assert_eq!(crate::replay::reconstruct(&loaded).unwrap(), record.session);
            let mut altered = record.clone();
            if let crate::runner::Reply::Response { finish_reason, .. } =
                &mut altered.session.calls[0].reply
            {
                *finish_reason = Some("stop".into());
            }
            assert!(crate::replay::reconstruct(&altered).is_err());
        }
    }

    #[tokio::test]
    async fn local_transport_failures_are_bounded_and_redacted() {
        let secret = "BODY_HEADER_CANARY";
        let cases = [
            (
                response(
                    "302 Found",
                    secret,
                    "Location: http://192.0.2.1/\r\nX-Secret: BODY_HEADER_CANARY\r\n",
                ),
                0,
                Status::HttpFailure,
            ),
            (response("500 Failed", secret, ""), 0, Status::HttpFailure),
            (
                response("200 OK", "not json", ""),
                0,
                Status::MalformedContent,
            ),
            (response("200 OK", r#"{"choices":[{"finish_reason":17,"message":{"content":"{}"}}]}"#, ""), 0, Status::MalformedContent),
            (response("200 OK", &serde_json::json!({"choices":[{"finish_reason":"x".repeat(65),"message":{"content":"{}"}}]}).to_string(), ""), 0, Status::MalformedContent),
            (
                response("200 OK", &"x".repeat(40_000), ""),
                0,
                Status::OversizedResponse,
            ),
            (
                response(
                    "200 OK",
                    r#"{"choices":[{"message":{"content":null}}]}"#,
                    "",
                ),
                0,
                Status::MalformedContent,
            ),
            (response("200 OK", "{}", ""), 200, Status::Timeout),
            (
                "HTTP/1.1 200 OK\r\nContent-Length: 100\r\nConnection: close\r\n\r\nx".into(),
                0,
                Status::TransportFailure,
            ),
            (
                format!(
                    "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n{:x}\r\n{}\r\n0\r\n\r\n",
                    40_000,
                    "x".repeat(40_000)
                ),
                0,
                Status::OversizedResponse,
            ),
        ];
        for (response, delay, expected) in cases {
            let (endpoint, task) = server(response, delay).await;
            let mut config = config();
            config.rounds = 1;
            config.communication_steps = 0;
            config.timeout_ms = 100;
            config.agents[0].backend = BackendConfig::LocalHttp {
                tokenizer: TokenizerConfig::Vllm {
                    endpoint: endpoint.clone(),
                },
                endpoint,
                model: "local".into(),
            };
            let session = runner::run(&config).await.unwrap();
            assert_eq!(session.calls[0].outcome.status, expected);
            assert!(
                !serde_json::to_string(&session.public)
                    .unwrap()
                    .contains(secret)
            );
            assert!(
                !serde_json::to_string(&session.calls[0].reply)
                    .unwrap()
                    .contains(secret)
            );
            task.abort();
            let _ = task.await;
        }
    }
}
