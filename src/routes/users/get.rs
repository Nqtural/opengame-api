use super::super::ErrorResponse;
use crate::storage::types::UserInformation;
use crate::storage::{GetUserStatus, Storage};
use axum::Json;
use axum::extract::{Path, State};
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
pub struct GetUserSuccessResponse {
    #[schema(example = true)]
    pub success: bool,
    pub user: UserInformation,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/users/user/{username}",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Success", body = GetUserSuccessResponse),
        (status = 401, description = "Invalid credentials", body = ErrorResponse),
        (status = 500, description = "Internal server error", body = ErrorResponse)
    )
)]
pub async fn get(
    Path(username): Path<String>,
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

    if !storage.validate_bearer(bearer).await {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                success: false,
                message: "Invalid credentials.".to_string(),
            }),
        )
            .into_response();
    }

    match storage.get_user(&username).await {
        Ok(GetUserStatus::Success(user)) => (
            StatusCode::OK,
            Json(GetUserSuccessResponse {
                success: true,
                user: UserInformation {
                    username: user.username,
                    created_at: user.created_at.to_string(),
                },
            }),
        )
            .into_response(),

        Ok(GetUserStatus::InvalidCredentials) => (
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                success: false,
                message: "Invalid credentials.".to_string(),
            }),
        )
            .into_response(),

        Ok(GetUserStatus::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                success: false,
                message: "Username not found.".to_string(),
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
