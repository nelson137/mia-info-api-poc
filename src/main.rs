use axum_prometheus::metrics;

mod settings;
#[cfg(test)]
mod test_utils;
mod web;

#[tokio::main]
async fn main() {
    let settings = &*settings::SETTINGS;

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
    eprintln!("Listening on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, routes.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c().await.expect("cancellation signal")
        })
        .await
        .unwrap();
}
