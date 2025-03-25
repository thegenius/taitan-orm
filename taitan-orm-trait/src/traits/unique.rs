use crate::traits::Location;
use crate::traits::Mutation;
use sqlx::{Database, MySql, Postgres, Sqlite};
use std::fmt::Debug;

pub trait Unique<DB: Database>: Location<DB> + Debug {
    type Mutation: Mutation<DB>;
}

pub trait MySqlUnique: Unique<MySql> + Sync {}
impl<T: Unique<MySql> + Sync> MySqlUnique for T {}

pub trait PostgresUnique: Unique<Postgres> + Sync {}
impl<T: Unique<Postgres> + Sync> PostgresUnique for T {}

pub trait SqliteUnique: Unique<Sqlite> + Sync {}
impl<T: Unique<Sqlite> + Sync> SqliteUnique for T {}
