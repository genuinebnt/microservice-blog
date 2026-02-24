use uuid::Uuid;

use async_trait::async_trait;
use common::{error::Result, outbox, pagination::Pagination};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter,
    QueryOrder, TransactionTrait, TryIntoModel,
};

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
    async fn create_user(&self, user: User) -> Result<User> {
        let tx = self.conn.begin().await?;

        let active_model = entities::user::ActiveModel::from(user);
        let user_model = active_model.insert(&tx).await?;

        outbox::insert_outbox_event(
            &tx,
            "user",
            user_model.id,
            "user_registered",
            serde_json::json!({
                "id": user_model.id,
                "username": user_model.username,
                "email": user_model.email,
            }),
        )
        .await?;

        let user: User = user_model.try_into_model()?;

        tx.commit().await?;
        Ok(user)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = entities::user::Entity::find_by_id(id)
            .one(&self.conn)
            .await?;

        Ok(user)
    }

    async fn get_user_by_name(&self, username: String) -> Result<Option<User>> {
        let user = entities::user::Entity::find()
            .filter(entities::user::Column::Username.eq(username.clone()))
            .one(&self.conn)
            .await?;

        Ok(user)
    }

    async fn update_user(&self, user: User) -> Result<()> {
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

        Ok(())
    }

    async fn delete_user(&self, id: Uuid) -> Result<()> {
        let user = entities::user::Entity::find_by_id(id)
            .one(&self.conn)
            .await?;

        if user.is_none() {
            return Err(common::error::AppError::NotFoundError(
                "User not found".into(),
            ));
        }

        user.unwrap().delete(&self.conn).await?;

        Ok(())
    }

    async fn list_users(&self, pagination: &Pagination) -> Result<(Vec<User>, u64)> {
        let paginator = entities::user::Entity::find()
            .order_by_desc(crate::domain::entities::user::Column::CreatedAt)
            .paginate(&self.conn, pagination.page_size);

        let total_users = paginator.num_items().await?;
        let users = paginator.fetch_page(pagination.page - 1).await?;

        Ok((users, total_users))
    }
}
