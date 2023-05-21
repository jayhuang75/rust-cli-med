use std::fs;

use walkdir::WalkDir;

use crate::utils::error::{MaskerError, MaskerErrorType};

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
