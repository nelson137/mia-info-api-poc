use axum_prometheus::metrics;
use tracing_subscriber::layer::SubscriberExt;

mod settings;
#[cfg(test)]
mod test_utils;
mod web;

#[tokio::main]
async fn main() {
    let settings = &*settings::SETTINGS;

    let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| settings.log_filter.parse().unwrap());

    let fmt_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_target(false)
        .with_file(true)
        .with_line_number(true);

    let subscriber = tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer);

    tracing_subscriber::util::SubscriberInitExt::init(subscriber);

    let routes = match web::router() {
        Ok(r) => r,
        Err(err) => panic!("{err}"),
    };

    metrics::gauge!(
        "mia_info",
        &[
            ("version_major", "2"),
            ("version_minor", "3"),
            ("version_patch", "4")
        ]
    )
    .set(1);

    let listener = tokio::net::TcpListener::bind(&settings.bind_addr)
        .await
        .unwrap();

    tracing::info!(
        addr = listener.local_addr().unwrap().to_string(),
        version = env!("CARGO_PKG_VERSION"),
        "Starting server",
    );

    axum::serve(listener, routes.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.expect("cancellation signal")
        })
        .await
        .unwrap();
}
