use crate::app::worker::Worker;
use crate::models::enums::Mode;
use crate::models::enums::Standard;
use crate::models::metrics::Metrics;
use crate::utils::config::JobConfig;
use crate::utils::crypto::Cypher;
use crate::utils::error::MaskerError;
use crate::utils::helpers::read_csv;
use crate::utils::helpers::write_csv;
use crate::utils::helpers::{create_output_dir, csv_fields_exist};
use crate::utils::progress_bar::get_progress_bar;
use async_trait::async_trait;
use csv::StringRecord;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use tracing::debug;
use walkdir::WalkDir;

use crate::app::core::Processor;

#[derive(Debug, Clone, Default)]
pub struct CsvFile {
    pub path: String,
    pub total_records: usize,
    pub failed_records: usize,
    pub record_failed_reason: Vec<MaskerError>,
    pub headers: StringRecord,
    pub data: Vec<StringRecord>,
}

#[derive(Debug, Default, Clone)]
pub struct CsvFileProcessor {
    pub metrics: Metrics,
    pub result: Vec<CsvFile>,
}

#[async_trait(?Send)]
impl Processor for CsvFileProcessor {
    async fn new() -> Self {
        CsvFileProcessor::default()
    }
    async fn load(&mut self, num_workers: &u16, file_path: &str) -> Result<(), MaskerError> {
        let (tx, rx) = flume::unbounded();
        let new_worker = Worker::new(num_workers.to_owned()).await?;
        let mut files_number: u64 = 0;

        for entry in WalkDir::new(file_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.path().is_dir())
        {
            let tx = tx.clone();
            debug!("load csv files: {:?}", entry.path().display().to_string());
            files_number += 1;
            new_worker.pool.execute(move || {
                read_csv(tx, entry.path().display().to_string()).unwrap();
            });
        }

        drop(tx);

        let bar = get_progress_bar(files_number, "load csv files to processor");
        rx.iter().for_each(|item| {
            bar.inc(1);
            self.metrics.total_files += 1;
            self.metrics.total_records += item.total_records;
            self.metrics.failed_records += item.failed_records;
            self.metrics
                .record_failed_reason
                .extend(item.record_failed_reason.clone());
            self.result.push(item);
        });
        bar.finish_and_clear();

        Ok(())
    }

    async fn run(
        &mut self,
        job_conf: &JobConfig,
        mode: &Mode,
        standard: Option<&Standard>,
        cypher: Option<&Cypher>,
    ) -> Result<(), MaskerError> {
        let bar = get_progress_bar(self.metrics.total_records as u64, "masking csv files");

        let new_result: Vec<CsvFile> = self
            .result
            .par_iter()
            .map(|item| {
                let mut new_csv = CsvFile {
                    headers: item.headers.clone(),
                    ..Default::default()
                };

                let indexs = csv_fields_exist(item.headers.clone(), &job_conf.fields);

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
                                    let mut masked: String = String::new();
                                    match mode {
                                        Mode::MASK => {
                                            masked = job_conf.mask_symbols.clone();
                                        }
                                        Mode::ENCRYPT => {
                                            if let Some(cypher) = cypher {
                                                if let Some(standard) = standard {
                                                    masked = cypher.encrypt(item, standard).unwrap()
                                                }
                                            }
                                        }
                                        Mode::DECRYPT => {
                                            if let Some(cypher) = cypher {
                                                if let Some(standard) = standard {
                                                    masked = cypher.decrypt(item, standard).unwrap()
                                                }
                                            }
                                        }
                                    }
                                    masked_record.push_field(&masked);
                                }
                                false => masked_record.push_field(item),
                            }
                        });
                        masked_record
                    })
                    .collect();
                new_csv.path = item.path.clone();
                new_csv.data = masked_data;
                new_csv
            })
            .collect::<Vec<CsvFile>>();

        self.result = new_result;
        bar.finish_and_clear();

        Ok(())
    }

    async fn write(&self, output_dir: &str, file_dir: &str) -> Result<Metrics, MaskerError> {
        create_output_dir(output_dir, file_dir).await?;
        let bar: indicatif::ProgressBar =
            get_progress_bar(self.metrics.total_records as u64, "write files");
        self.result.par_iter().for_each(|item| {
            let output_files = format!("{}/{}", output_dir, item.path);
            debug!("write to path: {:?}", output_files);
            write_csv(item, &output_files, &bar).unwrap();
        });
        bar.finish_and_clear();
        Ok(self.metrics.clone())
    }
}
