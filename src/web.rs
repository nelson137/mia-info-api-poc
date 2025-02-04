use axum::{Router, routing::get};
use axum_prometheus::PrometheusMetricLayerBuilder;

mod handler;
mod route;

pub(crate) fn router() -> Router {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayerBuilder::new()
        .with_ignore_pattern("/self")
        .with_ignore_pattern("/ready")
        .with_ignore_pattern("/metrics")
        .with_default_metrics()
        .build_pair();
    Router::new()
        .merge(route::health_routes::routes())
        .merge(route::hello_routes::routes())
        .route("/metrics", get(async move || metric_handle.render()))
        .layer(prometheus_layer)
}
