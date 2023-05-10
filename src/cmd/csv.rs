use crate::cmd::cli::Cli;
use crate::cmd::worker::Worker;
use crate::utils::config::JobConfig;
use crate::utils::crypto::CryptoData;
use crate::utils::error::MaskerError;
use crate::utils::progress_bar::get_progress_bar;
use csv::StringRecord;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use tracing::debug;
use walkdir::WalkDir;

const COUNT: u64 = 20;

#[derive(Debug, Clone, Default)]
pub struct CsvFile {
    pub path: String,
    pub headers: StringRecord,
    pub data: Vec<StringRecord>,
}

#[derive(Debug, Default, Clone)]
pub struct CsvFileProcessor {
    pub total_file: usize,
    pub result: Vec<CsvFile>,
}

impl CsvFileProcessor {
    pub async fn load(&mut self, params: &Cli) -> Result<(), MaskerError> {
        let (tx, rx) = flume::unbounded();

        let new_worker = Worker::new(params.worker).await?;

        let bar = get_progress_bar(COUNT, "load files");

        for entry in WalkDir::new(&params.file_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.path().is_dir())
        {
            let tx = tx.clone();
            bar.inc(1);
            new_worker.pool.execute(move || {
                Worker::read_csv(tx, entry.path().display().to_string()).unwrap();
            })
        }

        drop(tx);

        rx.iter().for_each(|item| {
            self.total_file += 1;
            self.result.push(item);
        });

        bar.finish_and_clear();

        Ok(())
    }

    pub async fn run_mask(&mut self, job_conf: &JobConfig) -> Result<(), MaskerError> {
        let bar = get_progress_bar(self.result.len() as u64, "masking files");

        let new_result: Vec<CsvFile> = self
            .result
            .par_iter()
            .map(|item| {
                let mut new_csv = CsvFile::default();
                new_csv.headers = item.headers.clone();
                let indexs = item
                    .headers
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| job_conf.fields.contains(&item.to_string()))
                    .map(|(i, _)| i)
                    .collect::<Vec<_>>();

                let masked_data: Vec<StringRecord> = item
                    .clone()
                    .data
                    .into_par_iter()
                    .inspect(|_| bar.inc(1))
                    .map(|records| {
                        let mut masked_record: StringRecord = StringRecord::new();
                        records.iter().enumerate().for_each(|(i, item)| {
                            match indexs.contains(&i) {
                                true => {
                                    let masked = job_conf.mask_symbols.clone();
                                    masked_record.push_field(&masked);
                                }
                                false => masked_record.push_field(item),
                            }
                        });

                        masked_record
                    })
                    .collect();
                new_csv.data = masked_data;
                new_csv
            })
            .collect::<Vec<CsvFile>>();

        self.result = new_result;
        bar.finish_and_clear();

        Ok(())
    }

    pub async fn run_cipher(&mut self, key: &str, job_conf: &JobConfig) -> Result<(), MaskerError> {
        let crypto = CryptoData::new(key);
        let bar: indicatif::ProgressBar =
            get_progress_bar(self.result.len() as u64, "cryptography files");

        let new_result: Vec<CsvFile> = self
            .result
            .par_iter()
            .map(|item| {
                let mut new_csv = CsvFile::default();
                new_csv.headers = item.headers.clone();
                let indexs = item
                    .headers
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| job_conf.fields.contains(&item.to_string()))
                    .map(|(i, _)| i)
                    .collect::<Vec<_>>();

                let masked_data: Vec<StringRecord> = item
                    .clone()
                    .data
                    .into_par_iter()
                    .inspect(|_| bar.inc(1))
                    .map(|records| {
                        let mut masked_record: StringRecord = StringRecord::new();
                        records.iter().enumerate().for_each(|(i, item)| {
                            match indexs.contains(&i) {
                                true => {
                                    let masked = crypto.encrypt(item).unwrap();
                                    masked_record.push_field(&masked);
                                }
                                false => masked_record.push_field(item),
                            }
                        });

                        masked_record
                    })
                    .collect();
                new_csv.data = masked_data;
                new_csv
            })
            .collect();
        self.result = new_result;
        bar.finish_and_clear();

        Ok(())
    }

    pub async fn write(&self) -> Result<(), MaskerError> {
        debug!("write : {:?}", self);

        Ok(())
    }
}
