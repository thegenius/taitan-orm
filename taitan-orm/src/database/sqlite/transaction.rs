// use crate::args_extractor::ArgsExtractor;
use taitan_orm_trait::result::CountResult;
use crate::brave_new_transaction_impl;
// use crate::sql_executor_mut::SqlExecutorMut;
use crate::executors::SqlGenericExecutor;
use sqlx::Sqlite;
use crate::executors::SqlExecutorMut;

#[derive(Debug)]
pub struct SqliteTransaction<'a> {
    transaction: sqlx::Transaction<'a, Sqlite>,
}

impl<'a> SqliteTransaction<'a> {
    pub fn new(trx: sqlx::Transaction<'a, Sqlite>) -> Self {
        Self { transaction: trx }
    }
    #[inline]
    pub async fn commit(self) -> crate::result::Result<()> {
        Ok(self.transaction.commit().await?)
    }

    #[inline]
    pub async fn rollback(self) -> crate::result::Result<()> {
        Ok(self.transaction.rollback().await?)
    }
}

impl<'t> SqlGenericExecutor for SqliteTransaction<'t> {
    type DB = Sqlite;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

impl<'t> SqlExecutorMut<sqlx::Sqlite> for SqliteTransaction<'t> {
    brave_new_transaction_impl!(sqlx::Sqlite);
}