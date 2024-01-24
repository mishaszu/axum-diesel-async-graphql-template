use diesel::prelude::*;
use diesel::{deserialize::Queryable, ExpressionMethods, RunQueryDsl};
use serde::Deserialize;
use uuid::Uuid;

use crate::db::model_manager::ModelManager;
use crate::web::crypt::pass::validate_pwd;
use crate::web::crypt::token::Token;
use crate::web::crypt::Error as CryptError;
use crate::{db::error::Result, schema::users};

#[derive(Queryable, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub hash: String,
    pub is_admin: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub struct UserBmc;

impl UserBmc {
    pub fn get_by_id(mm: &ModelManager, id: &Uuid) -> Result<User> {
        let mut conn = mm.conn()?;
        users::dsl::users
            .filter(users::dsl::id.eq(id))
            .first::<User>(&mut conn)
            .map_err(Into::into)
    }

    pub fn get_by_email(mm: &ModelManager, email: &str) -> Result<User> {
        let mut conn = mm.conn()?;
        users::dsl::users
            .filter(users::dsl::email.eq(email))
            .first::<User>(&mut conn)
            .map_err(Into::into)
    }

    pub fn is_admin(mm: &ModelManager, id: &Uuid) -> Result<bool> {
        let mut conn = mm.conn()?;
        users::dsl::users
            .filter(users::dsl::id.eq(id))
            .select(users::dsl::is_admin)
            .first::<bool>(&mut conn)
            .map_err(Into::into)
    }

    pub fn list(mm: &ModelManager) -> Result<Vec<User>> {
        let mut conn = mm.conn()?;
        users::dsl::users
            .load::<User>(&mut conn)
            .map_err(Into::into)
    }
}

impl User {
    pub fn validate_pwd(&self, pwd: &str) -> std::result::Result<(), CryptError> {
        validate_pwd(pwd, &self.hash)
    }

    pub fn into_token(self) -> std::result::Result<Token, CryptError> {
        let token = Token::new(&self.id, &self.email)?;
        Ok(token)
    }
}
