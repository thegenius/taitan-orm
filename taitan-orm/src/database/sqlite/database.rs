use super::transaction::SqliteTransaction;

use crate::args_extractor::ArgsExtractor;
use crate::count::CountResult;
use crate::new_executor_impl;
use crate::sql_executor::SqlExecutor;
use crate::sql_generic_executor::SqlGenericExecutor;
use sqlx::SqlitePool;
use sqlx::{Database, Sqlite};
use taitan_orm_trait::page::Pagination;
use taitan_orm_trait::result::Result;
use taitan_orm_trait::traits::Parameter;
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
}

impl ArgsExtractor for SqliteDatabase {
    fn extract_pagination_arguments(
        page: &Pagination,
    ) -> Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(<Pagination as Parameter<Sqlite>>::gen_args(page)?)
    }
}

impl SqlGenericExecutor for SqliteDatabase {
    type DB = Sqlite;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

impl SqlExecutor for SqliteDatabase {
    new_executor_impl! {}
}
