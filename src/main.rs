use axum::{middleware, routing::get, Router};
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod api;
pub mod config;
pub mod db;
pub mod domain;
pub mod graphql;
pub mod schema;
pub mod web;

use crate::api::auth_middleware::mw_ctx_resolve;
use crate::api::login_handler::routes as routes_login;
use web::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mm = db::ModelManager::new().await?;
    mm.run_migration();

    let routes_all = Router::new()
        .merge(routes_login(mm.clone()))
        .nest("/graphql", graphql::handler::routes(mm.clone()))
        .nest("/api", api::api_handler::routes(mm.clone()))
        .route("/test", get(hello_world))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
        .layer(CookieManagerLayer::new());

    info!("{:<12} - 3000", "LISTENING");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();

    Ok(())
}

async fn hello_world() -> &'static str {
    info!("{:<12} - test route", "GET");
    "Hello world from axum server!"
}
