use std::path::Path;

use serde::Deserialize;
use tracing::{debug, info};
use tracing_subscriber::fmt::format;

use crate::utils::error::MaskerError;

#[derive(Debug, Deserialize)]
pub struct JobConfig {
    pub fields: Vec<String>,
    pub aes: i16,
}

impl JobConfig {
    pub async fn new(path: &Path) -> Result<Self, MaskerError> {
        let f = std::fs::File::open(path)?;
        let config: JobConfig = serde_yaml::from_reader(f)?;
        Ok(config)
    }

    // pub async fn tracing(&self) {
    //     let subscriber = tracing_subscriber::fmt() // disabling time is handy because CloudWatch will add the ingestion time.
    //         .with_timer(tracing_subscriber::fmt::time::uptime())
    //         .with_line_number(true)
    //         .with_thread_names(true)
    //         .event_format(format().compact());

    //     match self.debug {
    //         true => {
    //             subscriber.with_max_level(tracing::Level::DEBUG).init();
    //         }
    //         false => {
    //             subscriber.with_max_level(tracing::Level::INFO).init();
    //         }
    //     }
    // }
}

#[cfg(test)]
#[path = "./tests/config_test.rs"]
mod config_test;