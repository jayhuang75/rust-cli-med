use serde::Serialize;
use serde_json::Error;
use std::fmt;
use tokio::io;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub enum MedErrorType {
    ConfigError,
    IoError,
    CryptoError,
    WorkerError,
    SerdeJsonError,
    DatabaseError,
    CsvError,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct MedError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: MedErrorType,
}

impl MedError {
    #[allow(dead_code)]
    pub fn message(&self) -> String {
        match self {
            MedError {
                message: Some(message),
                ..
            } => message.clone(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

impl From<serde_yaml::Error> for MedError {
    fn from(error: serde_yaml::Error) -> MedError {
        MedError {
            message: Some(error.to_string()),
            cause: Some("can not open the conf.yml".to_string()),
            error_type: MedErrorType::ConfigError,
        }
    }
}

impl From<io::Error> for MedError {
    fn from(error: io::Error) -> MedError {
        MedError {
            message: Some(error.to_string()),
            cause: Some(error.to_string()),
            error_type: MedErrorType::IoError,
        }
    }
}

impl From<csv::Error> for MedError {
    fn from(error: csv::Error) -> MedError {
        MedError {
            message: Some(error.to_string()),
            cause: Some(error.to_string()),
            error_type: MedErrorType::CsvError,
        }
    }
}

impl From<magic_crypt::MagicCryptError> for MedError {
    fn from(error: magic_crypt::MagicCryptError) -> MedError {
        MedError {
            message: Some(error.to_string()),
            cause: Some("magic_crypt error".to_string()),
            error_type: MedErrorType::CryptoError,
        }
    }
}

// impl From<rayon::ThreadPoolBuildError> for MedError {
//     fn from(error: rayon::ThreadPoolBuildError) -> MedError {
//         MedError {
//             message: Some(error.to_string()),
//             cause: Some("rayon worker error".to_string()),
//             error_type: MedErrorType::WorkerError,
//         }
//     }
// }
#[cfg(not(tarpaulin_include))]
impl From<Error> for MedError {
    fn from(error: Error) -> MedError {
        MedError {
            message: Some(error.to_string()),
            cause: Some("serde json error".to_string()),
            error_type: MedErrorType::SerdeJsonError,
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl From<sqlx::Error> for MedError {
    fn from(error: sqlx::Error) -> MedError {
        MedError {
            message: Some(error.to_string()),
            cause: Some("database error".to_string()),
            error_type: MedErrorType::DatabaseError,
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl From<sqlx::migrate::MigrateError> for MedError {
    fn from(error: sqlx::migrate::MigrateError) -> MedError {
        MedError {
            message: Some(error.to_string()),
            cause: Some("database migration error".to_string()),
            error_type: MedErrorType::DatabaseError,
        }
    }
}

impl fmt::Display for MedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
#[path = "./tests/error_test.rs"]
mod error_test;
