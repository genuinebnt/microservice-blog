use async_trait::async_trait;
use std::time::Duration;

use crate::cache::traits::Cache;

pub struct TieredCache {
    l1: Box<dyn Cache>,
    l2: Option<Box<dyn Cache>>,
    l1_ttl: Duration,
}

impl std::fmt::Debug for TieredCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TieredCache")
            .field("has_l2", &self.l2.is_some())
            .field("l1_ttl", &self.l1_ttl)
            .finish()
    }
}

impl TieredCache {
    pub fn new(l1: impl Cache, ttl: Duration) -> Self {
        Self {
            l1: Box::new(l1),
            l2: None,
            l1_ttl: ttl,
        }
    }

    pub fn add_l2(mut self, l2: impl Cache) -> Self {
        self.l2 = Some(Box::new(l2));
        self
    }
}

#[async_trait]
impl Cache for TieredCache {
    async fn get_str(&self, key: &str) -> Option<String> {
        if let Some(value) = self.l1.get_str(key).await {
            tracing::debug!(key = %key, "Cache L1 hit");
            return Some(value);
        }

        if let Some(ref l2) = self.l2
            && let Some(value) = l2.get_str(key).await
        {
            tracing::debug!(key = %key, "Cache L2 hit, promoting to L1");
            self.l1.set_str(key, &value, self.l1_ttl).await;
            return Some(value);
        }
        tracing::debug!(key = %key, "Cache miss");
        None
    }

    async fn set_str(&self, key: &str, value: &str, ttl: Duration) {
        self.l1.set_str(key, value, self.l1_ttl).await;

        if let Some(ref l2) = self.l2 {
            l2.set_str(key, value, ttl).await;
        }
    }

    async fn delete_str(&self, key: &str) {
        self.l1.delete_str(key).await;
        if let Some(ref l2) = self.l2 {
            l2.delete_str(key).await;
        }
    }

    async fn exists_str(&self, key: &str) -> bool {
        if self.l1.exists_str(key).await {
            return true;
        }
        if let Some(ref l2) = self.l2 {
            return l2.exists_str(key).await;
        }
        false
    }
}
