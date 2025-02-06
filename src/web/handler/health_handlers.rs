use axum::{http::StatusCode, response::IntoResponse};

use crate::web::tags;

#[utoipa::path(
    get,
    path = "/self",
    tag = tags::HEALTH,
    description = "Liveness probe.",
    responses((status = OK, description = "Alive.")),
)]
pub async fn self_() -> impl IntoResponse {
    StatusCode::OK
}

#[utoipa::path(
    get,
    path = "/ready",
    tag = tags::HEALTH,
    description = "Readiness probe.",
    responses((status = OK, description = "Ready for requests.")),
)]
pub async fn ready() -> impl IntoResponse {
    StatusCode::OK
}
