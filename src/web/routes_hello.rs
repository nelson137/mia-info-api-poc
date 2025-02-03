use axum::{
    Router,
    extract::{Path, Query},
    response::{self, IntoResponse},
    routing::get,
};
use axum_prometheus::metrics;
use serde::Deserialize;

pub(super) fn routes() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/hello2/{name}", get(hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn hello(params: Query<HelloParams>) -> impl IntoResponse {
    metrics::gauge!("said_hello", &[("type", "query")]).increment(1);
    let name = params.name.as_deref().unwrap_or("World");
    response::Html(format!("Hello <strong>{name}</strong>!\n"))
}

async fn hello2(Path(name): Path<String>) -> impl IntoResponse {
    metrics::gauge!("said_hello", &[("type", "path")]).increment(1);
    response::Html(format!("Hello <strong>{name}</strong>!\n"))
}
