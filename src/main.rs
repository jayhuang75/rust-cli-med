mod utils;

use utils::{config::Config, error::MaskerError};
use tracing::{info, debug};

#[tokio::main]
async fn main() -> Result<(), MaskerError> {
    
    // init the config
    let config = Config::new().await?;
    config.tracing().await;

    info!("test");
    debug!("debug");

    Ok(())
}
