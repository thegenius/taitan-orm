use crate::Location;
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;

#[derive(Clone, Debug)]
pub struct AllLocation {
    pub table_name: &'static str,
}
impl AllLocation {
    pub const fn new(table_name: &'static str) -> Self {
        Self { table_name }
    }
}
impl Location for AllLocation {

    fn get_table_name(&self) -> &'static str {
        self.table_name
    }

    fn get_where_clause(&self) -> String {
        String::default()
    }

    fn gen_location_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        Ok(SqliteArguments::default())
    }

    fn gen_location_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        Ok(MySqlArguments::default())
    }

    fn gen_location_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        Ok(PgArguments::default())
    }
}
