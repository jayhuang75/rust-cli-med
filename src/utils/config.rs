use std::path::Path;

use serde::Deserialize;

use crate::utils::error::MaskerError;

#[derive(Debug, Deserialize)]
pub struct JobConfig {
    pub fields: Vec<String>,
    pub aes: i16,
}

impl JobConfig {
    /// Returns new JobConfig
    ///
    /// # Examples
    /// 
    /// ```
    /// let new_job_config = JobConfig::new(Path).await?;
    /// ```
    /// 
    pub async fn new(path: &Path) -> Result<Self, MaskerError> {
        let f = std::fs::File::open(path)?;
        let config: JobConfig = serde_yaml::from_reader(f)?;
        Ok(config)
    }
}

#[cfg(test)]
#[path = "./tests/config_test.rs"]
mod config_test;