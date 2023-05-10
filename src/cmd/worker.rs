use csv::{StringRecord, Writer};
use threadpool::ThreadPool;

use crate::utils::error::MaskerError;

use crate::cmd::csv::CsvFile;
use crate::utils::progress_bar::get_progress_bar;

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
        reader.records().into_iter().for_each(|record| {
            data.push(record.unwrap());
        });
        tx.send(CsvFile {
            path: path,
            headers: headers,
            data: data,
        })
        .unwrap();
        Ok(())
    }

    pub fn write_csv(masked_data: &CsvFile, output_path: &str) -> Result<(), MaskerError> {
        let bar: indicatif::ProgressBar =
            get_progress_bar(masked_data.data.len() as u64, "write files");

        let mut wtr = Writer::from_path(output_path)?;
        // write the header
        wtr.write_record(&masked_data.headers)?;

        masked_data.data.iter().inspect(|_|bar.inc(1)).for_each(|item| {
            // write_event_to_csv(&mut wtr, item).unwrap();
            wtr.write_record(item).unwrap();
        });
        wtr.flush()?;
        bar.finish_and_clear();
        Ok(())
    }
}
