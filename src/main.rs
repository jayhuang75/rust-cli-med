mod utils;
mod cmd;

use utils::{error::MaskerError};
use tracing::{info, debug};

#[tokio::main]
async fn main() -> Result<(), MaskerError> {
    
    

    // info!("file location {:?}", app.get_file_dir().await?);
    // debug!("debug");

    Ok(())
}
