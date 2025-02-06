use utoipa_axum::{router::OpenApiRouter, routes};

use crate::web::state::DeploymentState;

use super::super::handler::deployment_handlers as handlers;

pub fn routes() -> OpenApiRouter<DeploymentState> {
    OpenApiRouter::new().routes(routes!(handlers::badge))
}
