use std::{path::PathBuf, str::FromStr, time::Duration};

use colored::Colorize;
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    Pool, Sqlite,
};
use tracing::info;

use crate::utils::error::MaskerError;

pub struct Database {
    pub pool: sqlx::Pool<sqlx::Sqlite>,
}

#[derive(Debug, Default, Clone)]
pub struct AuditSummary {
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

// const DATABASE_URL: &str = "../audit/data.db";
// const DATABASE_MIGRATE_URL: &str = "../audit/migrations";

impl Database {
    pub async fn new() -> Result<Database, MaskerError> {
        let database_url = Self::create_audit_db().await?;

        if !Sqlite::database_exists(database_url.to_str().unwrap())
            .await
            .unwrap_or(false)
        {
            Sqlite::create_database(database_url.to_str().unwrap()).await?;
            info!(
                "audit database {} created",
                database_url.display().to_string().bold().green()
            );
        } else {
            info!(
                "audit database {} exist",
                database_url.display().to_string().bold().green()
            );
        }
        let pool_timeout = Duration::from_secs(30);

        let connection_options =
            SqliteConnectOptions::from_str(&database_url.display().to_string().bold().green())?
                .create_if_missing(true)
                .journal_mode(SqliteJournalMode::Wal)
                .synchronous(SqliteSynchronous::Normal)
                .busy_timeout(pool_timeout);

        let pool = SqlitePoolOptions::new()
            .max_connections(2)
            .connect_with(connection_options)
            .await?;

        // Self::migrate(&pool).await?;
        Self::create_table(&pool).await?;
        Ok(Database { pool })
    }

    async fn create_audit_db() -> Result<PathBuf, MaskerError> {
        let path = dirs::config_dir().unwrap().join("com.dash_oui.med");
        Ok(path)
    }

    async fn create_table(pool: &Pool<Sqlite>) -> Result<(), MaskerError> {
        let result = sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS audit (
                id INTEGER PRIMARY KEY,
                user TEXT NOT NULL,
                hostname TEXT NOT NULL,
                total_files INTEGER NOT NULL,
                total_records INTEGER NOT NULL,
                failed_records INTEGER NOT NULL,
                record_failed_reason TEXT,
                runtime_conf TEXT NOT NULL,
                process_failure_reason TEXT,
                successed BOOLEAN NOT NULL DEFAULT FALSE,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
            );
            ",
        )
        .execute(pool)
        .await?;
        info!("audit database {:?} create successed", result);
        Ok(())
    }

    // #[allow(dead_code)]
    // async fn migrate(pool: &Pool<Sqlite>) -> Result<(), MaskerError> {
    //     // let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    //     // info!("crate dir {}", crate_dir);

    //     let migrations = Path::new(DATABASE_MIGRATE_URL);
    //     info!(
    //         "db migration path {}",
    //         migrations.display().to_string().bold().green()
    //     );

    //     sqlx::migrate::Migrator::new(migrations)
    //         .await
    //         .unwrap()
    //         .run(pool)
    //         .await?;

    //     info!(
    //         "audit database {} {}",
    //         DATABASE_URL,
    //         "migrated".bold().green()
    //     );

    //     Ok(())
    // }

    pub async fn insert(&mut self, summary: &AuditSummary) -> Result<i64, MaskerError> {
        let total_files = summary.total_files as i64;
        let total_records = summary.total_records as i64;
        let failed_records: i64 = summary.failed_records as i64;
        let record_failed_reason = serde_json::to_string(&summary.record_failed_reason)?;

        let id = sqlx::query!(
            r#"
                INSERT INTO audit ( user, hostname, total_files, total_records, failed_records, record_failed_reason, runtime_conf, process_failure_reason, successed )
                VALUES ( ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        "#,
        summary.user, summary.hostname, total_files, total_records, failed_records, record_failed_reason, summary.runtime_conf, summary.process_failure_reason, summary.successed
        )
        .execute(&self.pool)
        .await?
        .last_insert_rowid();
        Ok(id)
    }
}
