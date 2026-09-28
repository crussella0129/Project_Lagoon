use clap::{Parser, Subcommand};
use lovers_lagoon::{
    config::Experiment,
    observation::Observation,
    protocol::{Phase, PrivateState, PublicView},
    record::{self, Record, RecordError},
    replay,
};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

#[derive(Parser)]
#[command(
    version,
    about = "Run and replay voluntary partner-choice experiments; fusion requests are never executed"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Inspect one initial runtime contract without inference (operator-only output).
    Schema {
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        owner: String,
        #[arg(long, default_value = "selection")]
        phase: SchemaPhase,
    },
    /// Run explicitly configured fixtures or literal-loopback HTTP backends.
    Run {
        #[arg(long)]
        config: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Validate and reproduce recorded artifacts without inference.
    Replay {
        #[arg(long)]
        record: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// Validate a record and print descriptive aggregate metrics.
    Report {
        #[arg(long)]
        record: PathBuf,
    },
    /// Print offered plan cards and their exact consent fingerprints, without inference.
    Catalog {
        #[arg(long)]
        config: PathBuf,
    },
}

#[derive(Clone, clap::ValueEnum)]
enum SchemaPhase {
    Communication,
    Selection,
}

fn configuration(path: &Path) -> Result<Experiment, RecordError> {
    let value: Experiment = record::read_json(path)?;
    value
        .validate()
        .map_err(|_| RecordError("invalid experiment configuration"))?;
    Ok(value)
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), RecordError> {
    println!(
        "{}",
        serde_json::to_string_pretty(value).map_err(|_| RecordError("cannot serialize report"))?
    );
    Ok(())
}

async fn execute(cli: Cli) -> Result<(), RecordError> {
    match cli.command {
        Command::Schema {
            config,
            owner,
            phase,
        } => {
            let config = configuration(&config)?;
            let agent = config
                .agents
                .iter()
                .find(|agent| agent.handle == owner)
                .ok_or(RecordError("unknown schema owner"))?;
            let phase = match phase {
                SchemaPhase::Communication => Phase::Communication { step: 0 },
                SchemaPhase::Selection => Phase::Selection,
            };
            let observation = Observation::new(
                &config,
                &owner,
                &agent.initial_private,
                &PublicView::default(),
                0,
                phase.clone(),
            );
            let response_format = observation.response_format();
            print_json(&serde_json::json!({
                "config_fingerprint":lovers_lagoon::config::fingerprint(&config),
                "response_schema_fingerprint":lovers_lagoon::config::fingerprint(&response_format),
                "response_format":response_format,"messages":observation.messages(),
                "seed":lovers_lagoon::config::call_seed(config.seed,0,&phase,&owner),
                "inference":agent.inference,"backend":agent.backend,
                "decoding":config.decoding
            }))
        }
        Command::Run { config, output } => {
            let config = configuration(&config)?;
            if output.exists() {
                return Err(RecordError("output directory must be new and writable"));
            }
            let record = record::capture(config).await?;
            record::save(&record, &output)?;
            print_json(&record.report)
        }
        Command::Replay {
            record: input,
            output,
        } => {
            let record: Record = record::read_json(&input)?;
            replay::reconstruct(&record)?;
            record::save(&record, &output)?;
            print_json(&record.report)
        }
        Command::Report { record: input } => {
            let record: Record = record::read_json(&input)?;
            replay::reconstruct(&record)?;
            print_json(&record.report)
        }
        Command::Catalog { config } => {
            let config = configuration(&config)?;
            let observation = Observation::new(
                &config,
                &config.agents[0].handle,
                &PrivateState::default(),
                &PublicView::default(),
                0,
                Phase::Selection,
            );
            print_json(
                &serde_json::json!({"mode":config.mode,"max_siblings":config.max_siblings,"plans":observation.plans,
                    "operator_aliases":config.agents.iter().map(|a| (&a.handle,config.alias(&a.handle))).collect::<std::collections::BTreeMap<_,_>>()}),
            )
        }
    }
}

#[tokio::main]
async fn main() -> ExitCode {
    match execute(Cli::parse()).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
