use crate::routes::{
    auth::{
        login::{__path_login, LogInStatus},
        register::{__path_register, RegisterStatus},
        status::{__path_status, LoggedInStatus},
    },
    status::health::{__path_health, Health},
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(login, register, status, health,),
    components(schemas(LogInStatus, RegisterStatus, LoggedInStatus, Health,))
)]
pub struct ApiDoc;

pub mod app;
pub mod routes;
pub mod storage;
