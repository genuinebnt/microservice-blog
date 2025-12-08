use common::config::DatabaseSettings;
use common::error::Result;

pub async fn build_postgres_url(settings: &DatabaseSettings) -> Result<String> {
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        settings.username,
        settings.password,
        settings.hostname,
        settings.port,
        settings.database_name
    );
    Ok(url)
}
