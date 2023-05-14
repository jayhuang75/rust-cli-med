use colored::Colorize;
use csv::{StringRecord, Writer};
use threadpool::ThreadPool;
use tracing::{info};

use crate::utils::error::{MaskerError, MaskerErrorType};

use crate::core::csv::CsvFile;

#[derive(Debug)]
pub struct Worker {
    pub cpu_num: u16,
    pub pool: ThreadPool,
}

impl Worker {
    pub async fn new(cpu_num: u16) -> Result<Self, MaskerError> {
        let pool = ThreadPool::new(cpu_num as usize);
        rayon::ThreadPoolBuilder::new().num_threads(cpu_num as usize).build_global()?;
        Ok(Worker { cpu_num, pool })
    }

    pub fn read_csv(tx: flume::Sender<CsvFile>, path: String) -> Result<(), MaskerError> {

        let mut reader = csv::Reader::from_path(path.clone())?;
        let headers = reader.headers()?.to_owned();
        let mut data: Vec<StringRecord> = Vec::new();
        let mut total_records: usize = 0;
        reader.records().into_iter().for_each(|record| {
            match record {
                Ok(r) => {
                    total_records += 1;
                    data.push(r);
                },
                Err(err) => {
                    let error_str = serde_json::to_string(&MaskerError{
                        message: Some(format!("please check {} csv format", path)),
                        cause: Some(err.to_string()),
                        error_type: MaskerErrorType::CsvError,
                    }).unwrap();
                    info!("{}: {}", "warning".bold().yellow(), error_str);
                },
            };
        });
        tx.send(CsvFile {
            path: path,
            total_records: total_records,
            headers: headers,
            data: data,
        })
        .unwrap();
        Ok(())
    }

    pub fn write_csv(masked_data: &CsvFile, output_file: &str, bar: &indicatif::ProgressBar) -> Result<(), MaskerError> {

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