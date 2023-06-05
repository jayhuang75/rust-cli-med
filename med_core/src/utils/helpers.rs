use crate::app::json::JsonFile;
use crate::utils::config::JobConfig;
use crate::utils::error::MedError;
use csv::StringRecord;
use serde_json::Value;
use std::fs::{self, File};
use std::io::Write;
use walkdir::WalkDir;

use crate::app::csv::CsvFile;
use crate::models::enums::{Mode, Standard};
use crate::utils::crypto::Cypher;

pub fn csv_fields_exist(headers: StringRecord, fields: &[String]) -> Vec<usize> {
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

#[cfg(not(tarpaulin_include))]
pub async fn create_output_dir(output_dir: &str, file_dir: &str) -> Result<(), MedError> {
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

#[cfg(not(tarpaulin_include))]
pub fn json_med_core(
    value: &mut Value,
    job_conf: &JobConfig,
    mode: &Mode,
    standard: Option<&Standard>,
    cypher: Option<&Cypher>,
) -> Value {
    match value {
        Value::Array(arr) => {
            // debug!("[arr] {:?}", arr);
            for item in arr {
                if item.is_array() {
                    json_med_core(item, job_conf, mode, standard, cypher);
                }

                if item.is_object() {
                    // info!("is obj {:?} ", val);
                    item.as_object_mut()
                        .unwrap()
                        .into_iter()
                        .for_each(|(key, val)| {
                            //debug!("key: {:?}, val: {:?} ", key, val);
                            //mask parent lvl
                            if job_conf.fields.contains(key) {
                                if let Value::String(mut masked_val) = val.to_owned() {
                                    match mode {
                                        Mode::MASK => {
                                            masked_val.clear();
                                            masked_val.push_str(&job_conf.mask_symbols);
                                        }
                                        Mode::ENCRYPT => {
                                            if let Some(cypher) = cypher {
                                                if let Some(standard) = standard {
                                                    let masked = cypher
                                                        .encrypt(&masked_val, standard)
                                                        .unwrap();
                                                    masked_val.clear();
                                                    masked_val.push_str(&masked);
                                                }
                                            }
                                        }
                                        Mode::DECRYPT => {
                                            if let Some(cypher) = cypher {
                                                if let Some(standard) = standard {
                                                    let masked = cypher
                                                        .decrypt(&masked_val, standard)
                                                        .unwrap();
                                                    masked_val.clear();
                                                    masked_val.push_str(&masked);
                                                }
                                            }
                                        }
                                    }
                                    *val = Value::String(masked_val);
                                }
                            }

                            if val.is_array() {
                                json_med_core(val, job_conf, mode, standard, cypher);
                            }

                            if val.is_object() {
                                json_med_core(val, job_conf, mode, standard, cypher);
                            }
                        });
                }
            }
        }
        Value::Object(obj) => {
            for (key, val) in obj {
                // debug!("key : {:?}, val: {:?}", key, val);
                if val.is_array() {
                    json_med_core(val, job_conf, mode, standard, cypher);
                }
                if job_conf.fields.contains(key) {
                    if let Value::String(mut masked_val) = val.to_owned() {
                        match mode {
                            Mode::MASK => {
                                masked_val.clear();
                                masked_val.push_str(&job_conf.mask_symbols);
                            }
                            Mode::ENCRYPT => {
                                if let Some(cypher) = cypher {
                                    if let Some(standard) = standard {
                                        let masked = cypher.encrypt(&masked_val, standard).unwrap();
                                        masked_val.clear();
                                        masked_val.push_str(&masked);
                                    }
                                }
                            }
                            Mode::DECRYPT => {
                                if let Some(cypher) = cypher {
                                    if let Some(standard) = standard {
                                        let masked = cypher.decrypt(&masked_val, standard).unwrap();
                                        masked_val.clear();
                                        masked_val.push_str(&masked);
                                    }
                                }
                            }
                        }
                        *val = Value::String(masked_val);
                    }
                }
            }
        }
        _ => {}
    }
    value.clone()
}

#[cfg(not(tarpaulin_include))]
pub fn read_csv(tx: flume::Sender<CsvFile>, path: String) -> Result<(), MedError> {
    use colored::Colorize;
    use tracing::info;

    use crate::utils::error::MedErrorType;

    let mut reader = csv::Reader::from_path(path.clone())?;
    let headers = reader.headers()?.to_owned();
    let mut data: Vec<StringRecord> = Vec::new();
    let mut total_records: usize = 0;
    let mut failed_records: usize = 0;
    let mut record_failed_reason: Vec<MedError> = Vec::new();

    reader.records().for_each(|record| {
        match record {
            Ok(r) => {
                total_records += 1;
                data.push(r);
            }
            Err(err) => {
                let record_error = MedError {
                    message: Some(format!("please check {} csv format", path)),
                    cause: Some(err.to_string()),
                    error_type: MedErrorType::CsvError,
                };
                let error_str = serde_json::to_string(&record_error).unwrap();
                record_failed_reason.push(record_error);
                failed_records += 1;
                info!("{}: {}", "warning".bold().yellow(), error_str);
            }
        };
    });
    tx.send(CsvFile {
        path,
        total_records,
        failed_records,
        record_failed_reason,
        headers,
        data,
    })
    .unwrap();
    Ok(())
}

#[cfg(not(tarpaulin_include))]
pub fn write_csv(
    masked_data: &CsvFile,
    output_file: &str,
    bar: &indicatif::ProgressBar,
) -> Result<(), MedError> {
    use csv::Writer;

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

#[cfg(not(tarpaulin_include))]
pub fn read_json(tx: flume::Sender<JsonFile>, path: String) -> Result<(), MedError> {
    let text = std::fs::read_to_string(&path)?;
    let data = serde_json::from_str::<Value>(&text)?;
    let mut total_records: usize = 0;
    if data.is_array() {
        total_records = data.as_array().unwrap().len();
    }
    tx.send(JsonFile {
        path,
        total_records,
        data,
    })
    .unwrap();
    Ok(())
}

#[cfg(not(tarpaulin_include))]
pub fn write_json(masked_data: &Value, output_file: &str) -> Result<(), MedError> {
    let mut json_file = File::create(output_file)?;
    let data = serde_json::to_string(masked_data)?;
    json_file.write_all(data.as_bytes())?;
    json_file.sync_data()?;
    Ok(())
}

#[cfg(test)]
#[path = "./tests/helpers_test.rs"]
mod helpers_test;
