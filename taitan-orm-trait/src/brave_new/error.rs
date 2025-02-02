use sqlx::Error;
use sqlx::error::BoxDynError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaitanOrmError {


    #[error("database connection permanent error: `{0}`")]
    PermanentConnErr(String),

    #[error("database connection temporary error: `{0}`")]
    TemporaryConnErr(String),

    #[error("sql `{0}` has syntax error")]
    SqlSyntaxErr(String),

    #[error("type `{0}` is not supported")]
    TypeNotSupportedErr(String),

    #[error("row not: `{0}`")]
    ConstraintViolationErr(String),

    #[error("arguments encode error: `{0}`")]
    EncodeError(String),

    #[error("row not found error: `{0}`")]
    RowNotFoundErr(String),

    #[error("row decode error: `{0}`")]
    DecodeError(String),

    #[error("unexpected error `{0}`")]
    UnexpectedError(String),

    // #[error(transparent)]
    // BoxDynError(#[from] Box<dyn std::error::Error + 'static + Send + Sync>),
}

#[inline(always)]
pub fn wrap_encode<T>(result: Result<T, BoxDynError>) -> Result<T, TaitanOrmError> {
    match result {
        Ok(v) => Ok(v),
        Err(e) => Err(TaitanOrmError::EncodeError(e.to_string())),
    }
}

impl From<sqlx::Error> for TaitanOrmError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::Configuration(msg) => {
                TaitanOrmError::PermanentConnErr(msg.to_string().into())
            }
            /// This should indicate there is a programming error in a SQLx driver or there
            /// is something corrupted with the connection to the database itself.
            sqlx::Error::Protocol(e) => {
                TaitanOrmError::PermanentConnErr(e.to_string())
            }

            /// Error occurred while attempting to establish a TLS connection
            sqlx::Error::Tls(e) => TaitanOrmError::PermanentConnErr(e.to_string()),

            /// Error communicating with the database backend
            sqlx::Error::Io(e) => TaitanOrmError::TemporaryConnErr(e.to_string()),
            sqlx::Error::PoolTimedOut => {
                TaitanOrmError::TemporaryConnErr(e.to_string())
            }
            sqlx::Error::PoolClosed => {
                TaitanOrmError::TemporaryConnErr(e.to_string())
            }

            /// database or connection error
            sqlx::Error::Database(e) => {
                let msg = e.message();
                let code = e.code();
                let err_msg = format!("constraint violated: {}", msg);
                TaitanOrmError::ConstraintViolationErr(err_msg)
            },


            sqlx::Error::TypeNotFound{type_name} => TaitanOrmError::TypeNotSupportedErr(type_name.to_string()),

            // encode error
            sqlx::Error::Encode(e) => TaitanOrmError::EncodeError(e.to_string()),

            // decode error
            sqlx::Error::RowNotFound => TaitanOrmError::RowNotFoundErr(e.to_string()),
            sqlx::Error::Decode(e) => TaitanOrmError::DecodeError(e.to_string()),
            sqlx::Error::ColumnDecode { index, source } => TaitanOrmError::DecodeError(format!(
                "Column decode error at column {}: {}",
                index, source
            )),
            sqlx::Error::ColumnIndexOutOfBounds { index, len } => TaitanOrmError::DecodeError(
                format!("Column decode error at column {}: out of {}", index, len),
            ),
            sqlx::Error::ColumnNotFound(index) => {
                TaitanOrmError::DecodeError(format!("Column decode error at column {}", index))
            }


            // unexpected error
            sqlx::Error::WorkerCrashed => TaitanOrmError::UnexpectedError(e.to_string()),
            sqlx::Error::Migrate(e) => TaitanOrmError::UnexpectedError(e.to_string()),
            sqlx::Error::AnyDriverError(e) => TaitanOrmError::UnexpectedError(e.to_string()),
            _ => TaitanOrmError::UnexpectedError(e.to_string()),
        }
    }
}
