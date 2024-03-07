#![allow(clippy::needless_return)]

use axum::extract::Extension;
mod commands;
mod http;
mod queue;

extern crate dotenv;

use dotenv::dotenv;
use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // v1
    let queue_client = queue::connection::init().await;
    let v1_router = axum::Router::new()
        .nest("/", http::v1::register())
        .layer(Extension(queue_client))
        .fallback(http::not_found);

    // api
    let app = axum::Router::new().nest("/api/v1", v1_router);

    let server = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(server, app).await.unwrap();
}
