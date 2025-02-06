use utoipa_axum::{router::OpenApiRouter, routes};

use super::super::handler::hello_handlers as handlers;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(handlers::hello))
        .routes(routes!(handlers::hello2))
}
