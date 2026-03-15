use super::super::ErrorResponse;
use crate::storage::types::UserInformation;
use crate::storage::{GetCurrentUserStatus, Storage};
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
pub struct GetCurrentUserSuccessResponse {
    #[schema(example = true)]
    pub success: bool,
    pub user: UserInformation,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/users/me",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Success", body = GetCurrentUserSuccessResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get_current(
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

    match storage.get_current_user(bearer).await {
        Ok(GetCurrentUserStatus::Success(user)) => (
            StatusCode::OK,
            Json(GetCurrentUserSuccessResponse {
                success: true,
                user: UserInformation {
                    username: user.username,
                    created_at: user.created_at.to_string(),
                },
            }),
        )
            .into_response(),

        Ok(GetCurrentUserStatus::InvalidCredentials) => (
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
