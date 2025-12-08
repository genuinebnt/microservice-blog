use std::sync::Arc;

use async_trait::async_trait;
use common::error::Result;
use uuid::Uuid;

use super::entities::post::Post;

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create(&self, post: Post) -> Result<()>;
    async fn get(&self, id: Uuid) -> Result<Option<Post>>;
    async fn update(&self, post: Post) -> Result<()>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn list(&self) -> Result<Option<Vec<Post>>>;
}

pub type DynPostRepository = Arc<dyn PostRepository>;
