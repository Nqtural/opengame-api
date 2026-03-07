use axum::Json;
use axum::{Router, routing::get};
use serde::Serialize;
use utoipa::ToSchema;

pub fn routes() -> Router {
    Router::new().route("/health", get(health))
}

#[derive(Serialize, ToSchema)]
pub struct Health {
    status: &'static str,
}

#[utoipa::path(
    get,
    path = "/status/health",
    responses(
        (status = 200, description = "Health check", body = Health)
    )
)]
pub async fn health() -> Json<Health> {
    Json(Health { status: "ok" })
}
