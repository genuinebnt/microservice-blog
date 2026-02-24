use std::sync::Arc;

use common::{
    config::get_configuration,
    pubsub::PubSubSubscriber,
    telemetry::{get_subscriber, init_subscriber},
};
use notification_service::{
    infrastructure::{
        database::{bootstrap::bootstrap_db, factory::RepoProvider},
        http::create_router,
    },
    presentation::state::AppState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = get_subscriber(
        "notification-service".into(),
        "info".into(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let config = get_configuration::<common::config::Settings>("config")?;

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))
    .await
    .expect("Failed to bind to port");

    let conn = bootstrap_db(&config.database).await?;
    let repo_provider = RepoProvider::from_connection(conn).await?;
    let state = AppState::new(repo_provider);
    let state_arc = Arc::new(state);

    let pubsub_subscriber = PubSubSubscriber::new(&config.pubsub).await?;
    notification_service::subscriber::spawn_subscriber(state_arc.clone(), pubsub_subscriber);

    let router = create_router(state_arc);

    tracing::info!("server starting on port: {}...", config.application.port);
    axum::serve(listener, router).await?;

    Ok(())
}
