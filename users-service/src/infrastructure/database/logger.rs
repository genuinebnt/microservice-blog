use std::{sync::Arc, time::Instant};

use async_trait::async_trait;
use uuid::Uuid;

use common::{error::Result, pagination::Pagination};

use crate::domain::{entities::user::User, repository::UserRepository};

#[derive(Debug)]
pub struct LoggedUserRepository {
    inner: Arc<dyn UserRepository>,
}

impl LoggedUserRepository {
    pub fn new(inner: Arc<dyn UserRepository>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl UserRepository for LoggedUserRepository {
    async fn create_user(&self, user: User) -> Result<User> {
        let start = Instant::now();
        tracing::info!(username = %user.username, "Creating user");

        let result = self.inner.create_user(user).await;

        match &result {
            Ok(_) => tracing::info!(elapsed_ms = %start.elapsed().as_millis(), "User created"),
            Err(e) => tracing::error!(error = %e, "Failed to create user"),
        }
        result
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let start = Instant::now();
        let id_str = id.to_string();
        let result = self.inner.get_user_by_id(id).await;

        match &result {
            Ok(Some(u)) => {
                tracing::info!(user_id = %id_str, username = %u.username, elapsed_ms = %start.elapsed().as_millis(), "User found")
            }
            Ok(None) => tracing::warn!(user_id = %id_str, "User not found"),
            Err(e) => tracing::error!(user_id = %id_str, error = %e, "Failed to get user"),
        }
        result
    }

    async fn get_user_by_name(&self, username: String) -> Result<Option<User>> {
        let start = Instant::now();
        let name = username.clone();
        let result = self.inner.get_user_by_name(username).await;

        match &result {
            Ok(Some(u)) => {
                tracing::info!(username = %name, user_id = %u.id, elapsed_ms = %start.elapsed().as_millis(), "User found by name")
            }
            Ok(None) => tracing::warn!(username = %name, "User not found by name"),
            Err(e) => tracing::error!(username = %name, error = %e, "Failed to get user by name"),
        }
        result
    }

    async fn update_user(&self, user: User) -> Result<()> {
        let start = Instant::now();
        tracing::info!(user_id = %user.id, username = %user.username, "Updating user");

        let result = self.inner.update_user(user).await;

        match &result {
            Ok(_) => tracing::info!(elapsed_ms = %start.elapsed().as_millis(), "User updated"),
            Err(e) => tracing::error!(error = %e, "Failed to update user"),
        }
        result
    }

    async fn delete_user(&self, id: Uuid) -> Result<()> {
        let start = Instant::now();
        let id_str = id.to_string();
        tracing::info!(user_id = %id_str, "Deleting user");

        let result = self.inner.delete_user(id).await;

        match &result {
            Ok(_) => tracing::info!(elapsed_ms = %start.elapsed().as_millis(), "User deleted"),
            Err(e) => tracing::error!(user_id = %id_str, error = %e, "Failed to delete user"),
        }
        result
    }

    async fn list_users(&self, pagination: &Pagination) -> Result<(Vec<User>, u64)> {
        let start = Instant::now();
        let result = self.inner.list_users(pagination).await;

        match &result {
            Ok((users, total)) => {
                tracing::info!(count = users.len(), total = total, elapsed_ms = %start.elapsed().as_millis(), "Users listed")
            }
            Err(e) => tracing::error!(error = %e, "Failed to list users"),
        }
        result
    }
}
