use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

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
