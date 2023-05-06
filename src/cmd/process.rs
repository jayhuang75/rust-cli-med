use async_trait::async_trait;
use csv::StringRecord;

use crate::utils::error::MaskerError;

#[async_trait(?Send)]
pub trait Producer {
    async fn load(&self) -> Result<(), MaskerError>;
    async fn process(&self) -> Result<(), MaskerError>;
    async fn write(&self) -> Result<(), MaskerError>;
}

pub struct ProcessCsv {
    pub headers: StringRecord,
    pub data: Vec<StringRecord>,
}

pub struct ProcessJson {
    pub data: Vec<serde_json::Value>
}

pub struct FileProcess {
    pub csv: Option<ProcessCsv>,
    pub json: Option<ProcessJson>,
    pub producer: Box<dyn Producer>,
}

impl FileProcess {
    pub async fn producer(&self) {
        let _load = self.producer.load().await;
        let _p = self.producer.process().await;
        let _w = self.producer.write().await;
    }

    pub fn new_csv(file: ProcessCsv, producer: Box<dyn Producer>) -> Self {
        FileProcess { csv: Some(file), json: None, producer }
    }

    pub fn new_json(file: ProcessJson, producer: Box<dyn Producer>) -> Self {
        FileProcess { csv: None, json: Some(file), producer }
    }
}

