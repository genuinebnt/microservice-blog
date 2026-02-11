use async_trait::async_trait;
use redis::AsyncCommands;
use std::time::Duration;

use crate::cache::traits::Cache;

#[derive(Debug)]
pub struct RedisCache {
    client: redis::Client,
}

impl RedisCache {
    pub fn new(url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(url)?;
        Ok(Self { client })
    }

    async fn get_conn(&self) -> Option<redis::aio::MultiplexedConnection> {
        self.client.get_multiplexed_async_connection().await.ok()
    }
}

#[async_trait]
impl Cache for RedisCache {
    async fn get_str(&self, key: &str) -> Option<String> {
        let mut conn = self.get_conn().await?;
        conn.get(key).await.ok()
    }

    async fn set_str(&self, key: &str, value: &str, ttl: Duration) {
        if let Some(mut conn) = self.get_conn().await {
            let _: Result<(), _> = conn.set_ex(key, value, ttl.as_secs()).await;
        }
    }

    async fn delete_str(&self, key: &str) {
        if let Some(mut conn) = self.get_conn().await {
            let _: Result<(), _> = conn.del(key).await;
        }
    }

    async fn exists_str(&self, key: &str) -> bool {
        if let Some(mut conn) = self.get_conn().await {
            conn.exists(key).await.unwrap_or(false)
        } else {
            false
        }
    }
}
