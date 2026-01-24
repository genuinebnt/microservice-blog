use async_trait::async_trait;
use common::{error::Result, pagination::Pagination};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter,
    QueryOrder, TryIntoModel,
};
use uuid::Uuid;

use crate::domain::{
    entities::{self, user::User},
    repository::UserRepository,
};

#[derive(Debug, Clone)]
pub struct SeaOrmUserRepository {
    conn: DatabaseConnection,
}

impl SeaOrmUserRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    #[tracing::instrument(skip(self))]
    async fn create_user(&self, user: User) -> Result<User> {
        tracing::info!("Creating User: {}", user.username);

        let user = entities::user::ActiveModel::from(user);
        let user_model = user.insert(&self.conn).await?;
        let user: User = user_model.try_into_model()?;

        tracing::info!("User created successfully");
        Ok(user)
    }

    #[tracing::instrument(skip(self))]
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        tracing::info!("Getting User: {}", id);

        let user = entities::user::Entity::find_by_id(id)
            .one(&self.conn)
            .await?;

        if user.is_some() {
            tracing::info!("User found: {}", user.as_ref().unwrap().username);
            Ok(user)
        } else {
            tracing::info!("User not found");
            Ok(None)
        }
    }

    #[tracing::instrument(skip(self))]
    async fn get_user_by_name(&self, username: String) -> Result<Option<User>> {
        tracing::info!("Getting User by name: {}", username);

        let user = entities::user::Entity::find()
            .filter(entities::user::Column::Username.eq(username.clone()))
            .one(&self.conn)
            .await?;

        if user.is_some() {
            tracing::info!("User found: {}", user.as_ref().unwrap().username);
            Ok(user)
        } else {
            tracing::info!("User not found");
            Ok(None)
        }
    }

    #[tracing::instrument(skip(self))]
    async fn update_user(&self, user: User) -> Result<()> {
        tracing::info!("Updating User: {}", user.username);

        let user = entities::user::ActiveModel {
            id: Unchanged(user.id),
            username: Set(user.username),
            email: Set(user.email),
            created_at: Unchanged(user.created_at),
            updated_at: Set(chrono::Utc::now().into()),
        };

        entities::user::Entity::update(user)
            .exec(&self.conn)
            .await?;

        tracing::info!("User updated successfully");
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn delete_user(&self, id: Uuid) -> Result<()> {
        tracing::info!("Deleting User: {}", id);

        let user = entities::user::Entity::find_by_id(id)
            .one(&self.conn)
            .await?;

        if user.is_none() {
            tracing::info!("User not found");
            return Err(common::error::AppError::NotFoundError(
                "User not found".into(),
            ));
        }

        user.unwrap().delete(&self.conn).await?;
        tracing::info!("User deleted successfully");

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn list_users(&self, pagination: &Pagination) -> Result<(Vec<User>, u64)> {
        tracing::info!(
            "Listing Users for page {}, size {}",
            pagination.page,
            pagination.page_size
        );

        let paginator = entities::user::Entity::find()
            .order_by_desc(crate::domain::entities::user::Column::CreatedAt)
            .paginate(&self.conn, pagination.page_size);

        let total_users = paginator.num_items().await?;
        let users = paginator.fetch_page(pagination.page_size - 1).await?;

        Ok((users, total_users))
    }
}
