use crate::utils::error::MedError;
use std::fs;
use walkdir::WalkDir;

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

#[cfg(test)]
#[path = "./tests/helpers_test.rs"]
mod helpers_test;
