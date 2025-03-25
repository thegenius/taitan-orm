use crate::result::Result;
use crate::traits::Parameter;
use sqlx::{Database, MySql, Postgres, Sqlite};
use std::borrow::Cow;
use std::fmt::Debug;

pub trait Mutation<DB: Database>: Parameter<DB> + Debug {
    fn gen_update_set_sql<'a>(&self) -> Cow<'a, str>;
    // fn add_update_set_args<'a>(&'a self, args: &mut DB::Arguments<'a>) -> Result<()>;
    fn all_none(&self) -> bool;
}

pub trait MySqlMutation: Mutation<MySql> + Sync {}
impl<T: Mutation<MySql> + Sync> MySqlMutation for T {}

pub trait PostgresMutation: Mutation<Postgres> + Sync {}
impl<T: Mutation<Postgres> + Sync> PostgresMutation for T {}

pub trait SqliteMutation: Mutation<Sqlite> + Sync {}
impl<T: Mutation<Sqlite> + Sync> SqliteMutation for T {}
