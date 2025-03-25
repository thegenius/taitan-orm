use super::transaction::MySqlTransaction;

use sqlx::{MySql, MySqlPool, Postgres};
use taitan_orm_trait::result::Result;
use crate::count::CountResult;
use crate::sql_executor::{SqlExecutor};
use crate::args_extractor::ArgsExtractor;
use sqlx::{Database};
use taitan_orm_trait::page::Pagination;
use taitan_orm_trait::traits::Parameter;
use crate::database::postgres::PostgresDatabase;
use crate::new_executor::SqlExecutorNew;
use crate::sql_generic_executor::SqlGenericExecutor;
use crate::{brave_new_executor_impl, new_executor_impl};
#[derive(Debug, Clone)]
pub struct MySqlDatabase {
    pub(crate) pool: MySqlPool,
}

impl MySqlDatabase {
    pub async fn transaction<'a>(&'a self) -> Result<MySqlTransaction<'a>> {
        let trx = self.get_pool()?.begin().await?;
        let transaction = MySqlTransaction::new(trx);
        Ok(transaction)
    }

    pub fn get_pool(&self) -> Result<&MySqlPool> {
        Ok(&self.pool)
    }
}


impl ArgsExtractor for MySqlDatabase {
    fn extract_pagination_arguments(
        page: &Pagination,
    ) -> Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(<Pagination as Parameter<MySql>>::gen_args(page)?)
    }
}

impl SqlGenericExecutor for MySqlDatabase {
    type DB = MySql;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

impl SqlExecutor for MySqlDatabase {
    new_executor_impl! {}
}

impl SqlExecutorNew<MySql> for MySqlDatabase {
    brave_new_executor_impl!(sqlx::MySql);
}