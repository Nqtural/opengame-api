use crate::routes::{
    auth::{
        login::{__path_login, LoginErrorResponse, LoginSuccessResponse},
        register::{__path_register, RegisterErrorResponse, RegisterSuccessResponse},
        status::{__path_status, LoggedInStatus},
    },
    status::health::{__path_health, Health},
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(login, register, status, health,),
    components(schemas(
        LoginSuccessResponse,
        LoginErrorResponse,
        RegisterSuccessResponse,
        RegisterErrorResponse,
        LoggedInStatus,
        Health,
    ))
)]
pub struct ApiDoc;

pub mod app;
pub mod routes;
pub mod storage;
