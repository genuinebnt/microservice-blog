use common::config::{DatabaseSettings, PubSubSettings};
use common::error::Result;
use common::outbox::OutboxPoller;
use common::pubsub::PubSubPublisher;
use sea_orm::{Database, DatabaseConnection};

use crate::infrastructure::database::url::build_db_url;

pub async fn bootstrap_db(cfg: &DatabaseSettings) -> Result<DatabaseConnection> {
    let db_url = build_db_url(cfg).await?;
    let db = Database::connect(db_url).await?;
    Ok(db)
}

pub async fn bootstrap_outbox(
    conn: DatabaseConnection,
    pubsub_cfg: &PubSubSettings,
) -> Result<OutboxPoller> {
    let publisher = PubSubPublisher::new(pubsub_cfg).await?;
    Ok(OutboxPoller::new(
        conn,
        publisher,
        std::time::Duration::from_secs(5),
        100,
    ))
}
