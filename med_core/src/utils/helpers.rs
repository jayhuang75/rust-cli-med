use std::fs;

use crate::utils::config::JobConfig;
use crate::utils::error::{MaskerError, MaskerErrorType};
use serde_json::Value;
use walkdir::WalkDir;

use crate::utils::enums::{Mode, Standard};

use crate::utils::crypto::Cypher;

pub fn check_if_field_exist_in_job_conf(indexs: Vec<usize>) {
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

pub async fn create_output_dir(output_dir: &str, file_dir: &str) -> Result<(), MaskerError> {
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

pub fn json_find_and_mask(value: &mut Value, job_conf: &JobConfig) -> Value {
    match value {
        Value::Array(arr) => {
            // debug!("[arr] {:?}", arr);
            for item in arr {
                if item.is_array() {
                    json_find_and_mask(item, job_conf);
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
                                    masked_val.clear();
                                    masked_val.push_str(&job_conf.mask_symbols);
                                    *val = Value::String(masked_val);
                                }
                            }

                            if val.is_array() {
                                json_find_and_mask(val, job_conf);
                            }

                            if val.is_object() {
                                json_find_and_mask(val, job_conf);
                            }
                        });
                }
            }
        }
        Value::Object(obj) => {
            for (key, val) in obj {
                // debug!("key : {:?}, val: {:?}", key, val);
                if val.is_array() {
                    json_find_and_mask(val, job_conf);
                } else {
                    if job_conf.fields.contains(key) {
                        if let Value::String(mut masked_val) = val.to_owned() {
                            masked_val.clear();
                            masked_val.push_str(&job_conf.mask_symbols);
                            *val = Value::String(masked_val);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    value.clone()
}

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
                                    masked_val.clear();

                                    match mode {
                                        Mode::MASK => {
                                            masked_val.push_str(&job_conf.mask_symbols);
                                        }
                                        Mode::ENCRYPT => {
                                            if let Some(cypher) = cypher {
                                                if let Some(standard) = standard {
                                                    let masked = cypher
                                                        .encrypt(&masked_val, standard)
                                                        .unwrap();
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
                } else {
                    if job_conf.fields.contains(key) {
                        if let Value::String(mut masked_val) = val.to_owned() {
                            masked_val.clear();

                            match mode {
                                Mode::MASK => {
                                    masked_val.push_str(&job_conf.mask_symbols);
                                }
                                Mode::ENCRYPT => {
                                    if let Some(cypher) = cypher {
                                        if let Some(standard) = standard {
                                            let masked =
                                                cypher.encrypt(&masked_val, standard).unwrap();
                                            masked_val.push_str(&masked);
                                        }
                                    }
                                }
                                Mode::DECRYPT => {
                                    if let Some(cypher) = cypher {
                                        if let Some(standard) = standard {
                                            let masked =
                                                cypher.decrypt(&masked_val, standard).unwrap();
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
        }
        _ => {}
    }
    value.clone()
}
