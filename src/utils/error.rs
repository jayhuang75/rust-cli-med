use std::{fmt};

#[derive(Debug, PartialEq)]
pub enum MaskerErrorType {
    SystemError,
    DatabaseError,
    ConfigError
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

// impl From<sqlx::Error> for MachineError {
//     fn from(error: sqlx::Error) -> MachineError {
//         MachineError {
//             message: Some(error.to_string()),
//             cause: Some("database error".to_string()),
//             error_type: MachineErrorType::DatabaseError,
//         }
//     }
// }
// //
// impl From<sqlx::migrate::MigrateError> for MachineError {
//     fn from(error: sqlx::migrate::MigrateError) -> MachineError {
//         MachineError {
//             message: Some(error.to_string()),
//             cause: Some("database migration error".to_string()),
//             error_type: MachineErrorType::DatabaseError,
//         }
//     }
// }
// implement `
impl fmt::Display for MaskerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}