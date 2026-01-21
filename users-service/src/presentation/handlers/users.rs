use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use common::{
    error::Result,
    pagination::{PaginatedResponse, Pagination},
};

use crate::presentation::{responses::ListUserResponse, state::AppState};

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
