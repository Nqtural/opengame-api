use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    #[schema(example = false)]
    pub success: bool,
    pub message: String,
}
