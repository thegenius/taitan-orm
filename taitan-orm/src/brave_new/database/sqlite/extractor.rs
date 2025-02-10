use sqlx::{Database, Sqlite};
use super::SqliteDatabase;
use crate::prelude::{SqlGenericExecutor};

use taitan_orm_trait::brave_new::{Entity, Location, Mutation, Template, Unique};
use taitan_orm_trait::brave_new::Pagination;

use crate::brave_new::ArgsExtractor;
use crate::brave_new::database::sqlite::transaction::SqliteTransaction;

impl ArgsExtractor for SqliteDatabase {
    fn extract_pagination_arguments(page: &Pagination) -> taitan_orm_trait::brave_new::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(page.gen_page_arguments_sqlite().unwrap())
    }
}

impl <'t> ArgsExtractor for SqliteTransaction<'t> {
    fn extract_pagination_arguments(page: &Pagination) -> taitan_orm_trait::brave_new::result::Result<<Self::DB as Database>::Arguments<'_>> {
        Ok(page.gen_page_arguments_sqlite().unwrap())
    }
}