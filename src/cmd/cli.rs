use csv::StringRecord;
use serde_json::Value;
use std::{path::PathBuf, fmt};
use clap::{arg, command, value_parser, ArgAction, Command};

pub enum FileType {
    CSV,
    JSON,
}

impl Default for FileType {
    fn default() -> Self {
        FileType::CSV
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::CSV => write!(f, "csv"),
            FileType::JSON => write!(f, "json"),
        }
    }
}

pub enum Action {
    MASK,
    ENCRYPT,
    DECRYPT,
}

impl Default for Action {
    fn default() -> Self {
        Action::MASK
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::MASK => write!(f, "mask"),
            Action::ENCRYPT => write!(f, "encrypt"),
            Action::DECRYPT => write!(f, "decrypt")
        }
    }
}

pub struct CliApp {
    pub file_path: String,
    pub file_type: FileType,
    pub conf_path: String,
    pub output_path: String,
    pub action: Action,
}

impl CliApp {

    pub async fn new() -> Self {
        let matches = command!() // requires `cargo` feature
            .arg(
                arg!(
                    -c --config_path <FILE> "Sets a custom config yaml path"
                )
                .required(true)
                .default_value("./conf.yml")
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(
                    -f --file_path <FILE> "Sets a file/directory path"
                )
                .required(true)
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(
                    -o --output_path <FILE> "Sets a file/directory path for output"
                )
                .required(true)
                .default_value("./out")
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(
                    -t --file_type <FILE_TYPE> "Sets a process file type"
                )
                .required(true)
                .default_value("csv")
            )
            .get_matches();

        if let Some(config_path) = matches.get_one::<PathBuf>("config") {
            println!("Value for config: {}", config_path.display());

        }

    }
}
