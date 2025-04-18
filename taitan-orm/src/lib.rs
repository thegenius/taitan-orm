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



pub mod database;

#[doc(hidden)]
#[macro_use]
mod impl_macros;


mod executors;

pub mod prelude {
    pub use taitan_orm_macro::Parameter;
    pub use taitan_orm_macro::TemplateArg;
    pub use taitan_orm_macro::Template;
    pub use taitan_orm_macro::Mutation;
    pub use taitan_orm_macro::Location;
    pub use taitan_orm_macro::Entity;
    pub use taitan_orm_macro::Schema;
    pub use taitan_orm_macro::Selected;

    pub use crate::executors::SqlGenericExecutor;
    pub use crate::executors::SqlExecutor;
    pub use crate::executors::SqlExecutorMut;

    pub use crate::result::Result as TaitanOrmResult;
    pub use crate::result::CountResult;

    pub use crate::traits::*;
    pub use crate::op::*;
    pub use crate::order::*;
    pub use crate::page::*;
    pub use crate::result::*;
    pub use crate::error::*;
}

pub mod tracing {
    pub use taitan_orm_tracing::*;
}

pub mod macros {
    pub use taitan_orm_macro::Parameter;
    pub use taitan_orm_macro::TemplateArg;
    pub use taitan_orm_macro::Template;
    pub use taitan_orm_macro::Mutation;
    pub use taitan_orm_macro::Location;
    pub use taitan_orm_macro::Entity;
    pub use taitan_orm_macro::Schema;
    pub use taitan_orm_macro::Selected;
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
