// use crate::args_extractor::ArgsExtractor;
use crate::count::CountResult;
use crate::{brave_new_transaction_impl, new_transaction_impl};
// use crate::sql_executor_mut::SqlExecutorMut;
use crate::sql_generic_executor::SqlGenericExecutor;
use sqlx::{Database, Postgres};
use taitan_orm_trait::page::Pagination;
use taitan_orm_trait::result::Result;
use taitan_orm_trait::traits::Parameter;
use crate::database::sqlite::SqliteTransaction;
use crate::new_executor::SqlExecutorMutNew;

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

// impl<'t> ArgsExtractor for PostgresTransaction<'t> {
//     fn extract_pagination_arguments(
//         page: &Pagination,
//     ) -> Result<<Self::DB as Database>::Arguments<'_>> {
//         Ok(<Pagination as Parameter<Postgres>>::gen_args(page)?)
//     }
// }

impl<'t> SqlGenericExecutor for PostgresTransaction<'t> {
    type DB = Postgres;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

// impl<'t> SqlExecutorMut for PostgresTransaction<'t> {
//     new_transaction_impl! {}
// }

impl<'t> SqlExecutorMutNew<sqlx::Postgres> for PostgresTransaction<'t> {
    brave_new_transaction_impl!(sqlx::Postgres);
}