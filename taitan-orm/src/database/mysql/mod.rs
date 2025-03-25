mod builder;
mod database;
mod transaction;

pub mod api;
pub(crate) mod generator;

pub use builder::MySqlBuilder;
pub use database::MySqlDatabase;
pub use transaction::MySqlTransaction;

pub mod prelude {
    pub use crate::database::mysql::api::ReaderApi;
    pub use crate::database::mysql::api::ReaderMutApi;
    pub use crate::database::mysql::api::TemplateApi;
    pub use crate::database::mysql::api::TemplateMutApi;
    pub use crate::database::mysql::api::WriterApi;
    pub use crate::database::mysql::api::WriterMutApi;
    pub use crate::database::mysql::builder::MySqlBuilder;
    pub use crate::database::mysql::database::MySqlDatabase;
    pub use crate::database::mysql::transaction::MySqlTransaction;
}
