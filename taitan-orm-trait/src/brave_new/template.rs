use super::result::Result;
use crate::pagination::Pagination;
use crate::NotImplementError;
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use sqlx::{Database, MySql, Postgres, Sqlite};
use std::borrow::Cow;
use std::fmt::Debug;

pub trait Template<DB: Database>: Debug {
    fn get_sql<'a>(&self, page: Option<&Pagination>) -> Cow<'a, str>;
    fn gen_arguments<'a>(&self) -> Result<DB::Arguments<'a>>;

    fn get_count_sql<'a>(&self) -> Cow<'a, str> {
        Cow::from("")
    }
    fn gen_count_arguments<'a>(&self) -> Result<DB::Arguments<'a>> {
        Ok(DB::Arguments::default())
    }

    fn get_pagination(&self) -> Option<&Pagination> {
        None
    }
}

pub trait MysqlTemplate: Template<MySql> {}
impl<T: Template<MySql>> MysqlTemplate for T {}

pub trait PostgresTemplate: Template<Postgres> {}
impl<T: Template<Postgres>> PostgresTemplate for T {}

pub trait SqliteTemplate: Template<Sqlite> {}
impl<T: Template<Sqlite>> SqliteTemplate for T {}
