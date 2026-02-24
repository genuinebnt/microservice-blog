use std::{fmt::Debug, sync::Arc, time::Duration};

use async_trait::async_trait;
use common::cache::CacheExt;
use common::error::Result;

use crate::domain::{Post, PostId, PostRepository};

#[derive(Debug)]
pub struct CachedPostRepository<C: CacheExt + Send + Sync + Debug> {
    inner: Arc<dyn PostRepository>,
    cache: Arc<C>,
    ttl: Duration,
}

impl<C: CacheExt + Send + Sync + Debug> CachedPostRepository<C> {
    pub fn new(inner: Arc<dyn PostRepository>, cache: Arc<C>, ttl: Duration) -> Self {
        Self { inner, cache, ttl }
    }

    fn cache_key(id: &PostId) -> String {
        format!("post:{}", id)
    }
}

#[async_trait]
impl<C: CacheExt + Send + Sync + Debug + 'static> PostRepository for CachedPostRepository<C> {
    async fn create_post(&self, post: Post) -> Result<()> {
        let key = Self::cache_key(&post.id.into());
        self.inner.create_post(post.clone()).await?;
        self.cache.set(&key, &post, self.ttl).await;
        Ok(())
    }

    async fn get_post(&self, id: PostId) -> Result<Option<Post>> {
        let key = Self::cache_key(&id);

        if let Some(post) = self.cache.get::<_, Post>(&key).await {
            return Ok(Some(post));
        }

        let post = self.inner.get_post(id).await?;

        if let Some(ref p) = post {
            self.cache.set(&key, p, self.ttl).await;
        }

        Ok(post)
    }

    async fn update_post(&self, post: Post) -> Result<()> {
        let key = Self::cache_key(&post.id.into());
        self.inner.update_post(post.clone()).await?;
        self.cache.set(&key, &post, self.ttl).await;
        Ok(())
    }

    async fn delete_post(&self, id: PostId) -> Result<()> {
        let key = Self::cache_key(&id);
        self.inner.delete_post(id).await?;
        self.cache.delete(&key).await;
        Ok(())
    }

    async fn list_posts(&self) -> Result<Option<Vec<Post>>> {
        self.inner.list_posts().await
    }
}
