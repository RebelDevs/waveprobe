use super::command;

pub fn register() -> axum::Router {
    let router = axum::Router::new().nest("/command", command::router());

    return router;
}
