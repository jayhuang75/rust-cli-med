mod utils;
mod core;
mod cli;
mod audit;

use tokio::time::Instant;
use tracing::info;
use utils::{error::MaskerError};
use crate::core::app::App;
use crate::utils::enums::AppMode;

#[tokio::main]
async fn main() -> Result<(), MaskerError> {

    let now = Instant::now();

    let mut new_app = App::new(AppMode::CLI).await?;

    let summary = new_app.process().await?;

    info!(
        "completed summary {:?}",
        summary
    );

    info!(
        "total elapsed time {:?}",
        now.elapsed()
    );

    Ok(())
}
