use crate::CountSql;
use crate::pagination::Pagination;
use crate::NotImplementError;
use sqlx::any::AnyArguments;
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use std::fmt::Debug;

pub trait TemplateRecord: Sync + Debug {

    fn get_sql(&self, page: Option<&Pagination>) -> String;

    fn get_count_sql(&self) -> Option<String>;

    fn get_pagination(&self) -> Option<&Pagination> {
        None
    }

    fn get_variables(&self) -> Vec<String>;

    fn gen_template_count_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        Err(NotImplementError("gen_template_count_arguments_sqlite".to_string()).into())
    }
    fn gen_template_count_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_template_count_arguments_mysql".to_string()).into())
    }
    fn gen_template_count_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_template_count_arguments_postgres".to_string()).into())
    }

    fn gen_template_arguments_sqlite(&self) -> Result<SqliteArguments<'_>, BoxDynError> {
        Err(NotImplementError("gen_template_arguments_sqlite".to_string()).into())
    }
    fn gen_template_arguments_mysql(&self) -> Result<MySqlArguments, BoxDynError> {
        Err(NotImplementError("gen_template_arguments_mysql".to_string()).into())
    }
    fn gen_template_arguments_postgres(&self) -> Result<PgArguments, BoxDynError> {
        Err(NotImplementError("gen_template_arguments_postgres".to_string()).into())
    }
}
