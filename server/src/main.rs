#![allow(clippy::needless_return)]

use std::sync::Arc;

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
    let queue_client = Arc::new(queue::connection::init().await);
    let v1_router = axum::Router::new()
        .nest("/", http::v1::register())
        .layer(axum::Extension(queue_client.clone()))
        .fallback(http::not_found);

    // api
    let app = axum::Router::new().nest("/api/v1", v1_router);

    let server = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(server, app).await.unwrap();
}
