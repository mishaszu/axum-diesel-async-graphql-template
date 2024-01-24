use axum::extract::State;
use axum::routing::{post, put};
use axum::{Form, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;

use super::error::{Error, Result};
use crate::db::ModelManager;
use crate::domain::user::db_model::UserBmc;
use crate::web::cookie::{remove_token_cookie, set_token_cookie};

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/login", post(login_handler))
        .route("/api/logout", put(logout_handler))
        .with_state(mm)
}

async fn login_handler(
    State(mm): State<ModelManager>,
    cookies: Cookies,
    Form(payload): Form<UserLogin>,
) -> Result<Json<Value>> {
    let user = UserBmc::get_by_email(&mm, &payload.email)?;
    let is_valid = user.validate_pwd(&payload.password);
    match is_valid {
        Ok(_) => {
            let token = user.into_token()?;
            set_token_cookie(&cookies, token);
            Ok(Json(json!({ "success": true })))
        }
        Err(_) => Err(Error::LoginFailPwdNotMatching),
    }
}

async fn logout_handler(cookies: Cookies) -> Result<Json<Value>> {
    remove_token_cookie(&cookies);
    Ok(Json(json!({ "success": true })))
}

#[derive(Deserialize)]
struct UserLogin {
    email: String,
    password: String,
}
