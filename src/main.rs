mod utils;
mod cmd;

use utils::{error::MaskerError};
use tracing::{info, debug};

use crate::cmd::cli::CliApp;

#[tokio::main]
async fn main() -> Result<(), MaskerError> {
    
    let app = CliApp::new().await?;
    app.conf.tracing().await;

    info!("file location {:?}", app.get_file_dir().await?);
    debug!("debug");

    Ok(())
}
