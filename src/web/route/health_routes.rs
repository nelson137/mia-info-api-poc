use utoipa_axum::{router::OpenApiRouter, routes};

use super::super::handler::health_handlers as handlers;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(handlers::self_))
        .routes(routes!(handlers::ready))
}
