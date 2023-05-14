use axum::http::StatusCode;
use tracing::instrument;
use uuid::Uuid;

#[instrument(name = "Health check", skip_all, level = "trace", fields(req_id = Uuid::new_v4().to_string()))]
pub async fn health_check() -> StatusCode {
    tracing::debug!("health check request received.");
    StatusCode::OK
}
