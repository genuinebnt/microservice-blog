use std::{sync::Arc, time::Instant};

use async_trait::async_trait;
use common::error::Result;

use crate::domain::{Post, PostId, PostRepository};

#[derive(Debug)]
pub struct LoggedPostRepository {
    inner: Arc<dyn PostRepository>,
}

impl LoggedPostRepository {
    pub fn new(inner: Arc<dyn PostRepository>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl PostRepository for LoggedPostRepository {
    async fn create(&self, post: Post) -> Result<()> {
        let start = Instant::now();
        tracing::info!(title = %post.title, "Creating post");

        let result = self.inner.create(post).await;

        match &result {
            Ok(_) => tracing::info!(elapsed_ms = %start.elapsed().as_millis(), "Post created"),
            Err(e) => tracing::error!(error = %e, "Failed to create post"),
        }
        result
    }

    async fn get(&self, id: PostId) -> Result<Option<Post>> {
        let start = Instant::now();
        let id_str = id.to_string();
        let result = self.inner.get(id).await;

        match &result {
            Ok(Some(p)) => {
                tracing::info!(post_id = %id_str, title = %p.title, elapsed_ms = %start.elapsed().as_millis(), "Post found")
            }
            Ok(None) => tracing::warn!(post_id = %id_str, "Post not found"),
            Err(e) => tracing::error!(post_id = %id_str, error = %e, "Failed to get post"),
        }
        result
    }

    async fn update(&self, post: Post) -> Result<()> {
        let start = Instant::now();
        tracing::info!(post_id = %post.id, title = %post.title, "Updating post");

        let result = self.inner.update(post).await;

        match &result {
            Ok(_) => tracing::info!(elapsed_ms = %start.elapsed().as_millis(), "Post updated"),
            Err(e) => tracing::error!(error = %e, "Failed to update post"),
        }
        result
    }

    async fn delete(&self, id: PostId) -> Result<()> {
        let start = Instant::now();
        let id_str = id.to_string();
        tracing::info!(post_id = %id_str, "Deleting post");

        let result = self.inner.delete(id).await;

        match &result {
            Ok(_) => tracing::info!(elapsed_ms = %start.elapsed().as_millis(), "Post deleted"),
            Err(e) => tracing::error!(post_id = %id_str, error = %e, "Failed to delete post"),
        }
        result
    }

    async fn list(&self) -> Result<Option<Vec<Post>>> {
        let start = Instant::now();
        let result = self.inner.list().await;

        match &result {
            Ok(Some(posts)) => {
                tracing::info!(count = posts.len(), elapsed_ms = %start.elapsed().as_millis(), "Posts listed")
            }
            Ok(None) => tracing::info!("No posts found"),
            Err(e) => tracing::error!(error = %e, "Failed to list posts"),
        }
        result
    }
}
