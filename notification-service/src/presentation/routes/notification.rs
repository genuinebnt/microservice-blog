use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, get, put},
};

use crate::presentation::{
    handlers::{
        notification::{
            create_notification, delete_notification, get_notification, list_user_notifications,
            mark_notification_read,
        },
        websocket::ws_notifications,
    },
    state::AppState,
};

pub fn notifications_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", axum::routing::post(create_notification))
        .route("/{id}", get(get_notification).delete(delete_notification))
        .route("/{id}/read", put(mark_notification_read))
        .route("/user/{user_id}", get(list_user_notifications))
        .route("/ws/{user_id}", get(ws_notifications))
        .with_state(state)
}
