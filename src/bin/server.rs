use anyhow::{anyhow, Result};
use axum::serve;
use std::sync::Arc;
use tokio::net::TcpListener;
use opengame_api::app::app;
use opengame_api::storage;

#[tokio::main]
async fn main() -> Result<()> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|e| anyhow!("failed to read environment variable 'DATABASE_URL': {e}"))?;
    let storage: Arc<dyn storage::Storage> = Arc::new(
        storage::postgres::PostgresDatabase::new(&database_url)
            .await
            .map_err(|e| anyhow!("failed to connect to database: {e}"))?,
    );

    let app = app(storage);
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    serve(listener, app).await?;

    Ok(())
}
