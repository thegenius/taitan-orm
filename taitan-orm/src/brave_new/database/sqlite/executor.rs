use sqlx::Sqlite;
use crate::brave_new::database::sqlite::SqliteDatabase;
use crate::brave_new::SqlExecutor;
use crate::brave_new::count::CountResult;
use crate::executor_impl;
use crate::brave_new::sql_generic_executor::SqlGenericExecutor;

impl SqlGenericExecutor for SqliteDatabase {
    type DB = Sqlite;
    type CountType = CountResult;

    fn get_affected_rows(query_result: &<Self::DB as sqlx::Database>::QueryResult) -> u64 {
        query_result.rows_affected()
    }
}
// impl SqlExecutor for SqliteDatabase {
//     executor_impl!(SqliteConnection);
// }