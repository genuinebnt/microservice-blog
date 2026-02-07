use axum::{
    Json,
    extract::{Path, State},
    response::IntoResponse,
};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

use crate::presentation::state::AppState;
use crate::{
    domain::{Post, PostId},
    presentation::handlers::CreatePostRequest,
};
use common::error::Result;

pub async fn list_posts(State(state): State<Arc<AppState>>) -> Json<Vec<Post>> {
    Json(state.repos.posts.list().await.unwrap().unwrap())
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePostRequest>,
) -> impl IntoResponse {
    let post = Post {
        id: Uuid::new_v4(),
        title: payload.title,
        author_id: payload.author_id,
        content: payload.content,
        created_at: Utc::now().into(),
        updated_at: Utc::now().into(),
    };
    state.repos.posts.create(post.clone()).await.unwrap();
    Json(post)
}

pub async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<PostId>,
) -> Result<Json<Post>> {
    let post = state.repos.posts.get(id.into()).await?;
    match post.is_none() {
        true => Err(common::error::AppError::NotFoundError(
            "Post not found".to_string(),
        )),
        false => Ok(Json(post.unwrap())),
    }
}

pub async fn update_post(State(state): State<Arc<AppState>>, Json(post): Json<Post>) -> Result<()> {
    state.repos.posts.update(post.clone()).await.unwrap();
    Ok(())
}

pub async fn delete_post(State(state): State<Arc<AppState>>, Path(id): Path<PostId>) -> Result<()> {
    state.repos.posts.delete(id.into()).await.unwrap();
    Ok(())
}
