use crate::routes;
use crate::storage::Storage;
use axum::Router;
use std::sync::Arc;

pub fn app(storage: Arc<dyn Storage>) -> Router {
    Router::new()
        .nest("/auth", routes::auth::auth(storage))
        .nest("/status", routes::status::status())
}
