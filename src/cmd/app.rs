use std::path::Path;

use crate::{cmd::cli::Cli, utils::error::MaskerError, utils::config::JobConfig};
use tracing_subscriber::{fmt::format};

use crate::cmd::process::FileProcessor;
use crate::utils::enums::FileType;
use crate::cmd::csv::CsvFile;
use crate::cmd::process::Producer;

pub struct App{
    pub params: Cli,
}

impl App {
    pub async fn new() -> Self {
        let params = Cli::new().await;
        Self::logging(params.debug).await;
        App {params}
    }

    async fn load_job_config(&self) -> Result<JobConfig, MaskerError>{
        let conf = JobConfig::new(Path::new(&self.params.conf_path)).await?;
        Ok(conf)
    }

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

    pub async fn process(&self) -> Result<(), MaskerError> {

        match &self.params.file_type {
            FileType::CSV => {
                let csv_file = CsvFile::default();
                let csv_file_processor = self.file_processor(Box::new(csv_file)).await?;
                let _ = csv_file_processor.load().await?;
                let _ = csv_file_processor.run().await?;
                let _ = csv_file_processor.write().await?;
            },
            FileType::JSON => {

            },
        }
        Ok(())
    }

    async fn file_processor(&self, producer: Box<dyn Producer>) -> Result<FileProcessor, MaskerError> {
        let job_conf = self.load_job_config().await?;
        let file_processor = FileProcessor::new(self.params.clone(), job_conf, producer).await;
        Ok(file_processor)
    }

    // async fn process_csv(&self) -> Result<(), MaskerError> {
    //     let mut file: Box<_> = Box::new(CsvFile{
            
    //     });

    //     let new_process = FileProcess::new(self.params, )

    //     Ok(())
    // }

}