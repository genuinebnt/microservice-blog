use uuid::Uuid;

use async_trait::async_trait;
use common::{error::Result, pagination::Pagination};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter,
    QueryOrder,
};

use crate::domain::{
    entities::{self, notification::Notification},
    repository::NotificationRepository,
};

#[derive(Debug, Clone)]
pub struct SeaOrmNotificationRepository {
    conn: DatabaseConnection,
}

impl SeaOrmNotificationRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl NotificationRepository for SeaOrmNotificationRepository {
    async fn create_notification(&self, notification: Notification) -> Result<Notification> {
        let active = entities::notification::ActiveModel {
            id: Set(notification.id),
            user_id: Set(notification.user_id),
            kind: Set(notification.kind),
            title: Set(notification.title),
            message: Set(notification.message),
            is_read: Set(notification.is_read),
            created_at: Set(notification.created_at),
        };
        let model = active.insert(&self.conn).await?;
        Ok(model)
    }

    async fn get_notification_by_id(&self, id: Uuid) -> Result<Option<Notification>> {
        let notification = entities::notification::Entity::find_by_id(id)
            .one(&self.conn)
            .await?;
        Ok(notification)
    }

    async fn list_notifications_for_user(
        &self,
        user_id: Uuid,
        pagination: &Pagination,
    ) -> Result<(Vec<Notification>, u64)> {
        let paginator = entities::notification::Entity::find()
            .filter(entities::notification::Column::UserId.eq(user_id))
            .order_by_desc(entities::notification::Column::CreatedAt)
            .paginate(&self.conn, pagination.page_size);

        let total = paginator.num_items().await?;
        let notifications = paginator.fetch_page(pagination.page - 1).await?;

        Ok((notifications, total))
    }

    async fn mark_as_read(&self, id: Uuid) -> Result<()> {
        let notification = entities::notification::Entity::find_by_id(id)
            .one(&self.conn)
            .await?;

        if notification.is_none() {
            return Err(common::error::AppError::NotFoundError(
                "Notification not found".into(),
            ));
        }

        let active = entities::notification::ActiveModel {
            id: Unchanged(id),
            is_read: Set(true),
            ..Default::default()
        };

        active.update(&self.conn).await?;
        Ok(())
    }

    async fn delete_notification(&self, id: Uuid) -> Result<()> {
        let notification = entities::notification::Entity::find_by_id(id)
            .one(&self.conn)
            .await?;

        if notification.is_none() {
            return Err(common::error::AppError::NotFoundError(
                "Notification not found".into(),
            ));
        }

        notification.unwrap().delete(&self.conn).await?;
        Ok(())
    }
}
