use super::transaction::PostgresTransaction;

use sqlx::PgPool;
use taitan_orm_trait::brave_new::result::Result;
use crate::brave_new::count::CountResult;
use crate::brave_new::{ArgsExtractor, SqlExecutor};
use sqlx::{Database, Postgres};
use taitan_orm_trait::brave_new::Pagination;
use taitan_orm_trait::brave_new::param::Parameter;
use crate::brave_new::sql_generic_executor::SqlGenericExecutor;
use crate::new_executor_impl;
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
}


impl ArgsExtractor for PostgresDatabase {
    fn extract_pagination_arguments(
        page: &Pagination,
    ) -> Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(<Pagination as Parameter<Postgres>>::gen_args(page)?)
    }
}

impl SqlGenericExecutor for PostgresDatabase {
    type DB = Postgres;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

impl SqlExecutor for PostgresDatabase {
    new_executor_impl! {}
}