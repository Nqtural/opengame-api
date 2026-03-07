use axum::{Router, routing::get};

pub mod health;

pub fn status() -> Router {
    Router::new().route("/health", get(health::health))
}
