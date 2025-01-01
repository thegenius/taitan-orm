use crate::{FieldName, NotImplementError};
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use std::fmt::Debug;

pub trait Location: Sync + Debug {
    fn get_table_name(&self) -> &'static str;

    fn get_location_fields_name(&self) -> Vec<FieldName>;

    fn get_where_clause(&self, wrap_char: char, place_holder: char) -> String;

    // fn check_valid_order_by(&self, fields: &[&str]) -> bool;

    fn gen_location_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_sqlite".to_string()).into())
    }
    fn gen_location_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_mysql".to_string()).into())
    }
    fn gen_location_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_postgres".to_string()).into())
    }
}
