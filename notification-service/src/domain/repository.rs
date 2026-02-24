use async_trait::async_trait;
use common::{error::Result, pagination::Pagination};
use std::{fmt::Debug, sync::Arc};
use uuid::Uuid;

use crate::domain::entities::notification::Notification;

#[async_trait]
pub trait NotificationRepository: Send + Sync + Debug {
    async fn create_notification(&self, notification: Notification) -> Result<Notification>;
    async fn get_notification_by_id(&self, id: Uuid) -> Result<Option<Notification>>;
    async fn list_notifications_for_user(
        &self,
        user_id: Uuid,
        pagination: &Pagination,
    ) -> Result<(Vec<Notification>, u64)>;
    async fn mark_as_read(&self, id: Uuid) -> Result<()>;
    async fn delete_notification(&self, id: Uuid) -> Result<()>;
}

pub type DynNotificationRepository = Arc<dyn NotificationRepository>;
