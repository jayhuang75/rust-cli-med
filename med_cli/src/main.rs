use colored::Colorize;
use med_core::audit;
use tokio::time::Instant;
use tracing::info;
mod cli;

use med_core::app::core::App;
use med_core::audit::db::AuditSummary;
use med_core::utils::error::MaskerError;

use cli::app::Cli;

#[tokio::main]
async fn main() -> Result<(), MaskerError> {
    let now = Instant::now();
    let new_cli = Cli::new().await?;
    let params = new_cli.params;

    let mut new_app = App::new(params).await?;

    let mut audit_summary = AuditSummary {
        runtime_conf: serde_json::to_string(&new_app.params)?,
        ..Default::default()
    };

    let mut audit_db = audit::db::Database::new().await?;

    match new_app.process().await {
        Ok(metrics) => {
            audit_summary.total_files = metrics.total_files;
            audit_summary.total_records = metrics.total_records;
            audit_summary.failed_records = metrics.failed_records;
            audit_summary.record_failed_reason = metrics.record_failed_reason;
            audit_summary.successed = true;
        }
        Err(err) => {
            audit_summary.process_failure_reason = Some(serde_json::to_string(&err)?);
            info!("{} {:?}", "error".bold().red(), err.to_string());
        }
    }

    if new_app.params.key.is_some() {
        new_app.params.key = Some("****".to_owned());
    }
    audit_summary.user = new_app.user;
    audit_summary.hostname = new_app.hostname;

    let audit_id = audit_db.insert(&audit_summary).await?;

    info!(
        "total processed {} files, {} records, with {} records failed, elapsed time {:?}, audit record_id {}",
        audit_summary.total_files.to_string().bold().green(),
        audit_summary.total_records.to_string().bold().green(),
        audit_summary.failed_records.to_string().bold().green(),
        now.elapsed(),
        audit_id.to_string().bold().green()
    );

    Ok(())
}
