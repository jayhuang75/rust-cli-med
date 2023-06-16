use tracing::{debug};
use walkdir::WalkDir;

use crate::app::csv::csv_processor;
use crate::app::json::json_processor;
use crate::app::worker::Worker;
use crate::models::enums::{FileType, Mode, Standard};
use crate::models::metrics::Metrics;
use crate::utils::config::JobConfig;
use crate::utils::crypto::Cypher;
use crate::utils::error::MedErrorType;
use crate::utils::helpers::create_output_dir;
use crate::utils::progress_bar::get_progress_bar;
use crate::{models::params::Params, utils::error::MedError};

#[derive(Debug, Clone, Default)]
pub struct FileProcessor {
    metrics: Metrics,
    runtime_params: Params,
    pub process_runtime: ProcessRuntime,
}

#[derive(Debug, Clone, Default)]
pub struct ProcessRuntime {
    pub fields: Vec<String>,
    pub mask_symbols: Option<String>,
    pub cypher: Option<Cypher>,
    pub standard: Option<Standard>,
    pub mode: Mode,
}

impl FileProcessor {
    pub async fn new(runtime_params: Params, job_conf: JobConfig) -> Self {
        let mode = runtime_params.mode;
        FileProcessor {
            metrics: Metrics::default(),
            runtime_params,
            process_runtime: ProcessRuntime {
                fields: job_conf.fields,
                mask_symbols: Some(job_conf.mask_symbols),
                cypher: None,
                standard: None,
                mode,
            },
        }
    }
    pub async fn run(&mut self) -> Result<Metrics, MedError> {
        match self.runtime_params.mode {
            Mode::ENCRYPT | Mode::DECRYPT => match &self.runtime_params.key {
                Some(key) => {
                    self.process_runtime.cypher = Some(Cypher::new(key));
                    self.process_runtime.standard = Some(self.runtime_params.standard);
                }
                None => {
                    return Err(MedError {
                        message: Some(
                            "Missing key for Encyption and Decryption input!".to_string(),
                        ),
                        cause: Some("missing -k or --key".to_string()),
                        error_type: MedErrorType::ConfigError,
                    })
                }
            },
            Mode::MASK => (),
        }
        self.metrics = self.load().await?;

        Ok(self.metrics.clone())
    }

    async fn load(&mut self) -> Result<Metrics, MedError> {
        // prepare the channel to send back the metrics
        let (tx_metadata, rx_metadata) = flume::unbounded();

        // inital worker based on the input
        let new_worker = Worker::new(self.runtime_params.worker).await?;

        // init the files number as 0
        let mut files_number: u64 = 0;

        // create outpu dir
        create_output_dir(
            &self.runtime_params.output_path,
            &self.runtime_params.file_path,
        )
        .await?;

        // loop over the files path
        for entry in WalkDir::new(&self.runtime_params.file_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.path().is_dir())
        {
            // prepare the worker processing
            let tx_metadata = tx_metadata.clone();
            let files_path = entry.path().display().to_string();
            let output_dir = format!("{}/{}", self.runtime_params.output_path, files_path);
            let process_runtime = self.process_runtime.clone();

            // debug ensure the files have been process
            debug!(
                "load {:?} files: {:?}",
                self.runtime_params.file_type,
                entry.path().display().to_string()
            );

            // increase file number
            files_number += 1;

            match self.runtime_params.file_type {
                FileType::CSV => {
                    // worker execution
                    new_worker.pool.execute(move || {
                        csv_processor(tx_metadata, &files_path, &output_dir, process_runtime)
                            .unwrap();
                    });
                }
                FileType::JSON => {
                    new_worker.pool.execute(move || {
                        json_processor(tx_metadata, &files_path, &output_dir, process_runtime)
                            .unwrap();
                    });
                }
            }
        }

        // drop the channel once it done.
        drop(tx_metadata);

        let bar = get_progress_bar(
            files_number,
            &format!("processing {:?} files", self.runtime_params.file_type),
        );
        rx_metadata.iter().for_each(|item| {
            bar.inc(1);
            self.metrics.total_files = files_number as usize;
            self.metrics.metadata.total_records += item.total_records;
            self.metrics.metadata.failed_records += item.failed_records;
            self.metrics
                .metadata
                .record_failed_reason
                .extend(item.record_failed_reason);
        });
        bar.finish_and_clear();

        debug!("metrics {:?}", self.metrics);

        Ok(self.metrics.clone())
    }
}
