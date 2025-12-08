use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::presentation::{
    handlers::posts::{create_post, delete_post, get_post, list_posts, update_post},
    state::AppState,
};

pub fn posts_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(list_posts))
        .route("/", post(create_post))
        .route("/{id}", get(get_post))
        .route("/{id}", put(update_post))
        .route("/{id}", delete(delete_post))
        .with_state(state)
}
