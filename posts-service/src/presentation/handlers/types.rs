use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::{AuthorId, Post};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 200, message = "Title must be between 1-200 characters"))]
    pub title: String,
    pub author_id: AuthorId,
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author_id: String,
    pub created_at: String,
}
impl From<Post> for PostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id.to_string(),
            title: post.title,
            content: post.content,
            author_id: post.author_id.to_string(),
            created_at: post.created_at.to_rfc3339(),
        }
    }
}
