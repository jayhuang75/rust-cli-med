use csv::{StringRecord, Writer};
use threadpool::ThreadPool;

use crate::utils::error::MaskerError;

use crate::cmd::csv::CsvFile;

#[derive(Debug)]
pub struct Worker {
    pub cpu_num: usize,
    pub pool: ThreadPool,
}

impl Worker {
    pub async fn new(cpu_num: usize) -> Result<Self, MaskerError> {
        let pool = ThreadPool::new(cpu_num);
        Ok(Worker { cpu_num, pool })
    }

    pub fn read_csv(tx: flume::Sender<CsvFile>, path: String) -> Result<(), MaskerError> {
        let mut reader = csv::Reader::from_path(path.clone())?;
        let headers = reader.headers()?.to_owned();
        let mut data: Vec<StringRecord> = Vec::new();
        let mut total_records: usize = 0;
        reader.records().into_iter().for_each(|record| {
            total_records += 1;
            data.push(record.unwrap());
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
