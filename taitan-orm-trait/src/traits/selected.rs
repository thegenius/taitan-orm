use crate::result::Result;
use sqlx::{Database, MySql, Postgres, Sqlite};
use std::borrow::Cow;
use std::fmt::Debug;

pub fn selected<T>() -> Option<Option<T>> {
    Some(None)
}

pub trait Selected<DB: Database>: Sized + Default + Debug {
    fn gen_select_sql<'a>(&self) -> Cow<'a, str>;

    fn from_row(selection: &Self, row: DB::Row) -> Result<Self>;

    fn from_row_full(row: DB::Row) -> Result<Self> {
        Self::from_row(&Self::default(), row)
    }
}

pub trait MySqlSelected: Selected<MySql> + Sync {}
impl<T: Selected<MySql> + Sync> MySqlSelected for T {}

pub trait PostgresSelected: Selected<Postgres> + Sync {}
impl<T: Selected<Postgres> + Sync> PostgresSelected for T {}

pub trait SqliteSelected: Selected<Sqlite> + Sync {}
impl<T: Selected<Sqlite> + Sync> SqliteSelected for T {}
