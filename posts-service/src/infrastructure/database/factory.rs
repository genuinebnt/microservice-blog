use std::sync::Arc;

use common::config::DatabaseSettings;
use common::error::Result;

use crate::domain::repository::DynPostRepository;
use crate::infrastructure::database::postgres::build_postgres_url;

pub struct RepoProvider {
    pub posts: DynPostRepository,
}

impl RepoProvider {
    pub async fn build_repo_provider(settings: &DatabaseSettings) -> Result<RepoProvider> {
        let url = build_postgres_url(settings).await?;
        let posts_repo: DynPostRepository = match settings.backend.as_str() {
            "seaorm" => {
                let seaorm_conn = sea_orm::Database::connect(url).await?;
                let repo = super::seaorm::SeaOrmPostRepository::new(seaorm_conn);
                Arc::new(repo) as DynPostRepository
            }
            _ => {
                return Err(common::error::AppError::InvalidConfiguration);
            }
        };
        Ok(RepoProvider { posts: posts_repo })
    }
}
