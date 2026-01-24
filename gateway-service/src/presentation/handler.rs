use crate::presentation::query::QueryRoot;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::Extension;

type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn health_check() -> &'static str {
    "I'm alive!"
}

pub async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
