use crate::{audit::db::Database, models::metrics::Metrics, utils::error::MedError};

#[derive(Debug, Default, Clone)]
pub struct Summary {
    pub user: String,
    pub hostname: String,
    pub metrics: Metrics,
    pub runtime_conf: String,
    pub process_failure_reason: Option<String>,
    pub successed: bool,
}

pub struct Audit {
    pub db: Database,
    pub summary: Summary,
}

impl Audit {
    pub async fn new() -> Result<Self, MedError> {
        let db = Database::new().await?;
        let summary = Summary::default();
        Ok(Audit { db, summary })
    }
    pub async fn insert(&mut self) -> Result<i64, MedError> {
        let id = self.db.insert(&self.summary).await?;
        Ok(id)
    }
}
