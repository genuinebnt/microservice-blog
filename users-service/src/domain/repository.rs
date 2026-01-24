use async_trait::async_trait;
use common::{error::Result, pagination::Pagination};
use std::{fmt::Debug, sync::Arc};
use uuid::Uuid;

use crate::domain::entities::user::User;

#[async_trait]
pub trait UserRepository: Send + Sync + Debug {
    async fn create_user(&self, user: User) -> Result<User>;
    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn get_user_by_name(&self, username: String) -> Result<Option<User>>;
    async fn update_user(&self, user: User) -> Result<()>;
    async fn delete_user(&self, id: Uuid) -> Result<()>;
    async fn list_users(&self, pagination: &Pagination) -> Result<(Vec<User>, u64)>;
}

pub type DynUserRepository = Arc<dyn UserRepository>;
