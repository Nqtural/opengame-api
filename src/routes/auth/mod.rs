use axum::{Router, routing::get};

pub mod status;

pub fn auth() -> Router {
    Router::new().route("/status", get(status::status))
}
