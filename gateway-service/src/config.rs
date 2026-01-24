use common::config::ApplicationSettings;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GatewaySettings {
    pub application: ApplicationSettings,
    #[serde(rename = "users-service")]
    pub users_service: ServiceSettings,
    #[serde(rename = "posts-service")]
    pub posts_service: ServiceSettings,
}

#[derive(Debug, Deserialize)]
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
