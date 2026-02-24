use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use uuid::Uuid;

use crate::infrastructure::database::factory::RepoProvider;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationEvent {
    pub user_id: Uuid,
    pub kind: String,
    pub title: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub repos: RepoProvider,
    pub tx: broadcast::Sender<NotificationEvent>,
}

impl AppState {
    pub fn new(repos: RepoProvider) -> Self {
        let (tx, _) = broadcast::channel(1024);
        Self { repos, tx }
    }
}
