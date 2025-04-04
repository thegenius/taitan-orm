use crate::logic::And;
use crate::traits::{Location, LocationKind, Parameter};
use sqlx::Database;
use std::borrow::Cow;
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;

pub struct Or<DB, L, R>
where
    DB: Database,
    L: Location<DB> + Debug,
    R: Location<DB> + Debug,
{
    pub left: L,
    pub right: R,
    _marker: PhantomData<DB>,
}

impl<DB, L, R> Or<DB, L, R>
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

impl<DB, L, R> Debug for Or<DB, L, R>
where
    DB: Database,
    L: Location<DB> + Debug,
    R: Location<DB> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} OR {:?})", self.left, self.right)
    }
}

impl<DB, L, R> Parameter<DB> for Or<DB, L, R>
where
    DB: Database + Sync,
    L: Location<DB> + Debug + Sync,
    R: Location<DB> + Debug + Sync,
{
    fn add_to_args<'a, 'b>(
        &'a self,
        args: &'b mut <DB as Database>::Arguments<'a>,
    ) -> crate::result::Result<()> {
        if self.left.all_none() {
            self.right.add_to_args(args)?;
        } else if self.right.all_none() {
            self.left.add_to_args(args)?;
        } else {
            self.left.add_to_args(args)?;
            self.right.add_to_args(args)?;
        }
        Ok(())
    }
}

impl<DB, L, R> Location<DB> for Or<DB, L, R>
where
    DB: Database + Sync,
    L: Location<DB> + Debug + Sync,
    R: Location<DB> + Debug + Sync,
{
    fn table_name(&self) -> Cow<'static, str> {
        let left_table_name = self.left.table_name();
        let right_table_name = self.right.table_name();
        if left_table_name != right_table_name {
            Cow::Borrowed("")
        } else {
            left_table_name
        }
    }
    fn kind(&self) -> LocationKind {
        LocationKind::Or
    }
    fn gen_where_sql<'a>(&self) -> Cow<'a, str> {
        if self.left.all_none() {
            self.right.gen_where_sql()
        } else if self.right.all_none() {
            self.left.gen_where_sql()
        } else {
            format!(
                "({} OR {})",
                self.left.gen_where_sql(),
                self.right.gen_where_sql()
            )
            .into()
        }
    }

    // fn add_where_args<'a>(
    //     &'a self,
    //     args: &mut DB::Arguments<'a>,
    // ) -> crate::brave_new::result::Result<()> {
    //     if self.left.all_none() {
    //         self.right.add_where_args(args)?;
    //     } else if self.right.all_none() {
    //         self.left.add_where_args(args)?;
    //     } else {
    //         self.left.add_where_args(args)?;
    //         self.right.add_where_args(args)?;
    //     }
    //     Ok(())
    // }

    fn all_none(&self) -> bool {
        self.left.all_none() && self.right.all_none()
    }
}
