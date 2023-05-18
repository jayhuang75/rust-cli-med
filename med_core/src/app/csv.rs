use crate::app::core::App;
use crate::app::worker::Worker;
use crate::models::metrics::Metrics;
use crate::utils::config::JobConfig;
use crate::utils::crypto::Cypher;
use crate::utils::enums::{Mode, Standard};
use crate::utils::error::{MaskerError, MaskerErrorType};
use crate::utils::progress_bar::get_progress_bar;
use csv::StringRecord;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::fs;
use tracing::debug;
use walkdir::WalkDir;

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

impl CsvFileProcessor {
    pub async fn load(&mut self, app: &App) -> Result<(), MaskerError> {
        let (tx, rx) = flume::unbounded();
        let new_worker = Worker::new(app.params.worker).await?;
        let mut files_number: u64 = 0;
        for entry in WalkDir::new(&app.params.file_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.path().is_dir())
        {
            let tx = tx.clone();
            debug!("load files: {:?}", entry.path().display().to_string());
            files_number += 1;
            new_worker.pool.execute(move || {
                Worker::read_csv(tx, entry.path().display().to_string()).unwrap();
            });
        }

        drop(tx);

        let bar = get_progress_bar(files_number, "load files to processor");
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

    fn check_if_field_exist_in_job_conf(&self, indexs: Vec<usize>) {
        if indexs.is_empty() {
            eprintln!(
                "{:?}",
                MaskerError {
                    cause: Some("no field match".to_owned()),
                    error_type: MaskerErrorType::ConfigError,
                    message: Some("please check your job conf".to_owned()),
                }
            );
            std::process::exit(1);
        }
    }

    pub async fn run_mask(&mut self, job_conf: &JobConfig) -> Result<(), MaskerError> {
        let bar = get_progress_bar(self.metrics.total_records as u64, "masking files");

        let new_result: Vec<CsvFile> = self
            .result
            .par_iter()
            .map(|item| {
                let mut new_csv = CsvFile {
                    headers: item.headers.clone(),
                    ..Default::default()
                };
                let indexs = item
                    .headers
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| job_conf.fields.contains(&item.to_string()))
                    .map(|(i, _)| i)
                    .collect::<Vec<_>>();

                self.check_if_field_exist_in_job_conf(indexs.clone());

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
                new_csv.path = item.path.clone();
                new_csv.data = masked_data;
                new_csv
            })
            .collect::<Vec<CsvFile>>();

        self.result = new_result;
        bar.finish_and_clear();

        Ok(())
    }

    pub async fn run_cipher(
        &mut self,
        key: &str,
        mode: &Mode,
        standard: &Standard,
        job_conf: &JobConfig,
    ) -> Result<(), MaskerError> {
        let cypher = Cypher::new(key);
        let bar: indicatif::ProgressBar =
            get_progress_bar(self.metrics.total_records as u64, "cryptography files");

        let new_result: Vec<CsvFile> = self
            .result
            .par_iter()
            .map(|item| {
                let mut new_csv = CsvFile {
                    headers: item.headers.clone(),
                    ..Default::default()
                };

                let indexs = item
                    .headers
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| job_conf.fields.contains(&item.to_string()))
                    .map(|(i, _)| i)
                    .collect::<Vec<_>>();

                self.check_if_field_exist_in_job_conf(indexs.clone());

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
                                    let masked: String;
                                    match mode {
                                        Mode::MASK => {
                                            unimplemented!()
                                        }
                                        Mode::ENCRYPT => {
                                            masked = cypher.encrypt(item, standard).unwrap()
                                        }
                                        Mode::DECRYPT => {
                                            masked = cypher.decrypt(item, standard).unwrap()
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
            .collect();
        self.result = new_result;
        bar.finish_and_clear();

        Ok(())
    }

    fn create_output_dir(&self, output_dir: &str, file_dir: &str) -> Result<(), MaskerError> {
        WalkDir::new(file_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .for_each(|e| {
                let output_path = format!("{}/{}", output_dir, e.path().display());
                fs::create_dir_all(output_path).unwrap();
            });
        Ok(())
    }

    pub async fn write(&self, output_dir: &str, file_dir: &str) -> Result<Metrics, MaskerError> {
        self.create_output_dir(output_dir, file_dir)?;
        let bar: indicatif::ProgressBar =
            get_progress_bar(self.metrics.total_records as u64, "write files");
        self.result.par_iter().for_each(|item| {
            let output_files = format!("{}/{}", output_dir, item.path);
            debug!("write to path: {:?}", output_files);
            Worker::write_csv(item, &output_files, &bar).unwrap();
        });
        bar.finish_and_clear();
        Ok(self.metrics.clone())
    }
}
