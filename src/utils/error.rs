use std::{fmt};
use tokio::io;

#[derive(Debug, PartialEq)]
pub enum MaskerErrorType {
    ConfigError,
    IoError,
    CryptoError,
    WorkerError,
    SerdeJsonError,
    DatabaseError,
}

#[derive(Debug, PartialEq)]
pub struct MaskerError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: MaskerErrorType,
}

impl MaskerError {
    #[allow(dead_code)]
    pub fn message(&self) -> String {
        match &*self {
            MaskerError {
                message: Some(message),
                ..
            } => message.clone(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

impl From<serde_yaml::Error> for MaskerError {
    fn from(error: serde_yaml::Error) -> MaskerError {
        MaskerError {
            message: Some(error.to_string()),
            cause: Some("can not open the conf.yml".to_string()),
            error_type: MaskerErrorType::ConfigError,
        }
    }
}

impl From<io::Error> for MaskerError {
    fn from(error: io::Error) -> MaskerError {
        MaskerError {
            message: Some(error.to_string()),
            cause: Some(error.to_string()),
            error_type: MaskerErrorType::IoError,
        }
    }
}

impl From<csv::Error> for MaskerError {
    fn from(error: csv::Error) -> MaskerError {
        MaskerError {
            message: Some(error.to_string()),
            cause: Some(error.to_string()),
            error_type: MaskerErrorType::IoError,
        }
    }
}

impl From<magic_crypt::MagicCryptError> for MaskerError {
    fn from(error: magic_crypt::MagicCryptError) -> MaskerError {
        MaskerError {
            message: Some(error.to_string()),
            cause: Some("magic_crypt error".to_string()),
            error_type: MaskerErrorType::CryptoError,
        }
    }
}

impl From<rayon::ThreadPoolBuildError> for MaskerError {
    fn from(error: rayon::ThreadPoolBuildError) -> MaskerError {
        MaskerError {
            message: Some(error.to_string()),
            cause: Some("rayon worker error".to_string()),
            error_type: MaskerErrorType::WorkerError,
        }
    }
}

impl From<serde_json::Error> for MaskerError {
    fn from(error: serde_json::Error) -> MaskerError {
        MaskerError {
            message: Some(error.to_string()),
            cause: Some("serde json error".to_string()),
            error_type: MaskerErrorType::SerdeJsonError,
        }
    }
}



impl From<sqlx::Error> for MaskerError {
    fn from(error: sqlx::Error) -> MaskerError {
        MaskerError {
            message: Some(error.to_string()),
            cause: Some("database error".to_string()),
            error_type: MaskerErrorType::DatabaseError,
        }
    }
}

impl From<sqlx::migrate::MigrateError> for MaskerError {
    fn from(error: sqlx::migrate::MigrateError) -> MaskerError {
        MaskerError {
            message: Some(error.to_string()),
            cause: Some("database migration error".to_string()),
            error_type: MaskerErrorType::DatabaseError,
        }
    }
}

impl fmt::Display for MaskerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
#[path = "./tests/error_test.rs"]
mod error_test;