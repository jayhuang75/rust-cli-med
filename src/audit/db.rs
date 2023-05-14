use std::{str::FromStr, time::Duration};

use colored::Colorize;
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    Sqlite
};
use tracing::{info};

use crate::utils::error::MaskerError;

pub struct Database {
    pub pool: sqlx::Pool<sqlx::Sqlite>,
}

#[derive(Debug, Default, Clone)]
pub struct AuditSummary {
    pub total_files: usize,
    pub total_records: usize,
    pub failed_records: usize,
    pub runtime_conf: String,
    pub failure_reason: Option<String>,
    pub successed: bool,
}

impl Database {
    pub async fn new(db_url: &str) -> Result<Database, MaskerError> {
        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            Sqlite::create_database(db_url).await?;
            info!("audit database {} created", db_url.bold().green());
        } else {
            info!("audit database {} exist", db_url.bold().green());
        }
        let pool_timeout = Duration::from_secs(30);

        let connection_options =
            SqliteConnectOptions::from_str(db_url)?
                .create_if_missing(true)
                .journal_mode(SqliteJournalMode::Wal)
                .synchronous(SqliteSynchronous::Normal)
                .busy_timeout(pool_timeout);

        let pool = SqlitePoolOptions::new()
            .max_connections(2)
            .connect_with(connection_options)
            .await?;

        Ok(Database { pool })
    }

    #[allow(dead_code)]
    pub async fn migrate(&mut self, migrations_path: &str) -> Result<(), MaskerError> {
        // "./data/migrations"
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let migrations = std::path::Path::new(&crate_dir).join(migrations_path);

        sqlx::migrate::Migrator::new(migrations)
            .await
            .unwrap()
            .run(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn insert(&mut self, summary: &AuditSummary) -> Result<i64, MaskerError> {
        // Insert the audit item, then obtain the ID of this row
        // total_files INTEGER NOT NULL,
        // total_records INTEGER NOT NULL,
        // runtime_conf TEXT NOT NULL,
        // failure_reason TEXT,
        // successed BOOLEAN NOT NULL DEFAULT FALSE,
        let total_files = summary.total_files as i64;
        let total_records = summary.total_records as i64;
        let failed_records: i64 = summary.failed_records as i64;
        
        let id = sqlx::query!(
            r#"
                INSERT INTO audit ( total_files, total_records, failed_records, runtime_conf, failure_reason, successed )
                VALUES ( ?1, ?2, ?3, ?4, ?5, ?6 )
        "#,
        total_files, total_records, failed_records, summary.runtime_conf, summary.failure_reason, summary.successed
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }
}
