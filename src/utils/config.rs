use serde::Deserialize;
use tracing::{debug, info};
use tracing_subscriber::fmt::format;

use crate::utils::error::MaskerError;

use super::error::MaskerErrorType;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub fields: Vec<String>,
    pub aes: i16,
    pub debug: bool,
}

impl Config {
    pub async fn new() -> Result<Self, MaskerError> {
        let f = std::fs::File::open("conf.yml").expect(&MaskerError::message(&MaskerError {
            cause: Some("failed to load the conf.yml".to_owned()),
            message: Some("missing conf.yml".to_owned()),
            error_type: MaskerErrorType::ConfigError,
        }));
        let config: Config = serde_yaml::from_reader(f)?;
        Ok(config)
    }

    pub async fn tracing(&self) {
        let subscriber = tracing_subscriber::fmt() // disabling time is handy because CloudWatch will add the ingestion time.
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_line_number(true)
        .with_thread_names(true)
        .event_format(format().compact());

        match self.debug {
            true => {
                subscriber.with_max_level(tracing::Level::DEBUG).init();
            }
            false => {
                subscriber.with_max_level(tracing::Level::INFO).init();
            }
            _ => {
                subscriber.with_max_level(tracing::Level::INFO).init();
            },
        }
    }
}
