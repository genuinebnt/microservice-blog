use std::sync::Arc;

use axum::{Router, routing::get};

use crate::presentation::{handlers::health_check, state::AppState};

pub fn health_check_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/healthz", get(health_check))
        .with_state(state)
}
