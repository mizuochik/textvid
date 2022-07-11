use anyhow::{anyhow, Context};
use axum::routing::Router;
use std::net::SocketAddr;
use tokio::signal::unix;

pub struct Handler {}

impl Handler {
    pub async fn root(&self) -> &'static str {
        "Hello Textvid"
    }
}

pub struct Server {
    pub router: Router,
}

impl Server {
    pub async fn serve(self) -> anyhow::Result<()> {
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        tracing::info!("Listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(self.router.into_make_service())
            .with_graceful_shutdown(handle_signal())
            .await
            .context("serve")
    }
}

async fn handle_signal() {
    let mut term = unix::signal(unix::SignalKind::terminate()).unwrap();
    let mut int = unix::signal(unix::SignalKind::interrupt()).unwrap();
    tokio::select! {
        _ = term.recv() => {},
        _ = int.recv() => {},
    }
    tracing::info!("Shutting down ...");
}

pub struct Lambda {
    pub router: Router,
}

impl Lambda {
    pub async fn run(self) -> anyhow::Result<()> {
        let app = tower::ServiceBuilder::new()
            .layer(axum_aws_lambda::LambdaLayer::default())
            .service(self.router);
        lambda_http::run(app)
            .await
            .map_err(|e| anyhow!("lambda: {}", e))?;
        Ok(())
    }
}
