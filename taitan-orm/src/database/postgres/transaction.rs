
use taitan_orm_trait::result::CountResult;
use crate::brave_new_transaction_impl;
use crate::executors::SqlGenericExecutor;
use sqlx::Postgres;
use taitan_orm_trait::result::Result;
use crate::executors::SqlExecutorMut;

#[derive(Debug)]
pub struct PostgresTransaction<'a> {
    transaction: sqlx::Transaction<'a, Postgres>,
}

impl<'a> PostgresTransaction<'a> {
    pub fn new(trx: sqlx::Transaction<'a, Postgres>) -> Self {
        Self { transaction: trx }
    }
    #[inline]
    pub async fn commit(self) -> Result<()> {
        Ok(self.transaction.commit().await?)
    }

    #[inline]
    pub async fn rollback(self) -> Result<()> {
        Ok(self.transaction.rollback().await?)
    }
}

impl<'t> SqlGenericExecutor for PostgresTransaction<'t> {
    type DB = Postgres;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

impl<'t> SqlExecutorMut<sqlx::Postgres> for PostgresTransaction<'t> {
    brave_new_transaction_impl!(sqlx::Postgres);
}