use axum::{Json, response::IntoResponse};
use common::error::Result;

pub async fn health_check() -> Result<impl IntoResponse> {
    Ok(Json(serde_json::json!({
        "status": "health check passed",
    })))
}
