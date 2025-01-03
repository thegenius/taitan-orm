use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaitanOrmError {
    #[error("databse init fail with args: `{0}`")]
    DatabaseInitFail(String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    BoxDynError(#[from] Box<dyn std::error::Error + 'static + Send + Sync>),

    #[error(transparent)]
    NotValidCmpErr(#[from] taitan_orm_trait::NotValidCmpError),

    #[error(transparent)]
    NotValidConditionError(#[from] taitan_orm_trait::NotValidConditionError),

    #[error(transparent)]
    NotValidOrderByError(#[from] taitan_orm_trait::NotValidOrderByError),

    #[error(transparent)]
    NotImplementTrait(#[from] taitan_orm_trait::NotImplementError),


    // #[error(transparent)]
    // BoxDynNoStaticError(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error(transparent)]
    BoxDynError2(#[from] Box<dyn std::error::Error>),

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
