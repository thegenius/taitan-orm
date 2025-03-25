
mod builder;
mod database;
mod transaction;
mod generator;
pub mod api;
// mod apis;

pub use builder::SqliteBuilder;
pub use builder::SqliteLocalConfig;
pub use database::SqliteDatabase;
pub use transaction::SqliteTransaction;

pub mod prelude {
    pub use crate::database::sqlite::builder::SqliteBuilder;
    pub use crate::database::sqlite::builder::SqliteLocalConfig;
    pub use crate::database::sqlite::database::SqliteDatabase;
    pub use crate::database::sqlite::transaction::SqliteTransaction;
    pub use crate::database::sqlite::api::ReaderApi;
    pub use crate::database::sqlite::api::ReaderMutApi;
    pub use crate::database::sqlite::api::WriterApi;
    pub use crate::database::sqlite::api::WriterMutApi;
    pub use crate::database::sqlite::api::TemplateApi;
    pub use crate::database::sqlite::api::TemplateMutApi;
}

