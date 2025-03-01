use super::result::Result;
use sqlx::{Arguments, Database, MySql, Postgres, Sqlite};
use std::borrow::Cow;
use std::fmt::Debug;
use crate::brave_new::param::Parameter;

pub trait Entity<DB: Database>: Parameter<DB> + Debug {
    fn gen_insert_sql<'a>(&self) -> Cow<'a, str>;
    fn gen_upsert_sql<'a>(&self) -> Cow<'a, str>;
    fn gen_create_sql<'a>(&self) -> Cow<'a, str>;

    // fn add_insert_args<'a>(&'a self, args: &mut DB::Arguments<'a>) -> Result<()>;
    // fn add_upsert_args<'a>(&'a self, args: &mut DB::Arguments<'a>) -> Result<()>;
    // fn gen_insert_args(&self) -> Result<DB::Arguments<'_>> {
    //     let mut args = DB::Arguments::default();
    //     self.add_insert_args(&mut args)?;
    //     Ok(args)
    // }
    // fn gen_upsert_args(&self) -> Result<DB::Arguments<'_>> {
    //     let mut args = DB::Arguments::default();
    //     self.add_upsert_args(&mut args)?;
    //     Ok(args)
    // }
}



pub trait MysqlEntity: Entity<MySql> {}
impl<T: Entity<MySql>> MysqlEntity for T {}

pub trait PostgresEntity: Entity<Postgres> {}
impl<T: Entity<Postgres>> PostgresEntity for T {}

pub trait SqliteEntity: Entity<Sqlite> {}
impl<T: Entity<Sqlite>> SqliteEntity for T {}