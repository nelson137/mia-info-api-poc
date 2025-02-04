use axum::{
    extract::{Path, Query},
    response::{self, IntoResponse},
};
use axum_prometheus::metrics;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HelloParams {
    name: Option<String>,
}

pub async fn hello(params: Query<HelloParams>) -> impl IntoResponse {
    metrics::gauge!("said_hello", &[("type", "query")]).increment(1);
    let name = params.name.as_deref().unwrap_or("World");
    response::Html(format!("Hello <strong>{name}</strong>!\n"))
}

pub async fn hello2(Path(name): Path<String>) -> impl IntoResponse {
    metrics::gauge!("said_hello", &[("type", "path")]).increment(1);
    response::Html(format!("Hello <strong>{name}</strong>!\n"))
}
