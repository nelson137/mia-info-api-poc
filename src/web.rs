use axum::{Router, routing::get};
use axum_prometheus::PrometheusMetricLayerBuilder;

mod routes_health;
mod routes_hello;

pub(crate) fn router() -> Router {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayerBuilder::new()
        .with_ignore_pattern("/self")
        .with_ignore_pattern("/ready")
        .with_ignore_pattern("/metrics")
        .with_default_metrics()
        .build_pair();
    Router::new()
        .merge(routes_health::routes())
        .merge(routes_hello::routes())
        .route("/metrics", get(async move || metric_handle.render()))
        .layer(prometheus_layer)
}
