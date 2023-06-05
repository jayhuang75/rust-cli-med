use crate::app::json::JsonFileProcessor;
use crate::audit::app::Audit;
use crate::utils::crypto::Cypher;
use crate::utils::error::MedErrorType;
use crate::{utils::config::JobConfig, utils::error::MedError};
use async_trait::async_trait;
use colored::Colorize;
use std::path::Path;
use tokio::time::Instant;
use tracing::{debug, info};

use crate::app::csv::CsvFileProcessor;
use crate::models::enums::{FileType, Mode, Standard};
use crate::models::{metrics::Metrics, params::Params};
use crate::utils::logger::logging;

pub struct App {
    pub params: Params,
    pub user: String,
    pub hostname: String,
    pub audit: Audit,
    pub metrics: Metrics,
}

#[async_trait(?Send)]
pub trait Processor {
    async fn new() -> Self;
    async fn load(&mut self, num_worker: &u16, file_path: &str) -> Result<(), MedError>;
    async fn run(
        &mut self,
        job_conf: &JobConfig,
        mode: &Mode,
        standard: Option<&Standard>,
        cypher: Option<&Cypher>,
    ) -> Result<(), MedError>;
    async fn write(&self, output_dir: &str, file_dir: &str) -> Result<Metrics, MedError>;
}

impl App {
    pub async fn new(params: Params) -> Result<Self, MedError> {
        logging(params.debug).await;

        let user = whoami::username();
        let hostname = whoami::hostname();
        let audit = Audit::new().await?;
        let metrics = Metrics::default();

        info!(
            "{} on {} run {} mode for {}",
            user.bold().green(),
            hostname.bold().green(),
            params.app_mode.to_string().bold().green(),
            params.mode.to_string().bold().green()
        );

        debug!("app {} {:?}", "runtime params".bold().green(), params);

        Ok(App {
            params,
            user,
            hostname,
            audit,
            metrics,
        })
    }

    /// Privite function Returns job config
    async fn load_job_config(&self) -> Result<JobConfig, MedError> {
        let conf = JobConfig::new(Path::new(&self.params.conf_path)).await?;
        debug!("{} {:?}", "job config".bold().green(), conf);
        Ok(conf)
    }

    pub async fn process(&mut self) -> Result<Metrics, MedError> {
        info!(
            "processing '{}' files start",
            self.params.file_type.to_string().bold().green()
        );
        info!("file directory {} ", self.params.file_path.bold().green());
        info!(
            "number of workers {}",
            self.params.worker.to_string().bold().green()
        );

        let now = Instant::now();
        let job_conf = self.load_job_config().await?;
        info!(
            "load job conf from {} elapsed time {:?}",
            self.params.conf_path.bold().green(),
            now.elapsed()
        );

        match &self.params.file_type {
            FileType::CSV => {
                let mut processor: CsvFileProcessor = Processor::new().await;

                let now = Instant::now();
                processor
                    .load(&self.params.worker, &self.params.file_path)
                    .await?;
                info!(
                    "load {:?} files to processor {} elapsed time {:?}",
                    self.params.file_type,
                    "completed".bold().green(),
                    now.elapsed()
                );

                let now = Instant::now();
                match &self.params.mode {
                    Mode::MASK => {
                        processor
                            .run(&job_conf, &self.params.mode, None, None)
                            .await?;
                        info!(
                            "{} data completed elapsed time {:?}",
                            Mode::MASK.to_string().bold().green(),
                            now.elapsed()
                        );
                    }
                    Mode::ENCRYPT | Mode::DECRYPT => match &self.params.key {
                        Some(key) => {
                            let cypher = Cypher::new(key);
                            processor
                                .run(
                                    &job_conf,
                                    &self.params.mode,
                                    Some(&self.params.standard),
                                    Some(&cypher),
                                )
                                .await?;
                            info!(
                                "{} completed elapsed time {:?}",
                                "cipher".bold().green(),
                                now.elapsed()
                            );
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
                }

                let now = Instant::now();

                match processor
                    .write(&self.params.output_path, &self.params.file_path)
                    .await
                {
                    Ok(metrics) => {
                        self.metrics = metrics.clone();
                        self.audit.summary.total_files = metrics.total_files;
                        self.audit.summary.total_records = metrics.total_records;
                        self.audit.summary.failed_records = metrics.failed_records;
                        self.audit.summary.record_failed_reason = metrics.record_failed_reason;
                        self.audit.summary.successed = true;
                    }
                    Err(err) => {
                        self.audit.summary.process_failure_reason =
                            Some(serde_json::to_string(&err)?);
                        info!("{} {:?}", "error".bold().red(), err.to_string());
                    }
                }

                info!(
                    "write to folder {} completed elapsed time {:?}",
                    self.params.output_path.bold().green(),
                    now.elapsed()
                );
            }
            FileType::JSON => {
                let mut processor: JsonFileProcessor = Processor::new().await;

                let now = Instant::now();
                processor
                    .load(&self.params.worker, &self.params.file_path)
                    .await?;
                info!(
                    "load {:?} files to processor {} elapsed time {:?}",
                    self.params.file_type,
                    "completed".bold().green(),
                    now.elapsed()
                );

                let now = Instant::now();
                match &self.params.mode {
                    Mode::MASK => {
                        // processor.run_mask(&job_conf).await?;
                        processor
                            .run(&job_conf, &self.params.mode, None, None)
                            .await?;
                        info!(
                            "{} data completed elapsed time {:?}",
                            Mode::MASK.to_string().bold().green(),
                            now.elapsed()
                        );
                    }
                    Mode::ENCRYPT | Mode::DECRYPT => match &self.params.key {
                        Some(key) => {
                            let cypher = Cypher::new(key);
                            processor
                                .run(
                                    &job_conf,
                                    &self.params.mode,
                                    Some(&self.params.standard),
                                    Some(&cypher),
                                )
                                .await?;
                            info!(
                                "{} completed elapsed time {:?}",
                                "cipher".bold().green(),
                                now.elapsed()
                            );
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
                }

                let now = Instant::now();
                match processor
                    .write(&self.params.output_path, &self.params.file_path)
                    .await
                {
                    Ok(metrics) => {
                        self.metrics = metrics.clone();
                        self.audit.summary.total_files = metrics.total_files;
                        self.audit.summary.total_records = metrics.total_records;
                        self.audit.summary.failed_records = metrics.failed_records;
                        self.audit.summary.record_failed_reason = metrics.record_failed_reason;
                        self.audit.summary.successed = true;
                    }
                    Err(err) => {
                        self.audit.summary.process_failure_reason =
                            Some(serde_json::to_string(&err)?);
                        info!("{} {:?}", "error".bold().red(), err.to_string());
                    }
                }
                info!(
                    "write to folder {} completed elapsed time {:?}",
                    self.params.output_path.bold().green(),
                    now.elapsed()
                );
            }
        }

        debug!("metrics : {:?}", self.metrics);
        Ok(self.metrics.clone())
    }

    pub async fn update_audit(&mut self) -> Result<i64, MedError> {
        // update the runtime params for the audit record.
        if self.params.key.is_some() {
            self.params.key = Some("****".to_owned());
        }
        self.audit.summary.user = self.user.clone();
        self.audit.summary.hostname = self.hostname.clone();
        self.audit.summary.runtime_conf = serde_json::to_string(&self.params)?;
        debug!("audit summary : {:?}", self.audit.summary);

        // audit update
        let id = self.audit.insert().await?;
        Ok(id)
    }
}

#[cfg(test)]
#[path = "../tests/core_test.rs"]
mod core_test;
