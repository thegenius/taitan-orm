#![allow(async_fn_in_trait)]
#![allow(dead_code)]
#![forbid(unsafe_code)]
mod error;
mod result;

pub mod database;
mod sql_api;

mod db;
mod dto;
mod sql_executor;
mod sql_executor_mut;
mod sql_generator;
pub mod sql_generator_container;

#[macro_use]
mod api_macro;
mod sql_generic_executor;
mod extractor;
mod api;

pub use extractor::Extractor;
pub use taitan_orm_trait::Optional;

// pub use db::DB;
pub use dto::CountResult;
pub use error::TaitanOrmError;
pub use result::Result;
pub use sql_api::SqlApi;
pub use sql_executor::SqlExecutor;
pub use sql_executor_mut::SqlExecutorMut;
pub use sql_generic_executor::SqlGenericExecutor;
pub use sql_generator_container::SqlGeneratorContainer;
pub use sql_generator::DefaultSqlGenerator;
pub use sql_generator::SqlGenerator;

pub use taitan_orm_macro::Schema;
pub use taitan_orm_trait::FieldName;
pub use api::reader::ReaderApi;
pub use api::reader_mut::ReaderMutApi;
pub use api::writer::WriterApi;
pub use api::writer_mut::WriterMutApi;
pub use api::template::TemplateApi;
pub use api::template_mut::TemplateMutApi;

pub use taitan_orm_macro::TemplateRecord;
pub use db::Executor;
pub use taitan_orm_trait::CmpOperator;
pub use taitan_orm_trait::LocationExpr;

pub mod page {
    pub use taitan_orm_trait::pagination::Pagination;
    pub use taitan_orm_trait::paged_info::PagedInfo;
    pub use taitan_orm_trait::paged_list::PagedList;
    pub use taitan_orm_trait::build_paged_list;
}

pub mod traits {
    pub use taitan_orm_trait::{CountSql, Entity, Location, LocationExpr, Mutation, OrderBy, Schema, SelectedEntity, Selection, Unique};
    pub use taitan_orm_trait::validate_order_by;
    pub use taitan_orm_trait::pagination::Pagination;
    pub use taitan_orm_trait::paged_info::PagedInfo;
    pub use taitan_orm_trait::paged_list::PagedList;
    pub use taitan_orm_trait::ParsedTemplateSql;
    pub use taitan_orm_trait::TemplateValue;
    pub use taitan_orm_trait::TemplateRecord;
}