mod utils;
mod cmd;

use utils::{error::MaskerError};
use tracing::{info, debug};

use crate::cmd::cli::Cli;

#[tokio::main]
async fn main() -> Result<(), MaskerError> {
    
    let app = Cli::new().await;

    println!("cli : {:?}", app);

    // app.conf.tracing().await;

    // info!("file location {:?}", app.get_file_dir().await?);
    // debug!("debug");

    Ok(())
}
