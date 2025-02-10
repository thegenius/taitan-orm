use std::fmt::Debug;
use crate::brave_new::location::Location;
use sqlx::{Database, MySql, Postgres, Sqlite};
use crate::brave_new::Mutation;

pub trait Unique<DB: Database>: Location<DB> + Debug {
    type Mutation: Mutation<DB>;

}

pub trait MysqlUnique: Unique<MySql> {}
impl<T: Unique<MySql>> MysqlUnique for T {}

pub trait PostgresUnique: Unique<Postgres> {}
impl<T: Unique<Postgres>> PostgresUnique for T {}

pub trait SqliteUnique: Unique<Sqlite> {}
impl<T: Unique<Sqlite>> SqliteUnique for T {}
