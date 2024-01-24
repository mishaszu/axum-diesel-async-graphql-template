use super::{Error, Result};
use crate::config::config;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordVerifier, SaltString},
    Argon2, Params, PasswordHash, PasswordHasher,
};

// TODO: encrypt for new user
#[allow(dead_code)]
pub fn encrypt_pwd(pwd: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::new_with_secret(
        &config().PWD_KEY,
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        Params::default(),
    )?;

    Ok(argon2.hash_password(pwd.as_bytes(), &salt)?.to_string())
}

pub fn validate_pwd(pwd: &str, user_hash: &str) -> Result<()> {
    let argon2 = Argon2::new_with_secret(
        &config().PWD_KEY,
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        Params::default(),
    )
    .map_err(|_| Error::FailToCreateArgonEncoder)?;

    let hash = PasswordHash::new(user_hash).map_err(|_| Error::PwdHashBadFormat)?;

    argon2
        .verify_password(pwd.as_bytes(), &hash)
        .map_err(|_| Error::PwdNotMatching)
}
