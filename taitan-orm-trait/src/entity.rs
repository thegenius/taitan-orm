use crate::error::NotImplementError;
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use std::fmt::Debug;
use crate::FieldName;

pub trait Entity: Sync + Debug {
    fn get_table_name(&self) -> &str;

    fn get_insert_fields(&self) -> Vec<FieldName>;

    fn get_upsert_set_fields(&self) -> Vec<FieldName>;

    fn get_auto_increment_field(&self) -> Option<&str>;

    fn set_auto_increment_field(&mut self, value: Option<i64>) -> bool;

    fn gen_insert_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        Err(NotImplementError("gen_insert_arguments_sqlite".to_string()).into())
    }
    fn gen_upsert_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        Err(NotImplementError("gen_upsert_arguments_sqlite".to_string()).into())
    }
    fn gen_insert_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_insert_arguments_mysql".to_string()).into())
    }
    fn gen_upsert_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_upsert_arguments_mysql".to_string()).into())
    }

    fn gen_insert_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_insert_arguments_postgres".to_string()).into())
    }
    fn gen_upsert_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_upsert_arguments_postgres".to_string()).into())
    }
}
