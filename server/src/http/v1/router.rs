use super::hello;

pub fn register() -> axum::Router {
    let router = axum::Router::new().nest("/hello", hello::router());

    return router;
}
