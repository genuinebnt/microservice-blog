use async_graphql::SimpleObject;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}
