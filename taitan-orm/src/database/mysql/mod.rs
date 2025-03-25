mod builder;
mod database;
mod transaction;

pub mod api;
pub(crate) mod generator;

pub use builder::MySqlBuilder;
pub use database::MySqlDatabase;
pub use transaction::MySqlTransaction;

pub mod prelude {
    pub use crate::database::mysql::api::ReaderApiNew;
    pub use crate::database::mysql::api::ReaderMutApiNew;
    pub use crate::database::mysql::api::TemplateApiNew;
    pub use crate::database::mysql::api::TemplateMutApiNew;
    pub use crate::database::mysql::api::WriterApiNew;
    pub use crate::database::mysql::api::WriterMutApiNew;
    pub use crate::database::mysql::builder::MySqlBuilder;
    pub use crate::database::mysql::database::MySqlDatabase;
    pub use crate::database::mysql::transaction::MySqlTransaction;
}
