use crate::utils::error::{MaskerError, MaskerErrorType};

#[tokio::test]
async fn test_message() {
    let msg = "Io Error message".to_string();

    let err = MaskerError {
        message: Some(msg.to_string()),
        cause: None,
        error_type: MaskerErrorType::IoError,
    };
    assert_eq!(err.message(), msg, "Io Error message");
}

// async fn serde_json_error() -> Result<AccountDetails, LedgerError> {
//     let data = r#"
//     {
//         "name": "John Doe",
//         "age": 43,
//         "phones": [
//             "+44 1234567",
//             "+44 2345678"
//         ]
//     }"#;

//     let account_data: AccountDetails = serde_json::from_str(data)?;

//     Ok(account_data)
// }

// #[actix_rt::test]
// async fn test_serde_json_error() {
//     let expect_err = LedgerError {
//         message: Some("missing field `id` at line 9 column 5".to_string()),
//         cause: Some("serde_json error".to_string()),
//         error_type: LedgerErrorType::SerdeJsonError,
//     };

//     match serde_json_error().await {
//         Ok(_) => {
//             unimplemented!();
//         }
//         Err(err) => {
//             assert_eq!(err, expect_err);
//         }
//     }
//}
