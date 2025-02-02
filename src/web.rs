use axum::{Router, middleware, routing::get};

mod metrics;
mod routes_health;
mod routes_hello;

pub(crate) fn router() -> Router {
    let metric_handle = metrics::setup_metrics_recorder();
    Router::new()
        .merge(routes_health::routes())
        .merge(routes_hello::routes())
        .route("/metrics", get(async move || metric_handle.render()))
        .route_layer(middleware::from_fn(metrics::mw_track_metrics))
}
