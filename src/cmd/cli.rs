use clap::{arg, command, value_parser, ArgMatches};
use std::{fmt, path::PathBuf};
use tracing::log::info;

// use crate::cmd::process::FileProcess;
// use crate::utils::{
//     config::{Config},
//     error::MaskerError,
// };

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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Cli {
    pub file_path: String,
    pub file_type: FileType,
    pub conf_path: String,
    pub output_path: String,
    pub process_action: Action,
    pub key: Option<String>,
    pub debug: bool,
}

impl Default for Cli {
    fn default() -> Self {
        let file_path: String = String::default();
        let file_type: FileType = FileType::default();
        let conf_path: String = String::default();
        let output_path: String = String::default();
        let process_action: Action = Action::default();
        let key: String = String::default();
        let debug: bool = false;

        Cli {
            file_path,
            file_type,
            conf_path,
            output_path,
            process_action,
            key: Some(key),
            debug,
        }    
    }
}

impl Cli {
    /// Returns a Cli with the input config
    ///
    /// - Usage: masker --dir <DIR>
    ///
    /// - -c --config optional default is the conf.yml
    /// - -f --file  this is required which is point to the files directory
    /// - -o --output optional default is /output
    /// - -t --type optional default is csv, [csv, json] are the two optional choice
    /// - -a --action optional default is mask, [mask, encrypt, decrypt]
    /// - -k --key optional, its only for encrypt, and decrypt
    /// - 
    ///
    /// # Examples
    /// ```
    /// let CliApp = CliApp::new().await?;
    /// ```
    pub async fn new(&self) -> Self {
        // Get the cli input params
        let matches = self.get_params().await;

        // Initial Default CLI params
        let new_cli = Cli::default();

        // replace the default cli params by the cli input from the prompt
        let fulfilled_cli = self.fulfill_cli(matches, new_cli).await;
        
        // return the fulfilled CLI Params
        fulfilled_cli
    }

    async fn fulfill_cli(&self, matches: ArgMatches, mut cli: Cli) -> Cli {
        if let Some(path) = matches.get_one::<PathBuf>("config") {
            info!("conf.yml location {:?} : ", path.display());
            cli.conf_path = path.display().to_string();
        }

        if let Some(path) = matches.get_one::<PathBuf>("dir") {
            info!("file location {:?} : ", path.display());
            cli.file_path = path.display().to_string();
        }

        if let Some(path) = matches.get_one::<PathBuf>("output") {
            info!("output file location {:?} : ", path.display());
            cli.output_path = path.display().to_string();
        }

        if let Some(f_type) = matches.get_one::<String>("type") {
            info!("file type {:?} : ", f_type);
            if f_type.to_owned() != FileType::CSV.to_string() {
                cli.file_type = FileType::JSON;
            }
        }

        if let Some(action) = matches.get_one::<String>("action") {
            info!("action {:?} : ", action);
            if action.to_owned() == Action::ENCRYPT.to_string() {
                cli.process_action = Action::ENCRYPT;
            } else if action.to_owned() == Action::DECRYPT.to_string() {
                cli.process_action = Action::DECRYPT;
            }
        }

        if let Some(key) = matches.get_one::<String>("key") {
            info!("key {:?} : ", key);
            cli.key = Some(key.to_owned());
        }

        if let Some(debug) = matches.get_one::<bool>("debug") {
            info!("debug {:?} : ", debug);
            cli.debug = debug.to_owned();
        }

        cli
    }


    async fn get_params(&self) -> ArgMatches {
        let matches = command!() // requires `cargo` feature
            .arg(
                arg!(
                    -c --config <CONFIG> "Sets a custom config yml path, optional default is conf.yml"
                )
                .required(false)
                .default_value("conf.yml")
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(
                    -f --file <FILE> "Sets a file/directory path"
                )
                .required(true)
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(
                    -o --output <OUTPUT> "Sets a file/directory path for output, default is /output"
                )
                .required(false)
                .default_value("output")
                .value_parser(value_parser!(PathBuf)),
            )
            .arg(
                arg!(
                    -t --type <TYPE> "Sets a process file type [csv, json], csv is the default value"
                )
                .required(false)
                .default_value("csv"),
            )
            .arg(
                arg!(
                    -a --action <ACTION> "Sets a process file type [csv, json], csv is the default value"
                )
                .required(true)
                .default_value("mask"),
            )
            .arg(
                arg!(
                    -k --key <KEY> "Sets Key for encryption and decryption"
                )
                .required(false),
            )
            .get_matches();

        matches
    }
}