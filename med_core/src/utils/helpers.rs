use std::fs;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde_json::Value;
use tracing::{debug, info};
use walkdir::WalkDir;

use crate::utils::error::{MaskerError, MaskerErrorType};

use crate::utils::config::JobConfig;

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

pub fn find_key(value: &Value, job_conf: &JobConfig) -> Value {
    let mut array: Vec<Value> = vec![];
    let mut map = serde_json::Map::new();
    match value {
        Value::String(str_val) => {
            info!("[str] {:?}", str_val);
        }
        Value::Array(arr) => {
            // info!("[arr] {:?}", arr);
            for val in arr {
                find_key(val, job_conf);
            }
        }
        Value::Object(obj) => {
            for (key, val) in obj {
                if val.is_array() {
                    find_key(val, job_conf);
                } else {
                    // info!("[obj] key: {:?} : value: {:?}", key, val);
                    if job_conf.fields.contains(key) {
                        if let Value::String(mut mask_val) = val.to_owned() {
                            mask_val.clear();
                            mask_val.push_str(&job_conf.mask_symbols);
                            // map.insert(key.clone(), Value::String(val));
                            info!("[obj] key: {:?} : value: {:?}", key, mask_val);
                        }
                    } else {
                        info!("[obj] key: {:?} : value: {:?}", key, val);
                    }
                }
            }
        }
        _ => {}
    }
    Value::Array(array)
}

// pub fn find_key(value: &Value, job_conf: &JobConfig) -> Value {
//     let mut array: Vec<Value> = vec![];
//     match value {
//         Value::Array(arr) => {
//             for a in arr {
//                 // root level
//                 // info!("[array] is array: {:?}", a.is_array());
//                 // info!("[array] is object: {:?}", a.is_object());
//                 let mut map = serde_json::Map::new();
//                 if a.is_object() {
//                     for (key, value) in a.as_object().unwrap() {
//                         // info!("[array] {:?}:  {:?}", key, value);

//                         if value.is_array() {
//                             // info!("[array] before {:?} : {:?}", key, value);
//                             let value_is_array = find_key(value, job_conf);
//                             // info!("[array] after {:?} : {:?}", key, value_is_array);
//                             map.insert(key.clone(), value_is_array);
//                         } else {
//                             map.insert(key.clone(), value.clone());
//                             if job_conf.fields.contains(key) {
//                                 if let Value::String(mut val) = value.to_owned() {
//                                     val.clear();
//                                     val.push_str(&job_conf.mask_symbols);
//                                     map.insert(key.clone(), Value::String(val));
//                                 }
//                             } else {
//                                 map.insert(key.clone(), value.clone());
//                             }
//                         }
//                     }
//                 }
//                 array.push(Value::Object(map));
//             }
//         }
//         Value::Object(value) => {
//             for (key, value) in value {
//                 info!("[obj] {:?}:  {:?}", key, value);
//                 todo!("object based");
//             }
//         }
//         _ => {}
//     };
//     Value::Array(array)
// }
