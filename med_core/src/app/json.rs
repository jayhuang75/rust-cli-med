use colored::Colorize;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use tracing::info;

use crate::{
    models::{enums::Mode, metrics::Metadata},
    utils::error::{MedError, MedErrorType},
};

use crate::app::processor::ProcessRuntime;

pub fn json_processor(
    tx_metadata: flume::Sender<Metadata>,
    files_path: &str,
    output_path: &str,
    process_runtime: ProcessRuntime,
) -> Result<(), MedError> {
    let text = std::fs::read_to_string(files_path)?;

    // prepare the metrics
    let mut total_records: usize = 0;
    let mut failed_records: usize = 0;
    let mut record_failed_reason: Vec<MedError> = Vec::new();

    match serde_json::from_str::<Value>(&text) {
        Ok(data) => {
            if data.is_array() {
                total_records = data.as_array().unwrap().len();
                let mut json_data = data;
                let new_json_data = json_med_core(&mut json_data, &process_runtime);
                write_json(&new_json_data, output_path).unwrap();
            }
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
    }

    tx_metadata
        .send(Metadata {
            total_records,
            failed_records,
            record_failed_reason,
        })
        .unwrap();

    Ok(())
}

fn json_med_core(value: &mut Value, process_runtime: &ProcessRuntime) -> Value {
    match value {
        Value::Array(arr) => {
            // debug!("[arr] {:?}", arr);
            for item in arr {
                if item.is_array() {
                    json_med_core(item, process_runtime);
                }

                if item.is_object() {
                    // info!("is obj {:?} ", val);
                    item.as_object_mut()
                        .unwrap()
                        .into_iter()
                        .for_each(|(key, val)| {
                            //debug!("key: {:?}, val: {:?} ", key, val);
                            //mask parent lvl
                            if process_runtime.fields.contains(key) {
                                if let Value::String(mut masked_val) = val.to_owned() {
                                    match process_runtime.mode {
                                        Mode::MASK => {
                                            masked_val.clear();
                                            let symbols =
                                                process_runtime.to_owned().mask_symbols.unwrap();
                                            masked_val.push_str(&symbols);
                                        }
                                        Mode::ENCRYPT => {
                                            if let Some(cypher) = process_runtime.to_owned().cypher
                                            {
                                                if let Some(standard) = process_runtime.standard {
                                                    let masked = cypher
                                                        .encrypt(&masked_val, &standard)
                                                        .unwrap();
                                                    masked_val.clear();
                                                    masked_val.push_str(&masked);
                                                }
                                            }
                                        }
                                        Mode::DECRYPT => {
                                            if let Some(cypher) = process_runtime.to_owned().cypher
                                            {
                                                if let Some(standard) = process_runtime.standard {
                                                    let masked = cypher
                                                        .decrypt(&masked_val, &standard)
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
                                json_med_core(val, process_runtime);
                            }

                            if val.is_object() {
                                json_med_core(val, process_runtime);
                            }
                        });
                }
            }
        }
        Value::Object(obj) => {
            for (key, val) in obj {
                // debug!("key : {:?}, val: {:?}", key, val);
                if val.is_array() {
                    json_med_core(val, process_runtime);
                }
                if process_runtime.fields.contains(key) {
                    if let Value::String(mut masked_val) = val.to_owned() {
                        match process_runtime.mode {
                            Mode::MASK => {
                                masked_val.clear();
                                masked_val
                                    .push_str(&process_runtime.to_owned().mask_symbols.unwrap());
                            }
                            Mode::ENCRYPT => {
                                if let Some(cypher) = process_runtime.to_owned().cypher {
                                    if let Some(standard) = process_runtime.standard {
                                        let masked =
                                            cypher.encrypt(&masked_val, &standard).unwrap();
                                        masked_val.clear();
                                        masked_val.push_str(&masked);
                                    }
                                }
                            }
                            Mode::DECRYPT => {
                                if let Some(cypher) = process_runtime.to_owned().cypher {
                                    if let Some(standard) = process_runtime.standard {
                                        let masked =
                                            cypher.decrypt(&masked_val, &standard).unwrap();
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

pub fn write_json(masked_data: &Value, output_file: &str) -> Result<(), MedError> {
    let mut json_file = File::create(output_file)?;
    let data = serde_json::to_string(masked_data)?;
    json_file.write_all(data.as_bytes())?;
    json_file.sync_data()?;
    Ok(())
}

#[cfg(test)]
#[path = "../tests/json_test.rs"]
mod json_test;
