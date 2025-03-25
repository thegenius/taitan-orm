use crate::database::sqlite::generator::SqliteSqlGenerator;
use crate::executors::{SqlExecutorMut, SqlExecutor};
use crate::{
    reader_impl, reader_mut_impl, template_impl, template_mut_impl, writer_impl, writer_mut_impl,
};
use sqlx::{Sqlite};
use taitan_orm_trait::order::OrderBy;
use taitan_orm_trait::page::{build_paged_list, PagedInfo, PagedList, Pagination};
use taitan_orm_trait::result::Result;
use taitan_orm_trait::traits::{
    Parameter, SqliteEntity, SqliteLocation, SqliteMutation,
    SqliteSelected, SqliteTemplate, SqliteUnique, Selected,
};
use tracing::debug;

pub trait ReaderApiNew: SqlExecutor<Sqlite> {
    reader_impl!(
        Sqlite,
        SqliteSqlGenerator,
        SqliteSelected,
        SqliteMutation,
        SqliteLocation,
        SqliteUnique
    );
}
impl<T> ReaderApiNew for T where T: SqlExecutor<Sqlite> {}

pub trait ReaderMutApiNew: SqlExecutorMut<Sqlite> {
    reader_mut_impl!(
        Sqlite,
        SqliteSqlGenerator,
        SqliteSelected,
        SqliteMutation,
        SqliteLocation,
        SqliteUnique
    );
}
impl<T> ReaderMutApiNew for T where T: SqlExecutorMut<Sqlite> {}

pub trait WriterApiNew: SqlExecutor<Sqlite> {
    writer_impl!(
        SqliteSqlGenerator,
        SqliteEntity,
        SqliteMutation,
        SqliteLocation,
        SqliteUnique
    );
}
impl<T> WriterApiNew for T where T: SqlExecutor<Sqlite> {}

pub trait WriterMutApiNew: SqlExecutorMut<Sqlite> {
    writer_mut_impl!(
        SqliteSqlGenerator,
        SqliteEntity,
        SqliteMutation,
        SqliteLocation,
        SqliteUnique
    );
}
impl<T> WriterMutApiNew for T where T: SqlExecutorMut<Sqlite> {}

pub trait TemplateApiNew: SqlExecutor<Sqlite> {
    template_impl!(sqlx::Sqlite, SqliteSelected, SqliteTemplate);
}
impl<T> TemplateApiNew for T where T: SqlExecutor<Sqlite> {}

pub trait TemplateMutApiNew: SqlExecutorMut<Sqlite> {
    template_mut_impl!(sqlx::Sqlite, SqliteSelected, SqliteTemplate);
}
impl<T> TemplateMutApiNew for T where T: SqlExecutorMut<Sqlite> {}
