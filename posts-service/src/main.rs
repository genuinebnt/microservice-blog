use common::{
    config::get_configuration,
    telemetry::{get_subscriber, init_subscriber},
};
use posts::{
    infrastructure::{database::factory::RepoProvider, http::create_router},
    presentation::state::AppState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = get_subscriber("posts-service".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration("config")?;

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))
    .await
    .expect("Failed to bind to port");

    let repo_provider = RepoProvider::build_repo_provider(&config.database).await?;
    let state = AppState::new(repo_provider);
    let router = create_router(state);

    tracing::info!("server starting on port: {}...", config.application.port);

    axum::serve(listener, router).await?;

    Ok(())
}
