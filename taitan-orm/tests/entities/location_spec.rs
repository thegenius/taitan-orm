use serde::Deserialize;
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use time::PrimitiveDateTime;

use taitan_orm_trait::{Location, Optional};

#[derive(Default, Debug)]
pub struct LocationTest {
    age: i32,
    birthday: Optional<PrimitiveDateTime>
}

impl Location for LocationTest {
    fn get_table_name(&self) -> &'static str {
        todo!()
    }

    fn get_where_clause(&self) -> String {
        return "`age` > ? AND `birthday` > ?".to_string();
    }

    fn gen_location_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        todo!()
    }

    fn gen_location_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        todo!()
    }

    fn gen_location_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        todo!()
    }
}