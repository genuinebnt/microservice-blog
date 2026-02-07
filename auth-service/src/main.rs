use common::{
    config::get_configuration,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = get_subscriber("auth-service".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_configuration::<common::config::Settings>("config")?;

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        config.application.host, config.application.port
    ))
    .await
    .expect("Failed to bind to port");

    let router = create_router(state);

    tracing::info!("server starting on port: {}...", config.application.port);

    axum::serve(listener, router).await?;

    Ok(())
}
