use crate::{FieldName, Location, NotImplementError, Unique};
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use std::fmt::Debug;

pub trait Mutation: Sync + Debug {

    type Location: Location;

    fn get_mutation_fields_name(&self) -> Vec<FieldName>;

    // fn gen_update_arguments_sqlite<'a>(
    //     &'a self,
    //     primary: &'a Self::Primary,
    // ) -> Result<SqliteArguments<'a>, BoxDynError> {
    //     Err(NotImplementError("gen_update_arguments_sqlite".to_string()).into())
    // }
    // fn gen_update_arguments_mysql<'a>(
    //     &'a self,
    //     primary: &'a Self::Primary,
    // ) -> Result<MySqlArguments, BoxDynError> {
    //     Err(NotImplementError("gen_update_arguments_mysql".to_string()).into())
    // }
    // fn gen_update_arguments_postgres<'a>(
    //     &'a self,
    //     primary: &'a Self::Primary,
    // ) -> Result<PgArguments, BoxDynError> {
    //     Err(NotImplementError("gen_update_arguments_postgres".to_string()).into())
    // }

    fn gen_change_arguments_sqlite<'a>(
        &'a self,
        location: &'a Self::Location,
    ) -> Result<SqliteArguments<'a>, BoxDynError> {
        Err(NotImplementError("gen_change_arguments_sqlite".to_string()).into())
    }
    fn gen_change_arguments_mysql<'a>(
        &'a self,
        location: &'a Self::Location,
    ) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_change_arguments_mysql".to_string()).into())
    }
    fn gen_change_arguments_postgres<'a>(
        &'a self,
        location: &'a Self::Location,
    ) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_change_arguments_postgres".to_string()).into())
    }
}
