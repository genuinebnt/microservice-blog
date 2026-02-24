use std::fmt::Debug;
use std::sync::Arc;

use async_trait::async_trait;
use common::error::Result;

use crate::domain::PostId;

use super::entities::post::Post;

#[async_trait]
pub trait PostRepository: Send + Sync + Debug {
    async fn create_post(&self, post: Post) -> Result<()>;
    async fn get_post(&self, id: PostId) -> Result<Option<Post>>;
    async fn update_post(&self, post: Post) -> Result<()>;
    async fn delete_post(&self, id: PostId) -> Result<()>;
    async fn list_posts(&self) -> Result<Option<Vec<Post>>>;
}

pub type DynPostRepository = Arc<dyn PostRepository>;
