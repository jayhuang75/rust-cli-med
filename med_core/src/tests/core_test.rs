use crate::app::core::App;
use crate::models::enums::{FileType, Mode};
use crate::models::params::Params;
use crate::utils::config::JobConfig;
use crate::utils::error::{MedError, MedErrorType};
use crate::utils::error::MedErrorType::ConfigError;

#[tokio::test]
async fn test_csv_mask_app() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();
    new_params.debug = true;

    let mut new_app = App::new(new_params.clone()).await.unwrap();
    assert_eq!(new_app.hostname, whoami::hostname());
    assert_eq!(new_app.params, new_params);
    assert_eq!(new_app.user, whoami::username());

    match new_app.load_job_config().await {
        Ok(c) => {
            let mut new_field: Vec<String> = Vec::new();
            new_field.push("name".to_owned());
            assert_eq!(
                c,
                JobConfig {
                    mask_symbols: "#####".to_string(),
                    fields: new_field
                }
            )
        }
        Err(e) => {
            assert_eq!(
                e,
                MedError {
                    message: Some("No such file or directory (os error 2)".to_owned()),
                    cause: Some("load job configuration yaml file failed!".to_owned()),
                    error_type: ConfigError
                }
            )
        }
    }

    let metrics = new_app.process().await.unwrap();
    assert_eq!(metrics.total_files, 0);
}

#[tokio::test]
async fn test_load_job_config() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();
    new_params.debug = false;

    let new_app = App::new(new_params).await.unwrap();
    let conf = new_app.load_job_config().await.unwrap();
    assert_eq!(conf.mask_symbols, "#####".to_string());
}

#[tokio::test]
async fn test_file_processor_failed() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();
    new_params.file_path = "../demo/data/input/format_err/csv".to_owned();
    new_params.output_path = "../demo/data/output/csv/format_err/processor_err".to_owned();
    new_params.file_type = FileType::CSV;
    new_params.mode = Mode::MASK;

    match App::new(new_params).await {
        Ok(mut new_app) => {
            let metrics = new_app.process().await.unwrap();
            assert_eq!(metrics.metadata.record_failed_reason.is_empty(), false);
        }
        Err(err) => {
            assert_eq!(err.error_type, MedErrorType::DatabaseError);
        }
    }
    
}

#[tokio::test]
async fn test_processor_run_encrypt() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();
    new_params.file_path = "../demo/data/input/csv".to_owned();
    new_params.output_path = "../demo/data/output/csv/mask".to_owned();
    new_params.file_type = FileType::CSV;
    new_params.mode = Mode::ENCRYPT;

    let mut new_app = App::new(new_params).await.unwrap();
    let metrics = new_app.process().await.unwrap();
    assert_eq!(metrics.metadata.failed_records, 0);
}

#[tokio::test]
async fn test_processor_run_encrypt_without_key() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();
    new_params.file_path = "../demo/data/input/csv".to_owned();
    new_params.output_path = "../demo/data/output/csv/mask".to_owned();
    new_params.file_type = FileType::CSV;
    new_params.mode = Mode::ENCRYPT;
    new_params.key = None;

    let mut new_app = App::new(new_params).await.unwrap();
    let metrics = new_app.process().await.unwrap();
    assert_eq!(metrics.metadata.total_records, 0);
}

#[tokio::test]
async fn test_processor_run_json_mask() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_json.yaml".to_owned();
    new_params.file_path = "../demo/data/input/json".to_owned();
    new_params.output_path = "../demo/data/output/json/mask".to_owned();
    new_params.file_type = FileType::JSON;
    new_params.mode = Mode::MASK;

    let mut new_app = App::new(new_params).await.unwrap();
    let metrics = new_app.process().await.unwrap();
    assert_eq!(metrics.metadata.failed_records, 0);
}
