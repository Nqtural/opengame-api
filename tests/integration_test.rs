use anyhow::Result;
use axum::body::to_bytes;
use axum::body::Body;
use axum::http::Request;
use hyper::StatusCode;
use serde_json::Value;
use std::sync::Arc;
use tower::ServiceExt;
use opengame_api::app::app;
use opengame_api::storage;

struct TestStorage;
impl storage::Storage for TestStorage {
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
