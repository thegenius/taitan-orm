use std::fmt::Debug;
use crate::traits::Location;
use sqlx::{Database, MySql, Postgres, Sqlite};
use crate::traits::Mutation;

pub trait Unique<DB: Database>: Location<DB> + Debug {
    type Mutation: Mutation<DB>;

}

pub trait MysqlUnique: Unique<MySql> {}
impl<T: Unique<MySql>> MysqlUnique for T {}

pub trait PostgresUnique: Unique<Postgres> {}
impl<T: Unique<Postgres>> PostgresUnique for T {}

pub trait SqliteUnique: Unique<Sqlite> + Sync {}
impl<T: Unique<Sqlite> + Sync> SqliteUnique for T {}
