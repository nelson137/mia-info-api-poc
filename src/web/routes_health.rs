use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

pub(super) fn routes() -> Router {
    Router::new()
        .route("/self", get(self_))
        .route("/ready", get(ready))
}

async fn self_() -> impl IntoResponse {
    StatusCode::OK
}

async fn ready() -> impl IntoResponse {
    StatusCode::OK
}
