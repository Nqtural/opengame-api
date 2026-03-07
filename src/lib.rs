use crate::routes::{
    auth::status::{__path_status, LoggedInStatus},
    status::health::{__path_health, Health},
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(status, health), components(schemas(LoggedInStatus, Health)))]
pub struct ApiDoc;

pub mod app;
pub mod routes;
pub mod storage;
