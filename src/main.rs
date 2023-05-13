mod utils;
mod core;
mod cli;
mod audit;

use tokio::time::Instant;
use tracing::info;
use utils::{error::MaskerError};
use crate::audit::db::AuditSummary;
use crate::core::app::App;
use crate::utils::enums::AppMode;

const DATABASE_URL: &str = "./audit/data.db";

#[tokio::main]
async fn main() -> Result<(), MaskerError> {

    let now = Instant::now();

    let mut new_app = App::new(AppMode::CLI).await?;

    let mut audit_summary = AuditSummary::default();
    audit_summary.runtime_conf = serde_json::to_string(&new_app.params)?;

    // load audit db
    let mut audit_db = audit::db::Database::new(DATABASE_URL).await?;
    audit_db.migrate("./audit/migrations").await?;

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

    let audit_id = audit_db.insert(&audit_summary).await?;

    info!(
        "total elapsed time {:?} with audit record_id {:?}",
        now.elapsed(),
        audit_id
    );

    Ok(())
}
