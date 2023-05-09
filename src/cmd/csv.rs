use std::clone;

use crate::{cmd::cli::Cli, utils::enums::Mode};
use crate::cmd::worker::Worker;
use crate::utils::config::JobConfig;
use crate::utils::error::MaskerError;
use csv::StringRecord;
use rayon::prelude::{IntoParallelIterator, ParallelIterator, IntoParallelRefIterator};
use tracing::info;
use walkdir::WalkDir;
use crate::utils::crypto::{CryptoData, self};

#[derive(Debug, Clone, Default)]
pub struct CsvFile {
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

        for entry in WalkDir::new(&params.file_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.path().is_dir())
        {
            let tx = tx.clone();
            new_worker.pool.execute(move || {
                Worker::read_csv(tx, entry.path().display().to_string()).unwrap();
            })
        }

        drop(tx);

        rx.iter().for_each(|item| {
            self.total_file += 1;
            self.result.push(item);
        });

        info!("load completed: {:?}", self);

        Ok(())
    }

    pub async fn run_mask(&self, job_conf: &JobConfig) -> Result<(), MaskerError> {

        self.result.par_iter().for_each(|item| {
            let indexs = item
                .headers
                .iter()
                .enumerate()
                .filter(|(_, item)| job_conf.fields.contains(&item.to_string()))
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            let masked_data: Vec<StringRecord> =
                item.clone().data
                    .into_par_iter()
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
            info!("after masked : {:?}", masked_data);
        });
        Ok(())
    }

    pub async fn run_cipher(&self, key: &str, job_conf: &JobConfig) -> Result<(), MaskerError> {
        
        let crypto = CryptoData::new(key);

        self.result.par_iter().for_each(|item| {
            let indexs = item
                .headers
                .iter()
                .enumerate()
                .filter(|(_, item)| job_conf.fields.contains(&item.to_string()))
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            let masked_data: Vec<StringRecord> =
                item.clone().data
                    .into_par_iter()
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
            info!("after masked : {:?}", masked_data);
        });
        Ok(())
    }


}
