use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
};
use common::{
    error::{AppError, Result},
    pagination::{PaginatedResponse, Pagination},
};
use uuid::Uuid;

use crate::{
    domain::entities::user::User,
    presentation::{
        handlers::{CreateUserRequest, types::UserResponse},
        responses::ListUserResponse,
        state::AppState,
    },
};

pub async fn list_users(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<ListUserResponse>> {
    let pagination = pagination.normalize();
    let (users, total_users): (Vec<User>, u64) = state.repos.users.list_users(&pagination).await?;

    let count = users.len() as u64;
    let paginated_response = PaginatedResponse::new(
        users,
        count,
        total_users,
        pagination.page,
        pagination.page_size,
    );
    Ok(Json(paginated_response))
}

pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<UserResponse>> {
    let user = User {
        id: Uuid::new_v4(),
        username: payload.username,
        email: payload.email,
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let user = state.repos.users.create_user(user).await?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn get_user_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>> {
    let user = state.repos.users.get_user_by_id(id).await?;
    match user {
        Some(u) => Ok(Json(u)),
        None => Err(AppError::NotFoundError("User not found".to_string())),
    }
}

pub async fn update_user(
    State(state): State<Arc<AppState>>,
    Path(_id): Path<Uuid>,
    Json(user): Json<User>,
) -> Result<Json<()>> {
    state.repos.users.update_user(user).await?;
    Ok(Json(()))
}

pub async fn delete_user(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<()>> {
    state.repos.users.delete_user(id).await?;
    Ok(Json(()))
}
