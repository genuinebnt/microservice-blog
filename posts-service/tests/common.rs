use std::sync::LazyLock;

use common::telemetry;
use posts::{
    infrastructure::{database::factory::RepoProvider, http::create_router},
    presentation::state::AppState,
};
use tracing::info;

static TRACING: LazyLock<()> = LazyLock::new(|| {
    let subscriber = telemetry::get_subscriber("test".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
});

#[derive(Debug, Clone)]
pub struct TestApp {
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    LazyLock::force(&TRACING);

    let config = common::config::get_configuration("config").unwrap();

    let listener = tokio::net::TcpListener::bind(format!("{}:0", config.application.host))
        .await
        .unwrap();

    info!("Bound to port: {}", listener.local_addr().unwrap().port());

    let addr = listener.local_addr().unwrap();

    let repo_provider = RepoProvider::build_repo_provider(&config.database)
        .await
        .unwrap();
    let state = AppState::new(repo_provider);
    let router = create_router(state);

    tokio::spawn(async move { axum::serve(listener, router).await.unwrap() });

    TestApp {
        address: addr.to_string(),
    }
}
