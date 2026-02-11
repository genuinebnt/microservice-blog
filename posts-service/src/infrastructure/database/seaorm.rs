use crate::domain::{
    PostId,
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
    async fn create(&self, post: Post) -> Result<()> {
        let active_model = entities::post::ActiveModel::from(post);
        active_model.insert(&self.conn).await?;
        Ok(())
    }

    async fn get(&self, id: PostId) -> Result<Option<Post>> {
        let post = entities::post::Entity::find_by_id(uuid::Uuid::from(id))
            .one(&self.conn)
            .await?;
        Ok(post)
    }

    async fn update(&self, post: Post) -> Result<()> {
        use sea_orm::{EntityTrait, Set, Unchanged};

        let active_model = entities::post::ActiveModel {
            id: Unchanged(post.id),
            title: Set(post.title),
            author_id: Set(post.author_id),
            content: Set(post.content),
            created_at: Unchanged(post.created_at),
            updated_at: Set(chrono::Utc::now().into()),
        };

        entities::post::Entity::update(active_model)
            .exec(&self.conn)
            .await?;

        Ok(())
    }

    async fn delete(&self, id: PostId) -> Result<()> {
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

    async fn list(&self) -> Result<Option<Vec<Post>>> {
        let posts = entities::post::Entity::find().all(&self.conn).await?;
        Ok(Some(posts))
    }
}
