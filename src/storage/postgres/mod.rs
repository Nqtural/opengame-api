use super::types::{LoginRequest, User};
use super::{NewUserStatus, Storage};
use crate::storage::{DeleteSessionStatus, GetCurrentUserStatus, NewSessionStatus};
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

    // for tests
    pub async fn using_pool(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Storage for PostgresDatabase {
    async fn get_current_user(&self, bearer: Uuid) -> Result<GetCurrentUserStatus> {
        let user_id = match sqlx::query!(
            r#"
            SELECT user_id
            FROM sessions
            WHERE id = $1
            "#,
            bearer,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| anyhow!(e))?
        {
            Some(user_id) => user_id,
            None => return Ok(GetCurrentUserStatus::InvalidCredentials),
        }
        .user_id;

        match sqlx::query!(
            r#"
            SELECT *
            FROM users
            WHERE id = $1
            "#,
            user_id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| anyhow!(e))?
        {
            Some(user) => Ok(GetCurrentUserStatus::Success(User {
                id: user.id,
                email: user.email,
                username: user.username,
                password_hash: user.password_hash,
                created_at: user.created_at,
            })),
            None => Err(anyhow!("found session for user that does not exist")), // invalid state
        }
    }

    async fn delete_session(&self, bearer: Uuid) -> Result<DeleteSessionStatus> {
        let result = sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE id = $1
            "#,
            bearer,
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(res) => {
                if res.rows_affected() == 0 {
                    Ok(DeleteSessionStatus::InvalidCredentials)
                } else {
                    Ok(DeleteSessionStatus::Success)
                }
            }
            Err(e) => Err(anyhow!(e)),
        }
    }

    async fn new_session(&self, credentials: LoginRequest) -> Result<NewSessionStatus> {
        let user = match sqlx::query!(
            r#"
            SELECT id, password_hash
            FROM users
            WHERE username = $1
            "#,
            credentials.username
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| anyhow!(e))?
        {
            Some(user) => user,
            None => return Ok(NewSessionStatus::InvalidCredentials),
        };

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
