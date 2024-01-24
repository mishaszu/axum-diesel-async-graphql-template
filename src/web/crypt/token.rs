use chrono::Local;
use hmac::{Hmac, Mac};
use jwt::{Error as JWTError, Header, SignWithKey, Token as JWTToken, VerifyWithKey};
use sha2::Sha512;
use std::{collections::BTreeMap, str::FromStr};
use uuid::Uuid;

use crate::config;

use super::error::{Error, Result};

pub struct Token(String);

pub struct TokenClaims {
    pub id: Uuid,
    pub user_id: Uuid,
    pub email: String,
    pub exp: String,
}

impl Token {
    pub fn new(user_id: &Uuid, email: &str) -> Result<Token> {
        let key: Hmac<Sha512> = Hmac::new_from_slice(&config::config().TOKEN_SECRET)
            .map_err(|_| Error::TokenInvalidSecret)?;
        let mut claims = BTreeMap::new();
        claims.insert("id", Uuid::new_v4().to_string());
        claims.insert("user_id", user_id.to_string());
        claims.insert("email", email.to_string());
        let exp = Local::now() + chrono::Duration::seconds(config::config().TOKEN_DURATION);
        claims.insert("exp", exp.to_rfc3339());

        let token_str = claims
            .sign_with_key(&key)
            .map_err(|_| Error::TokenInvalidSecret)?;

        Ok(Token(token_str))
    }

    pub fn parse_claims(&self) -> Result<TokenClaims> {
        let key: Hmac<Sha512> = Hmac::new_from_slice(&config::config().TOKEN_SECRET)
            .map_err(|_| Error::TokenInvalidSecret)?;
        let claims: BTreeMap<String, String> = self
            .0
            .verify_with_key(&key)
            .map_err(|_| Error::TokenParseFailed)?;
        let id = Uuid::parse_str(claims.get("id").ok_or(Error::TokenParseFailed)?)
            .map_err(|_| Error::TokenParseFailed)?;
        let user_id = Uuid::parse_str(claims.get("user_id").ok_or(Error::TokenParseFailed)?)
            .map_err(|_| Error::TokenParseFailed)?;
        let email = claims
            .get("email")
            .ok_or(Error::TokenParseFailed)?
            .to_string();
        let exp = claims
            .get("exp")
            .ok_or(Error::TokenParseFailed)?
            .to_string();
        Ok(TokenClaims {
            id,
            user_id,
            email,
            exp,
        })
    }

    #[allow(dead_code)]
    fn update_exp(old_token: Token) -> Result<Token> {
        let key: Hmac<Sha512> = Hmac::new_from_slice(&config::config().TOKEN_SECRET)
            .map_err(|_| Error::TokenInvalidSecret)?;
        let mut claims: BTreeMap<String, String> = old_token
            .0
            .verify_with_key(&key)
            .map_err(|_| Error::TokenParseFailed)?;
        let exp = claims
            .get("exp")
            .ok_or(Error::TokenParseFailed)?
            .to_string();
        let exp_time =
            chrono::DateTime::parse_from_rfc3339(&exp).map_err(|_| Error::TokenParseFailed)?;
        let time = exp_time + chrono::Duration::seconds(config::config().TOKEN_DURATION);
        claims.insert("exp".to_string(), time.to_rfc3339());

        let token_str = claims
            .sign_with_key(&key)
            .map_err(|_| Error::TokenInvalidSecret)?;

        Ok(Token(token_str))
    }

    pub fn validate(&self) -> Result<()> {
        let key: Hmac<Sha512> = Hmac::new_from_slice(&config::config().TOKEN_SECRET)
            .map_err(|_| Error::TokenInvalidSecret)?;
        let token: std::result::Result<JWTToken<Header, BTreeMap<String, String>, _>, JWTError> =
            VerifyWithKey::verify_with_key(self.0.as_str(), &key);
        match token {
            Ok(token) => {
                let exp = token
                    .claims()
                    .get("exp")
                    .ok_or(Error::TokenParseFailed)?
                    .to_string();
                let exp_time = chrono::DateTime::parse_from_rfc3339(&exp)
                    .map_err(|_| Error::TokenParseFailed)?;
                match chrono::Local::now() < exp_time {
                    true => Ok(()),
                    false => Err(Error::TokenExpired),
                }
            }
            Err(_) => Err(Error::TokenParseFailed),
        }
    }

    pub fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Token(s.to_string()))
    }
}
