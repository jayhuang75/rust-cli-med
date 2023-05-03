mod utils;
mod cmd;
use std::path::Path;

use utils::{config::Config, error::MaskerError};
use tracing::{info, debug};

#[tokio::main]
async fn main() -> Result<(), MaskerError> {
    
    // init the config
    let path = Path::new("conf.yml");
    let config = Config::new(path).await?;

    // init the tracing for logging
    config.tracing().await;

    // cmd load
    // design:
    // masker 
    // --file_path= [required]
    // --file_type= [csv,json] [required] 
    // --conf_path=conf.yml [optional] default ./conf.yml
    // --action=[mask,encrypt,decrypt] [optional] default is mask
    // --output=


    info!("test");
    debug!("debug");

    Ok(())
}
