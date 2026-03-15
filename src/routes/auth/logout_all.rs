use super::super::ErrorResponse;
use crate::storage::{DeleteAllSessionsStatus, Storage};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum_extra::TypedHeader;
use headers::{Authorization, authorization::Bearer};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct LogoutSuccessResponse {
    #[schema(example = true)]
    pub success: bool,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/auth/logout-all",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User logged out form all devices successfully", body = LogoutSuccessResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn logout_all(
    State(storage): State<Arc<dyn Storage>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Response {
    let bearer = match Uuid::parse_str(auth.token()) {
        Ok(bearer) => bearer,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    success: false,
                    message: "Invalid credentials.".to_string(),
                }),
            )
                .into_response();
        }
    };

    match storage.delete_all_sessions(bearer).await {
        Ok(DeleteAllSessionsStatus::Success) => (
            StatusCode::OK,
            Json(LogoutSuccessResponse { success: true }),
        )
            .into_response(),

        Ok(DeleteAllSessionsStatus::InvalidCredentials) => (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                success: false,
                message: "Invalid credentials.".to_string(),
            }),
        )
            .into_response(),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                success: false,
                message: format!("Internal server error: {e}."),
            }),
        )
            .into_response(),
    }
}
