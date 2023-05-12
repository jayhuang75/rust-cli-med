
struct Database {
    pool: sqlx::Pool<sqlx::Sqlite>,
}

pub struct AuditSummary {
    pub line: String,
    db: Database,
}