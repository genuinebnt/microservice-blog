use common::cache::{LocalCache, RedisCache, TieredCache};
use common::config::CacheSettings;
use common::error::Result;
use migration::{Migrator, MigratorTrait};
use std::sync::Arc;

use crate::domain::repository::DynPostRepository;
use crate::infrastructure::database::types::DatabaseConn;

#[derive(Debug, Clone)]
pub struct RepoProvider {
    pub posts: DynPostRepository,
}

impl RepoProvider {
    pub async fn from_connection(
        conn: DatabaseConn,
        cache_config: &CacheSettings,
    ) -> Result<RepoProvider> {
        let posts_repo: DynPostRepository = match conn {
            DatabaseConn::SeaOrm(seaorm_conn) => {
                Migrator::up(&seaorm_conn, None).await.unwrap();

                let db_repo: DynPostRepository =
                    Arc::new(super::seaorm::SeaOrmPostRepository::new(seaorm_conn));

                let local_cache = LocalCache::new(cache_config);

                let cached: DynPostRepository = if let Some(ref redis_cfg) = cache_config.redis {
                    let redis_cache = RedisCache::new(&redis_cfg.url()).map_err(|e| {
                        common::error::AppError::InvalidConfiguration(e.to_string())
                    })?;
                    let tiered =
                        TieredCache::new(local_cache, cache_config.ttl()).add_l2(redis_cache);
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

                Arc::new(super::logger::LoggedPostRepository::new(cached))
            }
            _ => {
                return Err(common::error::AppError::InvalidConfiguration(
                    "Invalid database backend".to_string(),
                ));
            }
        };
        Ok(RepoProvider { posts: posts_repo })
    }
}
