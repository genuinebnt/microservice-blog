use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;
use sea_orm::{
    ActiveModelBehavior, DeriveEntityModel, DeriveRelation, EnumIter, prelude::DateTimeWithTimeZone,
};
use sea_orm::{DatabaseTransaction, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::pubsub::PubSubPublisher;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "outbox")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    #[sea_orm(column_type = "Text")]
    pub event_type: String,
    #[sea_orm(column_type = "JsonBinary")]
    pub payload: serde_json::Value,
    pub created_at: DateTimeWithTimeZone,
    pub send_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub type OutBoxEvent = Model;

pub async fn insert_outbox_event(
    tx: &DatabaseTransaction,
    aggregate_type: &str,
    aggregate_id: Uuid,
    event_type: &str,
    payload: serde_json::Value,
) -> crate::error::Result<OutBoxEvent> {
    let event = ActiveModel {
        id: Set(Uuid::new_v4()),
        aggregate_type: Set(aggregate_type.to_string()),
        aggregate_id: Set(aggregate_id),
        event_type: Set(event_type.to_string()),
        payload: Set(payload),
        created_at: Set(Utc::now().into()),
        send_at: Set(None),
    };
    Ok(event.insert(tx).await?)
}

pub struct OutboxPoller {
    conn: DatabaseConnection,
    publisher: PubSubPublisher,
    poll_interval: std::time::Duration,
    batch_size: u64,
}

impl OutboxPoller {
    pub fn new(
        conn: DatabaseConnection,
        publisher: PubSubPublisher,
        poll_interval: std::time::Duration,
        batch_size: u64,
    ) -> Self {
        Self {
            conn,
            publisher,
            poll_interval,
            batch_size,
        }
    }

    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            tracing::info!("Outbox poller started");
            loop {
                if let Err(e) = self.poll_and_publish().await {
                    tracing::error!("Outbox poller error: {:?}", e);
                }
                tokio::time::sleep(self.poll_interval).await;
            }
        })
    }

    async fn poll_and_publish(&self) -> crate::error::Result<()> {
        let events: Vec<OutBoxEvent> = Entity::find()
            .filter(Column::SendAt.is_null())
            .order_by_asc(Column::CreatedAt)
            .limit(self.batch_size)
            .all(&self.conn)
            .await?;

        if events.is_empty() {
            return Ok(());
        }

        let count = events.len();
        tracing::debug!("Outbox poller found {} unsend events", count);

        for event in events {
            let message = serde_json::to_string(&event)?;
            self.publisher.publish(message).await?;

            let mut active: ActiveModel = event.into();
            active.send_at = Set(Some(Utc::now().into()));
            active.update(&self.conn).await?;

            tracing::debug!("Published outbox event");
        }

        tracing::info!("Outbox poller published {} events", count);
        Ok(())
    }
}
