use crate::domain::{
    PostId,
    entities::{self, post::Post},
    repository::PostRepository,
};
use async_trait::async_trait;
use common::{error::Result, outbox};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{Set, Unchanged},
    DatabaseConnection, EntityTrait, ModelTrait, TransactionTrait,
};

#[derive(Debug, Clone)]
pub struct SeaOrmPostRepository {
    conn: DatabaseConnection,
}

impl SeaOrmPostRepository {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl PostRepository for SeaOrmPostRepository {
    async fn create_post(&self, post: Post) -> Result<()> {
        let tx = self.conn.begin().await?;

        let active = entities::post::ActiveModel::from(post);
        let post_model = active.insert(&tx).await?;

        outbox::insert_outbox_event(
            &tx,
            "post",
            post_model.id,
            "post_created",
            serde_json::json!({
                "post_id": post_model.id,
                "author_id": post_model.author_id,
                "title": post_model.title
            }),
        )
        .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn get_post(&self, id: PostId) -> Result<Option<Post>> {
        let post = entities::post::Entity::find_by_id(uuid::Uuid::from(id))
            .one(&self.conn)
            .await?;
        Ok(post)
    }

    async fn update_post(&self, post: Post) -> Result<()> {
        let tx = self.conn.begin().await?;

        let active_model = entities::post::ActiveModel {
            id: Unchanged(post.id),
            title: Set(post.title),
            author_id: Set(post.author_id),
            content: Set(post.content),
            created_at: Unchanged(post.created_at),
            updated_at: Set(chrono::Utc::now().into()),
        };

        let post_model = entities::post::Entity::update(active_model)
            .exec(&tx)
            .await?;

        outbox::insert_outbox_event(
            &tx,
            "post",
            post_model.id,
            "post_updated",
            serde_json::json!({
                "post_id": post_model.id,
                "author_id": post_model.author_id,
                "title": post_model.title
            }),
        )
        .await?;

        tx.commit().await?;

        Ok(())
    }

    async fn delete_post(&self, id: PostId) -> Result<()> {
        let post = entities::post::Entity::find_by_id(uuid::Uuid::from(id))
            .one(&self.conn)
            .await?;

        if post.is_none() {
            return Err(common::error::AppError::NotFoundError(
                "Post not found".to_string(),
            ));
        }

        post.unwrap().delete(&self.conn).await?;
        Ok(())
    }

    async fn list_posts(&self) -> Result<Option<Vec<Post>>> {
        let posts = entities::post::Entity::find().all(&self.conn).await?;
        Ok(Some(posts))
    }
}
