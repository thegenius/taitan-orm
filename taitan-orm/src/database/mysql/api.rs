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
use tracing::debug;

pub trait ReaderApiNew: SqlExecutor<MySql> {
    reader_impl!(
        MySql,
        MySqlSqlGenerator,
        MySqlSelected,
        MySqlMutation,
        MySqlLocation,
        MySqlUnique
    );
}
impl<T> ReaderApiNew for T where T: SqlExecutor<MySql> {}

pub trait ReaderMutApiNew: SqlExecutorMut<MySql> {
    reader_mut_impl!(
        MySql,
        MySqlSqlGenerator,
        MySqlSelected,
        MySqlMutation,
        MySqlLocation,
        MySqlUnique
    );
}
impl<T> ReaderMutApiNew for T where T: SqlExecutorMut<MySql> {}

pub trait WriterApiNew: SqlExecutor<MySql> {
    writer_impl!(
        MySqlSqlGenerator,
        MySqlEntity,
        MySqlMutation,
        MySqlLocation,
        MySqlUnique
    );
}
impl<T> WriterApiNew for T where T: SqlExecutor<MySql> {}

pub trait WriterMutApiNew: SqlExecutorMut<MySql> {
    writer_mut_impl!(
        MySqlSqlGenerator,
        MySqlEntity,
        MySqlMutation,
        MySqlLocation,
        MySqlUnique
    );
}
impl<T> WriterMutApiNew for T where T: SqlExecutorMut<MySql> {}

pub trait TemplateApiNew: SqlExecutor<MySql> {
    template_impl!(sqlx::MySql, MySqlSelected, MySqlTemplate);
}
impl<T> TemplateApiNew for T where T: SqlExecutor<MySql> {}

pub trait TemplateMutApiNew: SqlExecutorMut<MySql> {
    template_mut_impl!(sqlx::MySql, MySqlSelected, MySqlTemplate);
}
impl<T> TemplateMutApiNew for T where T: SqlExecutorMut<MySql> {}
