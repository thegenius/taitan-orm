use crate::database::postgres::generator::PostgresSqlGenerator;
use crate::executors::{SqlExecutorMut, SqlExecutor};
use crate::{
    reader_impl, reader_mut_impl, template_impl, template_mut_impl, writer_impl, writer_mut_impl,
};
use sqlx::{Postgres};
use taitan_orm_trait::order::OrderBy;
use taitan_orm_trait::page::{build_paged_list, PagedInfo, PagedList, Pagination};
use taitan_orm_trait::result::Result;
use taitan_orm_trait::traits::{
    Parameter, PostgresEntity, PostgresLocation, PostgresMutation,
    PostgresSelected, PostgresTemplate, PostgresUnique, Selected,
};
use tracing::debug;

pub trait ReaderApi: SqlExecutor<Postgres> {
    reader_impl!(
        Postgres,
        PostgresSqlGenerator,
        PostgresSelected,
        PostgresMutation,
        PostgresLocation,
        PostgresUnique
    );
}
impl<T> ReaderApi for T where T: SqlExecutor<Postgres> {}

pub trait ReaderMutApi: SqlExecutorMut<Postgres> {
    reader_mut_impl!(
        Postgres,
        PostgresSqlGenerator,
        PostgresSelected,
        PostgresMutation,
        PostgresLocation,
        PostgresUnique
    );
}
impl<T> ReaderMutApi for T where T: SqlExecutorMut<Postgres> {}

pub trait WriterApi: SqlExecutor<Postgres> {
    writer_impl!(
        PostgresSqlGenerator,
        PostgresEntity,
        PostgresMutation,
        PostgresLocation,
        PostgresUnique
    );
}
impl<T> WriterApi for T where T: SqlExecutor<Postgres> {}

pub trait WriterMutApi: SqlExecutorMut<Postgres> {
    writer_mut_impl!(
        PostgresSqlGenerator,
        PostgresEntity,
        PostgresMutation,
        PostgresLocation,
        PostgresUnique
    );
}
impl<T> WriterMutApi for T where T: SqlExecutorMut<Postgres> {}

pub trait TemplateApi: SqlExecutor<Postgres> {
    template_impl!(sqlx::Postgres, PostgresSelected, PostgresTemplate);
}
impl<T> TemplateApi for T where T: SqlExecutor<Postgres> {}

pub trait TemplateMutApi: SqlExecutorMut<Postgres> {
    template_mut_impl!(sqlx::Postgres, PostgresSelected, PostgresTemplate);
}
impl<T> TemplateMutApi for T where T: SqlExecutorMut<Postgres> {}
