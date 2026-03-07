use axum::Router;
use crate::routes;

pub fn app() -> Router {
    Router::new()
        .nest("/auth", routes::auth::auth())
        .nest("/status", routes::status::status())
}
