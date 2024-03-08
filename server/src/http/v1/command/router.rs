use super::get_result;
use super::post_request;

pub fn router() -> axum::Router {
    let router = axum::Router::new()
        .route("/:id", axum::routing::get(get_result))
        .route("/request", axum::routing::post(post_request));

    return router;
}
