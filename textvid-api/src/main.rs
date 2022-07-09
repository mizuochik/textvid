use anyhow::{self, Context};
use axum::{routing, Router};
use std::net::SocketAddr;

use tokio::signal::unix::{self, SignalKind};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", routing::get(root));
    run_server(app).await
}

async fn run_server(r: Router) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(r.into_make_service())
        .with_graceful_shutdown(wait_shutdown())
        .await
        .context("run_server")
}

async fn wait_shutdown() {
    let mut int = unix::signal(SignalKind::interrupt()).unwrap();
    let mut term = unix::signal(SignalKind::terminate()).unwrap();
    tokio::select! {
        _ = int.recv() => {},
        _ = term.recv() => {},
    }
    tracing::info!("Shutting down ...");
}

async fn root() -> &'static str {
    "Hello World!"
}
