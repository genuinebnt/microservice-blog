use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use common::{
    error::Result,
    pagination::{PaginatedResponse, Pagination},
};

use crate::{
    domain::entities::user::User,
    presentation::{responses::ListUserResponse, state::AppState},
};

pub async fn list_users(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<ListUserResponse>> {
    let pagination = pagination.normalize();
    let (users, total_users) = state.repos.users.list_users(&pagination).await?;

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
    Json(user): Json<User>,
) -> Result<Json<User>> {
    let user = state.repos.users.create_user(user).await?;
    Ok(Json(user))
}
