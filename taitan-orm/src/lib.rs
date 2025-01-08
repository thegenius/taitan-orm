#![allow(async_fn_in_trait)]
#![allow(dead_code)]
#![forbid(unsafe_code)]


mod sql_api;

mod db;
mod dto;
mod sql_executor;
mod sql_executor_mut;
mod sql_generator;
mod sql_generator_container;


#[doc(hidden)]
#[macro_use]
mod api_macro;
mod sql_generic_executor;
mod extractor;
mod api;

pub mod database;

pub mod error;
pub mod result;


pub mod prelude {
    pub use taitan_orm_macro::Schema;
    pub use taitan_orm_macro::TemplateRecord;
    pub use taitan_orm_macro::Selected;
    pub use taitan_orm_macro::Condition;
    // pub use crate::db::Executor;

    pub use crate::result::Optional;
    pub use crate::result::CountResult;
    pub use crate::result::Result as TaitanOrmResult;

    pub use crate::sql_executor::SqlExecutor;
    pub use crate::sql_generic_executor::SqlGenericExecutor;
    pub use crate::sql_executor_mut::SqlExecutorMut;
    pub use crate::sql_generator_container::SqlGeneratorContainer;
    pub use crate::extractor::Extractor;

    pub use crate::api::reader::ReaderApi;
    pub use crate::api::writer::WriterApi;
    pub use crate::api::template::TemplateApi;

    pub use crate::api::reader_mut::ReaderMutApi;
    pub use crate::api::writer_mut::WriterMutApi;
    pub use crate::api::template_mut::TemplateMutApi;


    pub use taitan_orm_trait::FieldName;
    pub use crate::traits::*;
    pub use crate::page::*;
}

pub mod page {
    pub use taitan_orm_trait::pagination::Pagination;
    pub use taitan_orm_trait::paged_info::PagedInfo;
    pub use taitan_orm_trait::paged_list::PagedList;
    pub use taitan_orm_trait::build_paged_list;
}

pub mod traits {
    pub use taitan_orm_trait::{CountSql, Entity, Location, LocationExpr, Mutation, OrderBy, Schema, SelectedEntity, Selection, Unique};
    pub use taitan_orm_trait::validate_order_by;
    pub use taitan_orm_trait::ParsedTemplateSql;
    pub use taitan_orm_trait::TemplateSqlValue;
    pub use taitan_orm_trait::TemplateRecord;
    pub use taitan_orm_trait::LocationMode;
    pub use taitan_orm_trait::NotValidCmpError;
    pub use taitan_orm_trait::CmpOperator;
}