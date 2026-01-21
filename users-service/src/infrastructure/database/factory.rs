use std::sync::Arc;

use common::error::Result;

use crate::{domain::repository::DynUserRepository, infrastructure::database::types::DatabaseConn};

#[derive(Debug, Clone)]
pub struct RepoProvider {
    pub users: DynUserRepository,
}

impl RepoProvider {
    pub async fn from_connection(conn: DatabaseConn) -> Result<RepoProvider> {
        let users_repo: DynUserRepository = match conn {
            DatabaseConn::SeaOrm(seaorm_conn) => {
                let repo = super::seaorm::SeaOrmUserRepository::new(seaorm_conn);
                Arc::new(repo) as DynUserRepository
            }
            _ => {
                return Err(common::error::AppError::InvalidConfiguration(
                    "Invalid database backend".to_string(),
                ));
            }
        };

        Ok(RepoProvider { users: users_repo })
    }
}
