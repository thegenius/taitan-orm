use crate::brave_new::location::Location;
use sqlx::Database;
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

pub struct And<DB, L, R>
where
    DB: Database,
    L: Location<DB> + Debug,
    R: Location<DB> + Debug,
{
    pub left: L,
    pub right: R,
    _marker: PhantomData<DB>,
}

impl<DB, L, R> And<DB, L, R>
where
    DB: Database,
    L: Location<DB> + Debug,
    R: Location<DB> + Debug,
{
    pub fn new(left: L, right: R) -> Self {
        Self {
            left,
            right,
            _marker: PhantomData,
        }
    }
}

impl<DB, L, R> Debug for And<DB, L, R>
where
    DB: Database,
    L: Location<DB> + Debug,
    R: Location<DB> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} AND {:?})", self.left, self.right)
    }
}

impl<DB, L, R> Location<DB> for And<DB, L, R>
where
    DB: Database,
    L: Location<DB> + Debug,
    R: Location<DB> + Debug,
{
    fn gen_where_sql<'a>(&self) -> Cow<'a, str> {
        if self.left.all_none() {
            self.right.gen_where_sql()
        } else if self.right.all_none() {
            self.left.gen_where_sql()
        } else {
            format!(
                "({} AND {})",
                self.left.gen_where_sql(),
                self.right.gen_where_sql()
            )
            .into()
        }
    }

    fn add_where_args<'a>(
        &'a self,
        args: &mut DB::Arguments<'a>,
    ) -> crate::brave_new::result::Result<()> {
        if self.left.all_none() {
            self.right.add_where_args(args)?;
        } else if self.right.all_none() {
            self.left.add_where_args(args)?;
        } else {
            self.left.add_where_args(args)?;
            self.right.add_where_args(args)?;
        }
        Ok(())
    }

    fn all_none(&self) -> bool {
        self.left.all_none() && self.right.all_none()
    }
}
