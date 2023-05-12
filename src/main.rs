mod utils;
mod core;
mod cli;

use tokio::time::Instant;
use tracing::info;
use utils::{error::MaskerError};
use crate::core::app::App;
use crate::utils::enums::AppMode;

#[tokio::main]
async fn main() -> Result<(), MaskerError> {

    let now = Instant::now();

    let mut new_app = App::new(AppMode::CLI).await?;

    let _ = new_app.process().await?;

    info!(
        "total elapsed time {:?}",
        now.elapsed()
    );

    Ok(())
}
