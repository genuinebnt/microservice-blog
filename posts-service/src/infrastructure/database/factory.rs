use common::cache::{LocalCache, RedisCache, TieredCache};
use common::config::CacheSettings;
use common::error::Result;
use migration::{Migrator, MigratorTrait};
use std::sync::Arc;

use crate::domain::repository::DynPostRepository;
use sea_orm::DatabaseConnection;

#[derive(Debug, Clone)]
pub struct RepoProvider {
    pub posts: DynPostRepository,
}

impl RepoProvider {
    pub async fn from_connection(
        conn: DatabaseConnection,
        cache_config: &CacheSettings,
    ) -> Result<RepoProvider> {
        Migrator::up(&conn, None).await.unwrap();

        let db_repo: DynPostRepository = Arc::new(super::seaorm::SeaOrmPostRepository::new(conn));

        let local_cache = LocalCache::new(cache_config);

        let cached: DynPostRepository = if let Some(ref redis_cfg) = cache_config.redis {
            let redis_cache = RedisCache::new(&redis_cfg.url())
                .map_err(|e| common::error::AppError::InvalidConfiguration(e.to_string()))?;
            let tiered = TieredCache::new(local_cache, cache_config.ttl()).add_l2(redis_cache);
            Arc::new(super::cache::CachedPostRepository::new(
                db_repo,
                Arc::new(tiered),
                cache_config.ttl(),
            )) as DynPostRepository
        } else {
            Arc::new(super::cache::CachedPostRepository::new(
                db_repo,
                Arc::new(local_cache),
                cache_config.ttl(),
            )) as DynPostRepository
        };

        let posts_repo = Arc::new(super::logger::LoggedPostRepository::new(cached));
        Ok(RepoProvider { posts: posts_repo })
    }
}
