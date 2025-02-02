use axum::{Router, routing::get};
use axum_prometheus::PrometheusMetricLayer;

mod routes_health;
mod routes_hello;

pub(crate) fn router() -> Router {
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();
    Router::new()
        .merge(routes_health::routes())
        .merge(routes_hello::routes())
        .route("/metrics", get(async move || metric_handle.render()))
        .layer(prometheus_layer)
}
