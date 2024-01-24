use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Serialize, Clone)]
pub enum Error {
    TokenInvalidSecret,
    TokenParseFailed,
    TokenExpired,

    FailToB64uDecode,

    FailToCreateArgonEncoder,

    PwdNotMatching,
    PwdEncryptionFailed,
    PwdHashBadFormat,
}

impl From<argon2::Error> for Error {
    fn from(_value: argon2::Error) -> Self {
        Self::PwdEncryptionFailed
    }
}

impl From<argon2::password_hash::Error> for Error {
    fn from(_value: argon2::password_hash::Error) -> Self {
        Self::PwdEncryptionFailed
    }
}
