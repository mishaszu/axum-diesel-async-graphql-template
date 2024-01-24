use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::debug;

use crate::api::auth_middleware::CtxExtError;
use crate::db;

use super::crypt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    LoginFailPwdNotMatching,

    CtxExt(CtxExtError),

    Model(db::Error),
    Crypt(crypt::Error),

    SerdeJson(String),

    BadRequest(String),
    BadRequestReturn(String),

    BadUuidFormat,
    AuthError,
}

impl From<db::Error> for Error {
    fn from(val: db::Error) -> Self {
        Error::Model(val)
    }
}

impl From<crypt::Error> for Error {
    fn from(val: crypt::Error) -> Self {
        Self::Crypt(val)
    }
}

impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        Self::SerdeJson(val.to_string())
    }
}

impl From<uuid::Error> for Error {
    fn from(_val: uuid::Error) -> Self {
        Self::BadUuidFormat
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        let mut response = match self {
            Error::CtxExt(_) => StatusCode::UNAUTHORIZED.into_response(),
            Error::BadRequestReturn(ref e) => (StatusCode::BAD_REQUEST, e.clone()).into_response(),
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        };

        response.extensions_mut().insert(self);

        response
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
