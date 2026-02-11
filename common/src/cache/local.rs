use async_trait::async_trait;
use moka::future::Cache as MokaCache;
use std::time::Duration;

use crate::{cache::Cache, config::CacheSettings};

#[derive(Debug)]
pub struct LocalCache {
    cache: MokaCache<String, String>,
}

impl LocalCache {
    pub fn new(config: &CacheSettings) -> Self {
        let cache = MokaCache::builder()
            .max_capacity(config.max_capacity)
            .time_to_live(config.ttl())
            .time_to_idle(config.tti())
            .build();

        Self { cache }
    }

    pub fn with_ttl(ttl: Duration) -> Self {
        Self::new(&CacheSettings {
            ttl_secs: ttl.as_secs(),
            ..Default::default()
        })
    }

    pub fn entry_count(&self) -> u64 {
        self.cache.entry_count()
    }

    pub async fn clear(&self) {
        self.cache.invalidate_all();
        self.cache.run_pending_tasks().await;
    }
}

#[async_trait]
impl Cache for LocalCache {
    async fn get_str(&self, key: &str) -> Option<String> {
        self.cache.get(key).await
    }

    async fn set_str(&self, key: &str, value: &str, _ttl: Duration) {
        self.cache.insert(key.to_string(), value.to_string()).await;
    }

    async fn delete_str(&self, key: &str) {
        self.cache.remove(key).await;
    }

    async fn exists_str(&self, key: &str) -> bool {
        self.cache.contains_key(key)
    }
}
