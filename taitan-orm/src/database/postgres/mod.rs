mod builder;
mod database;

mod transaction;
pub mod api;
pub (crate) mod generator;

pub use database::PostgresDatabase;
pub use transaction::PostgresTransaction;
pub use builder::PostgresBuilder;

pub mod prelude {
    pub use crate::database::postgres::api::ReaderApi;
    pub use crate::database::postgres::api::ReaderMutApi;
    pub use crate::database::postgres::api::TemplateApi;
    pub use crate::database::postgres::api::TemplateMutApi;
    pub use crate::database::postgres::api::WriterApi;
    pub use crate::database::postgres::api::WriterMutApi;
    pub use crate::database::postgres::builder::PostgresBuilder;
    pub use crate::database::postgres::database::PostgresDatabase;
    pub use crate::database::postgres::transaction::PostgresTransaction;
}