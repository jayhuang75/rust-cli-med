use crate::{audit::db::Database, utils::error::MaskerError};

#[derive(Debug, Default, Clone)]
pub struct Summary {
    pub user: String,
    pub hostname: String,
    pub total_files: usize,
    pub total_records: usize,
    pub failed_records: usize,
    pub record_failed_reason: Vec<MaskerError>,
    pub runtime_conf: String,
    pub process_failure_reason: Option<String>,
    pub successed: bool,
}

pub struct Audit {
    pub db: Database,
    pub summary: Summary,
}

impl Audit {
    pub async fn new() -> Result<Self, MaskerError> {
        let db = Database::new().await?;
        let summary = Summary::default();
        Ok(Audit { db, summary })
    }
    pub async fn insert(&mut self) -> Result<i64, MaskerError> {
        let id = self.db.insert(&self.summary).await?;
        Ok(id)
    }
}
