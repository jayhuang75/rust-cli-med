use async_trait::async_trait;
use csv::StringRecord;
use tracing::info;
use crate::utils::error::MaskerError;
use crate::cmd::process::Producer;

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
    async fn load(&self) -> Result<(), MaskerError> {
        info!("csv loaded");
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

