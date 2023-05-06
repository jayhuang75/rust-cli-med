use clap::{arg, command, value_parser, ArgMatches, Command};
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
    /// - -d --debug optional, default false
    ///
    /// # Examples
    /// ```
    /// let CliApp = CliApp::new().await?;
    /// ```
    pub async fn new() {

        // Initial Default CLI params
        let new_cli = Cli::default();

        // Get the cli input params
        let matches = Self::get_params().await;

        // replace the default cli params by the cli input from the prompt
        let fulfilled_cli = Self::fulfill_cli(matches, new_cli).await;

        // return the fulfilled CLI Params
        fulfilled_cli
    }

    async fn fulfill_cli(matches: ArgMatches, mut cli: Cli) {
        match matches.subcommand() {
            Some(("mask", sub_matches)) =>{
                println!("mask tigger");
            },
            Some(("encrypt", sub_matches)) => {
                if let Some(key) = sub_matches.get_one::<String>("key") {
                    println!("key {:?} : ", key);
                    cli.key = Some(key.to_owned());
                }
            },
            Some(("decrypt", sub_matches)) => { 
                if let Some(key) = sub_matches.get_one::<String>("key") {
                    info!("key {:?} : ", key);
                    cli.key = Some(key.to_owned());
                }
            },
            _ => unreachable!(
                "Exhausted list of subcommands and subcommand_required prevents `None`"
            ),
        }

        if let Some(path) = matches.get_one::<PathBuf>("config") {
            info!("conf.yml location {:?} : ", path.display());
            cli.conf_path = path.display().to_string();
        }

        if let Some(path) = matches.get_one::<PathBuf>("file") {
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

        if let Some(debug) = matches.get_one::<bool>("debug") {
            info!("debug {:?} : ", debug);
            cli.debug = debug.to_owned();
        }

        // cli
    }

    pub async fn get_params() -> ArgMatches {
        command!() // requires `cargo` feature
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("mask")
                .about("Mask file/files")
        )
        .subcommand(
            Command::new("encrypt")
                .about("Encrypt file/files")
                .arg(
                    arg!(
                        -k --key <KEY> "Sets Key for encryption"
                    )
                    .required(true)
                ),
        )
        .subcommand(
            Command::new("decrypt")
                .about("Decrypt file/files")
                .arg(
                    arg!(
                        -k --key <KEY> "Sets Key for decryption"
                    )
                    .required(true)
                ),
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
                -f --file <FILE> "Sets a file/directory path"
            )
            .required(true)
            .value_parser(value_parser!(PathBuf)),
        )
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
                -o --output <OUTPUT> "Sets a file/directory path for output, default is /output"
            )
            .required(false)
            .default_value("output")
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -d --debug <DEBUG> "Sets debug flag [true, false]"
            )
            .required(false)
        )
        .get_matches()
    }
}
