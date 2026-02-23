use std::time::Duration;

use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};

#[async_trait]
pub trait Cache: Send + Sync + 'static {
    async fn get_str(&self, key: &str) -> Option<String>;
    async fn set_str(&self, key: &str, value: &str, ttl: Duration);
    async fn delete_str(&self, key: &str);
    async fn exists_str(&self, key: &str) -> bool;
}

#[async_trait]
pub trait CacheExt: Cache {
    async fn get<K, V>(&self, key: K) -> Option<V>
    where
        K: AsRef<str> + Send,
        V: DeserializeOwned + Send,
    {
        self.get_str(key.as_ref())
            .await
            .and_then(|json| serde_json::from_str(&json).ok())
    }

    async fn set<K, V>(&self, key: K, value: V, ttl: Duration)
    where
        K: AsRef<str> + Send,
        V: Serialize + Send,
    {
        if let Ok(json) = serde_json::to_string(&value) {
            self.set_str(key.as_ref(), &json, ttl).await;
        }
    }

    async fn delete<K>(&self, key: K)
    where
        K: AsRef<str> + Send,
    {
        self.delete_str(key.as_ref()).await
    }

    async fn exists<K>(&self, key: K) -> bool
    where
        K: AsRef<str> + Send,
    {
        self.exists_str(key.as_ref()).await
    }
}

impl<C: Cache + ?Sized> CacheExt for C {}
