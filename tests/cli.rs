use lovers_lagoon::{
    config::{BackendConfig, Experiment, TokenizerConfig},
    record::{self, Record},
    report::Report,
};
use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
};

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_lovers-lagoon"))
}
fn example(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(name)
}
fn success(output: Output) -> Output {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    output
}
fn fixture(name: &str, output: &Path) -> Record {
    success(
        binary()
            .arg("run")
            .arg("--config")
            .arg(example(name))
            .arg("--output")
            .arg(output)
            .output()
            .unwrap(),
    );
    record::read_json(&output.join("operator-record.json")).unwrap()
}

fn check_replay(original: &Path, copy: &Path) {
    success(
        binary()
            .arg("replay")
            .arg("--record")
            .arg(original.join("operator-record.json"))
            .arg("--output")
            .arg(copy)
            .output()
            .unwrap(),
    );
    for name in [
        "operator-record.json",
        "public.json",
        "report.json",
        "merge-requests.json",
    ] {
        assert_eq!(
            std::fs::read(original.join(name)).unwrap(),
            std::fs::read(copy.join(name)).unwrap()
        );
    }
}

#[test]
fn cli_runtime_schema_contract() {
    for phase in ["communication", "selection"] {
        let output = success(
            binary()
                .args(["schema", "--config"])
                .arg(example("fixture-experiment.json"))
                .args(["--owner", "a", "--phase", phase])
                .output()
                .unwrap(),
        );
        let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(
            value["response_schema_fingerprint"],
            lovers_lagoon::config::fingerprint(&value["response_format"])
        );
        assert_eq!(value["messages"].as_array().unwrap().len(), 2);
        assert!(value["seed"].as_u64().is_some());
        assert_eq!(value["response_format"]["json_schema"]["strict"], true);
    }
    assert!(
        !binary()
            .args(["schema", "--config"])
            .arg(example("fixture-experiment.json"))
            .args(["--owner", "missing"])
            .output()
            .unwrap()
            .status
            .success()
    );
}

#[test]
fn cli_export_privacy_and_failures() {
    use lovers_lagoon::{config::FixtureResponse, protocol::Phase};
    let root = tempfile::tempdir().unwrap();
    let mut config: Experiment = record::read_json(&example("fixture-experiment.json")).unwrap();
    config.rounds = 1;
    for agent in &mut config.agents {
        if let BackendConfig::Fixture { responses } = &mut agent.backend {
            responses.retain(|response| response.round == 0);
        }
    }
    let BackendConfig::Fixture { responses } = &mut config.agents[0].backend else {
        panic!()
    };
    responses[0].body =
        Some(serde_json::json!({"public_message":"Comma, \"quote\"\nUnicode 🐟"}).to_string());
    config.agents[1].backend = BackendConfig::Fixture {
        responses: vec![FixtureResponse {
            round: 0,
            phase: Phase::Selection,
            delay_ms: 0,
            transport_failure: true,
            body: None,
        }],
    };
    let config_path = root.path().join("config.json");
    std::fs::write(&config_path, serde_json::to_vec(&config).unwrap()).unwrap();
    let run = root.path().join("run");
    success(
        binary()
            .arg("run")
            .arg("--config")
            .arg(config_path)
            .arg("--output")
            .arg(&run)
            .output()
            .unwrap(),
    );
    let tables = root.path().join("tables");
    let record_path = run.join("operator-record.json");
    let export = || {
        binary()
            .arg("export")
            .arg("--record")
            .arg(&record_path)
            .arg("--output")
            .arg(&tables)
            .output()
            .unwrap()
    };
    let result: serde_json::Value = serde_json::from_slice(&success(export()).stdout).unwrap();
    assert_eq!(result["choice_sets"], 3);
    assert_eq!(result["choice_rows"], 9);
    let choices = std::fs::read_to_string(tables.join("choices.csv")).unwrap();
    assert_eq!(
        choices
            .lines()
            .filter(|l| l.ends_with("\"1\",\"valid\""))
            .count(),
        1
    );
    assert_eq!(
        choices
            .lines()
            .filter(|l| l.ends_with("\"1\",\"abstained\""))
            .count(),
        1
    );
    assert_eq!(
        choices
            .lines()
            .filter(|l| l.ends_with("\"\",\"transport_failure\""))
            .count(),
        3
    );
    assert!(
        std::fs::read_to_string(tables.join("messages.csv"))
            .unwrap()
            .contains("\"Comma, \"\"quote\"\"\nUnicode 🐟\"")
    );
    for entry in std::fs::read_dir(&tables).unwrap() {
        assert!(
            !std::fs::read_to_string(entry.unwrap().path())
                .unwrap()
                .contains("fixture-private")
        );
    }
    assert!(!export().status.success());
    let mut invalid: Record = record::read_json(&record_path).unwrap();
    invalid.session.calls[0].seed ^= 1;
    std::fs::write(&record_path, serde_json::to_vec(&invalid).unwrap()).unwrap();
    assert!(
        !binary()
            .arg("export")
            .arg("--record")
            .arg(&record_path)
            .arg("--output")
            .arg(root.path().join("invalid"))
            .output()
            .unwrap()
            .status
            .success()
    );
    assert!(!root.path().join("invalid").exists());
}

#[test]
fn cli_fixture_run_replay_report() {
    let root = tempfile::tempdir().unwrap();
    let run = root.path().join("run");
    let record = fixture("fixture-experiment.json", &run);
    assert_eq!(record.report.total.social_pairs, 2);
    assert_eq!(record.report.total.voluntary_abstentions, 2);
    assert_eq!(record.report.total.pending_batches, 2);
    assert_eq!(record.report.total.pending_child_requests, 2);
    assert_eq!(record.session.private_states.len(), 3);
    assert_eq!(
        record.session.private_states["c"].thought.as_deref(),
        Some("fixture-private-c")
    );
    let public = std::fs::read_to_string(run.join("public.json")).unwrap();
    assert!(!public.contains("fixture-private"));
    let stdout = success(
        binary()
            .arg("report")
            .arg("--record")
            .arg(run.join("operator-record.json"))
            .output()
            .unwrap(),
    )
    .stdout;
    assert_eq!(
        serde_json::from_slice::<Report>(&stdout).unwrap(),
        record.report
    );
    assert!(!String::from_utf8_lossy(&stdout).contains("fixture-private"));
    check_replay(&run, &root.path().join("replay"));
    let existing = binary()
        .arg("run")
        .arg("--config")
        .arg(example("fixture-experiment.json"))
        .arg("--output")
        .arg(&run)
        .output()
        .unwrap();
    assert!(!existing.status.success());
    let catalog = success(
        binary()
            .arg("catalog")
            .arg("--config")
            .arg(example("fixture-experiment.json"))
            .output()
            .unwrap(),
    );
    let catalog: serde_json::Value = serde_json::from_slice(&catalog.stdout).unwrap();
    assert_eq!(
        catalog["plans"][0]["fingerprint"],
        record.config.plan_fingerprint(&record.config.plans[0])
    );
}

#[test]
fn cli_sibling_plan_records_requests_only() {
    let root = tempfile::tempdir().unwrap();
    let run = root.path().join("siblings");
    let record = fixture("sibling-experiment.json", &run);
    let counts = &record.report.total;
    assert_eq!(counts.social_pairs, 1);
    assert_eq!(counts.agreed_plan_batches, 1);
    assert_eq!(counts.pending_batches, 1);
    assert_eq!(counts.pending_child_requests, 2);
    assert_eq!(counts.executed_merges, 0);
    assert_eq!(counts.admitted_children, 0);
    let children = &record.session.batches[0].children;
    assert_eq!(children[0].parents, children[1].parents);
    assert_eq!(children[0].base, children[1].base);
    assert_ne!(children[0].recipe.method, children[1].recipe.method);
    check_replay(&run, &root.path().join("replay"));
}

async fn read_request(socket: &mut tokio::net::TcpStream) -> serde_json::Value {
    use tokio::io::AsyncReadExt;
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
                        .map(|v| v.trim().parse::<usize>().unwrap())
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
    serde_json::from_slice(&bytes[offset..offset + length]).unwrap()
}

fn local_config(endpoint: String, root: &Path) -> PathBuf {
    let mut config: Experiment = record::read_json(&example("fixture-experiment.json")).unwrap();
    config.rounds = 1;
    for agent in &mut config.agents {
        agent.inference = Some(lovers_lagoon::config::InferenceProvenance {
            server: "test-double".into(),
            server_version: "fixture".into(),
            grammar_backend: "fixture".into(),
            model_revision: "f".repeat(40),
            chat_template_sha256: "f".repeat(64),
            conformance_report_sha256: None,
        });
        agent.backend = BackendConfig::LocalHttp {
            endpoint: endpoint.clone(),
            model: "test-local".into(),
            tokenizer: TokenizerConfig::Vllm {
                endpoint: endpoint.clone(),
            },
        };
    }
    let path = root.join("config.json");
    std::fs::write(&path, serde_json::to_vec(&config).unwrap()).unwrap();
    path
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_local_http_pipeline() {
    use tokio::{io::AsyncWriteExt, net::TcpListener};
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!(
        "http://{}/v1/chat/completions",
        listener.local_addr().unwrap()
    );
    let server = tokio::spawn(async move {
        let mut owners = Vec::new();
        while owners.len() < 6 {
            let (mut socket, _) = listener.accept().await.unwrap();
            let request = read_request(&mut socket).await;
            if request.get("response_format").is_none() {
                let body = r#"{"count":500,"max_model_len":8192}"#;
                socket.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
                continue;
            }
            let observation: serde_json::Value =
                serde_json::from_str(request["messages"][1]["content"].as_str().unwrap()).unwrap();
            let owner = observation["owner"].as_str().unwrap();
            assert!(observation.get("procedure").is_none());
            assert!(observation["plans"][0].get("fingerprint").is_none());
            assert_eq!(request["response_format"]["type"], "json_schema");
            assert_eq!(request["response_format"]["json_schema"]["strict"], true);
            let serialized = observation.to_string();
            let aliases: Vec<_> = ["a", "b", "c"]
                .iter()
                .map(|h| {
                    (
                        *h,
                        lovers_lagoon::config::fingerprint(&("lagoon-alias-v1", 9_u64, h)),
                    )
                })
                .collect();
            let owner_handle = aliases
                .iter()
                .find(|(_, hash)| owner == format!("p-{}", &hash[..16]))
                .unwrap()
                .0;
            for other in ["a", "b", "c"].into_iter().filter(|a| *a != owner_handle) {
                assert!(!serialized.contains(&format!("fixture-private-{other}")));
            }
            assert!(serialized.contains(&format!("fixture-private-{owner_handle}")));
            let mut content = if observation["phase"]["kind"] == "communication" {
                serde_json::json!({"public_message":format!("public-{owner}")})
            } else {
                let partner = match owner_handle {
                    "a" => Some(format!("p-{}", &aliases[1].1[..16])),
                    "b" => Some(format!("p-{}", &aliases[0].1[..16])),
                    _ => None,
                };
                serde_json::json!({"partner":partner,"consent":{"state":"agree","plan_id":observation["plans"][0]["plan"]["id"]}})
            };
            if owner_handle == "c" && observation["phase"]["kind"] == "selection" {
                content["consent"]["fingerprint"] =
                    serde_json::json!("obsolete transposed fingerprint");
            }
            let content = if owner_handle == "b" && observation["phase"]["kind"] == "selection" {
                format!("```json\n{content}\n```")
            } else {
                content.to_string()
            };
            let body = serde_json::json!({"choices":[{"message":{"content":content}}]}).to_string();
            socket
                .write_all(
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                        body.len()
                    )
                    .as_bytes(),
                )
                .await
                .unwrap();
            owners.push(owner.to_string());
        }
        owners
    });
    let root = tempfile::tempdir().unwrap();
    let config = local_config(endpoint, root.path());
    let run = root.path().join("run");
    let output = tokio::task::spawn_blocking(move || {
        binary()
            .arg("run")
            .arg("--config")
            .arg(config)
            .arg("--output")
            .arg(run)
            .output()
            .unwrap()
    })
    .await
    .unwrap();
    success(output);
    let owners = server.await.unwrap();
    assert_eq!(owners.len(), 6);
    let record: Record = record::read_json(&root.path().join("run/operator-record.json")).unwrap();
    assert_eq!(record.report.total.social_pairs, 1);
    assert_eq!(record.report.total.pending_batches, 1);
    assert_eq!(record.report.total.failed_decisions, 0);
    assert_eq!(record.report.fenced_json_responses, 1);
    assert_eq!(record.report.total.consent_counts["agree"], 3);
    check_replay(&root.path().join("run"), &root.path().join("replay"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_local_failures_preserve_status() {
    use tokio::{io::AsyncWriteExt, net::TcpListener};
    for (body, status, delay, expected) in [
        ("BODY_CANARY", "200 OK", 0, "malformed_content"),
        ("BODY_CANARY", "500 Failed", 0, "http_failure"),
        ("{}", "200 OK", 200, "timeout"),
    ] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}/", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let _ = read_request(&mut socket).await;
            let token_body = r#"{"count":500,"max_model_len":8192}"#;
            socket.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{token_body}",token_body.len()).as_bytes()).await.unwrap();
            let (mut socket, _) = listener.accept().await.unwrap();
            let _ = read_request(&mut socket).await;
            tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
            let _=socket.write_all(format!("HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await;
        });
        let root = tempfile::tempdir().unwrap();
        let config_path = local_config(endpoint, root.path());
        let mut config: Experiment = record::read_json(&config_path).unwrap();
        config.agents.truncate(1);
        config.communication_steps = 0;
        config.timeout_ms = 100;
        std::fs::write(&config_path, serde_json::to_vec(&config).unwrap()).unwrap();
        let output_path = root.path().join("run");
        let output = tokio::task::spawn_blocking(move || {
            binary()
                .arg("run")
                .arg("--config")
                .arg(config_path)
                .arg("--output")
                .arg(output_path)
                .output()
                .unwrap()
        })
        .await
        .unwrap();
        let output = success(output);
        assert!(!String::from_utf8_lossy(&output.stdout).contains("BODY_CANARY"));
        assert!(!String::from_utf8_lossy(&output.stderr).contains("BODY_CANARY"));
        let record: Record =
            record::read_json(&root.path().join("run/operator-record.json")).unwrap();
        assert_eq!(record.report.total.failed_decisions, 1);
        assert_eq!(record.report.total.voluntary_abstentions, 0);
        assert_eq!(record.report.total.selection_status_counts[expected], 1);
        check_replay(&root.path().join("run"), &root.path().join("replay"));
        server.abort();
        let _ = server.await;
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_llama_template_tokenization_and_generation_share_projection() {
    use tokio::{io::AsyncWriteExt, net::TcpListener};
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let mut template_messages = None;
        for stage in 0..3 {
            let (mut socket, _) = listener.accept().await.unwrap();
            let request = read_request(&mut socket).await;
            let body = match stage {
                0 => {
                    template_messages = Some(request["messages"].clone());
                    serde_json::json!({"prompt":"<BOS>templated chat<ASSISTANT>"})
                }
                1 => {
                    assert_eq!(request["content"],"<BOS>templated chat<ASSISTANT>");
                    assert_eq!(request["add_special"],true);
                    assert_eq!(request["parse_special"],true);
                    serde_json::json!({"tokens":[1,2,3,4,5]})
                }
                _ => {
                    assert_eq!(Some(request["messages"].clone()),template_messages);
                    assert_eq!(request["response_format"]["type"],"json_schema");
                    serde_json::json!({"choices":[{"message":{"content":"{\"partner\":null,\"consent\":{\"state\":\"defer\"},\"private_update\":null}"}}]})
                }
            }.to_string();
            socket
                .write_all(
                    format!(
                        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                        body.len()
                    )
                    .as_bytes(),
                )
                .await
                .unwrap();
        }
    });
    let root = tempfile::tempdir().unwrap();
    let path = local_config(endpoint.clone(), root.path());
    let mut config: Experiment = record::read_json(&path).unwrap();
    config.agents.truncate(1);
    config.communication_steps = 0;
    if let BackendConfig::LocalHttp { tokenizer, .. } = &mut config.agents[0].backend {
        *tokenizer = TokenizerConfig::LlamaCpp {
            template_endpoint: endpoint.clone(),
            tokenize_endpoint: endpoint,
        };
    }
    std::fs::write(&path, serde_json::to_vec(&config).unwrap()).unwrap();
    let run = root.path().join("run");
    let output = tokio::task::spawn_blocking(move || {
        binary()
            .arg("run")
            .arg("--config")
            .arg(path)
            .arg("--output")
            .arg(run)
            .output()
            .unwrap()
    })
    .await
    .unwrap();
    success(output);
    server.await.unwrap();
    let record: Record = record::read_json(&root.path().join("run/operator-record.json")).unwrap();
    assert_eq!(
        record.session.calls[0].token_trials[0].count.prompt_tokens,
        5
    );
    assert_eq!(record.report.total.voluntary_abstentions, 1);
    check_replay(&root.path().join("run"), &root.path().join("replay"));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cli_tokenizer_http_failure_prevents_generation_and_redacts_body() {
    use tokio::{io::AsyncWriteExt, net::TcpListener};
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let request = read_request(&mut socket).await;
        assert!(request.get("response_format").is_none());
        socket.write_all(b"HTTP/1.1 500 Failed\r\nContent-Length: 12\r\nConnection: close\r\n\r\nTOKEN_CANARY").await.unwrap();
    });
    let root = tempfile::tempdir().unwrap();
    let path = local_config(endpoint, root.path());
    let mut config: Experiment = record::read_json(&path).unwrap();
    config.agents.truncate(1);
    config.communication_steps = 0;
    std::fs::write(&path, serde_json::to_vec(&config).unwrap()).unwrap();
    let run = root.path().join("run");
    success(
        tokio::task::spawn_blocking(move || {
            binary()
                .arg("run")
                .arg("--config")
                .arg(path)
                .arg("--output")
                .arg(run)
                .output()
                .unwrap()
        })
        .await
        .unwrap(),
    );
    server.await.unwrap();
    let record: Record = record::read_json(&root.path().join("run/operator-record.json")).unwrap();
    assert_eq!(
        record.report.total.selection_status_counts["tokenization_failure"],
        1
    );
    assert!(record.session.calls[0].token_trials.is_empty());
    assert!(
        !serde_json::to_string(&record)
            .unwrap()
            .contains("TOKEN_CANARY")
    );
    check_replay(&root.path().join("run"), &root.path().join("replay"));
}
