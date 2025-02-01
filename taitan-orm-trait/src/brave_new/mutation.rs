use super::result::Result;
use sqlx::Database;
use std::borrow::Cow;
pub trait Mutation<DB: Database> {
    fn gen_update_set_sql<'a>(&self) -> Cow<'a, str>;
    fn add_update_set_args<'a>(&'a self, args: &mut DB::Arguments<'a>) -> Result<()>;
    fn all_none(&self) -> bool;
}
