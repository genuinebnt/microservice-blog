use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::presentation::{
    handlers::users::{create_user, delete_user, get_user_by_id, list_users, update_user},
    state::AppState,
};

pub fn users_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route(
            "/{id}",
            get(get_user_by_id).put(update_user).delete(delete_user),
        )
        .with_state(state)
}
