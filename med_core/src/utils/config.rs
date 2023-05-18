use std::path::Path;

use serde::Deserialize;

use crate::utils::error::MaskerError;

#[derive(Debug, Deserialize, Clone)]
pub struct JobConfig {
    pub mask_symbols: String,
    pub fields: Vec<String>,
}

impl JobConfig {
    pub async fn new(path: &Path) -> Result<Self, MaskerError> {
        let f = std::fs::File::open(path)?;
        let config: JobConfig = serde_yaml::from_reader(f)?;
        Ok(config)
    }
}

#[cfg(test)]
#[path = "./tests/config_test.rs"]
mod config_test;
