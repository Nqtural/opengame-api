use axum::body::to_bytes;
use axum::body::Body;
use axum::http::Request;
use hyper::StatusCode;
use serde_json::Value;
use tower::ServiceExt;

use opengame_api::app::app;

#[tokio::test]
async fn health_check_works() {
    let app = app();

    let response = app
        .oneshot(Request::builder()
            .uri("/status/health")
            .body(Body::empty())
            .unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = to_bytes(response.into_body(), 1024).await.unwrap();
    let body: Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body, serde_json::json!({"status": "ok"}));
}
