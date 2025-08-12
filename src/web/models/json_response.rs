use axum::{Json, response::IntoResponse};
use serde::Serialize;

#[derive(utoipa::ToSchema, Serialize)]
pub struct JsonResponse<T = ()> {
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<JsonResponseData<T>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonResponseError>,
}

impl<T: Serialize> IntoResponse for JsonResponse<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl<T> JsonResponse<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self {
            data: Some(JsonResponseData { items }),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            data: None,
            error: Some(JsonResponseError { message }),
        }
    }
}

#[derive(utoipa::ToSchema, Serialize)]
pub struct JsonResponseData<T> {
    items: Vec<T>,
}

#[derive(utoipa::ToSchema, Serialize)]
pub struct JsonResponseError {
    message: String,
}
