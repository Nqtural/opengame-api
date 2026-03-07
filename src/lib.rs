use utoipa::OpenApi;
use crate::routes::{
    status::health::{__path_health, Health},
    auth::status::{__path_status, LoggedInStatus},
};

#[derive(OpenApi)]
#[openapi(paths(status, health), components(schemas(LoggedInStatus, Health)))]
pub struct ApiDoc;

pub mod app;
pub mod routes;
pub mod storage;
