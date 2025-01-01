use sqlx::mysql::MySqlRow;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, MySql, Postgres, Row, Sqlite};
use taitan_orm_trait::{SelectedEntity, Selection};

#[derive(Clone, Debug, Default)]
pub struct EmptySelection {}

impl Selection for EmptySelection {
    fn get_table_name(&self) -> &'static str {
        todo!()
    }

    fn get_selected_fields(&self) -> Vec<String> {
        todo!()
    }


    fn full_fields() -> Self {
        todo!()
    }
}

#[derive(Clone, Debug, Default)]
pub struct CountResult {
    pub count: u64,
}

impl SelectedEntity<Sqlite> for CountResult {
    type Selection = EmptySelection;

    fn from_row(_selection: &Self::Selection, row: SqliteRow) -> Result<Self, Error>
    where
        Self: Sized,
    {
        // TODO: 有可能使用try_get(0)更好
        let count: i64 = row.try_get("count")?;
        Ok(Self {
            count: count as u64,
        })
    }

    fn from_row_full(row: SqliteRow) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let count: i64 = row.try_get("count")?;
        Ok(Self {
            count: count as u64,
        })
    }
}

impl SelectedEntity<MySql> for CountResult {
    type Selection = EmptySelection;

    fn from_row(_selection: &Self::Selection, row: MySqlRow) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let count: i64 = row.try_get("count")?;
        Ok(Self {
            count: count as u64,
        })
    }
    fn from_row_full(row: MySqlRow) -> Result<Self, Error> {
        let count: i64 = row.try_get("count")?;
        Ok(Self {
            count: count as u64,
        })
    }
}

impl SelectedEntity<Postgres> for CountResult {
    type Selection = EmptySelection;

    fn from_row(_selection: &Self::Selection, row: PgRow) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let count: i64 = row.try_get("count")?;
        Ok(Self {
            count: count as u64,
        })
    }
    fn from_row_full(row: PgRow) -> Result<Self, Error> {
        let count: i64 = row.try_get("count")?;
        Ok(Self {
            count: count as u64,
        })
    }
}
