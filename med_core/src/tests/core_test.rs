use crate::app::core::App;
use crate::models::enums::{AppMode, FileType, Mode, Standard};
use crate::models::params::{Params};
use crate::utils::config::JobConfig;
use crate::utils::error::MaskerError;
use crate::utils::error::MaskerErrorType::ConfigError;

#[tokio::test]
async fn test_params_init() {
    let new_params = Params::default();
    assert_eq!(new_params.app_mode, AppMode::CLI);
    assert_eq!(new_params.debug, false);
    assert_eq!(new_params.file_type, FileType::CSV);
    assert_eq!(new_params.file_path, "");
    assert_eq!(new_params.conf_path, "");
    assert_eq!(new_params.mode, Mode::MASK);
    assert_eq!(new_params.standard, Standard::DES64);
    assert_eq!(new_params.key, Some("".to_owned()));
    assert_eq!(new_params.to_string(), "app_mode: cli, file_path: , file_type: csv, conf_path: , output_path: , mode: mask, key: Some(\"\"), debug: false, worker: 2");
}

#[tokio::test]
async fn test_app() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();

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
                MaskerError {
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
