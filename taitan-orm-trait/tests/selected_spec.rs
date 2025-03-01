use std::borrow::Cow;
use sqlx::Arguments;
use sqlx::{Database, Sqlite};
use sqlx::sqlite::SqliteRow;
use taitan_orm_trait::brave_new::error::wrap_encode;
use taitan_orm_trait::brave_new::param::Parameter;
use taitan_orm_trait::brave_new::result::Result;
use time::PrimitiveDateTime;
use taitan_orm_trait::brave_new::Selected;


type Optional<T> = Option<Option<T>>;

#[derive(Debug, Default)]
struct UserSelected {
    name: Optional<String>,
    created: Optional<PrimitiveDateTime>,
}

impl Parameter<Sqlite> for UserSelected {
    fn add_to_args<'a, 'b>(
        &'a self,
        args: &'b mut <Sqlite as Database>::Arguments<'a>,
    ) -> Result<()> {
        if let Some(f) =  &self.name {
            sqlx::Arguments::add(args, f)?;
        }
        if let Some(f) =  &self.created {
            sqlx::Arguments::add(args, f)?;
        }
        Ok(())
    }
}

impl Selected<Sqlite> for UserSelected {
    fn gen_select_sql<'a>(&self) -> Cow<'a, str> {
        todo!()
    }


    fn from_row(selection: &Self, row: SqliteRow) -> Result<Self> {
        todo!()
    }
}
