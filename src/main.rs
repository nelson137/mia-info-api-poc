#![feature(async_closure)]

mod web;

#[tokio::main]
async fn main() {
    let routes = web::router();

    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .unwrap();
    eprintln!("Listening on {:?}", listener.local_addr().unwrap());

    axum::serve(listener, routes.into_make_service())
        .await
        .unwrap();
}
