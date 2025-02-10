use crate::brave_new::location::{Location, LocationKind};
use sqlx::Database;
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

pub struct Not<DB, T>
where
    DB: Database,
    T: Location<DB> + Debug,
{
    pub expr: T,
    _marker: PhantomData<DB>,
}

impl<DB, T> Not<DB, T>
where
    DB: Database,
    T: Location<DB> + Debug,
{
    pub fn new(expr: T) -> Self {
        Self {
            expr,
            _marker: PhantomData,
        }
    }
}

impl<DB, T> Debug for Not<DB, T>
where
    DB: Database,
    T: Location<DB> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(NOT {:?})", self.expr)
    }
}

impl<DB, T> Location<DB> for Not<DB, T>
where
    DB: Database,
    T: Location<DB> + Debug,
{
    fn table_name(&self) -> Cow<'static, str> {
        self.expr.table_name()
    }
    fn kind(&self) -> LocationKind {
        LocationKind::Not
    }
    fn gen_where_sql<'a>(&self) -> Cow<'a, str> {
        if self.expr.all_none() {
            self.expr.gen_where_sql()
        } else {
            format!(
                "(NOT {})",
                self.expr.gen_where_sql(),
            )
            .into()
        }
    }

    fn add_where_args<'a>(
        &'a self,
        args: &mut DB::Arguments<'a>,
    ) -> crate::brave_new::result::Result<()> {
        if !self.expr.all_none() {
            self.expr.add_where_args(args)?;
        }
        Ok(())
    }

    fn all_none(&self) -> bool {
        self.expr.all_none()
    }
}
