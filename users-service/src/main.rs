use common::{
    config::get_configuration,
    telemetry::{get_subscriber, init_subscriber},
};
use users_service::{
    infrastructure::{
        database::{bootstrap::bootstrap_outbox, bootstrap_db, factory::RepoProvider},
        http::create_router,
    },
    presentation::state::AppState,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = get_subscriber("users-service".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration::<common::config::Settings>("config")?;

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))
    .await
    .expect("Failed to bind to port");

    let conn = bootstrap_db(&config.database).await?;
    let repo_provider = RepoProvider::from_connection(conn.clone(), &config.cache).await?;

    let outbox_poller = bootstrap_outbox(conn, &config.pubsub).await?;
    outbox_poller.spawn();

    let state = AppState::new(repo_provider);
    let router = create_router(state);

    tracing::info!("server starting on port: {}...", config.application.port);

    axum::serve(listener, router).await?;

    Ok(())
}
