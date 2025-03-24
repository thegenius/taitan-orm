use crate::args_extractor::ArgsExtractor;
use crate::count::CountResult;
use crate::{brave_new_transaction_impl, new_transaction_impl};
use crate::sql_executor_mut::SqlExecutorMut;
use crate::sql_generic_executor::SqlGenericExecutor;
use sqlx::{Database, Sqlite};
use taitan_orm_trait::page::Pagination;
use taitan_orm_trait::traits::Parameter;
use crate::new_executor::SqlExecutorMutNew;

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

impl<'t> ArgsExtractor for SqliteTransaction<'t> {
    fn extract_pagination_arguments(
        page: &Pagination,
    ) -> taitan_orm_trait::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(<Pagination as Parameter<Sqlite>>::gen_args(page)?)
    }
}

impl<'t> SqlGenericExecutor for SqliteTransaction<'t> {
    type DB = Sqlite;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

impl<'t> SqlExecutorMut for SqliteTransaction<'t> {
    new_transaction_impl! {}
}

impl<'t> SqlExecutorMutNew<sqlx::Sqlite> for SqliteTransaction<'t> {
    brave_new_transaction_impl!(sqlx::Sqlite);
}