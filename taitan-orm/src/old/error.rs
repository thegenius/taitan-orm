use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaitanOrmError {
    #[error("databse init fail with args: `{0}`")]
    DatabaseInitFail(String),

    #[error("sqlx error: `{0}`")]
    SqlxError(String),

    #[error(transparent)]
    NotValidCmpErr(#[from] taitan_orm_trait::NotValidCmpError),

    #[error(transparent)]
    NotValidConditionError(#[from] taitan_orm_trait::NotValidConditionError),

    #[error(transparent)]
    NotValidOrderByError(#[from] taitan_orm_trait::NotValidOrderByError),

    #[error(transparent)]
    NotImplementTrait(#[from] taitan_orm_trait::NotImplementError),

    #[error("box dyn error: `{0}`")]
    BoxDynError(String),

    #[error("execute template paged search must has count sql")]
    TemplatePagedNotHasCountSql,

    #[error("execute template paged search must has page field")]
    TemplatePageFieldNotFound,

    #[error("deserialize entity from row  error")]
    FromRowToEntityError,

    #[error("invalid order by fields")]
    OrderByFieldsError,

    #[error("method not implement error: {0}")]
    NotImplement(String),

    #[error("paged template sql can't execute with no count sql")]
    PagedTemplateHasNoCountSql,

    #[error("dynamic request parse error: {0}")]
    DynamicRequestParseError(String),
}

impl From<Box<dyn std::error::Error + Send + Sync>> for TaitanOrmError {
    fn from(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        TaitanOrmError::BoxDynError(e.to_string())
    }
}
impl From<Box<dyn std::error::Error>> for TaitanOrmError {
    fn from(value: Box<dyn Error>) -> Self {
        TaitanOrmError::BoxDynError(value.to_string())
    }
}

impl From<sqlx::Error> for TaitanOrmError {
    fn from(value: sqlx::Error) -> Self {
        TaitanOrmError::SqlxError(value.to_string())
    }
}

