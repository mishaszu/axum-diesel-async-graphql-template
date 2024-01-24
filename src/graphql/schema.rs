use async_graphql::{Context, EmptySubscription, MergedObject, Schema};

use super::error::Error;
use crate::{db::ModelManager, web::ctx::Ctx};

#[derive(Default)]
struct DefaultQuery;

#[async_graphql::Object]
impl DefaultQuery {
    async fn hello(&self, ctx: &Context<'_>) -> async_graphql::Result<String> {
        let user_ctx = ctx.data_opt::<Ctx>();
        match user_ctx {
            Some(ctx) => Ok(format!("User logged in: {}", ctx.user_email)),
            None => Err(Error::AuthError.into()),
        }
    }
}

#[derive(Default)]
struct DefaultMutation;

#[async_graphql::Object]
impl DefaultMutation {
    async fn hello(&self, input: String) -> String {
        format!("{} world!", input)
    }
}

#[derive(MergedObject, Default)]
pub struct QueryRoot(DefaultQuery);

#[derive(MergedObject, Default)]
pub struct MutationRoot(DefaultMutation);

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema(mm: ModelManager) -> AppSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        EmptySubscription::default(),
    )
    .data(mm)
    .finish()
}
