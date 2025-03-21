use crate::result::Result;
use sqlx::{Database, MySql, Postgres, Sqlite};
use std::borrow::Cow;
use std::fmt::Debug;
use crate::traits::Parameter;

pub trait Mutation<DB: Database>: Parameter<DB> +  Debug {
    fn gen_update_set_sql<'a>(&self) -> Cow<'a, str>;
    // fn add_update_set_args<'a>(&'a self, args: &mut DB::Arguments<'a>) -> Result<()>;
    fn all_none(&self) -> bool;
}

pub trait MysqlMutation: Mutation<MySql> {}
impl<T: Mutation<MySql>> MysqlMutation for T {}

pub trait PostgresMutation: Mutation<Postgres> {}
impl<T: Mutation<Postgres>> PostgresMutation for T {}

pub trait SqliteMutation: Mutation<Sqlite> {}
impl<T: Mutation<Sqlite>> SqliteMutation for T {}