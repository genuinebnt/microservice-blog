use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};

use crate::{domain::entities::post::Post, presentation::state::AppState};

pub async fn list_posts(State(state): State<Arc<AppState>>) -> Json<Vec<Post>> {
    Json(state.repos.posts.list().await.unwrap().unwrap())
}

pub async fn create_post() -> impl IntoResponse {}

pub async fn get_post() {}

pub async fn update_post() {}

pub async fn delete_post() {}
