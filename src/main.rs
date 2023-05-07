mod utils;
mod cmd;

use utils::{error::MaskerError};
use crate::cmd::app::App;

#[tokio::main]
async fn main() -> Result<(), MaskerError> {
    
    let new_app = App::new().await?;

    let result = new_app.process().await?;

    Ok(())
}
