use std::borrow::Cow;
use std::ops::{Deref, DerefMut};
use taitan_orm::database::sqlite::{SqliteDatabase, SqliteLocalConfig};
use taitan_orm::Executor;
use taitan_orm::{ReaderApi, SqlExecutor, SqlGenericExecutor, TemplateApi, WriterApi};
pub struct AppState {
    executor: SqliteDatabase,
}

impl AppState {
    pub fn new(db: SqliteDatabase) -> Self {
        AppState { executor: db }
    }
    pub async fn build_sqlite<'a, T: Into<Cow<'a, str>>>(dir: T, file: T) -> taitan_orm::Result<Self> {
        let config = SqliteLocalConfig {
            work_dir: dir.into(),
            db_file: file.into(),
        };
        let db = SqliteDatabase::build(config).await?;
        Ok(AppState { executor: db })
    }
}

impl Deref for AppState
{
    type Target = SqliteDatabase;
    fn deref(&self) -> &Self::Target {
        &self.executor
    }
}

impl DerefMut for AppState
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.executor
    }
}
