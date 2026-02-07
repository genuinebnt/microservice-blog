use std::{fmt, marker::PhantomData};

use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Id<T>(pub Uuid, #[serde(skip)] PhantomData<T>);

impl<T> Id<T> {
    pub fn new() -> Self {
        Self(Uuid::new_v4(), PhantomData)
    }
}

impl<T> Default for Id<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<Uuid> for Id<T> {
    fn from(id: Uuid) -> Self {
        Self(id, PhantomData)
    }
}

impl<T> From<Id<T>> for Uuid {
    fn from(id: Id<T>) -> Self {
        id.0
    }
}
