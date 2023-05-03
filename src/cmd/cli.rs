use csv::StringRecord;
use serde_json::Value;
use tracing::log::info;
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

        let mut file_path: String = "".to_owned();
        let mut file_type: FileType = FileType::default();
        let mut conf_path: String = "".to_owned();
        let mut output_path: String;
        let mut action: Action = Action::default();

        let matches = command!() // requires `cargo` feature
            .arg(
                arg!(
                    -c --config_path <CONFIG_LOCATION> "Sets a custom config yml path"
                )
                .required(true)
                .default_value("./conf.yml")
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(
                    -f --file_path <FILE_LOCATION> "Sets a file/directory path"
                )
                .required(true)
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(
                    -o --output_path <OUTPUT_FILE_LOCATION> "Sets a file/directory path for output"
                )
                .required(true)
                .default_value("./out")
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(
                    -t --file_type <FILE_TYPE> "Sets a process file type"
                )
                .required(false)
                .default_value("csv")
            )
            .get_matches();

        if let Some(config_path) = matches.get_one::<PathBuf>("config") {
            info!("conf.yml location {:?} : ", config_path.display());
            conf_path = config_path.display().to_string();
        }

        if let Some(output_path) = matches.get_one::<PathBuf>("output_path") {
            info!("conf.yml location {:?} : ", output_path.display());
            conf_path = output_path.display().to_string();
        }

        CliApp{
            file_path,
            file_type,
            conf_path,
            output_path,
            action,
        }

    }
}
