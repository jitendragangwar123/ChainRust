use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize, ToSchema)]
pub struct SuccessResponse {
    pub message: String,
}