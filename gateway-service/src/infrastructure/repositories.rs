use async_trait::async_trait;
use common::error::{AppError, Result};
use reqwest::Client;

use crate::domain::{models::Post, repositories::PostRepository};

#[derive(Clone)]
pub struct HttpPostRepository {
    client: Client,
    base_url: String,
}

impl HttpPostRepository {
    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }
}

#[async_trait]
impl PostRepository for HttpPostRepository {
    async fn find_all(&self) -> Result<Vec<Post>> {
        let url = format!("{}/posts", self.base_url);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(e.into()))?;

        if !response.status().is_success() {
            return Err(AppError::InternalServerError(
                anyhow::anyhow!("Service returned {}", response.status()).into(),
            ));
        }

        let posts = response
            .json::<Vec<Post>>()
            .await
            .map_err(|e| AppError::InternalServerError(e.into()))?;
        Ok(posts)
    }
}
