use anyhow::Result;
use axum::{Router, routing::get};
use axum_prometheus::PrometheusMetricLayerBuilder;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

use crate::web::state::AppState;

mod routes;
mod service;
mod state;

mod tags {
    pub const HEALTH: &str = "health";
    pub const HELLO: &str = "hello";
    pub const MIA_DEPLOYMENT: &str = "mia-deployment";
}

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = tags::HEALTH, description = "Health"),
        (name = tags::HELLO, description = "Hello"),
        (name = tags::MIA_DEPLOYMENT, description = "Mia Deployments"),
    )
)]
struct ApiDoc;

pub(crate) fn router() -> Result<Router> {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayerBuilder::new()
        .with_ignore_pattern("/self")
        .with_ignore_pattern("/ready")
        .with_ignore_pattern("/metrics")
        .with_default_metrics()
        .build_pair();

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(routes::health_routes::routes())
        .merge(routes::hello_routes::routes())
        .nest("/deployment", routes::deployment_routes::routes())
        .route("/metrics", get(async move || metric_handle.render()))
        .layer(prometheus_layer)
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/swagger").url("/apidoc/openapi.json", api));

    let state = AppState::new()?;

    Ok(router.with_state(state))
}
