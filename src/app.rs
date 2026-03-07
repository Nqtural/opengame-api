use axum::Router;
use crate::routes;

pub fn app() -> Router {
    Router::new().nest("/status", routes::status::routes())
}
