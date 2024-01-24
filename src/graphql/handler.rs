use async_graphql_axum::GraphQLSubscription;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::extract::State;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;

use crate::web::ctx::Ctx;

use crate::db::ModelManager;
use crate::web::Result;

use super::schema::{create_schema, AppSchema};

#[derive(Clone)]
pub struct GraphQlState {
    schema: AppSchema,
}

pub fn routes(mm: ModelManager) -> Router {
    let schema = create_schema(mm.clone());
    Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .with_state(GraphQlState { schema })
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(async_graphql::http::playground_source(
        async_graphql::http::GraphQLPlaygroundConfig::new("/graphql")
            .subscription_endpoint("/graphql/ws"),
    ))
}

pub async fn graphql_handler(
    State(graph_ql_state): State<GraphQlState>,
    ctx: Result<Ctx>,
    request: GraphQLRequest,
) -> impl IntoResponse {
    let state = graph_ql_state;
    let builer_schema = match ctx {
        Ok(ctx) => request.0.data(ctx),
        Err(_) => request.0.data(()),
    };
    let builder = state.schema.execute(builer_schema).await;
    let response = GraphQLResponse(builder.into());
    response.into_response()
}
