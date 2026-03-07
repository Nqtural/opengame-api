use crate::storage::{DeleteSessionStatus, Storage};
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

#[derive(Serialize, ToSchema)]
pub struct LogoutErrorResponse {
    #[schema(example = false)]
    pub success: bool,
    pub message: String,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/auth/logout",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, body = LogoutSuccessResponse),
        (status = 401, body = LogoutErrorResponse),
        (status = 500, body = LogoutErrorResponse)
    )
)]
pub async fn logout(
    State(storage): State<Arc<dyn Storage>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
) -> Response {
    let bearer = match Uuid::parse_str(auth.token()) {
        Ok(bearer) => bearer,
        Err(_) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(LogoutErrorResponse {
                    success: false,
                    message: "Invalid credentials.".to_string(),
                }),
            )
                .into_response();
        }
    };

    match storage.delete_session(bearer).await {
        Ok(DeleteSessionStatus::Success) => (
            StatusCode::OK,
            Json(LogoutSuccessResponse { success: true }),
        )
            .into_response(),

        Ok(DeleteSessionStatus::InvalidCredentials) => (
            StatusCode::UNAUTHORIZED,
            Json(LogoutErrorResponse {
                success: false,
                message: "Invalid credentials.".to_string(),
            }),
        )
            .into_response(),

        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LogoutErrorResponse {
                success: false,
                message: format!("Internal server error: {e}."),
            }),
        )
            .into_response(),
    }
}
