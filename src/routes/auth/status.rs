use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct LoggedInStatus {
    status: &'static str,
}

#[utoipa::path(
    get,
    path = "/auth/status",
    responses(
        (status = 200, description = "Get login status", body = LoggedInStatus)
    )
)]
pub async fn status() -> Json<LoggedInStatus> {
    Json(LoggedInStatus { status: "logged out" })
}
