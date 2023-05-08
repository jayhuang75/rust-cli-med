use csv::StringRecord;
use threadpool::ThreadPool;

use crate::utils::error::MaskerError;

use crate::cmd::csv::CsvFile;

#[derive(Debug)]
pub struct Worker {
    pub pool: ThreadPool,
}

impl Worker {
    pub async fn new(cpu_num: usize) -> Result<Self, MaskerError> {
        let pool = ThreadPool::new(num_cpus::get());
        Ok(Worker { pool })
    }
    pub fn read(tx: flume::Sender<CsvFile>, path: String) -> Result<(), MaskerError> {
        let mut reader = csv::Reader::from_path(path)?;
        let headers = reader.headers()?.to_owned();
        let mut data: Vec<StringRecord> = Vec::new();
        reader.records().into_iter().for_each(|record| {
            data.push(record.unwrap());
        });
        tx.send(CsvFile {
            headers: headers,
            data: data,
        })
        .unwrap();
        Ok(())
    
    }
}
