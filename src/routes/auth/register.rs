use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct RegisterStatus {
    status: &'static str,
}

#[utoipa::path(
    get,
    path = "/auth/login",
    responses(
        (status = 200, description = "Register", body = RegisterStatus)
    )
)]
pub async fn register() -> Json<RegisterStatus> {
    Json(RegisterStatus { status: "success" })
}
