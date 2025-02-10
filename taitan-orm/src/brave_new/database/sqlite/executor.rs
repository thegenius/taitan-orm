use crate::brave_new::count::CountResult;
use crate::brave_new::database::sqlite::SqliteDatabase;
use crate::brave_new::SqlExecutor;
use sqlx::{Database, Sqlite};

use crate::brave_new::sql_generic_executor::SqlGenericExecutor;
use crate::new_executor_impl;

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
