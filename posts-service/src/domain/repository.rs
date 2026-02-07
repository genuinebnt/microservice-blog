use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use common::error::Result;
use uuid::Uuid;

use crate::domain::PostId;

use super::entities::post::Post;

#[async_trait]
pub trait PostRepository: Send + Sync + Debug {
    async fn create(&self, post: Post) -> Result<()>;
    async fn get(&self, id: PostId) -> Result<Option<Post>>;
    async fn update(&self, post: Post) -> Result<()>;
    async fn delete(&self, id: PostId) -> Result<()>;
    async fn list(&self) -> Result<Option<Vec<Post>>>;
}

pub type DynPostRepository = Arc<dyn PostRepository>;
