use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaitanOrmError {
    #[error("databse init fail with args: `{0}`")]
    DatabaseInitFail(String),

    #[error(transparent)]
    BoxDynError(#[from] Box<dyn std::error::Error + 'static + Send + Sync>),

    // #[error(transparent)]
    // SqlxError(#[from] sqlx::error::DatabaseError),
}
