use std::sync::Arc;

use common::error::Result;
use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;

use crate::domain::repository::DynNotificationRepository;

#[derive(Debug, Clone)]
pub struct RepoProvider {
    pub notifications: DynNotificationRepository,
}

impl RepoProvider {
    pub async fn from_connection(conn: DatabaseConnection) -> Result<RepoProvider> {
        Migrator::up(&conn, None).await.unwrap();
        let notifications_repo: DynNotificationRepository =
            Arc::new(super::seaorm::SeaOrmNotificationRepository::new(conn));

        Ok(RepoProvider {
            notifications: notifications_repo,
        })
    }
}
