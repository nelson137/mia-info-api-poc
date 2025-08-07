use axum_prometheus::metrics;

#[cfg(test)]
mod test_utils;
mod web;

#[cfg(not(feature = "listen_public"))]
const BIND_ADDR: &str = "localhost:8080";

#[cfg(feature = "listen_public")]
const BIND_ADDR: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() {
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

    let listener = tokio::net::TcpListener::bind(BIND_ADDR).await.unwrap();
    eprintln!("Listening on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
