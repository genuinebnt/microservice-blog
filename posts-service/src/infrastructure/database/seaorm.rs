use crate::domain::{
    entities::{self, post::Post},
    repository::PostRepository,
};
use async_trait::async_trait;
use common::error::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, ModelTrait};

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
    #[tracing::instrument(skip(self))]
    async fn create(&self, post: Post) -> Result<()> {
        tracing::info!("Creating post: {}", post.title);

        let active_model = entities::post::ActiveModel::from(post);
        active_model.insert(&self.conn).await?;

        tracing::info!("Post created successfully");
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn get(&self, id: uuid::Uuid) -> Result<Option<Post>> {
        tracing::info!("Getting post: {}", id);

        let post = entities::post::Entity::find_by_id(id)
            .one(&self.conn)
            .await?;

        if post.is_some() {
            tracing::info!("Post found: {}", post.as_ref().unwrap().title);
        } else {
            tracing::info!("Post not found");
        }
        let post = entities::post::Entity::find_by_id(id)
            .one(&self.conn)
            .await?;

        Ok(post)
    }

    #[tracing::instrument(skip(self))]
    async fn update(&self, post: Post) -> Result<()> {
        use sea_orm::{EntityTrait, Set, Unchanged};
        tracing::info!("Updating post: {}", post.title);

        let active_model = entities::post::ActiveModel {
            id: Unchanged(post.id),
            title: Set(post.title),
            content: Set(post.content),
            created_at: Unchanged(post.created_at),
            updated_at: Set(chrono::Utc::now().into()),
        };

        entities::post::Entity::update(active_model)
            .exec(&self.conn)
            .await?;

        tracing::info!("Post updated successfully");

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn delete(&self, id: uuid::Uuid) -> Result<()> {
        tracing::info!("Deleting post: {}", id);

        let post = entities::post::Entity::find_by_id(id)
            .one(&self.conn)
            .await?;

        if post.is_none() {
            tracing::info!("Post not found");
            return Err(common::error::AppError::NotFoundError(
                "Post not found".to_string(),
            ));
        }

        post.unwrap().delete(&self.conn).await?;
        tracing::info!("Post deleted successfully");

        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn list(&self) -> Result<Option<Vec<Post>>> {
        tracing::info!("Fetching all posts");

        let posts = entities::post::Entity::find().all(&self.conn).await?;

        tracing::info!("{} posts fetched successfully", posts.len());
        Ok(Some(posts))
    }
}
