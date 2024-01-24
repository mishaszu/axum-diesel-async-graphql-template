use std::fmt::Display;

use tracing::debug;

use crate::db::error::Error as DbError;

#[derive(Debug)]
pub enum Error {
    ModalManagerNotInContext,
    ClientNotInContext,

    FailedToReadFile,

    DbError(DbError),

    AuthError,
    AccessError(String),

    NotFound(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        debug!("{:<12} - graphql error - {self:?}", "GRAPHQL");
        match self {
            Error::AuthError => write!(f, "User not logged in"),
            Error::DbError(DbError::DbEntityNotFound)
            | Error::AccessError(_)
            | Error::NotFound(_) => write!(f, "Not found"),
            Error::DbError(_)
            | Error::ClientNotInContext
            | Error::FailedToReadFile
            | Error::ModalManagerNotInContext => write!(f, "Internal server error"),
        }
    }
}

impl From<DbError> for Error {
    fn from(e: DbError) -> Self {
        Error::DbError(e)
    }
}
