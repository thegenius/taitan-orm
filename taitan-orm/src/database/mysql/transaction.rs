use crate::args_extractor::ArgsExtractor;
use crate::count::CountResult;
use crate::{brave_new_transaction_impl, new_transaction_impl};
use crate::sql_executor_mut::SqlExecutorMut;
use crate::sql_generic_executor::SqlGenericExecutor;
use sqlx::{Database, MySql};
use taitan_orm_trait::page::Pagination;
use taitan_orm_trait::result::Result;
use taitan_orm_trait::traits::Parameter;

#[derive(Debug)]
pub struct MySqlTransaction<'a> {
    transaction: sqlx::Transaction<'a, MySql>,
}

impl<'a> MySqlTransaction<'a> {
    pub fn new(trx: sqlx::Transaction<'a, MySql>) -> Self {
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

impl<'t> ArgsExtractor for MySqlTransaction<'t> {
    fn extract_pagination_arguments(
        page: &Pagination,
    ) -> Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(<Pagination as Parameter<MySql>>::gen_args(page)?)
    }
}

impl<'t> SqlGenericExecutor for MySqlTransaction<'t> {
    type DB = MySql;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

impl<'t> SqlExecutorMut for MySqlTransaction<'t> {
    new_transaction_impl! {}
}
