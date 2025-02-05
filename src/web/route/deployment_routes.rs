use axum::{Router, routing::get};

use crate::web::state::DeploymentState;

use super::super::handler::deployment_handlers as handlers;

pub fn routes() -> Router<DeploymentState> {
    Router::new().route(
        "/deployment/{namespace}/{service}/badge",
        get(handlers::badge),
    )
}
