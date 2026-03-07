use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct LogInStatus {
    status: &'static str,
}

#[utoipa::path(
    get,
    path = "/auth/login",
    responses(
        (status = 200, description = "Log in", body = LogInStatus)
    )
)]
pub async fn login() -> Json<LogInStatus> {
    Json(LogInStatus { status: "success" })
}
