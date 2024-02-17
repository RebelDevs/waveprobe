#![allow(clippy::needless_return)]

use warp::Filter;

mod commands;
mod queue;

extern crate dotenv;

use dotenv::dotenv;
use tokio;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let _client = queue::connection::init().await;

    let endpoint = warp::path("hello").and(warp::get()).map(|| {
        return warp::reply::with_status("hello world", warp::http::StatusCode::OK);
    });

    let router = endpoint.with(warp::log("http"));

    warp::serve(router).run(([0, 0, 0, 0], 3000)).await;
}
