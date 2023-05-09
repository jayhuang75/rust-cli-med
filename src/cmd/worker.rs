use csv::StringRecord;
use threadpool::ThreadPool;

use crate::utils::error::MaskerError;

use crate::cmd::csv::CsvFile;

#[derive(Debug)]
pub struct Worker {
    pub cpu_num : usize,
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
}
