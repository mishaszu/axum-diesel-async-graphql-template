use crate::{api::auth_middleware::mw_ctx_require, db::ModelManager};
use axum::{extract::DefaultBodyLimit, middleware, response::IntoResponse, routing::get, Router};

#[derive(Clone)]
pub struct ApiState {
    pub mm: ModelManager,
}

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/test", get(test))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_require))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 2))
        .with_state(ApiState { mm })
}

async fn test() -> impl IntoResponse {
    "User logged in".to_string()
}
