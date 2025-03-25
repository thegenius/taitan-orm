mod builder;
mod database;

mod transaction;
pub mod api;
pub (crate) mod generator;

pub use database::PostgresDatabase;
pub use transaction::PostgresTransaction;
pub use builder::PostgresBuilder;

pub mod prelude {
    pub use crate::database::postgres::api::ReaderApiNew;
    pub use crate::database::postgres::api::ReaderMutApiNew;
    pub use crate::database::postgres::api::TemplateApiNew;
    pub use crate::database::postgres::api::TemplateMutApiNew;
    pub use crate::database::postgres::api::WriterApiNew;
    pub use crate::database::postgres::api::WriterMutApiNew;
    pub use crate::database::postgres::builder::PostgresBuilder;
    pub use crate::database::postgres::database::PostgresDatabase;
    pub use crate::database::postgres::transaction::PostgresTransaction;
}