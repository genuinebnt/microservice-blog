use async_trait::async_trait;

use crate::config::DatabaseSettings;

use super::error::Result;

#[async_trait]
pub trait Database: Send + Sync {
    type Pool;
    type Options;

    async fn init(config: DatabaseSettings) -> Result<Self::Pool>;
    async fn sync_schema(pool: &Self::Pool) -> Result<()>;
}
