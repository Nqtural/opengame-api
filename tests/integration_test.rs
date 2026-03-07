use anyhow::Result;
use async_trait::async_trait;
use axum::body::Body;
use axum::body::to_bytes;
use axum::http::Request;
use hyper::StatusCode;
use opengame_api::app::app;
use opengame_api::storage;
use opengame_api::storage::types::LoginRequest;
use serde_json::Value;
use std::sync::Arc;
use storage::types::User;
use tower::ServiceExt;
use uuid::Uuid;

struct TestStorage;

#[async_trait]
impl storage::Storage for TestStorage {
    async fn new_session(&self, _credentials: LoginRequest) -> Result<storage::NewSessionStatus> {
        Ok(storage::NewSessionStatus::Success(Uuid::new_v4()))
    }
    async fn new_user(&self, _user: User) -> Result<storage::NewUserStatus> {
        Ok(storage::NewUserStatus::Success)
    }
}

#[tokio::test]
async fn health_check_works() -> Result<()> {
    let app = app(Arc::new(TestStorage));
    let response = app
        .oneshot(
            Request::builder()
                .uri("/status/health")
                .body(Body::empty())?,
        )
        .await?;

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), 1024).await?;
    let body: Value = serde_json::from_slice(&body_bytes)?;
    assert_eq!(body, serde_json::json!({"status": "ok"}));

    Ok(())
}
