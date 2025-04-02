use crate::database::mysql::generator::MySqlSqlGenerator;
use crate::executors::{SqlExecutorMut, SqlExecutor};
use crate::{
    reader_impl, reader_mut_impl, template_impl, template_mut_impl, writer_impl, writer_mut_impl,
};
use sqlx::MySql;
use taitan_orm_trait::order::OrderBy;
use taitan_orm_trait::page::{build_paged_list, PagedInfo, PagedList, Pagination};
use taitan_orm_trait::result::Result;
use taitan_orm_trait::traits::{
    MySqlEntity, MySqlLocation, MySqlMutation, MySqlSelected, MySqlTemplate,
    MySqlUnique, Parameter, Selected,
};
use taitan_orm_tracing::debug;

pub trait ReaderApi: SqlExecutor<MySql> {
    reader_impl!(
        MySql,
        MySqlSqlGenerator,
        MySqlSelected,
        MySqlMutation,
        MySqlLocation,
        MySqlUnique
    );
}
impl<T> ReaderApi for T where T: SqlExecutor<MySql> {}

pub trait ReaderMutApi: SqlExecutorMut<MySql> {
    reader_mut_impl!(
        MySql,
        MySqlSqlGenerator,
        MySqlSelected,
        MySqlMutation,
        MySqlLocation,
        MySqlUnique
    );
}
impl<T> ReaderMutApi for T where T: SqlExecutorMut<MySql> {}

pub trait WriterApi: SqlExecutor<MySql> {
    writer_impl!(
        MySqlSqlGenerator,
        MySqlEntity,
        MySqlMutation,
        MySqlLocation,
        MySqlUnique
    );
}
impl<T> WriterApi for T where T: SqlExecutor<MySql> {}

pub trait WriterMutApi: SqlExecutorMut<MySql> {
    writer_mut_impl!(
        MySqlSqlGenerator,
        MySqlEntity,
        MySqlMutation,
        MySqlLocation,
        MySqlUnique
    );
}
impl<T> WriterMutApi for T where T: SqlExecutorMut<MySql> {}

pub trait TemplateApi: SqlExecutor<MySql> {
    template_impl!(sqlx::MySql, MySqlSelected, MySqlTemplate);
}
impl<T> TemplateApi for T where T: SqlExecutor<MySql> {}

pub trait TemplateMutApi: SqlExecutorMut<MySql> {
    template_mut_impl!(sqlx::MySql, MySqlSelected, MySqlTemplate);
}
impl<T> TemplateMutApi for T where T: SqlExecutorMut<MySql> {}
