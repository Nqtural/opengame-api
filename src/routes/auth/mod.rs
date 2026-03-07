use crate::storage::Storage;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub mod login;
pub mod logout;
pub mod register;
pub mod status;

pub fn auth(storage: Arc<dyn Storage>) -> Router {
    Router::new()
        .route("/login", post(login::login))
        .route("/logout", post(logout::logout))
        .route("/register", post(register::register))
        .route("/status", get(status::status))
        .with_state(storage)
}
