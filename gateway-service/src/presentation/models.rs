use async_graphql::SimpleObject;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawPost {
    pub id: Uuid,
    pub title: String,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawUser {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct PostWithAuthor {
    pub id: Uuid,
    pub title: String,
    pub author_id: Uuid,
    pub author_name: String,
    pub content: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}
