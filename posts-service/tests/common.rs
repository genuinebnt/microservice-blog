use std::sync::LazyLock;

use common::telemetry;
use posts::{
    infrastructure::{
        database::{bootstrap::bootstrap, factory::RepoProvider},
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

    let mut config = common::config::get_configuration("config").unwrap();
    // Randomize database name
    config.database.database_name = Uuid::new_v4().to_string();

    configure_database(&config.database).await;

    let listener = tokio::net::TcpListener::bind(format!("{}:0", config.application.host))
        .await
        .unwrap();

    info!("Bound to port: {}", listener.local_addr().unwrap().port());

    let addr = listener.local_addr().unwrap();

    let conn = bootstrap(&config.database).await.unwrap();
    let repo_provider = RepoProvider::from_connection(conn).await.unwrap();
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

    // Connect to "postgres" database to create the new one
    let mut maintenance_config = config.clone();
    maintenance_config.database_name = "postgres".to_string();

    let db_url = posts::infrastructure::database::url::build_db_url(&maintenance_config)
        .await
        .unwrap();
    let db = Database::connect(db_url).await.unwrap();

    db.execute_unprepared(&format!("CREATE DATABASE \"{}\";", config.database_name))
        .await
        .unwrap();
}

async fn delete_database(config: &common::config::DatabaseSettings, db_name: &str) {
    use sea_orm::{ConnectionTrait, Database};

    // Connect to "postgres" database to drop the test one
    let mut maintenance_config = config.clone();
    maintenance_config.database_name = "postgres".to_string();

    let db_url = posts::infrastructure::database::url::build_db_url(&maintenance_config)
        .await
        .unwrap();
    let db = Database::connect(db_url).await.unwrap();

    // Terminate existing connections first (important for Postgres)
    let _ = db
        .execute_unprepared(&format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';",
            db_name
        ))
        .await;

    db.execute_unprepared(&format!("DROP DATABASE \"{}\";", db_name))
        .await
        .expect("Failed to drop database");
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreatePostResponse {
    pub id: uuid::Uuid,
}

#[derive(Serialize, Debug)]
pub struct PostRequest {
    pub title: String,
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
