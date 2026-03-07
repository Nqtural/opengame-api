use anyhow::Result;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use super::Storage;

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
}
