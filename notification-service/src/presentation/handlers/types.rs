use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::notification::Notification;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateNotificationRequest {
    pub user_id: Uuid,
    pub kind: String,
    pub title: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NotificationResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub kind: String,
    pub title: String,
    pub message: String,
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
}

impl From<Notification> for NotificationResponse {
    fn from(n: Notification) -> Self {
        Self {
            id: n.id,
            user_id: n.user_id,
            kind: n.kind,
            title: n.title,
            message: n.message,
            is_read: n.is_read,
            created_at: n.created_at.into(),
        }
    }
}
