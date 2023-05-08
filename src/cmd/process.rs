use crate::cmd::cli::Cli;
use crate::utils::config::JobConfig;
use crate::utils::error::MaskerError;
use async_trait::async_trait;
use walkdir::WalkDir;

use crate::cmd::worker::Worker;

#[async_trait(?Send)]
pub trait Producer {
    async fn load(&self, params: &Cli, job_conf: &JobConfig) -> Result<(), MaskerError>;
    async fn run(&self) -> Result<(), MaskerError>;
    async fn write(&self) -> Result<(), MaskerError>;
}

pub struct FileProcessor {
    pub params: Cli,
    pub job_conf: JobConfig,
    pub worker: Worker,
    pub producer: Box<dyn Producer>,
}

impl FileProcessor {
    pub async fn load(&self) -> Result<(), MaskerError> {
        let _l = self.producer.load(&self.params, &self.job_conf).await?;
        Ok(())
    }

    pub async fn run(&self) -> Result<(), MaskerError> {
        let _p = self.producer.run().await?;
        Ok(())
    }

    pub async fn write(&self) -> Result<(), MaskerError> {
        let _w = self.producer.write().await?;
        Ok(())
    }

    pub async fn new(
        params: Cli,
        worker: Worker,
        job_conf: JobConfig,
        producer: Box<dyn Producer>,
    ) -> Self {
        FileProcessor {
            params: params,
            job_conf,
            worker,
            producer,
        }
    }
}
