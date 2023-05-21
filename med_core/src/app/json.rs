// use async_trait::async_trait;
// use tracing::info;
// use crate::utils::config::JobConfig;
use crate::{models::metrics::Metrics, utils::error::MaskerError};
// use crate::cmd::cli::Cli;
// use crate::cmd::worker::Worker;

#[derive(Debug, Clone, Default)]
pub struct JsonFile {
    pub path: String,
    pub total_keys: usize,
    pub failed_keys: usize,
    pub key_failed_reason: Vec<MaskerError>,
    pub keys: Vec<String>,
    pub data: Vec<serde_json::Value>,
}

pub struct JsonFileProcessor {
    pub metrics: Metrics,
    pub result: Vec<JsonFile>,
}
