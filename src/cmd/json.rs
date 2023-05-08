use async_trait::async_trait;
use tracing::info;
use crate::utils::config::JobConfig;
use crate::utils::error::MaskerError;
use crate::cmd::process::Producer;
use crate::cmd::cli::Cli;
use crate::cmd::worker::Worker;

pub struct JsonFile {
    pub data: Option<Vec<serde_json::Value>>
}

impl Default for JsonFile{
    fn default() -> Self {
        Self { data: None }
    }
}

#[async_trait(?Send)]
impl Producer for JsonFile {
    async fn load(&self, params: &Cli, job_conf: &JobConfig) -> Result<(), MaskerError> {
        info!("json loaded");
        let new_worker = Worker::new(params.worker).await?;
        Ok(())
    }
    async fn run(&self) -> Result<(), MaskerError> {
        info!("json run");
        Ok(())
    }
    async fn write(&self) -> Result<(), MaskerError> {
        info!("json write");
        Ok(())
    }
}