use crate::database::sqlite::generator::SqliteSqlGenerator;
use crate::executors::{SqlExecutor, SqlExecutorMut};
use crate::{
    reader_impl, reader_mut_impl, template_impl, template_mut_impl, writer_impl, writer_mut_impl,
};
use sqlx::Sqlite;
use taitan_orm_trait::order::OrderBy;
use taitan_orm_trait::page::{build_paged_list, PagedInfo, PagedList, Pagination};
use taitan_orm_trait::result::Result;
use taitan_orm_trait::traits::{
    Parameter, Selected, SqliteEntity, SqliteLocation, SqliteMutation, SqliteSelected,
    SqliteTemplate, SqliteUnique,
};
use tracing::debug;

pub trait ReaderApi: SqlExecutor<Sqlite> {
    reader_impl!(
        Sqlite,
        SqliteSqlGenerator,
        SqliteSelected,
        SqliteMutation,
        SqliteLocation,
        SqliteUnique
    );
}
impl<T> ReaderApi for T where T: SqlExecutor<Sqlite> {}

pub trait ReaderMutApi: SqlExecutorMut<Sqlite> {
    reader_mut_impl!(
        Sqlite,
        SqliteSqlGenerator,
        SqliteSelected,
        SqliteMutation,
        SqliteLocation,
        SqliteUnique
    );
}
impl<T> ReaderMutApi for T where T: SqlExecutorMut<Sqlite> {}

pub trait WriterApi: SqlExecutor<Sqlite> {
    writer_impl!(
        SqliteSqlGenerator,
        SqliteEntity,
        SqliteMutation,
        SqliteLocation,
        SqliteUnique
    );
}
impl<T> WriterApi for T where T: SqlExecutor<Sqlite> {}

pub trait WriterMutApi: SqlExecutorMut<Sqlite> {
    writer_mut_impl!(
        SqliteSqlGenerator,
        SqliteEntity,
        SqliteMutation,
        SqliteLocation,
        SqliteUnique
    );
}
impl<T> WriterMutApi for T where T: SqlExecutorMut<Sqlite> {}

pub trait TemplateApi: SqlExecutor<Sqlite> {
    template_impl!(sqlx::Sqlite, SqliteSelected, SqliteTemplate);
}
impl<T> TemplateApi for T where T: SqlExecutor<Sqlite> {}

pub trait TemplateMutApi: SqlExecutorMut<Sqlite> {
    template_mut_impl!(sqlx::Sqlite, SqliteSelected, SqliteTemplate);
}
impl<T> TemplateMutApi for T where T: SqlExecutorMut<Sqlite> {}
