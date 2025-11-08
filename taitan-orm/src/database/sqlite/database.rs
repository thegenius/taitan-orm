use super::transaction::SqliteTransaction;
use crate::brave_new_executor_impl;
use crate::executors::SqlExecutor;
use crate::executors::SqlGenericExecutor;
use sqlx::Sqlite;
use sqlx::SqlitePool;
use taitan_orm_trait::result::CountResult;
use taitan_orm_trait::result::Result;

#[derive(Debug, Clone)]
pub struct SqliteDatabase {
    pub(crate) sqlite_pool: SqlitePool,
}

impl SqliteDatabase {
    pub async fn transaction<'a>(&'a self) -> Result<SqliteTransaction<'a>> {
        let trx = self.get_pool()?.begin().await?;
        let transaction = SqliteTransaction::new(trx);
        Ok(transaction)
    }

    pub fn get_pool(&self) -> Result<&SqlitePool> {
        Ok(&self.sqlite_pool)
    }

    pub fn from_pool(pool: SqlitePool) -> Self {
        Self { sqlite_pool: pool }
    }
}

impl SqlGenericExecutor for SqliteDatabase {
    type DB = Sqlite;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

impl SqlExecutor<Sqlite> for SqliteDatabase {
    brave_new_executor_impl!(sqlx::Sqlite);
}
