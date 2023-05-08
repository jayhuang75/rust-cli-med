use async_trait::async_trait;
use csv::StringRecord;
use tracing::info;
use crate::utils::config::JobConfig;
use crate::utils::error::MaskerError;
use crate::cmd::process::Producer;
use crate::cmd::cli::Cli;
use crate::cmd::worker::Worker;

#[derive(Debug)]
pub struct CsvFile {
    pub headers: Option<StringRecord>,
    pub data: Option<Vec<StringRecord>>,
}

impl Default for CsvFile{
    fn default() -> Self {
        Self { headers: None, data: None }
    }
}

#[async_trait(?Send)]
impl Producer for CsvFile {
    async fn load(&self, params: &Cli, job_conf: &JobConfig) -> Result<(), MaskerError> {
        info!("csv loaded");
        let new_worker = Worker::new(params.worker).await?;
        Ok(())
    }
    async fn run(&self) -> Result<(), MaskerError> {
        info!("csv run");
        Ok(())
    }
   async fn write(&self) -> Result<(), MaskerError> {
        info!("csv write");
        Ok(())
    }
}

