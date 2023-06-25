use crate::app::core::App;
use crate::models::enums::{FileType, Mode};
use crate::models::params::Params;
use crate::utils::config::JobConfig;
use crate::utils::error::MedErrorType::ConfigError;
use crate::utils::error::{MedError, MedErrorType};

#[tokio::test]
async fn test_csv_mask_app() {
    let new_params = Params {
        conf_path: "../demo/conf/conf_csv.yaml".to_owned(),
        debug: true,
        ..Default::default()
    };

    let mut new_app = App::new(new_params.clone()).await.unwrap();
    assert_eq!(new_app.hostname, whoami::hostname());
    assert_eq!(new_app.params, new_params);
    assert_eq!(new_app.user, whoami::username());

    let metrics = new_app.process().await.unwrap();
    assert_eq!(metrics.total_files, 0);
}

#[tokio::test]
async fn test_load_job_config() {
    let new_params = Params {
        conf_path: "../demo/conf/conf_csv.yaml".to_owned(),
        debug: false,
        ..Default::default()
    };

    let new_app = App::new(new_params).await.unwrap();
    let conf = new_app.load_job_config().await.unwrap();
    assert_eq!(conf.mask_symbols, "#####".to_string());
}

#[tokio::test]
async fn test_load_job_config_failed() {
    let new_params = Params {
        conf_path: "".to_owned(),
        debug: false,
        ..Default::default()
    };

    let new_app = App::new(new_params).await.unwrap();
    match new_app.load_job_config().await {
        Ok(_) => {}
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
}

#[tokio::test]
async fn test_file_processor_failed() {
    let new_params = Params {
        conf_path: "../demo/conf/conf_csv.yaml".to_owned(),
        file_path: "../demo/data/input/format_err/csv".to_owned(),
        output_path: "../demo/data/output/csv/format_err/processor_err".to_owned(),
        file_type: FileType::CSV,
        mode: Mode::MASK,
        ..Default::default()
    };

    match App::new(new_params).await {
        Ok(mut new_app) => {
            let metrics = new_app.process().await.unwrap();
            assert!(!metrics.metadata.record_failed_reason.is_empty());
        }
        Err(err) => {
            assert_eq!(err.error_type, MedErrorType::DatabaseError);
        }
    }
}

#[tokio::test]
async fn test_processor_run_encrypt() {
    let new_params = Params {
        conf_path: "../demo/conf/conf_csv.yaml".to_owned(),
        file_path: "../demo/data/input/csv".to_owned(),
        output_path: "../demo/data/output/csv/mask".to_owned(),
        file_type: FileType::CSV,
        mode: Mode::ENCRYPT,
        ..Default::default()
    };

    let mut new_app = App::new(new_params).await.unwrap();
    let metrics = new_app.process().await.unwrap();
    assert_eq!(metrics.metadata.failed_records, 0);
}

#[tokio::test]
async fn test_processor_run_encrypt_without_key() {
    let new_params = Params {
        conf_path: "../demo/conf/conf_csv.yaml".to_owned(),
        file_path: "../demo/data/input/csv".to_owned(),
        output_path: "../demo/data/output/csv/mask".to_owned(),
        file_type: FileType::CSV,
        mode: Mode::ENCRYPT,
        key: None,
        ..Default::default()
    };

    let mut new_app = App::new(new_params).await.unwrap();
    let metrics = new_app.process().await.unwrap();
    assert_eq!(metrics.metadata.total_records, 0);
}

#[tokio::test]
async fn test_processor_run_json_mask() {
    let new_params = Params {
        conf_path: "../demo/conf/conf_json.yaml".to_owned(),
        file_path: "../demo/data/input/json".to_owned(),
        output_path: "../demo/data/output/json/mask".to_owned(),
        file_type: FileType::JSON,
        mode: Mode::MASK,
        ..Default::default()
    };

    let mut new_app = App::new(new_params).await.unwrap();
    let metrics = new_app.process().await.unwrap();
    assert_eq!(metrics.metadata.failed_records, 0);
}
