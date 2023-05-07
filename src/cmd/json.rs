use async_trait::async_trait;
use tracing::info;
use crate::utils::error::MaskerError;
use crate::cmd::process::Producer;

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
    async fn load(&self) -> Result<(), MaskerError> {
        info!("json loaded");
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