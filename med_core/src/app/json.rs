use async_trait::async_trait;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tracing::debug;
use walkdir::WalkDir;

// use async_trait::async_trait;
// use tracing::info;
// use crate::utils::config::JobConfig;
use crate::{
    models::{enums::Mode, enums::Standard, metrics::Metrics},
    utils::{
        config::JobConfig,
        crypto::Cypher,
        error::MaskerError,
        helpers::{create_output_dir, json_med_core},
        progress_bar::get_progress_bar,
    },
};

use super::{core::Processor, worker::Worker};
// use crate::cmd::cli::Cli;
// use crate::cmd::worker::Worker;

#[derive(Debug, Clone, Default)]
pub struct JsonFile {
    pub path: String,
    pub total_records: usize,
    pub data: serde_json::Value,
}

#[derive(Debug, Default, Clone)]
pub struct JsonFileProcessor {
    pub metrics: Metrics,
    pub result: Vec<JsonFile>,
}

#[async_trait(?Send)]
impl Processor for JsonFileProcessor {
    async fn new() -> Self {
        JsonFileProcessor::default()
    }
    async fn load(&mut self, num_workers: &u16, file_path: &str) -> Result<(), MaskerError> {
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
            self.metrics.total_records += item.total_records;
            self.result.push(item);
        });
        bar.finish_and_clear();
        Ok(())
    }

    async fn run(
        &mut self,
        job_conf: &JobConfig,
        mode: &Mode,
        standard: Option<&Standard>,
        cypher: Option<&Cypher>,
    ) -> Result<(), MaskerError> {
        let bar = get_progress_bar(self.metrics.total_files as u64, "processing json files");
        let new_result: Vec<JsonFile> = self
            .result
            .par_iter()
            .inspect(|_| bar.inc(1))
            .map(|item| {
                let mut new_json = JsonFile::default();
                let masked =
                    json_med_core(&mut item.data.clone(), job_conf, mode, standard, cypher);
                new_json.path = item.path.clone();
                new_json.data = masked;
                new_json.total_records = self.metrics.total_records;
                new_json
            })
            .collect::<Vec<JsonFile>>();
        bar.finish_and_clear();
        self.result = new_result;
        Ok(())
    }

    async fn write(&self, output_dir: &str, file_dir: &str) -> Result<Metrics, MaskerError> {
        create_output_dir(output_dir, file_dir).await?;
        let bar: indicatif::ProgressBar =
            get_progress_bar(self.metrics.total_records as u64, "write files");
        self.result
            .par_iter()
            .inspect(|_| bar.inc(1))
            .for_each(|item| {
                let output_files = format!("{}/{}", output_dir, item.path);
                debug!("write to path: {:?}", output_files);
                Worker::write_json(&item.data, &output_files).unwrap();
            });
        bar.finish_and_clear();
        Ok(self.metrics.clone())
    }
}
