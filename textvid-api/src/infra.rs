use anyhow::Context;
use axum::{self, Router};
use std::net::SocketAddr;

pub struct Handler {}

impl Handler {
    pub async fn root(&self) -> &'static str {
        "hello world"
    }
}

pub struct Server {
    pub router: Router,
}

impl Server {
    pub async fn serve(self) -> anyhow::Result<()> {
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        tracing::info!("Listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(self.router.into_make_service())
            .await
            .context("serve")
    }
}
