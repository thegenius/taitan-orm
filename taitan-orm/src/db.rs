//! The db module defines the core database execution interface.
//!
//! # Overview
//! The `Executor` trait combines all essential database operation capabilities into a single interface.
//! This provides a unified way to perform CRUD operations, template-based queries, and SQL generation.

use crate::extractor::Extractor;
use crate::prelude::{
    ReaderApi, SqlExecutor, SqlGeneratorContainer, SqlGenericExecutor, TemplateApi, WriterApi,
};

/// The core database execution trait that combines all essential capabilities.
///
/// # Capabilities
/// - `ReaderApi`: Read operations (SELECT)
/// - `WriterApi`: Write operations (INSERT, UPDATE, DELETE)
/// - `TemplateApi`: Template-based query operations
/// - `Extractor`: Result extraction and mapping
/// - `SqlExecutor`: Raw SQL execution
/// - `SqlGenericExecutor`: Generic SQL execution
/// - `SqlGeneratorContainer`: SQL generation utilities
///
/// # Usage
/// Any type that implements all the required traits automatically implements `Executor`
/// through the blanket implementation below.
pub trait Executor:
    ReaderApi
    + WriterApi
    + TemplateApi
    + Extractor
    + SqlExecutor
    + SqlGenericExecutor
    + SqlGeneratorContainer
{
}

/// Blanket implementation of Executor for any type that implements all required traits
impl<T> Executor for T where
    T: ReaderApi
        + WriterApi
        + TemplateApi
        + Extractor
        + SqlExecutor
        + SqlGenericExecutor
        + SqlGeneratorContainer
{
}

// #[derive(Debug, Clone)]
// pub struct DB<
//     T: Executor,
// >(pub T);
//
// impl<T> Deref for DB<T>
// where
//     T: Executor
// {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// impl<T> DerefMut for DB<T>
// where
//     T: Executor,
// {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
