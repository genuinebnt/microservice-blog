use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{domain::entities::post::Post, presentation::state::AppState};
use common::error::Result;

#[tracing::instrument(skip(state))]
pub async fn list_posts(State(state): State<Arc<AppState>>) -> Json<Vec<Post>> {
    Json(state.repos.posts.list().await.unwrap().unwrap())
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

#[tracing::instrument(skip(state))]
pub async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePostRequest>,
) -> impl IntoResponse {
    let post = Post {
        id: Uuid::new_v4(),
        title: payload.title,
        content: payload.content,
        created_at: Utc::now().into(),
        updated_at: Utc::now().into(),
    };
    state.repos.posts.create(post.clone()).await.unwrap();
    Json(post)
}

pub async fn get_post(State(state): State<Arc<AppState>>, path: Path<Uuid>) -> Result<Json<Post>> {
    let post = state.repos.posts.get(path.0).await.unwrap();
    if post.is_none() {
        return Err(common::error::AppError::NotFoundError(
            "Post not found".to_string(),
        ));
    } else {
        Ok(Json(post.unwrap()))
    }
}

pub async fn update_post(State(state): State<Arc<AppState>>, Json(post): Json<Post>) -> Result<()> {
    state.repos.posts.update(post.clone()).await.unwrap();
    Ok(())
}

pub async fn delete_post(State(state): State<Arc<AppState>>, path: Path<Uuid>) -> Result<()> {
    state.repos.posts.delete(path.0).await.unwrap();
    Ok(())
}
