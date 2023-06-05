use std::path::Path;

use crate::utils::{config::JobConfig, error::MedErrorType};

#[tokio::test]
async fn test_new_config_failed_load() {
    let test_config_path = Path::new("");
    let test_config = JobConfig::new(test_config_path).await;
    match test_config {
        Ok(_) => {
            unimplemented!()
        }
        Err(err) => {
            assert_eq!(err.error_type, MedErrorType::ConfigError);
        }
    }
}
