use std::path::Path;

use crate::utils::error::MaskerErrorType;
use crate::{cmd::cli::Cli, utils::config::JobConfig, utils::error::MaskerError};
use colored::Colorize;
use tokio::time::Instant;
use tracing::{info, debug};
use tracing_subscriber::fmt::format;

use crate::cmd::csv::CsvFileProcessor;
use crate::utils::enums::{FileType, Mode};

pub struct App {
    pub params: Cli,
}

#[derive(Debug, Default, Clone)]
pub struct AuditSummary {
    pub params: Cli,
    pub total_file: usize,
    pub total_line: usize,
    pub elapsed: String,
}

impl App {
    /// Returns an App struct
    ///
    /// # Examples
    ///
    /// ```
    /// let new_app = App::new().await?;
    /// ```
    ///
    pub async fn new() -> Result<Self, MaskerError> {
        let params = Cli::new().await?;
        Self::logging(params.debug).await;
        debug!("app {} {:?}", "runtime params".bold().green(),params);
        Ok(App { params })
    }

    /// Privite function Returns job config
    async fn load_job_config(&self) -> Result<JobConfig, MaskerError> {
        let conf = JobConfig::new(Path::new(&self.params.conf_path)).await?;
        debug!("{} {:?}", "job config".bold().green(), conf);

        Ok(conf)
    }

    /// Privite function init the tracing
    /// params: debug bool
    async fn logging(debug: bool) {
        let subscriber = tracing_subscriber::fmt() // disabling time is handy because CloudWatch will add the ingestion time.
            .event_format(format().compact());

        match debug {
            true => {
                subscriber
                    .with_line_number(true)
                    .with_target(true)
                    .with_file(true)
                    .with_max_level(tracing::Level::DEBUG)
                    .init();
            }
            false => {
                subscriber
                    .with_target(false)
                    .with_max_level(tracing::Level::INFO)
                    .init();
            }
        }
    }
    /// Returns process result
    ///
    /// # Examples
    ///
    /// ```
    /// let new_app = App::new().await;
    /// let result = new_app.process().await?;
    /// ```
    ///
    pub async fn process(&self) -> Result<(), MaskerError> {
        info!(
            "processing '{}' files start",
            self.params.file_type.to_string().bold().green()
        );
        info!(
            "file root directory {} ",
            self.params.file_path.bold().green()
        );
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
                let mut csv_processor = CsvFileProcessor::default();

                let now = Instant::now();
                csv_processor.load(&self.params).await?;
                info!(
                    "load files {} elapsed time {:?}",
                    "completed".bold().green(),
                    now.elapsed()
                );

                match &self.params.mode {
                    Mode::MASK => {
                        let now = Instant::now();
                        csv_processor.run_mask(&job_conf).await?;
                        info!(
                            "{} data completed elapsed time {:?}",
                            Mode::MASK.to_string().bold().green(),
                            now.elapsed()
                        );
                    }
                    Mode::ENCRYPT | Mode::DECRYPT => match &self.params.key {
                        Some(key) => {
                            let now = Instant::now();
                            csv_processor.run_cipher(key, &job_conf).await?;
                            info!(
                                "{} completed elapsed time {:?}",
                                "cipher".bold().green(),
                                now.elapsed()
                            );
                        }
                        None => {
                            return Err(MaskerError {
                                message: Some(format!(
                                    "Missing key for Encyption and Decryption input!"
                                )),
                                cause: Some(format!("missing -k or --key")),
                                error_type: MaskerErrorType::ConfigError,
                            })
                        }
                    },
                }

                let now = Instant::now();
                csv_processor.write(&self.params.output_path, &self.params.file_path).await?;
                info!(
                    "write to folder {} completed elapsed time {:?}",
                    self.params.output_path.bold().green(),
                    now.elapsed()
                );
            }
            FileType::JSON => {
                todo!()
            }
        }
        Ok(())
    }
}
