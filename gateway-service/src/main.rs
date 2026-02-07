use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use common::{
    config::get_configuration,
    telemetry::{get_subscriber, init_subscriber},
};
use gateway_service::{
    config::GatewaySettings,
    presentation::{query::QueryRoot, http::create_router, state::AppState},
};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = get_subscriber(
        "gateway-service".to_string(),
        "info".to_string(),
        std::io::stdout,
    );
    init_subscriber(subscriber);

    let config = get_configuration::<GatewaySettings>("config")?;
    let state = AppState::new(config.clone());

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(state.clone())
        .finish();

    let app = create_router(state, schema);

    let addr = format!("{}:{}", config.application.host, config.application.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("listening on {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
