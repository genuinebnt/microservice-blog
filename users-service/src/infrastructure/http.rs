use std::sync::Arc;

use axum::Router;

use crate::presentation::{
    routes::{health::health_check_router, users::users_router},
    state::AppState,
};

pub fn create_router(state: AppState) -> Router {
    let state = Arc::new(state);

    Router::new()
        .merge(health_check_router(state.clone()))
        .nest("/users", users_router(state.clone()))
}
