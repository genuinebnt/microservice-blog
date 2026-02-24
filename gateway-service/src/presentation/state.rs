use crate::config::GatewaySettings;

use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub http_client: Client,
    pub posts_service_url: String,
    pub users_service_url: String,
    pub notification_service_url: String,
}

impl AppState {
    pub fn new(config: GatewaySettings) -> Self {
        let http_client = Client::new();
        let posts_service_url = config.posts_service.url();
        let users_service_url = config.users_service.url();
        let notification_service_url = config.notification_service.url();

        Self {
            http_client,
            posts_service_url,
            users_service_url,
            notification_service_url,
        }
    }
}
