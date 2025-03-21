//! TaiTan-ORM has gone to great lengths to enhance the developer experience.
//! The primary guiding principle behind all API designs is to prioritize ease of use and developer satisfaction.
//! This commitment ensures that APIs are intuitive, consistent, and efficient,
//! reducing the learning curve and increasing productivity for developers.

//! First of all, TaiTan-ORM has introduced several concepts designed to simplify and facilitate the learning of its API.
//! 1. Entity
//! An Entity corresponds one database table
//! 2.

#![allow(async_fn_in_trait)]
#![allow(dead_code)]
#![forbid(unsafe_code)]





mod api;

pub mod database;
#[doc(hidden)]
#[macro_use]
mod macros;

pub (crate) mod args_extractor;

mod count;
mod sql_executor;
mod sql_executor_mut;
mod sql_generator;

mod sql_generic_executor;




pub mod prelude {
    pub use taitan_orm_macro::Parameter;
    pub use taitan_orm_macro::TemplateArg;
    pub use taitan_orm_macro::Template;
    pub use taitan_orm_macro::Mutation;
    pub use taitan_orm_macro::Location;
    pub use taitan_orm_macro::Entity;
    pub use taitan_orm_macro::Schema;
    pub use taitan_orm_macro::Selected;


    pub use crate::sql_executor::SqlExecutor;
    pub use crate::sql_executor_mut::SqlExecutorMut;
    pub use crate::sql_generic_executor::SqlGenericExecutor;


    pub use crate::api::reader::ReaderApi;
    pub use crate::api::writer::WriterApi;
    pub use crate::api::template::TemplateApi;

    pub use crate::api::reader_mut::ReaderMutApi;
    pub use crate::api::writer_mut::WriterMutApi;
    pub use crate::api::template_mut::TemplateMutApi;


    pub use crate::result::Result as TaitanOrmResult;
    pub use crate::count::CountResult;

    pub use crate::traits::*;
    pub use crate::op::*;
    pub use crate::order::*;
    pub use crate::page::*;
    pub use crate::result::*;
    pub use crate::error::*;
}



pub mod traits {
    pub use taitan_orm_trait::traits::*;
}


pub mod op {
    pub use taitan_orm_trait::op::*;
}
pub mod order {
    pub use taitan_orm_trait::order::*;
}
pub mod page {
    pub use taitan_orm_trait::page::*;
}

pub mod result {
    pub use taitan_orm_trait::result::*;
}
pub mod error {
    pub use taitan_orm_trait::error::*;
}
