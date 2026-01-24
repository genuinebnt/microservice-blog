use std::sync::Arc;

use crate::{presentation::handler, presentation::query::QueryRoot, presentation::state::AppState};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{
    Extension, Router,
    routing::{get, post},
};

pub fn create_router(
    state: AppState,
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
) -> Router {
    let state = Arc::new(state);

    Router::new()
        .route("/health_check", get(handler::health_check))
        .route("/graphql", post(handler::graphql_handler))
        .layer(Extension(schema))
        .with_state(state.clone())
}
