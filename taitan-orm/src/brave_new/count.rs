use std::borrow::Cow;
use sqlx::{MySql, Postgres, Row, Sqlite};
use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use taitan_orm_trait::brave_new::Selected;
use taitan_orm_trait::brave_new::result::Result;

#[derive(Clone, Debug, Default)]
pub struct CountResult {
    pub count: u64,
}


impl Selected<Sqlite> for CountResult {
    fn gen_select_sql<'a>(&self) -> Cow<'a, str> {
        Cow::Borrowed(" COUNT(1) ")
    }

    fn from_row(_selection: &Self, row: SqliteRow) -> Result<Self> {
        let count: i64 = row.try_get(0)?;
        Ok(Self {
            count: count as u64,
        })
    }

}

impl Selected<MySql> for CountResult {
    fn gen_select_sql<'a>(&self) -> Cow<'a, str> {
        Cow::Borrowed(" COUNT(1) ")
    }


    fn from_row(_selection: &Self, row: MySqlRow) -> Result<Self> {
        let count: i64 = row.try_get(0)?;
        Ok(Self {
            count: count as u64,
        })
    }
}

impl Selected<Postgres> for CountResult {
    fn gen_select_sql<'a>(&self) -> Cow<'a, str> {
        Cow::Borrowed(" COUNT(1) ")
    }


    fn from_row(_selection: &Self, row: PgRow) -> Result<Self> {
        let count: i64 = row.try_get(0)?;
        Ok(Self {
            count: count as u64,
        })
    }
}