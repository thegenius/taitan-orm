use crate::database::postgres::generator::PostgresSqlGenerator;
use crate::new_executor::{SqlExecutorMutNew, SqlExecutorNew};
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

pub trait ReaderApiNew: SqlExecutorNew<Postgres> {
    reader_impl!(
        Postgres,
        PostgresSqlGenerator,
        PostgresSelected,
        PostgresMutation,
        PostgresLocation,
        PostgresUnique
    );
}
impl<T> ReaderApiNew for T where T: SqlExecutorNew<Postgres> {}

pub trait ReaderMutApiNew: SqlExecutorMutNew<Postgres> {
    reader_mut_impl!(
        Postgres,
        PostgresSqlGenerator,
        PostgresSelected,
        PostgresMutation,
        PostgresLocation,
        PostgresUnique
    );
}
impl<T> ReaderMutApiNew for T where T: SqlExecutorMutNew<Postgres> {}

pub trait WriterApiNew: SqlExecutorNew<Postgres> {
    writer_impl!(
        PostgresSqlGenerator,
        PostgresEntity,
        PostgresMutation,
        PostgresLocation,
        PostgresUnique
    );
}
impl<T> WriterApiNew for T where T: SqlExecutorNew<Postgres> {}

pub trait WriterMutApiNew: SqlExecutorMutNew<Postgres> {
    writer_mut_impl!(
        PostgresSqlGenerator,
        PostgresEntity,
        PostgresMutation,
        PostgresLocation,
        PostgresUnique
    );
}
impl<T> WriterMutApiNew for T where T: SqlExecutorMutNew<Postgres> {}

pub trait TemplateApiNew: SqlExecutorNew<Postgres> {
    template_impl!(sqlx::Postgres, PostgresSelected, PostgresTemplate);
}
impl<T> TemplateApiNew for T where T: SqlExecutorNew<Postgres> {}

pub trait TemplateMutApiNew: SqlExecutorMutNew<Postgres> {
    template_mut_impl!(sqlx::Postgres, PostgresSelected, PostgresTemplate);
}
impl<T> TemplateMutApiNew for T where T: SqlExecutorMutNew<Postgres> {}
