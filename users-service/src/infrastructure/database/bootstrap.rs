use common::config::{DatabaseSettings, DbBackend, DbEngine};
use common::error::Result;
use sea_orm::Database;

use crate::infrastructure::database::types::DatabaseConn;
use crate::infrastructure::database::url::build_db_url;

pub async fn bootstrap(cfg: &DatabaseSettings) -> Result<DatabaseConn> {
    let db_url = build_db_url(cfg).await?;

    match cfg.backend {
        DbBackend::Seaorm => {
            let db = Database::connect(db_url).await?;
            Ok(DatabaseConn::SeaOrm(db))
        }
        DbBackend::Sqlx => match cfg.engine {
            DbEngine::Postgres => {
                let db = sqlx::postgres::PgPool::connect(&db_url).await?;
                Ok(DatabaseConn::Sqlx(db))
            }
        },
    }
}
