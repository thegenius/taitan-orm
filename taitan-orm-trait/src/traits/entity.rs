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



pub trait MySqlEntity: Entity<MySql> + Sync {}
impl<T: Entity<MySql> + Sync> MySqlEntity for T {}

pub trait PostgresEntity: Entity<Postgres> + Sync{}
impl<T: Entity<Postgres> + Sync> PostgresEntity for T {}

pub trait SqliteEntity: Entity<Sqlite> + Sync {}
impl<T: Entity<Sqlite> + Sync> SqliteEntity for T {}