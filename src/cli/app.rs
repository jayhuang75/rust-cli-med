use crate::core::models::Params;
use crate::utils::{
    enums::{FileType, Mode},
    error::{MaskerError},
};
use clap::{arg, command, value_parser, ArgMatches};
use std::path::{PathBuf};
use tracing::log::info;
use crate::cli::custom_validation::{worker_in_range, dir_exist};

pub struct Cli {
    pub params: Params,
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
    /// let CliApp = Cli::new().await?;
    /// ```
    pub async fn new() -> Result<Self, MaskerError> {
        // Initial Default CLI params
        let new_cli = Params::default();

        // Get the cli input params
        let matches = Self::get_params().await;

        // replace the default cli params by the cli input from the prompt
        let params = Self::fulfill_cli(matches, new_cli).await?;

        // return the fulfilled CLI Params
        Ok(Cli { params })
    }

    /// Privite function fulfill the Cli Struct
    async fn fulfill_cli(matches: ArgMatches, mut params: Params) -> Result<Params, MaskerError> {
        // Note, it's safe to call unwrap() because the arg is required
        match matches
            .get_one::<Mode>("MODE")
            .expect("'MODE' is required and parsing will fail if its missing")
        {
            Mode::MASK => {
                params.mode = Mode::MASK;
                params.key = None;
            }
            Mode::ENCRYPT => {
                params.mode = Mode::ENCRYPT;
                if let Some(key) = matches.get_one::<String>("key") {
                    params.key = Some(key.to_owned());
                }
            }
            Mode::DECRYPT => {
                params.mode = Mode::DECRYPT;
                if let Some(key) = matches.get_one::<String>("key") {
                    params.key = Some(key.to_owned());
                }
            }
        }

        if let Some(path) = matches.get_one::<PathBuf>("config") {
            info!("conf.yml location {:?} : ", path.display());
            params.conf_path = path.display().to_string();
        }

        if let Some(path) = matches.get_one::<PathBuf>("file") {
            info!("file location {:?} : ", path.display());
            params.file_path = path.display().to_string();
        }

        if let Some(path) = matches.get_one::<PathBuf>("output") {
            info!("output file location {:?} : ", path.display());
            params.output_path = path.display().to_string();
        }

        if let Some(f_type) = matches.get_one::<String>("type") {
            if f_type.to_owned() != FileType::CSV.to_string() {
                params.file_type = FileType::JSON;
            }
        }

        if let Some(debug) = matches.get_one::<bool>("debug") {
            params.debug = debug.to_owned();
        }

        if let Some(worker) = matches.get_one::<usize>("worker") {
            params.worker = worker.to_owned();
        }

        Ok(params)
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
                    -t --type <TYPE> "Sets a process file type [csv, json]"
                )
                .required(false)
                .help("type of file we will process, available option [csv, json]")
                .default_value("csv")
            )
            .arg(
                arg!(
                    -k --key <KEY> "Sets a KEY to process file"
                )
                .help("key for Encrypt and Decrypt the file.")
                .required_if_eq_any([("MODE", "decrypt"),("MODE", "encrypt")])
            )
            .arg(
                arg!(
                    -f --file <FILE> "Sets a file/directory path"
                )
                .required(true)
                .help("file path for the ")
                .value_parser(dir_exist),
            )
            .arg(
                arg!(
                    -c --config <CONFIG> "Sets a custom config yml path"
                )
                .required(false)
                .default_value("conf.yaml")
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
                    -d --debug <DEBUG> "Sets debug flag"
                )
                .required(false)
                .value_parser(clap::value_parser!(bool)),
            )
            .arg(
                arg!(
                    -w --worker <WORKER> "Sets work flag"
                )
                .required(false)
                .value_parser(worker_in_range),
            )
            .get_matches()
    }
}
