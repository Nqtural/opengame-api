use crate::storage::types::{RegisterRequest, User};
use crate::storage::{NewUserStatus, Storage};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use bcrypt::{DEFAULT_COST, hash};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct RegisterSuccessResponse {
    #[schema(example = true)]
    success: bool,
}

#[derive(Serialize, ToSchema)]
pub struct RegisterErrorResponse {
    #[schema(example = false)]
    success: bool,
    message: String,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = RegisterSuccessResponse),
        (status = 409, description = "Username taken", body = RegisterErrorResponse),
        (status = 500, description = "Internal server error", body = RegisterErrorResponse)
    )
)]
pub async fn register(
    State(storage): State<Arc<dyn Storage>>,
    Json(user): Json<RegisterRequest>,
) -> Response {
    let password_hash = match hash(user.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RegisterErrorResponse {
                    success: false,
                    message: e.to_string(),
                }),
            )
                .into_response();
        }
    };

    let user = User {
        id: Uuid::new_v4(),
        username: user.username,
        email: user.email,
        password_hash,
        created_at: chrono::Utc::now().naive_utc(),
    };

    match storage.new_user(user).await {
        Ok(s) => match s {
            NewUserStatus::Success => (
                StatusCode::CREATED,
                Json(RegisterSuccessResponse { success: true }),
            )
                .into_response(),
            NewUserStatus::AlreadyExists => (
                StatusCode::CONFLICT,
                Json(RegisterErrorResponse {
                    success: false,
                    message: "username taken".to_string(),
                }),
            )
                .into_response(),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RegisterErrorResponse {
                success: false,
                message: e.to_string(),
            }),
        )
            .into_response(),
    }
}
