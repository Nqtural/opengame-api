use utoipa::OpenApi;
use crate::routes::status::*;

#[derive(OpenApi)]
#[openapi(paths(health), components(schemas(Health)))]
pub struct ApiDoc;

pub mod routes;
