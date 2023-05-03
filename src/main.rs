mod utils;
mod cmd;
use std::path::Path;

use utils::{config::Config, error::MaskerError};
use tracing::{info, debug};

use crate::cmd::cli::CliApp;

#[tokio::main]
async fn main() -> Result<(), MaskerError> {
    
    // cmd load
    // design:
    // masker 
    // --file_path= [required]
    // --file_type= [csv,json] [required] 
    // --conf_path=conf.yml [optional] default ./conf.yml
    // --action=[mask,encrypt,decrypt] [optional] default is mask
    // --output=
    let app = CliApp::new().await?;
    app.conf.tracing().await;

    info!("file location {:?}", app.get_file_dir().await?);
    debug!("debug");

    Ok(())
}
