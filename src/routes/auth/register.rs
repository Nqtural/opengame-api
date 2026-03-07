use crate::storage::types::{NewUser, User};
use crate::storage::{NewUserStatus, Storage};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use bcrypt::{DEFAULT_COST, hash};
use serde::Serialize;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
pub struct RegisterStatus {
    status: String,
}

#[axum::debug_handler]
#[utoipa::path(
    post,
    path = "/auth/register",
    request_body = NewUser,
    responses(
        (status = 200, description = "User registered successfully", body = RegisterStatus),
        (status = 409, description = "Username taken", body = RegisterStatus),
        (status = 500, description = "Internal server error", body = RegisterStatus)
    )
)]
pub async fn register(
    State(storage): State<Arc<dyn Storage>>,
    Json(user): Json<NewUser>,
) -> (StatusCode, Json<RegisterStatus>) {
    let password_hash = match hash(user.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RegisterStatus {
                    status: e.to_string(),
                }),
            );
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
                StatusCode::OK,
                Json(RegisterStatus {
                    status: "success".to_string(),
                }),
            ),
            NewUserStatus::AlreadyExists => (
                StatusCode::CONFLICT,
                Json(RegisterStatus {
                    status: "username taken".to_string(),
                }),
            ),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RegisterStatus {
                status: e.to_string(),
            }),
        ),
    }
}
