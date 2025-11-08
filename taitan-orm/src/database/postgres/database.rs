use super::transaction::PostgresTransaction;
use crate::brave_new_executor_impl;
use crate::executors::SqlExecutor;
use crate::executors::SqlGenericExecutor;
use sqlx::PgPool;
use sqlx::Postgres;
use taitan_orm_trait::result::CountResult;
use taitan_orm_trait::result::Result;

#[derive(Debug, Clone)]
pub struct PostgresDatabase {
    pub(crate) pool: PgPool,
}

impl PostgresDatabase {
    pub async fn transaction<'a>(&'a self) -> Result<PostgresTransaction<'a>> {
        let trx = self.get_pool()?.begin().await?;
        let transaction = PostgresTransaction::new(trx);
        Ok(transaction)
    }

    pub fn get_pool(&self) -> Result<&PgPool> {
        Ok(&self.pool)
    }

    pub fn from_pool(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlGenericExecutor for PostgresDatabase {
    type DB = Postgres;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

impl SqlExecutor<Postgres> for PostgresDatabase {
    brave_new_executor_impl!(sqlx::Postgres);
}
