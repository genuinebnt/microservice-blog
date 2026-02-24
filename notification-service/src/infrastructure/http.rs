use std::sync::Arc;

use axum::Router;

use crate::presentation::{
    routes::{health::health_check_router, notification::notifications_router},
    state::AppState,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(health_check_router(state.clone()))
        .nest("/notifications", notifications_router(state.clone()))
}
