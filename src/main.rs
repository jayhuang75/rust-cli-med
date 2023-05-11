mod utils;
mod core;

use tokio::time::Instant;
use tracing::info;
use utils::{error::MaskerError};
use crate::core::app::App;

#[tokio::main]
async fn main() -> Result<(), MaskerError> {

    let now = Instant::now();

    let new_app = App::new().await?;

    let _ = new_app.process().await?;

    info!(
        "total elapsed time {:?}",
        now.elapsed()
    );

    Ok(())
}
