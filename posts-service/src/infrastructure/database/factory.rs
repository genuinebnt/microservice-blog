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
    pub async fn from_connection(conn: DatabaseConn) -> Result<RepoProvider> {
        let posts_repo: DynPostRepository = match conn {
            DatabaseConn::SeaOrm(seaorm_conn) => {
                Migrator::up(&seaorm_conn, None).await.unwrap();
                let repo = super::seaorm::SeaOrmPostRepository::new(seaorm_conn);
                Arc::new(repo) as DynPostRepository
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
