use axum::{Router, routing::get};

use super::super::handler::health_handlers as handlers;

pub fn routes() -> Router {
    Router::new()
        .route("/self", get(handlers::self_))
        .route("/ready", get(handlers::ready))
}
