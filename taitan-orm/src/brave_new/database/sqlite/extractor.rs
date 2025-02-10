use super::SqliteDatabase;
use crate::prelude::SqlGenericExecutor;
use sqlx::{Database, Sqlite};

use crate::brave_new::database::sqlite::transaction::SqliteTransaction;
use crate::brave_new::ArgsExtractor;
use taitan_orm_trait::brave_new::param::Parameter;
use taitan_orm_trait::brave_new::Pagination;
use taitan_orm_trait::brave_new::result::Result;
impl ArgsExtractor for SqliteDatabase {
    fn extract_pagination_arguments(
        page: &Pagination,
    ) -> Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(<Pagination as Parameter<Sqlite>>::gen_args(page)?)
    }
}

impl<'t> ArgsExtractor for SqliteTransaction<'t> {
    fn extract_pagination_arguments(
        page: &Pagination,
    ) -> Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(<Pagination as Parameter<Sqlite>>::gen_args(page)?)
    }
}
