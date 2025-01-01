use crate::NotImplementError;
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use std::fmt::Debug;

pub trait UpdateCommand: Sync + Debug {
    fn gen_update_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        Err(
            NotImplementError("PrimaryMutationPair::gen_update_arguments_sqlite".to_string())
                .into(),
        )
    }
    fn gen_update_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("PrimaryMutationPair::gen_update_arguments_mysql".to_string()).into())
    }
    fn gen_update_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        Err(
            NotImplementError("PrimaryMutationPair::gen_update_arguments_postgres".to_string())
                .into(),
        )
    }
}
