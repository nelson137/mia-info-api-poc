#![feature(async_closure)]

mod web;

#[cfg(not(feature = "listen_public"))]
const BIND_ADDR: &str = "localhost:8080";

#[cfg(feature = "listen_public")]
const BIND_ADDR: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() {
    let routes = web::router();

    let listener = tokio::net::TcpListener::bind(BIND_ADDR).await.unwrap();
    eprintln!("Listening on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
