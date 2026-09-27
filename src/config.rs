use std::{collections::BTreeSet, net::IpAddr};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::protocol::{Phase, PrivateState};

#[derive(Debug, thiserror::Error)]
#[error("invalid experiment configuration: {0}")]
pub struct ConfigError(pub &'static str);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Experiment {
    pub seed: u64,
    pub rounds: u32,
    pub communication_steps: u32,
    pub workers: usize,
    pub timeout_ms: u64,
    pub max_response_bytes: usize,
    pub max_context_bytes: usize,
    pub memory: MemoryPolicy,
    pub max_private_bytes: usize,
    pub max_siblings: usize,
    pub mode: Mode,
    pub decoding: Decoding,
    pub agents: Vec<Agent>,
    pub recipes: Vec<Recipe>,
    pub plans: Vec<ReproductionPlan>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    FixedPlan,
    MutualChoice,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Decoding {
    pub temperature: f64,
    pub max_tokens: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MemoryPolicy {
    pub recent_rounds: u32,
    pub max_public_events: usize,
    pub max_prompt_tokens: u32,
    pub context_window_tokens: u32,
    pub safety_margin_tokens: u32,
}

impl Default for MemoryPolicy {
    fn default() -> Self {
        Self {
            recent_rounds: 2,
            max_public_events: 64,
            max_prompt_tokens: 6144,
            context_window_tokens: 8192,
            safety_margin_tokens: 128,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum TokenizerConfig {
    Vllm {
        endpoint: String,
    },
    LlamaCpp {
        template_endpoint: String,
        tokenize_endpoint: String,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Agent {
    pub handle: String,
    pub checkpoint: Option<Checkpoint>,
    pub merge_metadata: Option<MergeMetadata>,
    #[serde(default)]
    pub initial_private: PrivateState,
    pub backend: BackendConfig,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Checkpoint {
    pub model: String,
    pub revision: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MergeMetadata {
    pub base: Checkpoint,
    pub architecture: String,
    pub tensor_layout: String,
    pub tokenizer: String,
    pub license: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum BackendConfig {
    Fixture {
        responses: Vec<FixtureResponse>,
    },
    LocalHttp {
        endpoint: String,
        model: String,
        tokenizer: TokenizerConfig,
    },
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FixtureResponse {
    pub round: u32,
    pub phase: Phase,
    #[serde(default)]
    pub delay_ms: u64,
    pub body: Option<String>,
    #[serde(default)]
    pub transport_failure: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Recipe {
    pub id: String,
    pub method: Method,
    pub parent_weights: [f64; 2],
    pub density: Option<f64>,
    pub epsilon: Option<f64>,
    pub lambda: f64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    Linear,
    Ties,
    DareTies,
    Della,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChildRecipe {
    pub recipe_id: String,
    pub seed: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReproductionPlan {
    pub id: String,
    pub children: Vec<ChildRecipe>,
    pub max_compute_seconds: u64,
    pub max_output_bytes: u64,
}

pub fn fingerprint<T: Serialize>(value: &T) -> String {
    // All fingerprinted catalog types have fixed field order and no unordered maps.
    let bytes = serde_json::to_vec(value).expect("typed finite catalog is serializable");
    format!("{:x}", Sha256::digest(bytes))
}

impl Experiment {
    pub fn validate(&self) -> Result<(), ConfigError> {
        let require = |condition, message| {
            if condition {
                Ok(())
            } else {
                Err(ConfigError(message))
            }
        };
        require((1..=100).contains(&self.rounds), "rounds must be 1..=100")?;
        require(
            self.communication_steps <= 10,
            "too many communication steps",
        )?;
        require(
            (1..=128).contains(&self.agents.len()),
            "agent count must be 1..=128",
        )?;
        require((1..=128).contains(&self.workers), "workers must be 1..=128")?;
        require(
            (1..=300_000).contains(&self.timeout_ms),
            "invalid call timeout",
        )?;
        require(
            (128..=1_048_576).contains(&self.max_response_bytes),
            "invalid response bound",
        )?;
        require(
            (1024..=4_194_304).contains(&self.max_context_bytes),
            "invalid context bound",
        )?;
        require(
            (1..=65_536).contains(&self.max_private_bytes),
            "invalid private-state bound",
        )?;
        require(
            (1..=3).contains(&self.max_siblings),
            "sibling limit must be 1..=3",
        )?;
        require(
            self.decoding.temperature.is_finite()
                && (0.0..=2.0).contains(&self.decoding.temperature),
            "invalid temperature",
        )?;
        require(
            (1..=8192).contains(&self.decoding.max_tokens),
            "invalid token bound",
        )?;
        require(
            self.memory.recent_rounds <= 100
                && self.memory.max_public_events <= 2048
                && (256..=1_048_576).contains(&self.memory.max_prompt_tokens)
                && self.memory.context_window_tokens <= 1_048_576
                && self.memory.max_prompt_tokens as u64
                    + self.decoding.max_tokens as u64
                    + self.memory.safety_margin_tokens as u64
                    <= self.memory.context_window_tokens as u64,
            "invalid memory policy or insufficient output token reserve",
        )?;
        require(
            !self.recipes.is_empty() && self.recipes.len() <= 32,
            "invalid recipe count",
        )?;
        require(
            !self.plans.is_empty() && self.plans.len() <= 32,
            "invalid plan count",
        )?;
        require(
            self.mode != Mode::FixedPlan || self.plans.len() == 1,
            "fixed mode requires one plan",
        )?;
        let mut handles = BTreeSet::new();
        for agent in &self.agents {
            require(
                valid_id(&agent.handle) && handles.insert(&agent.handle),
                "invalid or duplicate handle",
            )?;
            require(
                agent.initial_private.byte_len() <= self.max_private_bytes,
                "initial private state exceeds bound",
            )?;
            if let Some(checkpoint) = &agent.checkpoint {
                validate_checkpoint(checkpoint)?;
            }
            if let Some(meta) = &agent.merge_metadata {
                validate_checkpoint(&meta.base)?;
                require(
                    [
                        &meta.architecture,
                        &meta.tensor_layout,
                        &meta.tokenizer,
                        &meta.license,
                    ]
                    .into_iter()
                    .all(|s| valid_metadata(s)),
                    "malformed merge metadata",
                )?;
            }
            match &agent.backend {
                BackendConfig::LocalHttp {
                    endpoint,
                    model,
                    tokenizer,
                } => {
                    validate_endpoint(endpoint)?;
                    require(valid_metadata(model), "missing local model")?;
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
                }
                BackendConfig::Fixture { responses } => {
                    require(
                        responses.len() <= (self.rounds * (self.communication_steps + 1)) as usize,
                        "too many fixture responses",
                    )?;
                    let mut keys = BTreeSet::new();
                    for response in responses {
                        require(response.round < self.rounds, "invalid fixture round")?;
                        let phase_key = match response.phase {
                            Phase::Communication { step } => {
                                require(step < self.communication_steps, "invalid fixture step")?;
                                step
                            }
                            Phase::Selection => self.communication_steps,
                        };
                        require(
                            keys.insert((response.round, phase_key)),
                            "duplicate fixture response",
                        )?;
                        require(response.delay_ms <= 300_001, "fixture delay too large")?;
                        require(
                            response.body.as_ref().is_none_or(|s| s.len() <= 2_097_152),
                            "fixture body too large",
                        )?;
                    }
                }
            }
        }
        let mut recipes = BTreeSet::new();
        require(
            self.agents
                .iter()
                .map(|a| self.alias(&a.handle))
                .collect::<BTreeSet<_>>()
                .len()
                == self.agents.len(),
            "public alias collision",
        )?;
        for recipe in &self.recipes {
            require(
                valid_id(&recipe.id) && recipes.insert(&recipe.id),
                "invalid or duplicate recipe",
            )?;
            require(
                recipe.parent_weights[0].is_finite()
                    && recipe.parent_weights[0] > 0.0
                    && recipe.parent_weights[0] <= 1.0
                    && recipe.parent_weights[0] == recipe.parent_weights[1],
                "parent weights must be finite positive symmetric coefficients",
            )?;
            require(
                recipe.lambda.is_finite() && recipe.lambda > 0.0 && recipe.lambda <= 2.0,
                "invalid lambda",
            )?;
            match recipe.method {
                Method::Linear => require(
                    recipe.density.is_none() && recipe.epsilon.is_none(),
                    "linear recipe cannot prune",
                )?,
                Method::Ties | Method::DareTies | Method::Della => {
                    let density = recipe.density.ok_or(ConfigError("missing density"))?;
                    require(
                        density.is_finite() && density > 0.0 && density <= 1.0,
                        "invalid density",
                    )?;
                    if recipe.method == Method::Della {
                        let epsilon = recipe.epsilon.ok_or(ConfigError("missing DELLA epsilon"))?;
                        require(
                            epsilon.is_finite()
                                && epsilon >= 0.0
                                && density - epsilon > 0.0
                                && density + epsilon < 1.0,
                            "invalid DELLA probability bounds",
                        )?;
                    } else {
                        require(recipe.epsilon.is_none(), "epsilon only applies to DELLA")?;
                    }
                }
            }
        }
        let mut plans = BTreeSet::new();
        for plan in &self.plans {
            require(
                valid_id(&plan.id) && plans.insert(&plan.id),
                "invalid or duplicate plan",
            )?;
            require(
                !plan.children.is_empty() && plan.children.len() <= self.max_siblings,
                "invalid sibling count",
            )?;
            require(
                (1..=86_400).contains(&plan.max_compute_seconds)
                    && (1..=1_099_511_627_776).contains(&plan.max_output_bytes),
                "invalid batch resource envelope",
            )?;
            let mut jobs = BTreeSet::new();
            for child in &plan.children {
                require(
                    recipes.contains(&child.recipe_id)
                        && jobs.insert((&child.recipe_id, child.seed)),
                    "missing recipe or duplicate child job",
                )?;
            }
        }
        Ok(())
    }

    pub fn plan_fingerprint(&self, plan: &ReproductionPlan) -> String {
        let resolved: Vec<_> = plan
            .children
            .iter()
            .map(|child| self.recipes.iter().find(|r| r.id == child.recipe_id))
            .collect();
        fingerprint(&(plan, resolved))
    }

    pub fn alias(&self, handle: &str) -> String {
        format!(
            "p-{}",
            &fingerprint(&("lagoon-alias-v1", self.seed, handle))[..16]
        )
    }

    pub fn handle_for_alias(&self, alias: &str) -> Option<String> {
        self.agents
            .iter()
            .find(|a| self.alias(&a.handle) == alias)
            .map(|a| a.handle.clone())
    }
}

fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
}

fn valid_metadata(value: &str) -> bool {
    !value.trim().is_empty() && value.len() <= 256 && !value.chars().any(char::is_control)
}

fn validate_checkpoint(value: &Checkpoint) -> Result<(), ConfigError> {
    if valid_metadata(&value.model)
        && value.revision.len() == 40
        && value.revision.bytes().all(|b| b.is_ascii_hexdigit())
    {
        Ok(())
    } else {
        Err(ConfigError(
            "checkpoints require a model and immutable 40-hex revision",
        ))
    }
}

pub fn validate_endpoint(endpoint: &str) -> Result<reqwest::Url, ConfigError> {
    let url = reqwest::Url::parse(endpoint).map_err(|_| ConfigError("invalid local endpoint"))?;
    let host = url.host_str().unwrap_or("").trim_matches(['[', ']']);
    let ip: IpAddr = host
        .parse()
        .map_err(|_| ConfigError("endpoint requires a literal loopback IP"))?;
    if url.scheme() != "http"
        || !ip.is_loopback()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(ConfigError(
            "endpoint must be HTTP loopback without credentials/query/fragment",
        ));
    }
    Ok(url)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub fn config() -> Experiment {
        Experiment {
            seed: 9,
            rounds: 2,
            communication_steps: 1,
            workers: 2,
            timeout_ms: 100,
            max_response_bytes: 4096,
            max_context_bytes: 100_000,
            memory: MemoryPolicy::default(),
            max_private_bytes: 1024,
            max_siblings: 1,
            mode: Mode::MutualChoice,
            decoding: Decoding {
                temperature: 0.0,
                max_tokens: 256,
            },
            agents: ["a", "b", "c"]
                .into_iter()
                .map(|handle| Agent {
                    handle: handle.into(),
                    checkpoint: None,
                    merge_metadata: None,
                    initial_private: PrivateState::default(),
                    backend: BackendConfig::Fixture { responses: vec![] },
                })
                .collect(),
            recipes: vec![Recipe {
                id: "ties".into(),
                method: Method::Ties,
                parent_weights: [0.5, 0.5],
                density: Some(0.5),
                epsilon: None,
                lambda: 1.0,
            }],
            plans: vec![ReproductionPlan {
                id: "single".into(),
                children: vec![ChildRecipe {
                    recipe_id: "ties".into(),
                    seed: 1,
                }],
                max_compute_seconds: 60,
                max_output_bytes: 1024,
            }],
        }
    }

    #[test]
    fn valid_fixture_and_local_configs() {
        let mut value = config();
        value.validate().unwrap();
        value.agents[0].backend = BackendConfig::LocalHttp {
            endpoint: "http://127.0.0.1:8000/v1/chat/completions".into(),
            model: "local".into(),
            tokenizer: TokenizerConfig::Vllm {
                endpoint: "http://127.0.0.1:8000/tokenize".into(),
            },
        };
        value.validate().unwrap();
        assert!(validate_endpoint("http://[::1]:8000/v1/chat/completions").is_ok());
        value.mode = Mode::FixedPlan;
        for method in [
            Method::Linear,
            Method::Ties,
            Method::DareTies,
            Method::Della,
        ] {
            value.recipes[0].method = method.clone();
            value.recipes[0].density = if method == Method::Linear {
                None
            } else {
                Some(0.5)
            };
            value.recipes[0].epsilon = if method == Method::Della {
                Some(0.1)
            } else {
                None
            };
            value.validate().unwrap();
        }
    }

    #[test]
    fn rejects_invalid_config_before_backend_call() {
        let mut value = config();
        value.agents[1].handle = "a".into();
        assert!(value.validate().is_err());
        for endpoint in [
            "http://localhost:8000/",
            "http://example.com/",
            "https://127.0.0.1/",
            "http://u:p@127.0.0.1/",
            "http://127.0.0.1/?token=x",
        ] {
            assert!(validate_endpoint(endpoint).is_err());
        }
        let mut value = config();
        value.rounds = 0;
        assert!(value.validate().is_err());
        let mut value = config();
        value.agents[0].checkpoint = Some(Checkpoint {
            model: "x".into(),
            revision: "main".into(),
        });
        assert!(value.validate().is_err());
    }

    #[test]
    fn rejects_invalid_bounds_and_incomplete_catalogs() {
        use serde_json::json;
        let original = serde_json::to_value(config()).unwrap();
        for (pointer, invalid) in [
            ("/workers", json!(0)),
            ("/rounds", json!(101)),
            ("/communication_steps", json!(11)),
            ("/timeout_ms", json!(300001)),
            ("/max_response_bytes", json!(127)),
            ("/max_context_bytes", json!(0)),
            ("/max_private_bytes", json!(0)),
            ("/decoding/temperature", json!(3.0)),
            ("/decoding/max_tokens", json!(0)),
            (
                "/agents/0/backend",
                json!({"kind":"local_http","endpoint":"http://127.0.0.1/","model":""}),
            ),
            (
                "/agents/0/backend",
                json!({"kind":"local_http","endpoint":"http://127.0.0.1/"}),
            ),
            (
                "/agents/0/merge_metadata",
                json!({"base":{"model":"base","revision":"a".repeat(40)},"architecture":"","tensor_layout":"x","tokenizer":"x","license":"x"}),
            ),
            ("/recipes", json!([])),
            ("/plans", json!([])),
            ("/plans/0/children", json!([])),
            ("/plans/0/max_compute_seconds", json!(0)),
            ("/plans/0/max_output_bytes", json!(0)),
            ("/recipes/0/lambda", json!(0)),
            ("/recipes/0/density", json!(1.1)),
            ("/recipes/0/method", json!("execute arbitrary code")),
        ] {
            let mut value = original.clone();
            *value.pointer_mut(pointer).unwrap() = invalid;
            assert!(
                !serde_json::from_value::<Experiment>(value).is_ok_and(|v| v.validate().is_ok()),
                "{pointer}"
            );
        }
        let mut value = config();
        let duplicate = value.recipes[0].clone();
        value.recipes.push(duplicate);
        assert!(value.validate().is_err());
        let mut value = config();
        let mut extra = value.plans[0].clone();
        extra.id = "extra".into();
        value.plans.push(extra);
        value.mode = Mode::FixedPlan;
        assert!(value.validate().is_err());
    }

    #[test]
    fn recipe_catalog_validates_ids_payloads_and_mode() {
        let mut value = config();
        value.recipes[0].parent_weights = [0.5, 0.4];
        assert!(value.validate().is_err());
        let mut value = config();
        value.recipes[0].density = Some(f64::NAN);
        assert!(value.validate().is_err());
        let mut value = config();
        let duplicate = value.plans[0].children[0].clone();
        value.plans[0].children.push(duplicate);
        assert!(value.validate().is_err());
        let mut value = config();
        value.plans[0].children[0].recipe_id = "missing".into();
        assert!(value.validate().is_err());
        let mut value = config();
        value.max_siblings = 4;
        assert!(value.validate().is_err());
        let mut value = config();
        value.recipes[0].method = Method::Della;
        value.recipes[0].epsilon = Some(0.5);
        assert!(value.validate().is_err());
        let mut value = config();
        let old = value.plan_fingerprint(&value.plans[0]);
        value.recipes[0].density = Some(0.4);
        assert_ne!(old, value.plan_fingerprint(&value.plans[0]));
        let unknown =
            serde_json::json!({"id":"x","method":"shell","parent_weights":[0.5,0.5],"lambda":1.0});
        assert!(serde_json::from_value::<Recipe>(unknown).is_err());
    }
}
