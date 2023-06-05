use crate::app::core::App;
use crate::models::enums::{FileType, Mode};
use crate::models::params::Params;
use crate::utils::config::JobConfig;
use crate::utils::error::MedErrorType::ConfigError;
use crate::utils::error::{MedError, MedErrorType};

#[tokio::test]
async fn test_csv_mask_app() {
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
async fn test_csv_encrypt_app() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();
    new_params.mode = Mode::ENCRYPT;
    new_params.key = Some("12345".to_owned());

    let mut new_app = App::new(new_params.clone()).await.unwrap();
    new_app.load_job_config().await.unwrap();

    let metrics = new_app.process().await.unwrap();
    assert_eq!(metrics.total_files, 0);
}

#[tokio::test]
async fn test_csv_cypher_without_key() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();
    new_params.mode = Mode::ENCRYPT;
    new_params.key = None;

    let mut new_app = App::new(new_params.clone()).await.unwrap();
    new_app.load_job_config().await.unwrap();

    match new_app.process().await {
        Ok(_) => {
            unimplemented!()
        }
        Err(e) => {
            assert_eq!(e.error_type, MedErrorType::ConfigError);
        }
    }
}

#[tokio::test]
async fn test_csv_decrypt_app() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();
    new_params.mode = Mode::DECRYPT;
    new_params.key = Some("12345".to_owned());

    let mut new_app = App::new(new_params.clone()).await.unwrap();
    new_app.load_job_config().await.unwrap();

    let metrics = new_app.process().await.unwrap();
    assert_eq!(metrics.total_files, 0);
}

#[tokio::test]
async fn test_json_mask_app() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_json.yaml".to_owned();
    new_params.mode = Mode::MASK;
    new_params.file_type = FileType::JSON;

    let mut new_app = App::new(new_params.clone()).await.unwrap();
    new_app.load_job_config().await.unwrap();

    let metrics = new_app.process().await.unwrap();
    assert_eq!(metrics.total_files, 0);
}

#[tokio::test]
async fn test_json_encrypt_app() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_json.yaml".to_owned();
    new_params.mode = Mode::ENCRYPT;
    new_params.file_type = FileType::JSON;
    new_params.key = Some("12345".to_owned());

    let mut new_app = App::new(new_params.clone()).await.unwrap();
    new_app.load_job_config().await.unwrap();

    let metrics = new_app.process().await.unwrap();
    assert_eq!(metrics.total_files, 0);
}

#[tokio::test]
async fn test_json_decrypt_app() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_json.yaml".to_owned();
    new_params.mode = Mode::DECRYPT;
    new_params.file_type = FileType::JSON;
    new_params.key = Some("12345".to_owned());

    let mut new_app = App::new(new_params.clone()).await.unwrap();
    new_app.load_job_config().await.unwrap();

    let metrics = new_app.process().await.unwrap();
    assert_eq!(metrics.total_files, 0);
}

#[tokio::test]
async fn test_json_cypher_without_key() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_json.yaml".to_owned();
    new_params.mode = Mode::ENCRYPT;
    new_params.file_type = FileType::JSON;
    new_params.key = None;

    let mut new_app = App::new(new_params.clone()).await.unwrap();
    new_app.load_job_config().await.unwrap();

    match new_app.process().await {
        Ok(_) => {
            unimplemented!()
        }
        Err(e) => {
            assert_eq!(e.error_type, MedErrorType::ConfigError);
        }
    }
}
