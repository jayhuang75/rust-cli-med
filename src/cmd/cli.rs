use crate::utils::{
    config::{self, Config},
    error::MaskerError,
};
use clap::{arg, command, value_parser, ArgAction, Command};
use csv::StringRecord;
use serde_json::Value;
use std::{
    fmt,
    path::{Path, PathBuf},
};
use tracing::{log::info, warn};

#[derive(Debug, Clone)]
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
            Action::DECRYPT => write!(f, "decrypt"),
        }
    }
}

pub struct CliApp {
    pub file_path: String,
    pub file_type: FileType,
    pub conf_path: String,
    pub output_path: String,
    pub action: Action,
    pub conf: Config,
}

impl CliApp {
    /// Returns a CliApp with the input config
    ///
    /// # Examples
    /// ```
    /// let CliApp = CliApp::new().await?;
    /// ```
    pub async fn new() -> Result<Self, MaskerError> {
        //
        let mut file_path: String = String::default();
        let mut file_type: FileType = FileType::default();
        let mut conf_path: String = String::default();
        let mut output_path: String = String::default();
        let mut action: Action = Action::default();

        let matches = command!() // requires `cargo` feature
            .arg(
                arg!(
                    -c --config <CONFIG> "Sets a custom config yml path"
                )
                .required(false)
                .default_value("conf.yml")
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(
                    -d --dir <DIR> "Sets a file/directory path"
                )
                .required(true)
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(
                    -o --output <OUTPUT> "Sets a file/directory path for output"
                )
                .required(false)
                .default_value("output")
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(
                    -t --type <TYPE> "Sets a process file type"
                )
                .required(false)
                .default_value("csv"),
            )
            .get_matches();

        if let Some(path) = matches.get_one::<PathBuf>("config") {
            info!("conf.yml location {:?} : ", path.display());
            conf_path = path.display().to_string();
        }

        if let Some(path) = matches.get_one::<PathBuf>("dir") {
            info!("file location {:?} : ", path.display());
            file_path = path.display().to_string();
        }

        if let Some(path) = matches.get_one::<PathBuf>("output") {
            info!("output file location {:?} : ", path.display());
            output_path = path.display().to_string();
        }

        if let Some(f_type) = matches.get_one::<String>("type") {
            info!("file type {:?} : ", f_type);
            if f_type.to_owned() != FileType::CSV.to_string() {
                file_type = FileType::JSON;
            }
        }

        if let Some(action) = matches.get_one::<String>("action") {
            info!("action {:?} : ", action);
            
        }

        // init the config
        let path = Path::new(&conf_path);
        let conf = Config::new(path).await?;

        Ok(CliApp {
            file_path,
            file_type,
            conf_path,
            output_path,
            action,
            conf,
        })
    }

    /// Returns a file dir
    /// ```
    /// let app = CliApp::new().await?;
    /// let file_dir = app.get_file_dir().await?;
    /// ```
    pub async fn get_file_dir(&self) -> Result<String, MaskerError> {
        Ok(self.file_path.to_owned())
    }
}
