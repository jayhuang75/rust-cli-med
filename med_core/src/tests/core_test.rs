use crate::app::core::App;
use crate::app::processor::FileProcessor;
use crate::models::enums::{FileType, Mode};
use crate::models::metrics;
use crate::models::params::Params;
use crate::utils::config::JobConfig;
use crate::utils::error::MedError;
use crate::utils::error::MedErrorType::{self, ConfigError};

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

    let new_app = App::new(new_params).await.unwrap();
    let conf = new_app.load_job_config().await.unwrap();
    assert_eq!(conf.mask_symbols, "#####".to_string());
}

#[tokio::test]
async fn test_file_processor_failed() {
    let mut new_params = Params::default();
    new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();
    new_params.file_path = "../demo/data/format_err/csv".to_owned();
    new_params.output_path = "../output/demo/data/format_err/csv/test".to_owned();
    new_params.file_type = FileType::CSV;
    new_params.mode = Mode::MASK;

    let mut new_app = App::new(new_params).await.unwrap();
    let metrics = new_app.process().await.unwrap();
    assert_eq!(metrics.metadata.failed_records, 2);
}

// #[tokio::test]
// async fn test_csv_encrypt_app() {
//     let mut new_params = Params::default();
//     new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();
//     new_params.mode = Mode::ENCRYPT;
//     new_params.key = Some("12345".to_owned());

//     let mut new_app = App::new(new_params.clone()).await.unwrap();
//     new_app.load_job_config().await.unwrap();

//     let metrics = new_app.process().await.unwrap();
//     assert_eq!(metrics.total_files, 0);
// }

// // #[tokio::test]
// // async fn test_csv_cypher_without_key() {
// //     let mut new_params = Params::default();
// //     new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();
// //     new_params.mode = Mode::ENCRYPT;
// //     new_params.key = None;

// //     let mut new_app = App::new(new_params.clone()).await.unwrap();
// //     new_app.load_job_config().await.unwrap();

// //     match new_app.process().await {
// //         Ok(_) => {
// //            unimplemented!();
// //         }
// //         Err(e) => {
// //             assert_eq!(e.error_type, MedErrorType::ConfigError);
// //         }
// //     }
// // }

// #[tokio::test]
// async fn test_csv_decrypt_app() {
//     let mut new_params = Params::default();
//     new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();
//     new_params.mode = Mode::DECRYPT;
//     new_params.key = Some("12345".to_owned());

//     let mut new_app = App::new(new_params.clone()).await.unwrap();
//     new_app.load_job_config().await.unwrap();

//     let metrics = new_app.process().await.unwrap();
//     assert_eq!(metrics.total_files, 0);
// }

// #[tokio::test]
// async fn test_json_mask_app() {
//     let mut new_params = Params::default();
//     new_params.conf_path = "../demo/conf/conf_json.yaml".to_owned();
//     new_params.mode = Mode::MASK;
//     new_params.file_type = FileType::JSON;

//     let mut new_app = App::new(new_params.clone()).await.unwrap();
//     new_app.load_job_config().await.unwrap();

//     let metrics = new_app.process().await.unwrap();
//     assert_eq!(metrics.total_files, 0);
// }

// #[tokio::test]
// async fn test_json_encrypt_app() {
//     let mut new_params = Params::default();
//     new_params.conf_path = "../demo/conf/conf_json.yaml".to_owned();
//     new_params.mode = Mode::ENCRYPT;
//     new_params.file_type = FileType::JSON;
//     new_params.key = Some("12345".to_owned());

//     let mut new_app = App::new(new_params.clone()).await.unwrap();
//     new_app.load_job_config().await.unwrap();

//     let metrics = new_app.process().await.unwrap();
//     assert_eq!(metrics.total_files, 0);
// }

// #[tokio::test]
// async fn test_json_decrypt_app() {
//     let mut new_params = Params::default();
//     new_params.conf_path = "../demo/conf/conf_json.yaml".to_owned();
//     new_params.mode = Mode::DECRYPT;
//     new_params.file_type = FileType::JSON;
//     new_params.key = Some("12345".to_owned());

//     let mut new_app = App::new(new_params.clone()).await.unwrap();
//     new_app.load_job_config().await.unwrap();

//     let metrics = new_app.process().await.unwrap();
//     assert_eq!(metrics.total_files, 0);
// }

// // #[tokio::test]
// // async fn test_json_cypher_without_key() {
// //     let mut new_params = Params::default();
// //     new_params.conf_path = "../demo/conf/conf_json.yaml".to_owned();
// //     new_params.mode = Mode::ENCRYPT;
// //     new_params.file_type = FileType::JSON;
// //     new_params.key = None;

// //     let mut new_app = App::new(new_params.clone()).await.unwrap();
// //     new_app.load_job_config().await.unwrap();

// //     match new_app.process().await {

// // }

// #[tokio::test]
// async fn test_new_file_processor() {
//     let mut new_params = Params::default();
//     new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();
//     new_params.mode = Mode::MASK;
//     new_params.worker = 3;

//     let new_app = App::new(new_params.clone()).await.unwrap();
//     let job_conf = new_app.load_job_config().await.unwrap();

//     let file_process = FileProcessor::new(new_params.clone(), job_conf.clone()).await;
//     assert_eq!(file_process.process_runtime.mask_symbols.unwrap(), "#####");

//     let mut file_process = FileProcessor::new(new_params, job_conf).await;
//     let metrics = file_process.run().await.unwrap();
//     assert_eq!(metrics.total_files, 0);
// }

// #[tokio::test]
// async fn test_csv_encrypt_without_key() {
//     let mut new_params = Params::default();
//     new_params.conf_path = "../demo/conf/conf_csv.yaml".to_owned();
//     new_params.mode = Mode::ENCRYPT;
//     new_params.worker = 3;
//     new_params.key = None;

//     let new_app = App::new(new_params.clone()).await.unwrap();
//     let job_conf = new_app.load_job_config().await.unwrap();

//     let file_process = FileProcessor::new(new_params.clone(), job_conf.clone()).await;
//     assert_eq!(file_process.process_runtime.mask_symbols.unwrap(), "#####");

//     let mut file_process = FileProcessor::new(new_params, job_conf).await;
//     match file_process.run().await {
//         Ok(_) => {
//             unimplemented!()
//         }
//         Err(e) => {
//             assert_eq!(e.error_type, MedErrorType::ConfigError);
//         }
//     }
// }

// #[tokio::test]
// async fn test_json_decrypt_without_key() {
//     let mut new_params = Params::default();
//     new_params.conf_path = "../demo/conf/conf_json.yaml".to_owned();
//     new_params.mode = Mode::DECRYPT;
//     new_params.worker = 3;
//     new_params.key = None;
//     new_params.file_type = FileType::JSON;
//     new_params.file_path = "../demo/data/json".to_owned();
//     new_params.debug = true;

//     let new_app = App::new(new_params.clone()).await.unwrap();
//     let job_conf = new_app.load_job_config().await.unwrap();

//     let file_process = FileProcessor::new(new_params.clone(), job_conf.clone()).await;
//     assert_eq!(file_process.process_runtime.mask_symbols.unwrap(), "#####");

//     let mut file_process = FileProcessor::new(new_params, job_conf).await;
//     match file_process.run().await {
//         Ok(_) => {
//             unimplemented!()
//         }
//         Err(e) => {
//             assert_eq!(e.error_type, MedErrorType::ConfigError);
//         }
//     }
// }
