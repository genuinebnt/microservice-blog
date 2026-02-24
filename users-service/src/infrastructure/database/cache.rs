use std::{fmt::Debug, sync::Arc, time::Duration};

use async_trait::async_trait;
use common::{cache::CacheExt, error::Result, pagination::Pagination};
use uuid::Uuid;

use crate::domain::{entities::user::User, repository::UserRepository};

#[derive(Debug)]
pub struct CachedUserRepository<C: CacheExt + Send + Sync + Debug> {
    inner: Arc<dyn UserRepository>,
    cache: Arc<C>,
    ttl: Duration,
}

impl<C: CacheExt + Send + Sync + Debug> CachedUserRepository<C> {
    pub fn new(inner: Arc<dyn UserRepository>, cache: Arc<C>, ttl: Duration) -> Self {
        Self { inner, cache, ttl }
    }

    fn cache_key(id: &Uuid) -> String {
        format!("user:{}", id)
    }

    fn username_key(username: &str) -> String {
        format!("user:name:{}", username)
    }
}

#[async_trait]
impl<C: CacheExt + Send + Sync + Debug + 'static> UserRepository for CachedUserRepository<C> {
    async fn create_user(&self, user: User) -> Result<User> {
        let id_key = Self::cache_key(&user.id);
        let name_key = Self::username_key(&user.username);

        let user = self.inner.create_user(user).await?;

        self.cache.set(&id_key, &user, self.ttl).await;
        self.cache.set(&name_key, &user, self.ttl).await;

        Ok(user)
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let key = Self::cache_key(&id);

        if let Some(user) = self.cache.get::<_, User>(&key).await {
            return Ok(Some(user));
        }

        let user = self.inner.get_user_by_id(id).await?;

        if let Some(ref u) = user {
            self.cache.set(&key, u, self.ttl).await;
        }

        Ok(user)
    }

    async fn get_user_by_name(&self, username: String) -> Result<Option<User>> {
        let key = Self::username_key(&username);

        if let Some(user) = self.cache.get::<_, User>(&key).await {
            return Ok(Some(user));
        }

        let user = self.inner.get_user_by_name(username).await?;

        if let Some(ref u) = user {
            let id_key = Self::cache_key(&u.id);
            self.cache.set(&key, u, self.ttl).await;
            self.cache.set(&id_key, u, self.ttl).await;
        }

        Ok(user)
    }

    async fn update_user(&self, user: User) -> Result<()> {
        let id_key = Self::cache_key(&user.id);
        let name_key = Self::username_key(&user.username);

        self.inner.update_user(user.clone()).await?;

        self.cache.set(&id_key, &user, self.ttl).await;
        self.cache.set(&name_key, &user, self.ttl).await;

        Ok(())
    }

    async fn delete_user(&self, id: Uuid) -> Result<()> {
        let id_key = Self::cache_key(&id);

        if let Ok(Some(user)) = self.inner.get_user_by_id(id).await {
            let name_key = Self::username_key(&user.username);
            self.inner.delete_user(id).await?;
            self.cache.delete(&id_key).await;
            self.cache.delete(&name_key).await;
        } else {
            self.inner.delete_user(id).await?;
        }

        Ok(())
    }

    async fn list_users(&self, pagination: &Pagination) -> Result<(Vec<User>, u64)> {
        self.inner.list_users(pagination).await
    }
}
