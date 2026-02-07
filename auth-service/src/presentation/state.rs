use reqwest::blocking::Client;

use crate::config::AuthSettings;

#[derive(Debug, Clone)]
pub struct AppState {
    pub http_client: Client,
    pub users_service: String,
}

impl AppState {
    pub fn new(config: AuthSettings) -> Self {
        AppState {
            http_client: Client::new(),
            users_service: config.users_service.url(),
        }
    }
}
