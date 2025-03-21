use crate::{Mutation, NotImplementError};
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use std::fmt::Debug;

pub trait Unique: Sync + Debug {
    type Mutation: Mutation;
    fn get_table_name(&self) -> &'static str;

    fn get_unique_field_names(&self) -> &'static [&'static str];


    fn gen_update_arguments_sqlite<'a>(
        &'a self,
        mutation: &'a Self::Mutation,
    ) -> Result<SqliteArguments<'a>, BoxDynError> {
        Err(NotImplementError("gen_update_arguments_sqlite".to_string()).into())
    }
    fn gen_update_arguments_mysql<'a>(
        &'a self,
        mutation: &'a Self::Mutation,
    ) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_update_arguments_mysql".to_string()).into())
    }
    fn gen_update_arguments_postgres<'a>(
        &'a self,
        mutation: &'a Self::Mutation,
    ) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_update_arguments_postgres".to_string()).into())
    }

    fn gen_unique_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_sqlite".to_string()).into())
    }
    fn gen_unique_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_mysql".to_string()).into())
    }
    fn gen_unique_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_primary_arguments_postgres".to_string()).into())
    }
}
