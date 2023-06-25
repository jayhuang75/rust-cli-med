use crate::app::processor::FileProcessor;
use crate::audit::app::Audit;
use crate::{utils::config::JobConfig, utils::error::MedError};
use colored::Colorize;
use std::path::Path;
use tokio::time::Instant;
use tracing::{debug, info};

use crate::models::{metrics::Metrics, params::Params};
use crate::utils::logger::logging;

pub struct App {
    pub params: Params,
    pub user: String,
    pub hostname: String,
    pub audit: Audit,
    pub metrics: Metrics,
}

impl App {
    /// Returns a App Struct for processing
    ///
    /// # Arguments
    ///
    /// * `params` [Params] - params passed by the CLI or other integration
    ///
    /// # Examples
    ///
    /// ```
    /// use med_core::app::core::App;
    /// use med_core::utils::error::MedError;
    /// use med_core::models::params::Params;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), MedError> {
    ///     let params = Params::default();
    ///     let app = App::new(params).await.unwrap();
    ///     Ok(())
    /// }
    ///
    /// ```
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

    /// Returns metrics [Metrics]
    ///
    /// # Examples
    ///
    /// ```
    /// use med_core::app::core::App;
    /// use med_core::utils::error::MedError;
    /// use med_core::models::params::Params;
    /// use med_core::models::enums::{FileType, Mode};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), MedError> {
    ///     let params = Params::default();
    ///     let params = Params {
    ///         conf_path: "../demo/conf/conf_csv.yaml".to_owned(),
    ///         file_path: "../demo/data/input/format_err/csv".to_owned(),
    ///         output_path: "../demo/data/output/csv/format_err/processor_err".to_owned(),
    ///         file_type: FileType::CSV,
    ///         mode: Mode::MASK,
    ///         ..Default::default()
    ///     };
    ///     let mut app = App::new(params).await.unwrap();
    ///     let metrics = app.process().await.unwrap();
    ///     Ok(())
    /// }
    ///
    /// ```
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

        let now = Instant::now();
        let mut processor = FileProcessor::new(self.params.clone(), job_conf).await;
        match processor.run().await {
            Ok(metrics) => {
                self.metrics = metrics.clone();
                self.audit.summary.metrics = metrics.clone();
                if !metrics.metadata.record_failed_reason.is_empty() {
                    info!(
                        "{}: {:?}",
                        "warning".bold().yellow(),
                        metrics.metadata.record_failed_reason
                    );
                }
                self.audit.summary.successed = true;
            }
            Err(err) => {
                self.audit.summary.process_failure_reason = Some(serde_json::to_string(&err)?);
                info!("{} {:?}", "error".bold().red(), err.to_string());
            }
        }
        info!(
            "process {} completed elapsed time {:?}",
            self.params.output_path.bold().green(),
            now.elapsed()
        );
        Ok(self.metrics.clone())
    }

    /// Returns audit_id [i64]
    ///
    /// # Arguments
    ///
    /// * `elapsed_time` - A string slice for the elasped_time
    ///
    /// # Examples
    ///
    /// ```
    /// use med_core::app::core::App;
    /// use med_core::utils::error::MedError;
    /// use med_core::models::params::Params;
    /// use med_core::models::enums::{FileType, Mode};
    /// use tokio::time::Instant;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), MedError> {
    ///     let now = Instant::now();
    ///     let params = Params {
    ///         conf_path: "../demo/conf/conf_csv.yaml".to_owned(),
    ///         file_path: "../demo/data/input/format_err/csv".to_owned(),
    ///         output_path: "../demo/data/output/csv/format_err/processor_err".to_owned(),
    ///         file_type: FileType::CSV,
    ///         mode: Mode::MASK,
    ///         ..Default::default()
    ///     };    
    ///     let mut app = App::new(params).await.unwrap();
    ///     let metrics = app.process().await.unwrap();
    ///     let audit_id = app.update_audit(format!("{:?}", now.elapsed())).await?;
    ///     Ok(())
    /// }
    ///
    /// ```
    #[cfg(not(tarpaulin_include))]
    pub async fn update_audit(&mut self, elapsed_time: String) -> Result<i64, MedError> {
        // update the runtime params for the audit record.
        if self.params.key.is_some() {
            self.params.key = Some("****".to_owned());
        }
        self.audit.summary.user = self.user.clone();
        self.audit.summary.hostname = self.hostname.clone();
        self.audit.summary.runtime_conf = serde_json::to_string(&self.params)?;
        self.audit.summary.elapsed_time = elapsed_time;
        debug!("audit summary : {:?}", self.audit.summary);

        // audit update
        let id = self.audit.insert().await?;
        Ok(id)
    }
}

#[cfg(test)]
#[path = "../tests/core_test.rs"]
mod core_test;
