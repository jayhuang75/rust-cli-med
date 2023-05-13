use std::env;

use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tracing::{info, log::warn};

use crate::utils::error::MaskerError;

pub struct Database {
    pub pool: sqlx::Pool<sqlx::Sqlite>,
}

#[derive(Debug, Default, Clone)]
pub struct AuditSummary {
    pub total_files: usize,
    pub total_records: usize,
    pub runtime_conf: String,
    pub failure_reason: Option<String>,
    pub successed: bool,
}

impl Database {
    pub async fn new(db_url: &str) -> Result<Database, MaskerError> {
        if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
            info!("[Database] create database...");
            Sqlite::create_database(db_url).await?;
        } else {
            warn!("[Database] database already exists");
        }
        info!("[Database] connect to database...");
        let pool = SqlitePool::connect(&env::var("DATABASE_URL").unwrap()).await?;
        Ok(Database { pool })
    }

    pub async fn migrate(&mut self, migrations_path: &str) -> Result<(), MaskerError> {
        // "./data/migrations"
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let migrations = std::path::Path::new(&crate_dir).join(migrations_path);

        info!("[Database] migration...");
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

        let id = sqlx::query!(
            r#"
                INSERT INTO audit ( total_files, total_records, runtime_conf, failure_reason, successed )
                VALUES ( ?1, ?2, ?3, ?4, ?5 )
        "#,
        total_files, total_records, summary.runtime_conf, summary.failure_reason, summary.successed
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }
}
