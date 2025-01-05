use crate::error::TaitanOrmError;

pub type Result<T> = std::result::Result<T, TaitanOrmError>;

pub use taitan_orm_trait::Optional;

pub use crate::dto::CountResult;