use anyhow::{anyhow, Context};
use axum::{self, body::HttpBody, routing::Router};
use lambda_http::Service;
use std::{net::SocketAddr, sync::Arc};
use tokio::signal::unix;
use tokio::sync::Mutex;

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
        let r = Arc::new(Mutex::new(self.router));
        lambda_http::run(lambda_http::service_fn(|lam_req: lambda_http::Request| {
            let r = r.clone();
            async move {
                let ax_req = http::Request::builder()
                    .uri(lam_req.uri())
                    .method(lam_req.method())
                    .body(match lam_req.body() {
                        lambda_http::Body::Binary(b) => axum::body::Body::from(b.clone()),
                        lambda_http::Body::Text(t) => axum::body::Body::from(t.clone()),
                        _ => axum::body::Body::empty(),
                    })?;
                let mut r = r.lock().await;
                let mut ax_res = r.call(ax_req).await?;
                let lam_res = lambda_http::Response::builder()
                    .status(ax_res.status())
                    .body(match ax_res.body_mut().data().await {
                        Some(Ok(b)) => lambda_http::Body::Binary(b.to_vec()),
                        _ => lambda_http::Body::Empty,
                    })?;
                Ok(lam_res)
            }
        }))
        .await
        .map_err(|e| anyhow!("lambda: {}", e))
    }
}
