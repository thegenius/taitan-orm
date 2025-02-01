use std::borrow::Cow;
use sqlx::Database;

pub trait Location<DB: Database> {
    fn gen_where_sql<'a>(&self) -> Cow<'a, str>;
    fn add_where_args<'a>(&'a self, args: &mut DB::Arguments<'a>) -> crate::brave_new::result::Result<()>;
    fn all_none(&self) -> bool;
}
