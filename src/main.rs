use axum::{
    Router,
    extract::{Path, Query},
    response::{self, IntoResponse},
    routing,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let routes = Router::new().merge(routes_hello());

    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .unwrap();
    eprintln!("Listening on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", routing::get(handler_hello))
        .route("/hello2/{name}", routing::get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(params: Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World");
    response::Html(format!("Hello <strong>{name}</strong>!"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    response::Html(format!("Hello <strong>{name}</strong>!"))
}
