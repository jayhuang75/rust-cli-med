use std::path::Path;

use crate::{cmd::cli::Cli, utils::error::MaskerError, utils::config::JobConfig};
use tracing::info;
use tracing_subscriber::{fmt::format};

use crate::cmd::process::FileProcessor;
use crate::utils::enums::FileType;
use crate::cmd::csv::CsvFile;
use crate::cmd::process::Producer;

use super::worker::Worker;

pub struct App{
    pub params: Cli,
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
        Ok(App {params})
    }

    /// Privite function Returns job config
    async fn load_job_config(&self) -> Result<JobConfig, MaskerError>{
        let conf = JobConfig::new(Path::new(&self.params.conf_path)).await?;
        Ok(conf)
    }

    /// Privite function init the tracing
    /// params: debug bool
    async fn logging(debug: bool) {      
        let subscriber = tracing_subscriber::fmt() // disabling time is handy because CloudWatch will add the ingestion time.
            .with_timer(tracing_subscriber::fmt::time::uptime())
            .with_line_number(true)
            .with_thread_names(true)
            .event_format(format().compact());

        match debug {
            true => {
                subscriber.with_max_level(tracing::Level::DEBUG).init();
            }
            false => {
                subscriber.with_max_level(tracing::Level::INFO).init();
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

        info!("processing {:?} files start", self.params.file_type );
        info!("file directory {:?} ", self.params.file_path );

        match &self.params.file_type {
            FileType::CSV => {
                let csv_file = CsvFile::default();
                let csv_file_processor = self.file_processor(Box::new(csv_file)).await?;
                let _ = csv_file_processor.load().await?;
                let _ = csv_file_processor.run().await?;
                let _ = csv_file_processor.write().await?;
            },
            FileType::JSON => {
                todo!()
            },
        }
        Ok(())
    }

    /// Privite function to build the new FileProcessor [`crate::cmd::utils::config::JobConfig`]
    /// 
    /// 1. load the job config
    /// 2. init the new FileProcessor
    /// 
    async fn file_processor(&self, producer: Box<dyn Producer>) -> Result<FileProcessor, MaskerError> {
        info!("load job config from {:?}", self.params.conf_path );
        let job_conf = self.load_job_config().await?;

        info!("processing worker number {:?} ", self.params.worker );
        let worker = Worker::new(self.params.worker).await?;

        let file_processor = FileProcessor::new(self.params.clone(), worker, job_conf, producer).await;
        Ok(file_processor)
    }

}