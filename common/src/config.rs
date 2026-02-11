use std::path::Path;

use config::ConfigError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub cache: CacheSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSettings {
    #[serde(default = "default_max_capacity")]
    pub max_capacity: u64,
    #[serde(default = "default_ttl_secs")]
    pub ttl_secs: u64,
    #[serde(default = "default_tti_secs")]
    pub tti_secs: u64,
    pub redis: Option<RedisSettings>,
}

fn default_max_capacity() -> u64 {
    10_000
}
fn default_ttl_secs() -> u64 {
    300
}
fn default_tti_secs() -> u64 {
    60
}

impl Default for CacheSettings {
    fn default() -> Self {
        Self {
            max_capacity: default_max_capacity(),
            ttl_secs: default_ttl_secs(),
            tti_secs: default_tti_secs(),
            redis: None,
        }
    }
}

impl CacheSettings {
    pub fn ttl(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.ttl_secs)
    }

    pub fn tti(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.tti_secs)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisSettings {
    pub hostname: String,
    pub port: u16,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub database: Option<u8>,
}

impl RedisSettings {
    pub fn url(&self) -> String {
        let auth = match (&self.username, &self.password) {
            (Some(user), Some(pass)) => format!("{}:{}@", user, pass),
            (None, Some(pass)) => format!(":{}@", pass),
            _ => String::new(),
        };
        let db = self.database.map(|d| format!("/{}", d)).unwrap_or_default();
        format!("redis://{}{}:{}{}", auth, self.hostname, self.port, db)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServiceSettings {
    pub host: String,
    pub port: u16,
}

impl ServiceSettings {
    pub fn url(&self) -> String {
        format!("http:{}/{}", self.host, self.port)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub backend: DbBackend,
    pub engine: DbEngine,
    pub username: String,
    pub password: String,
    pub hostname: String,
    pub port: u16,
    pub database_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DbBackend {
    Seaorm,
    Sqlx,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DbEngine {
    Postgres,
}

use serde::de::DeserializeOwned;

pub fn get_configuration<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory");
    let config_dir = base_path.join(path);

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or("local".into())
        .try_into()
        .expect("failed to parse app environment");
    let environment_filename = format!("{}.yaml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(config_dir.join("base.yaml")))
        .add_source(config::File::from(config_dir.join(environment_filename)))
        .add_source(
            config::Environment::with_prefix("APP")
                .prefix_separator("_")
                .separator("__"),
        )
        .build()?;

    settings.try_deserialize::<T>()
}

pub enum Environment {
    Local,
    Production,
    Test,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Production => "production",
            Self::Test => "test",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            "test" => Ok(Self::Test),
            other => Err(format!(
                "{} is not a supported environment. Use either 'local' or 'production'",
                other
            )),
        }
    }
}
