use std::path::Path;

use crate::utils::{
    config::JobConfig,
    crypto::Cypher,
    error::{MedError, MedErrorType},
};

#[tokio::test]
async fn test_message() {
    let msg = "Io Error message".to_string();

    let err = MedError {
        message: Some(msg.to_string()),
        cause: None,
        error_type: MedErrorType::IoError,
    };
    assert_eq!(err.message(), msg, "Io Error message");
}

#[tokio::test]
async fn test_serde_yaml_error() {
    let test_config_path = Path::new("../demo/conf/invalid_conf.yaml");
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

#[tokio::test]
async fn test_magic_crypt_error() {
    // let sparkle_heart: Vec<i32> = vec![0, 159, 146, 150];

    let expect_magic_crypt_err = MedError {
        message: Some("Invalid byte 240, offset 0.".to_string()),
        cause: Some("magic_crypt error".to_string()),
        error_type: MedErrorType::CryptoError,
    };

    let key = "key".to_string();
    let new_cypto = Cypher::new(&key);

    // let sparkle_heart = vec![0, 159, 146, 150];
    let sparkle_heart = vec![240, 159, 146, 150];

    // We know these bytes are valid, so we'll use `unwrap()`.
    let sparkle_heart_str = String::from_utf8(sparkle_heart).unwrap();

    // testing the decryption failed.
    match new_cypto.decrypt(&sparkle_heart_str, &crate::models::enums::Standard::DES64) {
        Ok(_) => {
            unimplemented!()
        }
        Err(err) => {
            assert_eq!(err, expect_magic_crypt_err);
        }
    }
}

// #[tokio::test]
// async fn test_io_error() {
//     let data = r#"
//         {
//             "name": "John Doe",
//             "age": 43,
//             "phones": [
//                 "+44 1234567",
//                 "+44 2345678"
//             ]
//         }"#;

//     // Parse the string of data into serde_json::Value.
//     let v: Value = serde_json::from_str(data).unwrap();
//     match write_json(&v, "") {
//         Ok(_) => {
//             unimplemented!()
//         }
//         Err(e) => {
//             assert_eq!(e.error_type, MedErrorType::IoError);
//         }
//     }
// }

// #[tokio::test]
// async fn test_rayon_thread_pool_error() {
//     match Worker::new(0).await {
//         Ok(_) => unimplemented!(),
//         Err(e) => {
//             assert_eq!(e.error_type, MedErrorType::WorkerError);
//         }
//     }
// }
