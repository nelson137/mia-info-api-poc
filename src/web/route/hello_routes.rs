use axum::{
    extract::{Path, Query},
    response::{self, IntoResponse},
};
use axum_prometheus::metrics;
use serde::Deserialize;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::web::tags;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(hello))
        .routes(routes!(hello2))
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
pub struct HelloParams {
    name: Option<String>,
}

#[utoipa::path(
    get,
    path = "/hello",
    tag = tags::HELLO,
    summary = "hello by query",
    params(HelloParams),
    responses((
        status = OK,
        description = "Hello HTML.",
        body = String,
        content_type = "text/html",
        example = "<p>Hello 👋</p>"
    )),
)]
pub async fn hello(params: Query<HelloParams>) -> impl IntoResponse {
    metrics::gauge!("said_hello", &[("type", "query")]).increment(1);
    let name = params.name.as_deref().unwrap_or("World");
    response::Html(format!("Hello <strong>{name}</strong>!\n"))
}

#[utoipa::path(
    get,
    path = "/hello2/{name}",
    tag = tags::HELLO,
    summary = "hello by path",
    responses((
        status = OK,
        description = "Hello HTML.",
        body = String,
        content_type = "text/html",
        example = "<p>Hello, {name} 👋</p>"
    )),
)]
pub async fn hello2(Path(name): Path<String>) -> impl IntoResponse {
    metrics::gauge!("said_hello", &[("type", "path")]).increment(1);
    response::Html(format!("Hello <strong>{name}</strong>!\n"))
}
