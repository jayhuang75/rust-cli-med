mod utils;
mod core;
mod cli;
mod audit;

use tokio::time::Instant;
use tracing::info;
use dotenv::dotenv;
use utils::{error::MaskerError};
use crate::audit::db::AuditSummary;
use crate::core::app::App;
use crate::utils::enums::AppMode;

#[tokio::main]
async fn main() -> Result<(), MaskerError> {

    let now = Instant::now();

    // load env
    dotenv().ok();

    // load audit db
    let mut audit_db = audit::db::Database::new("./audit/data.db").await?;

    let mut new_app = App::new(AppMode::CLI).await?;

    let mut audit_summary = AuditSummary::default();
    audit_summary.runtime_conf = serde_json::from_str(&new_app.params.to_string())?;

    match new_app.process().await {
        Ok(metrics) => {
            audit_summary.total_files = metrics.total_files;
            audit_summary.total_records = metrics.total_records;
            audit_summary.successed = true;
        },
        Err(err) => {
            audit_summary.failure_reason = serde_json::from_str(Err(err)?)?;
        }
    }

    let id = audit_db.insert(&audit_summary).await?;

    info!(
        "total elapsed time {:?}",
        now.elapsed()
    );

    Ok(())
}
