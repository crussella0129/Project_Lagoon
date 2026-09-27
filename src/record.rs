use crate::{
    config::Experiment,
    report::{Report, describe},
    runner::{self, Session},
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
pub const SCHEMA_VERSION: u32 = 2;
pub const MAX_FILE_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct RecordError(pub &'static str);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub schema_version: u32,
    pub harness_version: String,
    pub started_unix_ms: u64,
    pub finished_unix_ms: u64,
    pub config: Experiment,
    pub config_fingerprint: String,
    pub session: Session,
    pub report: Report,
}

fn timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(u64::MAX as u128) as u64
}

pub async fn capture(config: Experiment) -> Result<Record, RecordError> {
    config
        .validate()
        .map_err(|_| RecordError("invalid experiment configuration"))?;
    let started = timestamp();
    let session = runner::run(&config)
        .await
        .map_err(|_| RecordError("cannot initialize configured backends"))?;
    Ok(Record::from_session(
        config,
        session,
        started,
        timestamp().max(started),
    ))
}

impl Record {
    pub fn from_session(
        config: Experiment,
        session: Session,
        started_unix_ms: u64,
        finished_unix_ms: u64,
    ) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            harness_version: env!("CARGO_PKG_VERSION").into(),
            started_unix_ms,
            finished_unix_ms,
            config_fingerprint: crate::config::fingerprint(&config),
            report: describe(&config, &session),
            config,
            session,
        }
    }
}

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, RecordError> {
    let file = File::open(path).map_err(|_| RecordError("cannot open input file"))?;
    if file
        .metadata()
        .map_err(|_| RecordError("cannot inspect input file"))?
        .len()
        > MAX_FILE_BYTES
    {
        return Err(RecordError("input file exceeds 64 MiB limit"));
    }
    let mut bytes = Vec::new();
    file.take(MAX_FILE_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| RecordError("cannot read input file"))?;
    if bytes.len() as u64 > MAX_FILE_BYTES {
        return Err(RecordError("input file exceeds 64 MiB limit"));
    }
    serde_json::from_slice(&bytes).map_err(|_| RecordError("invalid input JSON or schema"))
}

struct BoundedBuffer(Vec<u8>);
impl Write for BoundedBuffer {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if bytes.len() as u64 > MAX_FILE_BYTES - self.0.len() as u64 {
            return Err(std::io::Error::other("artifact size limit"));
        }
        self.0.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>, RecordError> {
    let mut buffer = BoundedBuffer(Vec::new());
    serde_json::to_writer_pretty(&mut buffer, value)
        .map_err(|_| RecordError("artifact exceeds size limit or cannot serialize"))?;
    Ok(buffer.0)
}

pub fn save(record: &Record, output: &Path) -> Result<(), RecordError> {
    crate::replay::reconstruct(record)?;
    let artifacts = [
        ("operator-record.json", encode(record)?),
        ("public.json", encode(&record.session.public)?),
        ("report.json", encode(&record.report)?),
        ("merge-requests.json", encode(&record.session.batches)?),
    ];
    if let Some(parent) = output.parent().filter(|p| !p.as_os_str().is_empty()) {
        fs::create_dir_all(parent).map_err(|_| RecordError("cannot create output parent"))?;
    }
    fs::create_dir(output).map_err(|_| RecordError("output directory must be new and writable"))?;
    let mut created = Vec::new();
    let result = (|| {
        for (name, bytes) in artifacts {
            let path = output.join(name);
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .map_err(|_| RecordError("cannot create output artifact"))?;
            created.push(path);
            file.write_all(&bytes)
                .and_then(|_| file.sync_all())
                .map_err(|_| RecordError("cannot persist output artifact"))?;
        }
        Ok(())
    })();
    if result.is_err() {
        for path in created {
            let _ = fs::remove_file(path);
        }
        let _ = fs::remove_dir(output);
    }
    result
}
