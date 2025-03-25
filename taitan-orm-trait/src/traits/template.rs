use crate::result::Result;
use crate::page::Pagination;
use crate::traits::Parameter;
use crate::parsers::{CountSqlParser, PlaceholderParser};
use crate::error::NotImplementError;
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

pub trait MySqlTemplate: Template<MySql> + Sync{}
impl<T: Template<MySql>+ Sync> MySqlTemplate for T {}

pub trait PostgresTemplate: Template<Postgres>+ Sync {}
impl<T: Template<Postgres>+ Sync> PostgresTemplate for T {}

pub trait SqliteTemplate: Template<Sqlite>+ Sync {}
impl<T: Template<Sqlite>+ Sync> SqliteTemplate for T {}


