use crate::traits::Parameter;
use sqlx::{Database, MySql, Postgres, Sqlite};
use std::borrow::Cow;
use std::fmt::Debug;

pub enum LocationKind {
    Plain,
    Not,
    And,
    Or,
}

#[derive(Debug, Clone, Copy)]
pub enum LogicOp {
    And,
    Or,
}

pub trait Location<DB: Database>: Parameter<DB> + Debug {
    fn table_name(&self) -> Cow<'static, str>;
    fn gen_where_sql<'a>(&self) -> Cow<'a, str>;
    fn all_none(&self) -> bool;
    fn kind(&self) -> LocationKind {
        LocationKind::Plain
    }
}

pub trait MySqlLocation: Location<MySql>+ Sync {}
impl<T: Location<MySql>+ Sync> MySqlLocation for T {}

pub trait PostgresLocation: Location<Postgres> + Sync{}
impl<T: Location<Postgres>+ Sync> PostgresLocation for T {}

pub trait SqliteLocation: Location<Sqlite>+ Sync {}
impl<T: Location<Sqlite>+ Sync> SqliteLocation for T {}
