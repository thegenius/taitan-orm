use std::borrow::Cow;
use std::fmt::Debug;
use sqlx::{Database, MySql, Postgres, Sqlite};


pub trait Location<DB: Database>: Debug {
    fn gen_where_sql<'a>(&self) -> Cow<'a, str>;
    fn add_where_args<'a>(&'a self, args: &mut DB::Arguments<'a>) -> crate::brave_new::result::Result<()>;
    fn all_none(&self) -> bool;
}

pub trait MysqlLocation: Location<MySql> {}
impl<T: Location<MySql>> MysqlLocation for T {}

pub trait PostgresLocation: Location<Postgres> {}
impl<T: Location<Postgres>> PostgresLocation for T {}

pub trait SqliteLocation: Location<Sqlite> {}
impl<T: Location<Sqlite>> SqliteLocation for T {}