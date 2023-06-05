use crate::cli::custom_validation::{dir_exist, worker_in_range};
use clap::{arg, command, value_parser, ArgMatches};
use med_core::models::enums::{FileType, Mode, Standard};
use med_core::models::params::Params;
use med_core::utils::error::MedError;
use std::path::PathBuf;
use tracing::log::info;

pub struct Cli {
    pub params: Params,
}

#[cfg(not(tarpaulin_include))]
impl Cli {
    /// Returns a Cli with the input config
    ///
    /// Usage: masker <MODE> --file <FILE> [OPTIONS]
    ///
    /// Arguments:
    ///     <MODE>
    ///         What mode to run the program in
    ///         Possible values:
    ///             - mask:    Mask the data by *
    ///             - encrypt: Encrypt the data with provided KEY
    ///             - decrypt: Decrypt the data with provided KEY
    ///    
    /// Options:
    ///     -t, --type <TYPE>
    ///         type of file we will process, available option [csv, json]
    ///         [default: csv]
    ///     -k, --key <KEY>
    ///         key for Encrypt and Decrypt the file
    ///     -s, --standard <STANDARD>
    ///         encryption standard, available option [DES64, AES128, AES192, AES256]
    ///     -f, --file <FILE>
    ///         file path for the
    ///     -c, --config <CONFIG>
    ///         Sets a custom config yml path
    ///         [default: conf.yaml]
    ///     -o, --output <OUTPUT>
    ///         Sets a file/directory path for output
    ///         [default: output]
    ///     -d, --debug <DEBUG>
    ///         Sets debug flag
    ///         [possible values: true, false]
    ///     -w, --worker <WORKER>
    ///         Sets work flag
    ///     -h, --help
    ///         Print help (see a summary with '-h')
    ///     -V, --version
    ///         Print version
    ///
    /// # Examples
    /// ```
    /// let CliApp = Cli::new().await?;
    /// ```
    pub async fn new() -> Result<Self, MedError> {
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
    async fn fulfill_cli(matches: ArgMatches, mut params: Params) -> Result<Params, MedError> {
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
            if *f_type != FileType::CSV.to_string() {
                params.file_type = FileType::JSON;
            }
        }

        if let Some(debug) = matches.get_one::<bool>("debug") {
            params.debug = debug.to_owned();
        }

        if let Some(worker) = matches.get_one::<u16>("worker") {
            params.worker = worker.to_owned();
        }

        if let Some(standard) = matches.get_one::<Standard>("standard") {
            match standard {
                Standard::AES128 => {
                    params.standard = Standard::AES128;
                }
                Standard::AES192 => {
                    params.standard = Standard::AES192;
                }
                Standard::AES256 => {
                    params.standard = Standard::AES256;
                }
                Standard::DES64 => {
                    params.standard = Standard::DES64;
                }
            }
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
                .help("Type of file we will process, available option [csv, json]")
                .default_value("csv"),
            )
            .arg(
                arg!(
                    -k --key <KEY> "Sets a KEY to process file"
                )
                .help("Key for Encrypt and Decrypt the file.")
                .required_if_eq_any([("MODE", "decrypt"), ("MODE", "encrypt")])
                .requires("standard"),
            )
            .arg(
                arg!(
                    -s --standard <STANDARD> "Sets a Encrypt or Decrypt Standard"
                )
                .help("Set the Encrypt and Decrypt standard")
                .value_parser(value_parser!(Standard)),
            )
            .arg(
                arg!(
                    -f --file <FILE> "Sets a file/directory path"
                )
                .required(true)
                .help("Path for the process target files")
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
