use std::borrow::Cow;
use std::fmt::Debug;
use sqlx::{Database, MySql, Postgres, Sqlite};
use super::result::Result;

pub trait Selected<DB: Database>: Sized + Default + Debug {

    fn gen_select_sql<'a>(&self) -> Cow<'a, str>;

    fn gen_select_full_sql<'a>(&self) -> Cow<'a, str>;

    fn from_row(selection: &Self, row: DB::Row) -> Result<Self>;

    fn from_row_full(row: DB::Row) -> Result<Self>;
}

pub trait MysqlSelected: Selected<MySql> {}
impl<T: Selected<MySql>> MysqlSelected for T {}

pub trait PostgresSelected: Selected<Postgres> {}
impl<T: Selected<Postgres>> PostgresSelected for T {}

pub trait SqliteSelected: Selected<Sqlite> {}
impl<T: Selected<Sqlite>> SqliteSelected for T {}