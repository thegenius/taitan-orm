use std::borrow::Cow;
use std::fs;
use std::path::Path;
use path_absolutize::Absolutize;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::SqlitePool;
use crate::error::TaitanOrmError;
use super::SqliteDatabase;
pub struct SqliteBuilder {}

impl SqliteBuilder {
    async fn init_local(workspace_dir: &str, db_file: &str) -> crate::result::Result<SqlitePool> {
        let workspace = Path::new(workspace_dir);
        let workspace_absolute = workspace
            .absolutize()
            .map_err(|_e| TaitanOrmError::DatabaseInitFail("workdir absolute fail".to_string()))?;

        fs::create_dir_all(&workspace_absolute)
            .map_err(|_e| TaitanOrmError::DatabaseInitFail("create dir fail".to_string()))?;
        let db_file_path = workspace_absolute.join(db_file);

        let options = SqliteConnectOptions::new()
            .filename(db_file_path.clone())
            .synchronous(SqliteSynchronous::Full)
            .journal_mode(SqliteJournalMode::Wal)
            .create_if_missing(true);
        let sqlite_pool = SqlitePool::connect_with(options)
            .await
            .map_err(|_e| TaitanOrmError::DatabaseInitFail("create is missing fail".to_string()))?;
        Ok(sqlite_pool)
    }

    pub async fn build(config: SqliteLocalConfig<'_>) -> crate::result::Result<SqliteDatabase> {
        let pool = SqliteBuilder::init_local(&config.work_dir, &config.db_file).await?;
        let database = SqliteDatabase {
            sqlite_pool: pool,
        };
        Ok(database)
    }
}

pub struct SqliteLocalConfig<'a> {
    pub work_dir: Cow<'a, str>,
    pub db_file: Cow<'a, str>,
}

impl<'a> SqliteLocalConfig<'a> {
    pub fn new<S>(work_dir: S, db_file: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            work_dir: work_dir.into(),
            db_file: db_file.into(),
        }
    }
}
