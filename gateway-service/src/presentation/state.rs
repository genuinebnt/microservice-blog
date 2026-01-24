use crate::config::ServiceSettings;
use crate::domain::repositories::PostRepository;
use crate::infrastructure::repositories::HttpPostRepository;
use reqwest::Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub post_repository: Arc<dyn PostRepository>,
}

impl AppState {
    pub fn new(posts_service_settings: ServiceSettings) -> Self {
        let client = Client::new();
        let posts_service_url = posts_service_settings.url();

        let post_repo = HttpPostRepository::new(client, posts_service_url);

        Self {
            post_repository: Arc::new(post_repo),
        }
    }
}
