use super::transaction::SqliteTransaction;
use crate::brave_new_executor_impl;
use crate::count::CountResult;
use crate::new_executor::SqlExecutorNew;
use crate::sql_generic_executor::SqlGenericExecutor;
use sqlx::SqlitePool;
use sqlx::Sqlite;
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
}

// impl ArgsExtractor for SqliteDatabase {
//     fn extract_pagination_arguments(
//         page: &Pagination,
//     ) -> Result<<Self::DB as Database>::Arguments<'_>> {
//         Ok(<Pagination as Parameter<Sqlite>>::gen_args(page)?)
//     }
// }

impl SqlGenericExecutor for SqliteDatabase {
    type DB = Sqlite;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}

// impl SqlExecutor for SqliteDatabase {
//     new_executor_impl! {}
// }

impl SqlExecutorNew<Sqlite> for SqliteDatabase {
    brave_new_executor_impl!(sqlx::Sqlite);
}
