use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, FromRow, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
}
