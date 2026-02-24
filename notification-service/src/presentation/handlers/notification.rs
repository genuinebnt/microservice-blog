use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
};
use common::{
    error::{AppError, Result},
    pagination::{PaginatedResponse, Pagination},
};
use uuid::Uuid;

use crate::{
    domain::entities::notification::Notification,
    presentation::{
        handlers::{CreateNotificationRequest, types::NotificationResponse},
        response::ListNotificationResponse,
        state::{AppState, NotificationEvent},
    },
};

pub async fn create_notification(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateNotificationRequest>,
) -> Result<Json<NotificationResponse>> {
    let notification = Notification {
        id: Uuid::new_v4(),
        user_id: payload.user_id,
        kind: payload.kind.clone(),
        title: payload.title.clone(),
        message: payload.message.clone(),
        is_read: false,
        created_at: chrono::Utc::now().into(),
    };

    let notification = state
        .repos
        .notifications
        .create_notification(notification)
        .await?;

    // Broadcast to WebSocket subscribers
    let _ = state.tx.send(NotificationEvent {
        user_id: payload.user_id,
        kind: payload.kind,
        title: payload.title,
        message: payload.message,
    });

    Ok(Json(NotificationResponse::from(notification)))
}

pub async fn get_notification(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<NotificationResponse>> {
    let notification = state.repos.notifications.get_notification_by_id(id).await?;
    match notification {
        Some(n) => Ok(Json(NotificationResponse::from(n))),
        None => Err(AppError::NotFoundError(
            "Notification not found".to_string(),
        )),
    }
}

pub async fn list_user_notifications(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<Uuid>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<ListNotificationResponse>> {
    let pagination = pagination.normalize();
    let (notifications, total) = state
        .repos
        .notifications
        .list_notifications_for_user(user_id, &pagination)
        .await?;

    let count = notifications.len() as u64;
    let paginated_response = PaginatedResponse::new(
        notifications,
        count,
        total,
        pagination.page,
        pagination.page_size,
    );
    Ok(Json(paginated_response))
}

pub async fn mark_notification_read(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>> {
    state.repos.notifications.mark_as_read(id).await?;
    Ok(Json(()))
}

pub async fn delete_notification(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>> {
    state.repos.notifications.delete_notification(id).await?;
    Ok(Json(()))
}
