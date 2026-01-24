use async_trait::async_trait;
use common::error::Result;

use super::models::Post;

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Post>>;
}
