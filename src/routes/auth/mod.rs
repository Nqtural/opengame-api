use axum::{Router, routing::get};
use std::sync::Arc;
use crate::storage::Storage;

pub mod login;
pub mod register;
pub mod status;

pub fn auth(storage: Arc<dyn Storage>) -> Router {
    Router::new()
        .route("/login", get(login::login))
        .route("/register", get(register::register))
        .route("/status", get(status::status))
        .with_state(storage)
}
