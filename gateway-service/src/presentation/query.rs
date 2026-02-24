use async_graphql::{Context, Object};
use common::error::AppError;

use crate::presentation::{
    models::{PaginatedResponse, PostWithAuthor, RawPost, RawUser},
    state::AppState,
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn posts(&self, ctx: &Context<'_>) -> Result<Vec<PostWithAuthor>, AppError> {
        let state = ctx.data::<AppState>().unwrap();
        let post_url = format!("{}/posts", state.posts_service_url);
        let user_url = format!("{}/users", state.users_service_url);

        let (post_response, user_response) = tokio::try_join!(
            state.http_client.get(&post_url).send(),
            state.http_client.get(&user_url).send()
        )
        .map_err(|e| AppError::InternalServerError(e.into()))?;

        if !post_response.status().is_success() {
            return Err(AppError::InternalServerError(anyhow::anyhow!(
                "Posts service error: {}",
                post_response.status()
            )));
        }
        if !user_response.status().is_success() {
            return Err(AppError::InternalServerError(anyhow::anyhow!(
                "Users service error: {}",
                user_response.status()
            )));
        }

        let (posts, users_response) = tokio::try_join!(
            post_response.json::<Vec<RawPost>>(),
            user_response.json::<PaginatedResponse<RawUser>>()
        )
        .map_err(|e| AppError::InternalServerError(e.into()))?;

        let users = users_response.data;

        let user_map: std::collections::HashMap<_, _> =
            users.into_iter().map(|u| (u.id, u.username)).collect();

        let posts_with_author = posts
            .into_iter()
            .map(|post| {
                let author_name = user_map
                    .get(&post.author_id)
                    .cloned()
                    .unwrap_or_else(|| "Unknown".to_string());

                PostWithAuthor {
                    id: post.id,
                    title: post.title,
                    author_id: post.author_id,
                    author_name,
                    content: post.content,
                    created_at: post.created_at,
                    updated_at: post.updated_at,
                }
            })
            .collect();

        Ok(posts_with_author)
    }
}
