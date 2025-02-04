use axum::{http::StatusCode, response::IntoResponse};

pub async fn self_() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn ready() -> impl IntoResponse {
    StatusCode::OK
}
