use std::fs;

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde_json::Value;
use tracing::{info, debug};
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

pub fn find_key(value: &mut Value, job_conf: &JobConfig) {
   match value {
        Value::Array(arr) => {
            for a in arr {
                if a.is_object() {
                    find_key(a, job_conf);
                }
                job_conf.fields.par_iter().for_each(|field| {
                    if let Some(key) = a.to_owned().get_mut(field) {
                        if let Value::String(val) = key {
                            val.clear();
                            val.push_str(&job_conf.mask_symbols);
                        }  
                        debug!("mask output {:?}", key);                      
                    }
                });
            }
        }
        Value::Object(value) => {
            value.values().into_iter().for_each(|item| {
                if item.is_array() {
                    find_key(&mut item.clone(), job_conf);
                }
            });
        }
        _ => {}
    };
    
}
