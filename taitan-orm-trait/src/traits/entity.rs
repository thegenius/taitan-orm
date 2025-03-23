use crate::result::Result;
use sqlx::{Arguments, Database, MySql, Postgres, Sqlite};
use std::borrow::Cow;
use std::fmt::Debug;
use crate::traits::Parameter;

pub trait Entity<DB: Database>: Parameter<DB> + Debug {
    fn gen_insert_sql<'a>(&self) -> Cow<'a, str>;
    fn gen_upsert_sql<'a>(&self) -> Cow<'a, str>;
    fn gen_create_sql<'a>(&self) -> Cow<'a, str>;
}



pub trait MysqlEntity: Entity<MySql> {}
impl<T: Entity<MySql>> MysqlEntity for T {}

pub trait PostgresEntity: Entity<Postgres> {}
impl<T: Entity<Postgres>> PostgresEntity for T {}

pub trait SqliteEntity: Entity<Sqlite> {}
impl<T: Entity<Sqlite>> SqliteEntity for T {}