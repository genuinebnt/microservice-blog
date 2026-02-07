use common::config::{ApplicationSettings, ServiceSettings};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AuthSettings {
    pub application: ApplicationSettings,
    pub users_service: ServiceSettings,
}
