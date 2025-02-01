use super::result::Result;
use sqlx::{Arguments, Database};
use std::borrow::Cow;

pub trait Entity<DB: Database> {
    fn gen_insert_sql<'a>(&self) -> Cow<'a, str>;
    fn gen_upsert_sql<'a>(&self) -> Cow<'a, str>;
    fn gen_create_sql<'a>(&self) -> Cow<'a, str>;
    fn add_insert_args<'a>(&'a self, args: &mut DB::Arguments<'a>) -> Result<()>;
    fn add_upsert_args<'a>(&'a self, args: &mut DB::Arguments<'a>) -> Result<()>;
    fn gen_insert_args(&self) -> Result<DB::Arguments<'_>> {
        let mut args = DB::Arguments::default();
        self.add_insert_args(&mut args)?;
        Ok(args)
    }
    fn gen_upsert_args(&self) -> Result<DB::Arguments<'_>> {
        let mut args = DB::Arguments::default();
        self.add_upsert_args(&mut args)?;
        Ok(args)
    }
}
