use axum::{Router, routing::get};

use super::super::handler::hello_handlers as handlers;

pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(handlers::hello))
        .route("/hello2/{name}", get(handlers::hello2))
}
