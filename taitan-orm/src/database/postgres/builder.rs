use std::borrow::Cow;
use std::fs;
use std::path::Path;
use path_absolutize::Absolutize;
use sqlx::postgres::PgConnectOptions;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous};
use sqlx::{PgPool, SqlitePool};
use crate::error::TaitanOrmError;
use super::PostgresDatabase;
pub struct PostgresBuilder {}

impl PostgresBuilder {
    pub async fn build(config: PgConnectOptions)-> crate::result::Result<PostgresDatabase> {
        let pool = PgPool::connect_with(config).await?;

        let database = PostgresDatabase {
            pool,
        };
        Ok(database)
    }
}
