use std::path::Path;

use crate::{cmd::cli::Cli, utils::error::MaskerError, utils::config::JobConfig};
use tracing_subscriber::fmt::format;

pub struct App{
    pub params: Cli,
}

impl App {
    pub async fn new() -> Self {

        let params = Cli::new().await;

        Self::logging(params.debug).await;

        App {params}
    }

    async fn load_job_config(&self) -> Result<JobConfig, MaskerError>{
        let conf = JobConfig::new(Path::new(&self.params.conf_path)).await?;
        Ok(conf)
    }

    async fn logging(debug: bool) {      
        let subscriber = tracing_subscriber::fmt() // disabling time is handy because CloudWatch will add the ingestion time.
            .with_timer(tracing_subscriber::fmt::time::uptime())
            .with_line_number(true)
            .with_thread_names(true)
            .event_format(format().compact());

        match debug {
            true => {
                subscriber.with_max_level(tracing::Level::DEBUG).init();
            }
            false => {
                subscriber.with_max_level(tracing::Level::INFO).init();
            }
        }
    }
}