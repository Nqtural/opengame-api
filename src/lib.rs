use crate::routes::{
    ErrorResponse,
    auth::{
        login::{__path_login, LoginSuccessResponse},
        logout::{__path_logout, LogoutSuccessResponse},
        logout_all::{__path_logout_all, LogoutAllSuccessResponse},
        register::{__path_register, RegisterSuccessResponse},
        status::{__path_status, LoggedInStatus},
    },
    status::health::{__path_health, Health},
    users::{
        get::{__path_get, GetUserSuccessResponse},
        get_current::{__path_get_current, GetCurrentUserSuccessResponse},
    }
};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().expect("components must exist");
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("UUID")
                    .build(),
            ),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(get, get_current, login, logout, logout_all, register, status, health,),
    components(schemas(
        ErrorResponse,
        GetCurrentUserSuccessResponse,
        GetUserSuccessResponse,
        LoginSuccessResponse,
        LogoutSuccessResponse,
        LogoutAllSuccessResponse,
        RegisterSuccessResponse,
        LoggedInStatus,
        Health,
    )),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

pub mod app;
pub mod routes;
pub mod storage;
