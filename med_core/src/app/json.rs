use async_trait::async_trait;
use tracing::{debug, info};
use walkdir::WalkDir;

// use async_trait::async_trait;
// use tracing::info;
// use crate::utils::config::JobConfig;
use crate::{models::{metrics::Metrics}, utils::{error::MaskerError, enums::{Mode, Standard}, config::JobConfig, progress_bar::get_progress_bar}};

use super::{core::{Processor}, worker::Worker};
// use crate::cmd::cli::Cli;
// use crate::cmd::worker::Worker;

#[derive(Debug, Clone, Default)]
pub struct JsonFile {
    pub path: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Default, Clone)]
pub struct JsonFileProcessor {
    pub metrics: Metrics,
    pub result: Vec<JsonFile>,
}

#[async_trait(?Send)]
impl Processor for JsonFileProcessor{
    async fn new() -> Self {
        JsonFileProcessor::default()
    }
    async fn load(&mut self, num_workers: &u16, file_path: &str) -> Result<(), MaskerError>{
        let (tx, rx) = flume::unbounded();
        let new_worker = Worker::new(num_workers.to_owned()).await?;
        let mut files_number: u64 = 0;

        for entry in WalkDir::new(file_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.path().is_dir())
        {
            let tx = tx.clone();
            debug!("load json files: {:?}", entry.path().display().to_string());
            files_number += 1;
            new_worker.pool.execute(move || {
                Worker::read_json(tx, entry.path().display().to_string()).unwrap();
            });
        }

        drop(tx);

        let bar = get_progress_bar(files_number, "load json files to processor");
        rx.iter().for_each(|item| {
            bar.inc(1);
            self.metrics.total_files += 1;
            self.result.push(item);
        });
        bar.finish_and_clear();
        Ok(())

    }
    async fn run_mask(&mut self, job_conf: &JobConfig) -> Result<(), MaskerError> {
        todo!()
    }
    async fn run_cipher(
        &mut self,
        key: &str,
        mode: &Mode,
        standard: &Standard,
        job_conf: &JobConfig,
    ) -> Result<(), MaskerError> {
        todo!()
    }
    async fn write(&self, output_dir: &str, file_dir: &str) -> Result<Metrics, MaskerError> {
        todo!()
    }
}