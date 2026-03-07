use super::types::User;
use super::{NewUserStatus, Storage};
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};

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
                Err(anyhow::anyhow!(e))
            }
        }
    }
}
