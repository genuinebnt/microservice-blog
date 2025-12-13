use common::config::{DatabaseSettings, DbEngine};
use common::error::Result;

pub async fn build_db_url(cfg: &DatabaseSettings) -> Result<String> {
    Ok(match cfg.engine {
        DbEngine::Postgres => {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                cfg.username,
                urlencoding::encode(&cfg.password),
                cfg.hostname,
                cfg.port,
                cfg.database_name
            )
        }
    })
}
