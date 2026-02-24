use std::sync::LazyLock;

use anyhow::Context;
use common::telemetry;
use posts_service::{
    infrastructure::{
        database::{RepoProvider, bootstrap_db, build_db_url},
        http::create_router,
    },
    presentation::state::AppState,
};
use tracing::info;
use uuid::Uuid;

static TRACING: LazyLock<()> = LazyLock::new(|| {
    let subscriber = telemetry::get_subscriber("test".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
});

#[derive(Debug, Clone)]
pub struct TestApp {
    pub address: String,
    pub repo_provider: RepoProvider,
    pub db_name: String,
    pub db_config: common::config::DatabaseSettings,
    pub api_client: reqwest::Client,
}

impl Drop for TestApp {
    fn drop(&mut self) {
        let db_config = self.db_config.clone();
        let db_name = self.db_name.clone();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                delete_database(&db_config, &db_name).await;
            });
        })
        .join()
        .expect("failed to join cleanup thread");
    }
}

pub async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);

    // Make integration tests use `config/test.yaml` (see `docker-compose.yaml` -> `posts-db-test`).
    // SAFETY: tests are executed in a separate process; we set this env var before spinning up
    // the application and before spawning any threads in the test harness.
    unsafe {
        std::env::set_var("APP_ENVIRONMENT", "test");
    }

    let mut config =
        common::config::get_configuration::<common::config::Settings>("config").unwrap();
    // Randomize database name
    config.database.database_name = Uuid::new_v4().to_string();

    configure_database(&config.database).await;

    let listener = tokio::net::TcpListener::bind(format!("{}:0", config.application.host))
        .await
        .unwrap();

    info!("Bound to port: {}", listener.local_addr().unwrap().port());

    let addr = listener.local_addr().unwrap();

    let conn = bootstrap_db(&config.database).await.unwrap();
    let repo_provider = RepoProvider::from_connection(conn, &config.cache)
        .await
        .unwrap();
    let state = AppState::new(repo_provider.clone());
    let router = create_router(state);

    tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    TestApp {
        address: addr.to_string(),
        repo_provider,
        db_name: config.database.database_name.clone(),
        db_config: config.database.clone(),
        api_client: client,
    }
}

async fn configure_database(config: &common::config::DatabaseSettings) {
    use sea_orm::{ConnectionTrait, Database};

    let mut maintenance_config = config.clone();
    maintenance_config.database_name = "postgres".to_string();

    let db_url = build_db_url(&maintenance_config)
        .await
        .context(format!(
            "Failed to build database url for maintenance_config: {:?}",
            maintenance_config
        ))
        .unwrap();

    let db = Database::connect(&db_url)
        .await
        .with_context(|| {
            format!(
                "Failed to connect to Postgres for tests at db_url: {db_url}\n\
\n\
Hint: start the test database with:\n\
  docker compose --profile test up -d posts-db-test\n"
            )
        })
        .unwrap();

    db.execute_unprepared(&format!("CREATE DATABASE \"{}\";", config.database_name))
        .await
        .context(format!("Failed to create database for db_url: {}", &db_url))
        .unwrap();
}

async fn delete_database(config: &common::config::DatabaseSettings, db_name: &str) {
    use sea_orm::{ConnectionTrait, Database};

    let mut maintenance_config = config.clone();
    maintenance_config.database_name = "postgres".to_string();

    let db_url = build_db_url(&maintenance_config)
        .await
        .context(format!(
            "Failed to build database url for maintenance_config: {:?}",
            maintenance_config
        ))
        .unwrap();

    let db = Database::connect(&db_url)
        .await
        .context(format!(
            "Failed to connect to database for db_url: {}",
            &db_url
        ))
        .unwrap();

    let _ = db
        .execute_unprepared(&format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';",
            db_name
        ))
        .await
        .context(format!(
            "Failed to terminate connections for db_name: {}",
            db_name
        ))
        .unwrap();

    db.execute_unprepared(&format!("DROP DATABASE \"{}\";", db_name))
        .await
        .context(format!("Failed to drop database for db_name: {}", db_name))
        .unwrap();
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreatePostResponse {
    pub id: uuid::Uuid,
}

#[derive(Serialize, Debug)]
pub struct PostRequest {
    pub title: String,
    pub author_id: uuid::Uuid,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ListPostResponse {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct GetPostResponse {
    pub id: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl TestApp {
    pub async fn post_post(&self, body: &PostRequest) -> reqwest::Response {
        self.api_client
            .post(format!("http://{}/posts", self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn get_post(&self, id: Uuid) -> reqwest::Response {
        self.api_client
            .get(format!("http://{}/posts/{}", self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn list_posts(&self) -> reqwest::Response {
        self.api_client
            .get(format!("http://{}/posts", self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn update_post<T: Serialize + ?Sized>(
        &self,
        id: Uuid,
        body: &T,
    ) -> reqwest::Response {
        self.api_client
            .put(format!("http://{}/posts/{}", self.address, id))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn delete_post(&self, id: Uuid) -> reqwest::Response {
        self.api_client
            .delete(format!("http://{}/posts/{}", self.address, id))
            .send()
            .await
            .expect("Failed to execute request.")
    }
}
