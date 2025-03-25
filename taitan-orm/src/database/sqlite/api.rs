use crate::database::sqlite::generator::SqliteSqlGenerator;
use crate::new_executor::{SqlExecutorMutNew, SqlExecutorNew};
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

pub trait ReaderApiNew: SqlExecutorNew<Sqlite> {
    reader_impl!(
        Sqlite,
        SqliteSqlGenerator,
        SqliteSelected,
        SqliteMutation,
        SqliteLocation,
        SqliteUnique
    );
}
impl<T> ReaderApiNew for T where T: SqlExecutorNew<Sqlite> {}

pub trait ReaderMutApiNew: SqlExecutorMutNew<Sqlite> {
    reader_mut_impl!(
        Sqlite,
        SqliteSqlGenerator,
        SqliteSelected,
        SqliteMutation,
        SqliteLocation,
        SqliteUnique
    );
}
impl<T> ReaderMutApiNew for T where T: SqlExecutorMutNew<Sqlite> {}

pub trait WriterApiNew: SqlExecutorNew<Sqlite> {
    writer_impl!(
        SqliteSqlGenerator,
        SqliteEntity,
        SqliteMutation,
        SqliteLocation,
        SqliteUnique
    );
}
impl<T> WriterApiNew for T where T: SqlExecutorNew<Sqlite> {}

pub trait WriterMutApiNew: SqlExecutorMutNew<Sqlite> {
    writer_mut_impl!(
        SqliteSqlGenerator,
        SqliteEntity,
        SqliteMutation,
        SqliteLocation,
        SqliteUnique
    );
}
impl<T> WriterMutApiNew for T where T: SqlExecutorMutNew<Sqlite> {}

pub trait TemplateApiNew: SqlExecutorNew<Sqlite> {
    template_impl!(sqlx::Sqlite, SqliteSelected, SqliteTemplate);
}
impl<T> TemplateApiNew for T where T: SqlExecutorNew<Sqlite> {}

pub trait TemplateMutApiNew: SqlExecutorMutNew<Sqlite> {
    template_mut_impl!(sqlx::Sqlite, SqliteSelected, SqliteTemplate);
}
impl<T> TemplateMutApiNew for T where T: SqlExecutorMutNew<Sqlite> {}
