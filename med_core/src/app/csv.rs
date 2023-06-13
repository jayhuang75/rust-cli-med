use std::sync::Arc;

use colored::Colorize;
use csv::{StringRecord, Writer};
use indicatif::{MultiProgress, ProgressStyle, ProgressBar};
use tracing::{debug, info};
use walkdir::WalkDir;

use crate::app::worker::Worker;
use crate::models::enums::{Mode, Standard};
use crate::models::metrics::{Metadata, Metrics};
use crate::utils::config::JobConfig;
use crate::utils::crypto::Cypher;
use crate::utils::error::MedErrorType;
use crate::utils::helpers::{create_output_dir, csv_fields_exist};
use crate::utils::progress_bar::get_progress_bar;
use crate::{models::params::Params, utils::error::MedError};

pub struct CsvFileProcessor {
    metrics: Metrics,
    runtime_params: Params,
    fields: Vec<String>,
    mask_symbols: Option<String>,
    cypher: Option<Cypher>,
    standard: Option<Standard>,
}

impl CsvFileProcessor {
    pub async fn new(runtime_params: Params, job_conf: JobConfig) -> Self {
        CsvFileProcessor {
            metrics: Metrics::default(),
            runtime_params,
            fields: job_conf.fields,
            mask_symbols: Some(job_conf.mask_symbols),
            cypher: None,
            standard: None,
        }
    }

    pub async fn run(&mut self) -> Result<Metrics, MedError> {
        match self.runtime_params.mode {
            Mode::ENCRYPT | Mode::DECRYPT => match &self.runtime_params.key {
                Some(key) => {
                    self.cypher = Some(Cypher::new(key));
                    self.standard = Some(self.runtime_params.standard);
                }
                None => {
                    return Err(MedError {
                        message: Some(
                            "Missing key for Encyption and Decryption input!".to_string(),
                        ),
                        cause: Some("missing -k or --key".to_string()),
                        error_type: MedErrorType::ConfigError,
                    })
                }
            },
            Mode::MASK => (),
        }
        self.metrics = self.load().await?;

        Ok(self.metrics.clone())
    }

    async fn load(&mut self) -> Result<Metrics, MedError> {
        // prepare the channel to send back the metrics
        let (tx_metadata, rx_metadata) = flume::unbounded();

        // inital worker based on the input
        let new_worker = Worker::new(self.runtime_params.worker).await?;

        // init the files number as 0
        let mut files_number: u64 = 0;

        // create outpu dir
        create_output_dir(
            &self.runtime_params.output_path,
            &self.runtime_params.file_path,
        )
        .await?;

        // loop over the files path
        for entry in WalkDir::new(&self.runtime_params.file_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| !e.path().is_dir())
        {
            // prepare the worker processing
            let tx_metadata = tx_metadata.clone();
            let mode = self.runtime_params.mode.clone();
            let files_path = entry.path().display().to_string();
            let output_dir = format!("{}/{}", self.runtime_params.output_path, files_path);
            let fields = self.fields.clone();

            let cypher = self.cypher.clone();
            let standard = self.standard.clone();
            let mask_symbols = self.mask_symbols.clone();

            // debug ensure the files have been process
            debug!("load csv files: {:?}", entry.path().display().to_string());

            // increase file number
            files_number += 1;

            // worker execution
            new_worker.pool.execute(move || {
                Self::process(
                    tx_metadata,
                    cypher,
                    standard,
                    mask_symbols,
                    &files_path,
                    &output_dir,
                    &mode,
                    &fields,
                )
                .unwrap();
            });
        }

        // drop the channel once it done.
        drop(tx_metadata);

        let bar = get_progress_bar(files_number, "processing csv files");
        rx_metadata.iter().for_each(|item| {
            bar.inc(1);
            self.metrics.total_files = files_number as usize;
            self.metrics.metadata.total_records += item.total_records;
            self.metrics.metadata.failed_records += item.failed_records;
            self.metrics
                .metadata
                .record_failed_reason
                .extend(item.record_failed_reason);
        });
        bar.finish_and_clear();

        debug!("metrics {:?}", self.metrics);

        Ok(self.metrics.clone())
    }

    fn process(
        tx_metadata: flume::Sender<Metadata>,
        cypher: Option<Cypher>,
        standard: Option<Standard>,
        mask_symbols: Option<String>,
        files_path: &str,
        output_path: &str,
        mode: &Mode,
        fields: &Vec<String>,
    ) -> Result<(), MedError> {
        // prepare the reader and read the file
        let mut reader = csv::Reader::from_path(files_path)?;

        // get the header of the file
        let headers = reader.headers()?.to_owned();

        // prepare the metrics
        let mut failed_records: usize = 0;
        let mut record_failed_reason: Vec<MedError> = Vec::new();

        let indexs = csv_fields_exist(headers.clone(), fields);
        debug!("write to location : {:?}", output_path);

        let mut total_records = 0;

        // prepare the writer
        let mut wtr = Writer::from_path(output_path)?;

        // write the header
        wtr.write_record(&headers)?;

        reader.into_records().inspect(|_| {
            total_records += 1;
        }).for_each(|record| {
            match record {
                Ok(records) => {
                    let mut masked_record: StringRecord = StringRecord::new();
                    records.iter().enumerate().for_each(|(i, item)| {
                        match indexs.contains(&i) {
                            true => {
                                let mut masked: String = String::new();
                                match mode {
                                    Mode::MASK => {
                                        if let Some(symbols) = mask_symbols.clone() {
                                            masked = symbols;
                                        }
                                    }
                                    Mode::ENCRYPT => {
                                        if let Some(cypher) = cypher.clone() {
                                            if let Some(standard) = standard {
                                                masked = cypher.encrypt(item, &standard).unwrap()
                                            }
                                        }
                                    }
                                    Mode::DECRYPT => {
                                        if let Some(cypher) = cypher.clone() {
                                            if let Some(standard) = standard {
                                                masked = cypher.decrypt(item, &standard).unwrap()
                                            }
                                        }
                                    }
                                }
                                masked_record.push_field(&masked);
                            }
                            false => masked_record.push_field(item),
                        };
                    });
                    wtr.write_record(&masked_record).unwrap();
                }
                Err(err) => {
                    let record_error = MedError {
                        message: Some(format!("please check {} csv format", files_path)),
                        cause: Some(err.to_string()),
                        error_type: MedErrorType::CsvError,
                    };
                    let error_str = serde_json::to_string(&record_error).unwrap();
                    info!("{}: {}", "warning".bold().yellow(), error_str);
                    record_failed_reason.push(record_error);
                    failed_records += 1;
                }
            };
        });
        // clear the writer
        wtr.flush()?;

        tx_metadata
            .send(Metadata {
                total_records,
                failed_records,
                record_failed_reason,
            })
            .unwrap();

        Ok(())
    }
}
