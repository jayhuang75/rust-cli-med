use async_trait::async_trait;
use crate::utils::config::JobConfig;
use crate::utils::error::MaskerError;
use crate::cmd::cli::Cli;

#[async_trait(?Send)]
pub trait Producer {
    async fn load(&self) -> Result<(), MaskerError>;
    async fn run(&self) -> Result<(), MaskerError>;
    async fn write(&self) -> Result<(), MaskerError>;
}

// pub struct ProcessJson {
//     pub data: Vec<serde_json::Value>
// }

pub struct FileProcessor {
    pub params: Cli,
    pub job_conf: JobConfig,
    pub producer: Box<dyn Producer>,
}

impl FileProcessor {
    pub async fn load(&self) -> Result<(), MaskerError> {
        let _load = self.producer.load().await?;
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

    pub async fn new(params: Cli, job_conf: JobConfig, producer: Box<dyn Producer>) -> Self {
        FileProcessor { 
            params: params, 
            job_conf,
            producer 
        }
    }
}

