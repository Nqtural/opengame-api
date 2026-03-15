use crate::storage::Storage;
use axum::{Router, routing::get};
use std::sync::Arc;

pub mod get;
pub mod get_current;

pub fn users(storage: Arc<dyn Storage>) -> Router {
    Router::new()
        .route("/me", get(get_current::get_current))
        .route("/user/{username}", get(get::get))
        .with_state(storage)
}
