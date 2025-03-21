use super::result::Result;
use crate::brave_new::pagination::Pagination;
use crate::brave_new::param::Parameter;
use crate::brave_new::{CountSqlParser, PlaceholderParser};
use crate::NotImplementError;
use sqlx::error::BoxDynError;
use sqlx::mysql::MySqlArguments;
use sqlx::postgres::PgArguments;
use sqlx::sqlite::SqliteArguments;
use sqlx::{Database, MySql, Postgres, Sqlite};
use std::borrow::Cow;
use std::fmt::Debug;
// 有3套SQL需要生成
// （1）普通SQL
// （2）count SQL
// （3）paged SQL
// 如果配合indexed，其实有6套

pub trait Template<DB: Database>: Debug {
    fn get_sql(&self) -> Result<(String, DB::Arguments<'_>)>;

    fn get_paged_sql<'a>(&'a self, pagination: &'a Pagination) -> Result<(String, DB::Arguments<'a>)>;
    fn get_count_sql(&self) -> Result<(String, DB::Arguments<'_>)>;
}

pub trait MysqlTemplate: Template<MySql> {}
impl<T: Template<MySql>> MysqlTemplate for T {}

pub trait PostgresTemplate: Template<Postgres> {}
impl<T: Template<Postgres>> PostgresTemplate for T {}

pub trait SqliteTemplate: Template<Sqlite> {}
impl<T: Template<Sqlite>> SqliteTemplate for T {}


