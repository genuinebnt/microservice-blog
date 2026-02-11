mod local;
mod redis;
mod tiered;
mod traits;

pub use local::LocalCache;
pub use redis::RedisCache;
pub use tiered::TieredCache;
pub use traits::CacheExt;

pub use traits::Cache;
