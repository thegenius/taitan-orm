use taitan_orm_trait::{Optional, Selection};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SelectedTest {
    age: i32,
    name: Optional<String>,
}

impl taitan_orm::traits::SelectedEntity<sqlx::Sqlite> for SelectedTest {
    fn from_row(
        selection: &Self,
        row: <sqlx::Sqlite as sqlx::Database>::Row,
    ) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let mut selected = Self::default();
        let mut i = 0;
        selected.age = sqlx::Row::try_get(&row, i)?;
        i += 1;
        if selection.name.is_selected() {
            selected.name = taitan_orm::result::Optional::Some(sqlx::Row::try_get(&row, i)?);
            i += 1;
        };
        Ok(selected)
    }
    fn from_row_full(row: <sqlx::Sqlite as sqlx::Database>::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let mut selected = Self::default();
        let mut i = 0;
        selected.age = sqlx::Row::try_get(&row, i)?;
        i += 1;
        selected.name = taitan_orm::result::Optional::Some(sqlx::Row::try_get(&row, i)?);
        i += 1;
        Ok(selected)
    }
}
impl taitan_orm::traits::SelectedEntity<sqlx::MySql> for SelectedTest {
    fn from_row(
        selection: &Self,
        row: <sqlx::MySql as sqlx::Database>::Row,
    ) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let mut selected = Self::default();
        let mut i = 0;
        selected.age = sqlx::Row::try_get(&row, i)?;
        i += 1;
        if selection.name.is_selected() {
            selected.name = taitan_orm::result::Optional::Some(sqlx::Row::try_get(&row, i)?);
            i += 1;
        };
        Ok(selected)
    }
    fn from_row_full(row: <sqlx::MySql as sqlx::Database>::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let mut selected = Self::default();
        let mut i = 0;
        selected.age = sqlx::Row::try_get(&row, i)?;
        i += 1;
        selected.name = taitan_orm::result::Optional::Some(sqlx::Row::try_get(&row, i)?);
        i += 1;
        Ok(selected)
    }
}
impl taitan_orm::traits::SelectedEntity<sqlx::Postgres> for SelectedTest {
    fn from_row(
        selection: &Self,
        row: <sqlx::Postgres as sqlx::Database>::Row,
    ) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let mut selected = Self::default();
        let mut i = 0;
        selected.age = sqlx::Row::try_get(&row, i)?;
        i += 1;
        if selection.name.is_selected() {
            selected.name = taitan_orm::result::Optional::Some(sqlx::Row::try_get(&row, i)?);
            i += 1;
        };
        Ok(selected)
    }
    fn from_row_full(row: <sqlx::Postgres as sqlx::Database>::Row) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let mut selected = Self::default();
        let mut i = 0;
        selected.age = sqlx::Row::try_get(&row, i)?;
        i += 1;
        selected.name = taitan_orm::result::Optional::Some(sqlx::Row::try_get(&row, i)?);
        i += 1;
        Ok(selected)
    }
}
impl taitan_orm::traits::Selection for SelectedTest {
    fn get_table_name(&self) -> &'static str {
        "SelectedTest"
    }
    fn get_selected_fields(&self) -> Vec<String> {
        let mut fields = Vec::new();
        fields.push("age".to_string());
        if self.name.is_selected() {
            fields.push("name".to_string());
        };
        return fields;
    }
    fn full_fields() -> Self
    where
        Self: Sized + Default,
    {
        Self {
            name: taitan_orm::result::Optional::Null,
            ..Default::default()
        }
    }
}

