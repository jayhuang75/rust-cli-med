use colored::Colorize;
use csv::{StringRecord, Writer};
use tracing::{debug, info};

use crate::{
    models::{enums::Mode, metrics::Metadata},
    utils::error::{MedError, MedErrorType},
};

use crate::app::processor::ProcessRuntime;

pub fn csv_processor(
    tx_metadata: flume::Sender<Metadata>,
    files_path: &str,
    output_path: &str,
    process_runtime: ProcessRuntime,
) -> Result<(), MedError> {
    // prepare the reader and read the file
    let mut reader = csv::Reader::from_path(files_path)?;

    // get the header of the file
    let headers = reader.headers()?.to_owned();

    // prepare the metrics
    let mut failed_records: usize = 0;
    let mut record_failed_reason: Vec<MedError> = Vec::new();

    let indexs = csv_fields_exist(headers.clone(), &process_runtime.fields);
    debug!("write to location : {:?}", output_path);

    let mut total_records = 0;

    // prepare the writer
    let mut wtr = Writer::from_path(output_path)?;

    // write the header
    wtr.write_record(&headers)?;

    reader.into_records().for_each(|record| {
        total_records += 1;
        match record {
            Ok(records) => {
                let mut masked_record: StringRecord = StringRecord::new();
                records.iter().enumerate().for_each(|(i, item)| {
                    match indexs.contains(&i) {
                        true => {
                            let mut masked: String = String::new();
                            match process_runtime.mode {
                                Mode::MASK => {
                                    if let Some(symbols) = process_runtime.mask_symbols.clone() {
                                        masked = symbols;
                                    }
                                }
                                Mode::ENCRYPT => {
                                    if let Some(cypher) = process_runtime.cypher.clone() {
                                        if let Some(standard) = process_runtime.standard {
                                            match cypher.encrypt(item, &standard) {
                                                Ok(m) => masked = m,
                                                Err(err) => {
                                                    let record_error = MedError {
                                                        message: Some(format!(
                                                            "please check {} {:?} format",
                                                            files_path, process_runtime.mode
                                                        )),
                                                        cause: Some(err.to_string()),
                                                        error_type: MedErrorType::CsvError,
                                                    };
                                                    let error_str =
                                                        serde_json::to_string(&record_error)
                                                            .unwrap();
                                                    info!(
                                                        "{}: {}",
                                                        "warning".bold().yellow(),
                                                        error_str
                                                    );
                                                    record_failed_reason.push(record_error);
                                                    failed_records += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                                Mode::DECRYPT => {
                                    if let Some(cypher) = process_runtime.cypher.clone() {
                                        if let Some(standard) = process_runtime.standard {
                                            match cypher.decrypt(item, &standard) {
                                                Ok(m) => masked = m,
                                                Err(err) => {
                                                    let record_error = MedError {
                                                        message: Some(format!(
                                                            "please check {} {:?} format",
                                                            files_path, process_runtime.mode
                                                        )),
                                                        cause: Some(err.to_string()),
                                                        error_type: MedErrorType::CsvError,
                                                    };
                                                    let error_str =
                                                        serde_json::to_string(&record_error)
                                                            .unwrap();
                                                    info!(
                                                        "{}: {}",
                                                        "warning".bold().yellow(),
                                                        error_str
                                                    );
                                                    record_failed_reason.push(record_error);
                                                    failed_records += 1;
                                                }
                                            }
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
                    message: Some(format!(
                        "please check {} {:?} format",
                        files_path, process_runtime.mode
                    )),
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

fn csv_fields_exist(headers: StringRecord, fields: &[String]) -> Vec<usize> {
    let indexs = headers
        .iter()
        .enumerate()
        .filter(|(_, item)| fields.contains(&item.to_string()))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    if indexs.is_empty() {
        std::process::exit(1);
    }
    indexs
}

#[cfg(test)]
#[path = "../tests/csv_test.rs"]
mod csv_test;
