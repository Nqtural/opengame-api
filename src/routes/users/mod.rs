use crate::storage::Storage;
use axum::{Router, routing::get};
use std::sync::Arc;

pub mod get_current;

pub fn users(storage: Arc<dyn Storage>) -> Router {
    Router::new()
        .route("/me", get(get_current::get_current))
        .with_state(storage)
}
