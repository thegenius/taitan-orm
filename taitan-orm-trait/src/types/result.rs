use super::error::TaitanOrmError;

pub type Result<T> = std::result::Result<T, TaitanOrmError>;
pub type Optional<T> = Option<Option<T>>;