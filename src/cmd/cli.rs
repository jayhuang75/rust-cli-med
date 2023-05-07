use crate::utils::{enums::{FileType, Mode}, error::{MaskerError, MaskerErrorType}};
use clap::{arg, command, value_parser, ArgMatches};
use std::{path::PathBuf};
use tracing::log::info;

#[derive(Debug, Clone)]
pub struct Cli {
    pub file_path: String,
    pub file_type: FileType,
    pub conf_path: String,
    pub output_path: String,
    pub mode: Mode,
    pub worker: usize,
    pub key: Option<String>,
    pub debug: bool,
}

impl Default for Cli {
    fn default() -> Self {
        let file_path: String = String::default();
        let file_type: FileType = FileType::default();
        let conf_path: String = String::default();
        let output_path: String = String::default();
        let mode: Mode = Mode::default();
        let key: String = String::default();
        let debug: bool = false;
        let worker = 2;

        Cli {
            file_path,
            file_type,
            conf_path,
            output_path,
            mode,
            key: Some(key),
            debug,
            worker
        }
    }
}

impl Cli {
    /// Returns a Cli with the input config
    ///
    /// - Usage: masker [MODE] --file <FILE_PATH> <OPTIONS>
    ///
    /// OPTIONS available
    /// - -c --config optional default is the conf.yml
    /// - -f --file  this is required which is point to the files directory
    /// - -o --output optional default is /output
    /// - -t --type optional default is csv, [csv, json] are the two optional choice
    /// - -k --key optional, its only for encrypt, and decrypt
    /// - -d --debug optional, default false
    /// - -w --worker optional, worker for processing, default is 2
    ///
    /// # Examples
    /// ```
    /// let CliApp = CliApp::new().await?;
    /// ```
    pub async fn new() -> Result<Self, MaskerError> {
        // Initial Default CLI params
        let new_cli = Cli::default();

        // Get the cli input params
        let matches = Self::get_params().await;

        // replace the default cli params by the cli input from the prompt
        let fulfilled_cli = Self::fulfill_cli(matches, new_cli).await?;

        // return the fulfilled CLI Params
        Ok(fulfilled_cli)
    }

    /// Privite function fulfill the Cli Struct
    async fn fulfill_cli(matches: ArgMatches, mut cli: Cli) -> Result<Cli, MaskerError>{
        // Note, it's safe to call unwrap() because the arg is required
        match matches
            .get_one::<Mode>("MODE")
            .expect("'MODE' is required and parsing will fail if its missing")
        {
            Mode::MASK => {
                cli.mode = Mode::MASK;
                cli.key = None;
            }
            Mode::ENCRYPT => {
                cli.mode = Mode::ENCRYPT;
                if let Some(key) = matches.get_one::<String>("key") {
                    cli.key = Some(key.to_owned());
                }
            }
            Mode::DECRYPT => {
                cli.mode = Mode::DECRYPT;
                if let Some(key) = matches.get_one::<String>("key") {
                    cli.key = Some(key.to_owned());
                }
            }
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

        if let Some(worker) = matches.get_one::<usize>("worker") {
            info!("worker {:?} : ", worker);
            let cpu_nums = num_cpus::get();
            if worker > &cpu_nums {
                return Err(MaskerError {
                    message: Some(format!(
                        "worker is over your current max CPU number: {:?}, consider lower the worker", cpu_nums
                    )),
                    cause: Some(format!("max worker reach {:?}", cpu_nums)),
                    error_type: MaskerErrorType::ConfigError,
                });
            }
            cli.worker = worker.to_owned();
        }

        Ok(cli)
    }

    /// Privite function get the Clap parsed params.
    async fn get_params() -> ArgMatches {
        command!()
            .propagate_version(true)
            .arg_required_else_help(true)
            .arg(
                arg!(<MODE>)
                    .required(true)
                    .help("What mode to run the program in")
                    .value_parser(value_parser!(Mode)),
            )
            .arg(
                arg!(
                    -t --type <TYPE> "Sets a process file type [csv, json], csv is the default value"
                )
                .required(false)
                .default_value("csv")
            )
            .arg(
                arg!(
                    -k --key <KEY> "Sets a KEY to process file"
                )
                .required_if_eq_any([("MODE", "decrypt"),("MODE", "encrypt")])
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
            .arg(
                arg!(
                    -w --worker <WORKER> "Sets work flag, default is 2"
                )
                .required(false)
                .value_parser(clap::value_parser!(usize)),
            )
            .get_matches()
    }
}
