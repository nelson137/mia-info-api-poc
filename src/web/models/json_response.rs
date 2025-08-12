use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(utoipa::ToSchema, Serialize)]
pub struct JsonResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonResponseError>,
}

impl IntoResponse for JsonResponse {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl JsonResponse {
    pub fn error(message: String) -> Self {
        Self {
            error: Some(JsonResponseError { message }),
        }
    }
}

#[derive(utoipa::ToSchema, Serialize)]
pub struct JsonResponseError {
    message: String,
}
