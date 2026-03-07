use utoipa::OpenApi;
use crate::routes::status::*;
use crate::routes::auth::status::*;

#[derive(OpenApi)]
#[openapi(paths(status, health), components(schemas(LoggedInStatus, Health)))]
pub struct ApiDoc;

pub mod app;
pub mod routes;
