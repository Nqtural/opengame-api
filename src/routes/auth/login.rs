use crate::storage::types::LoginRequest;
use crate::storage::{NewSessionStatus, Storage};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct LoginSuccessResponse {
    #[schema(example = true)]
    pub success: bool,
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub bearer: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginErrorResponse {
    #[schema(example = false)]
    pub success: bool,
    pub message: String,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description="User successfully logged in", body = LoginSuccessResponse),
        (status = 401, description="Invalid credentials", body = LoginErrorResponse),
        (status = 500, description="Internal server error", body = LoginErrorResponse)
    )
)]
pub async fn login(
    State(storage): State<Arc<dyn Storage>>,
    Json(credentials): Json<LoginRequest>,
) -> Response {
    match storage.new_session(credentials).await {
        Ok(NewSessionStatus::Success(uuid)) => (
            StatusCode::OK,
            Json(LoginSuccessResponse {
                success: true,
                bearer: uuid.to_string(),
            }),
        )
            .into_response(),

        Ok(NewSessionStatus::InvalidCredentials) => (
            StatusCode::UNAUTHORIZED,
            Json(LoginErrorResponse {
                success: false,
                message: "Invalid credentials.".to_string(),
            }),
        )
            .into_response(),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LoginErrorResponse {
                success: false,
                message: format!("Internal server error: {e}."),
            }),
        )
            .into_response(),
    }
}
