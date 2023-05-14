mod utils;
mod core;
mod cli;
mod audit;

use colored::Colorize;
use tokio::time::Instant;
use tracing::info;
use utils::{error::MaskerError};
use crate::audit::db::AuditSummary;
use crate::core::app::App;
use crate::utils::enums::AppMode;

const DATABASE_URL: &str = "./audit/data.db";
// const DATABASE_MIGRATIONS: &str = "./audit/migrations";

#[tokio::main]
async fn main() -> Result<(), MaskerError> {

    let now = Instant::now();

    let mut new_app = App::new(AppMode::CLI).await?;

    let mut audit_summary = AuditSummary::default();
    audit_summary.runtime_conf = serde_json::to_string(&new_app.params)?;

    // load audit db
    let mut audit_db = audit::db::Database::new(DATABASE_URL).await?;
    // audit_db.migrate(DATABASE_MIGRATIONS).await?;

    match new_app.process().await {
        Ok(metrics) => {
            audit_summary.total_files = metrics.total_files;
            audit_summary.total_records = metrics.total_records;
            audit_summary.successed = true;
        },
        Err(err) => {
            audit_summary.failure_reason = Some(serde_json::to_string(&err.clone())?);
            info!("{} {:?}", "error".bold().red(), err.to_string());
        }
    }

    let audit_id = audit_db.insert(&audit_summary).await?;

    info!(
        "total processed {:?} files and {:?}, elapsed time {:?} with audit record_id {:?}",
        audit_summary.total_files,
        audit_summary.total_records,
        now.elapsed(),
        audit_id
    );

    Ok(())
}
