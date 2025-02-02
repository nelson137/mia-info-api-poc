use axum::Router;

mod routes_hello;

pub(crate) fn router() -> Router {
    Router::new().merge(routes_hello::routes())
}
