use axum::{Router, routing::get};

pub mod login;
pub mod register;
pub mod status;

pub fn auth() -> Router {
    Router::new()
        .route("/login", get(login::login))
        .route("/register", get(register::register))
        .route("/status", get(status::status))
}
