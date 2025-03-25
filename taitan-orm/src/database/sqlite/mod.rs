mod args_extractor;
mod builder;
mod database;
mod transaction;
mod generator;
pub mod api;

pub use builder::SqliteBuilder;
pub use builder::SqliteLocalConfig;
pub use database::SqliteDatabase;
pub use transaction::SqliteTransaction;

pub mod prelude {
    pub use crate::database::sqlite::builder::SqliteBuilder;
    pub use crate::database::sqlite::builder::SqliteLocalConfig;
    pub use crate::database::sqlite::database::SqliteDatabase;
    pub use crate::database::sqlite::transaction::SqliteTransaction;
    pub use crate::database::sqlite::api::ReaderApiNew;
    pub use crate::database::sqlite::api::ReaderMutApiNew;
    pub use crate::database::sqlite::api::WriterApiNew;
    pub use crate::database::sqlite::api::WriterMutApiNew;
    pub use crate::database::sqlite::api::TemplateApiNew;
    pub use crate::database::sqlite::api::TemplateMutApiNew;
}

