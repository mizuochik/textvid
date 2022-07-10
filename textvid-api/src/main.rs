use anyhow::Context;
use axum::{routing, Router, Server};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", routing::get(root));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("serve")
}

async fn root() -> &'static str {
    "hello world"
}
