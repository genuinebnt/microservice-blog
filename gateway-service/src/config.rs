use common::config::{ApplicationSettings, ServiceSettings};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GatewaySettings {
    pub application: ApplicationSettings,
    pub users_service: ServiceSettings,
    pub posts_service: ServiceSettings,
    pub notification_service: ServiceSettings,
}
