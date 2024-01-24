use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::Display;
use tracing::debug;

use crate::db::Error as DbError;
use crate::web::crypt::Error as CryptError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, Clone)]
pub enum Error {
    #[display(fmt = "Database error: {}", _0)]
    DbError(DbError),

    #[display(fmt = "Authorization error")]
    AuthorizationError(CryptError),

    #[display(fmt = "Login failed")]
    LoginFailPwdNotMatching,
}

impl From<DbError> for Error {
    fn from(e: DbError) -> Self {
        Error::DbError(e)
    }
}

impl From<CryptError> for Error {
    fn from(e: CryptError) -> Self {
        Error::AuthorizationError(e)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");
        let mut response = match self {
            Error::DbError(ref e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }

            Error::LoginFailPwdNotMatching | Error::AuthorizationError(_) => {
                StatusCode::UNAUTHORIZED.into_response()
            }
        };

        response.extensions_mut().insert(self);

        response
    }
}
