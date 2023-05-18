use colored::Colorize;
use csv::{StringRecord, Writer};
use threadpool::ThreadPool;
use tracing::info;

use crate::utils::error::{MaskerError, MaskerErrorType};

use crate::app::csv::CsvFile;

#[derive(Debug)]
pub struct Worker {
    pub cpu_num: u16,
    pub pool: ThreadPool,
}

impl Worker {
    pub async fn new(cpu_num: u16) -> Result<Self, MaskerError> {
        let pool = ThreadPool::new(cpu_num as usize);
        rayon::ThreadPoolBuilder::new()
            .num_threads(cpu_num as usize)
            .build_global()?;
        Ok(Worker { cpu_num, pool })
    }

    pub fn read_csv(tx: flume::Sender<CsvFile>, path: String) -> Result<(), MaskerError> {
        let mut reader = csv::Reader::from_path(path.clone())?;
        let headers = reader.headers()?.to_owned();
        let mut data: Vec<StringRecord> = Vec::new();
        let mut total_records: usize = 0;
        let mut failed_records: usize = 0;
        let mut record_failed_reason: Vec<MaskerError> = Vec::new();

        reader.records().for_each(|record| {
            match record {
                Ok(r) => {
                    total_records += 1;
                    data.push(r);
                }
                Err(err) => {
                    let record_error = MaskerError {
                        message: Some(format!("please check {} csv format", path)),
                        cause: Some(err.to_string()),
                        error_type: MaskerErrorType::CsvError,
                    };
                    let error_str = serde_json::to_string(&record_error).unwrap();
                    record_failed_reason.push(record_error);
                    failed_records += 1;
                    info!("{}: {}", "warning".bold().yellow(), error_str);
                }
            };
        });
        tx.send(CsvFile {
            path,
            total_records,
            failed_records,
            record_failed_reason,
            headers,
            data,
        })
        .unwrap();
        Ok(())
    }

    pub fn write_csv(
        masked_data: &CsvFile,
        output_file: &str,
        bar: &indicatif::ProgressBar,
    ) -> Result<(), MaskerError> {
        let mut wtr = Writer::from_path(output_file)?;
        // write the header
        wtr.write_record(&masked_data.headers)?;

        masked_data.data.iter().for_each(|item| {
            bar.inc(1);
            wtr.write_record(item).unwrap();
        });
        wtr.flush()?;
        Ok(())
    }
}
