use common::config::ApplicationSettings;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GatewaySettings {
    pub application: ApplicationSettings,
    pub users_service: ServiceSettings,
    pub posts_service: ServiceSettings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceSettings {
    pub host: String,
    pub port: Option<u16>,
}

impl ServiceSettings {
    pub fn url(&self) -> String {
        match self.port {
            Some(port) => format!("http://{}:{}", self.host, port),
            None => format!("http://{}", self.host),
        }
    }
}
