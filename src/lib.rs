use crate::routes::{
    auth::{
        login::{__path_login, LoginErrorResponse, LoginSuccessResponse},
        logout::{__path_logout, LogoutErrorResponse, LogoutSuccessResponse},
        register::{__path_register, RegisterErrorResponse, RegisterSuccessResponse},
        status::{__path_status, LoggedInStatus},
    },
    status::health::{__path_health, Health},
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
                    .bearer_format("UUID") // or "UUID" in your case
                    .build(),
            ),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(login, logout, register, status, health,),
    components(schemas(
        LoginSuccessResponse,
        LoginErrorResponse,
        LogoutSuccessResponse,
        LogoutErrorResponse,
        RegisterSuccessResponse,
        RegisterErrorResponse,
        LoggedInStatus,
        Health,
    )),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

pub mod app;
pub mod routes;
pub mod storage;
