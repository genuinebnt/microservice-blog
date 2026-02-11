use axum::{
    Json,
    extract::{Path, State},
};
use chrono::Utc;
use std::sync::Arc;

use crate::presentation::{handlers::types::PostResponse, state::AppState};
use crate::{
    domain::{Post, PostId},
    presentation::handlers::CreatePostRequest,
};
use common::error::Result;

pub async fn list_posts(State(state): State<Arc<AppState>>) -> Result<Json<Vec<PostResponse>>> {
    let posts = state
        .repos
        .posts
        .list()
        .await?
        .unwrap_or_default()
        .into_iter()
        .map(PostResponse::from)
        .collect();

    Ok(Json(posts))
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<PostResponse>> {
    let post = Post::builder()
        .id(PostId::new().into())
        .title(payload.title)
        .author_id(payload.author_id.into())
        .content(payload.content)
        .created_at(Utc::now().into())
        .updated_at(Utc::now().into())
        .build();

    state.repos.posts.create(post.clone()).await?;
    Ok(Json(post.into()))
}

pub async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<PostId>,
) -> Result<Json<PostResponse>> {
    let post = state
        .repos
        .posts
        .get(id.into())
        .await?
        .ok_or_else(|| common::error::AppError::NotFoundError("Post not found".to_string()))?;

    Ok(Json(post.into()))
}

pub async fn update_post(State(state): State<Arc<AppState>>, Json(post): Json<Post>) -> Result<()> {
    state.repos.posts.update(post.clone()).await?;
    Ok(())
}

pub async fn delete_post(State(state): State<Arc<AppState>>, Path(id): Path<PostId>) -> Result<()> {
    state.repos.posts.delete(id.into()).await?;
    Ok(())
}
