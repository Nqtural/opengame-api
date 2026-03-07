use super::types::{LoginRequest, User};
use super::{NewUserStatus, Storage};
use crate::storage::NewSessionStatus;
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use bcrypt::verify;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresDatabase {
    pool: Pool<Postgres>,
}

impl PostgresDatabase {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }
}

#[async_trait]
impl Storage for PostgresDatabase {
    async fn new_session(&self, credentials: LoginRequest) -> Result<NewSessionStatus> {
        let user = sqlx::query!(
            r#"
            SELECT id, password_hash
            FROM users
            WHERE username = $1
            "#,
            credentials.username
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| anyhow!(e))?;

        match verify(&credentials.password, &user.password_hash) {
            Err(e) => return Err(anyhow!("failed to verify password: {e}")),
            Ok(false) => return Ok(NewSessionStatus::InvalidCredentials),
            _ => {}
        }

        let session_id = Uuid::new_v4();
        let result = sqlx::query!(
            r#"
            INSERT INTO sessions (id, user_id, created_at)
            VALUES ($1, $2, $3)
            "#,
            session_id,
            user.id,
            chrono::Utc::now().naive_utc(),
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(NewSessionStatus::Success(session_id)),
            Err(e) => Err(anyhow!(e)),
        }
    }

    async fn new_user(&self, user: User) -> Result<NewUserStatus> {
        let result = sqlx::query!(
            r#"
            INSERT INTO users (id, email, username, password_hash, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            user.id,
            user.email,
            user.username,
            user.password_hash,
            user.created_at,
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => Ok(NewUserStatus::Success),
            Err(e) => {
                if let Some(db_err) = e.as_database_error()
                    && db_err.message().contains("duplicate key value")
                {
                    return Ok(NewUserStatus::AlreadyExists);
                }
                Err(anyhow!(e))
            }
        }
    }
}
