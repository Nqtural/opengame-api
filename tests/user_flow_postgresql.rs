use anyhow::Result;
use hyper::StatusCode;
use opengame_api::app::app;
use opengame_api::storage;
use sqlx::PgPool;
use std::sync::Arc;

mod request_helpers;
use request_helpers::{extract_bearer, login, logout, register, send_request};

async fn setup(pool: PgPool) -> axum::Router {
    let storage = Arc::new(storage::postgres::PostgresDatabase::using_pool(pool).await);
    app(storage)
}

#[sqlx::test(migrations = "./migrations")]
async fn logout_without_auth(pool: PgPool) -> Result<()> {
    let app = setup(pool).await;
    let response = send_request(&app, "POST", "/auth/logout", None, None).await?;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn logout_when_not_logged_in(pool: PgPool) -> Result<()> {
    let app = setup(pool).await;
    let response = logout(&app, "550e8400-e29b-41d4-a716-446655440000").await?;

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn register_user(pool: PgPool) -> Result<()> {
    let app = setup(pool).await;

    let user = serde_json::json!({
        "username": "testuser",
        "email": "test@example.com",
        "password": "password"
    });

    let response = register(&app, &user).await?;

    assert_eq!(response.status(), StatusCode::CREATED);
    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn register_same_user_again(pool: PgPool) -> Result<()> {
    let app = setup(pool).await;

    let user = serde_json::json!({
        "username": "testuser",
        "email": "test@example.com",
        "password": "password"
    });

    // first registration
    let _ = register(&app, &user).await?;

    // second registration should fail
    let response = register(&app, &user).await?;

    assert_eq!(response.status(), StatusCode::CONFLICT);
    Ok(())
}

#[sqlx::test(migrations = "./migrations")]
async fn login_and_logout(pool: PgPool) -> Result<()> {
    let app = setup(pool).await;

    // register user
    let user = serde_json::json!({
        "username": "testuser",
        "email": "test@example.com",
        "password": "password"
    });

    let _ = register(&app, &user).await?;

    // login
    let credentials = serde_json::json!({
        "username": "testuser",
        "password": "password"
    });

    let response = login(&app, &credentials).await?;

    assert_eq!(response.status(), StatusCode::OK);

    // parse token
    let bearer = extract_bearer(response).await?;

    // logout
    let response = logout(&app, &bearer).await?;

    assert_eq!(response.status(), StatusCode::OK);
    Ok(())
}
