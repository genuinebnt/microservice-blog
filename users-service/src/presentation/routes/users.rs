use std::sync::Arc;

use axum::{Router, routing::get};

use crate::presentation::{handlers::users::list_users, state::AppState};

pub fn users_router(state: Arc<AppState>) -> Router {
    Router::new().route("/", get(list_users)).with_state(state)
}
