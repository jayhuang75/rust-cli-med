use std::path::Path;

use crate::utils::{config::JobConfig, error::MaskerErrorType};

#[tokio::test]
async fn test_new_config_failed_load() {
    let test_config_path = Path::new("");
    let test_config = JobConfig::new(test_config_path).await;
    match test_config {
        Ok(_) => {
            unimplemented!()
        }
        Err(err) => {
            assert_eq!(err.error_type, MaskerErrorType::IoError);
        }
    }
}

// #[tokio::test]
// async fn test_tracing_setup() {
//     let path = Path::new("conf.yml");
//     let config = Config::new(path).await.unwrap();
//     config.tracing().await;

// }