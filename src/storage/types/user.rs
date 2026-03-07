use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, FromRow, ToSchema)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

pub struct User {
    pub id: Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
}
