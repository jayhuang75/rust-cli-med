use serde::Deserialize;
use std::path::Path;

use crate::utils::error::{MaskerError, MaskerErrorType};

#[derive(Debug, Deserialize, Clone)]
pub struct JobConfig {
    pub mask_symbols: String,
    pub fields: Vec<String>,
}

impl JobConfig {
    pub async fn new(path: &Path) -> Result<Self, MaskerError> {
        let f = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(e) => {
                return Err(MaskerError {
                    message: Some(e.to_string()),
                    cause: Some("load job configuration yaml file failed!".to_string()),
                    error_type: MaskerErrorType::ConfigError,
                })
            }
        };
        let config: JobConfig = serde_yaml::from_reader(f)?;
        Ok(config)
    }
}

#[cfg(test)]
#[path = "./tests/config_test.rs"]
mod config_test;
