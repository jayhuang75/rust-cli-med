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
