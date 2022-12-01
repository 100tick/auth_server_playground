#[macro_use]
extern crate dotenv;

use std::{env, net::SocketAddr};

use dotenv::dotenv;

mod config;
mod db;
mod error;
mod handler;
mod lib;
mod model;
mod repo;
mod router;
// mod schema;
mod extractor;
mod server;
mod util;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let host = env::var("HOST").expect("`HOST` must be set");
    let port = env::var("PORT")
        .expect("`PORT` must be set")
        .parse::<i32>()
        .expect("`PORT` cannot be parsed to i32");

    let addr = format!("{host}:{port}").parse::<SocketAddr>().unwrap();

    server::start(&addr).await;
}
