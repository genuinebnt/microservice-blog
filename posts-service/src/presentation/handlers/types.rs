use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 200, message = "Title must be between 1-200 characters"))]
    pub title: String,
    pub author_id: Uuid,
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,
}
