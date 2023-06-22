use colored::Colorize;
use tokio::time::Instant;
use tracing::info;
mod cli;

use med_core::app::core::App;
use med_core::utils::error::MedError;

use cli::app::Cli;

#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> Result<(), MedError> {
    let now = Instant::now();
    let new_cli = Cli::new().await?;
    let params = new_cli.params;

    let mut new_app = App::new(params).await?;
    let metrics = new_app.process().await?;
    let audit_id = new_app.update_audit(format!("{:?}", now.elapsed())).await?;

    info!(
        "total processed {} files, {} records, with {} records failed, elapsed time {:?}, audit record_id {}",
        metrics.total_files.to_string().bold().green(),
        metrics.metadata.total_records.to_string().bold().green(),
        metrics.metadata.failed_records.to_string().bold().green(),
        now.elapsed(),
        audit_id
    );

    Ok(())
}
