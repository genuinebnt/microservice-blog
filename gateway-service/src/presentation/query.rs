use async_graphql::{Context, Object};
use common::error::AppError;

use crate::{domain::models::Post, presentation::state::AppState};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<Post>, AppError> {
        let state = ctx.data::<AppState>().unwrap();
        let posts = state.post_repository.find_all().await?;
        Ok(posts)
    }
}
