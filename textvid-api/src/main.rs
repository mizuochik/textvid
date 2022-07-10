use anyhow::{self, Context};
use axum::{routing, Router};
use http::Request;
use lambda_http::{tower::ServiceExt, Service};
use std::{net::SocketAddr, sync::Arc};

use tokio::{
    signal::unix::{self, SignalKind},
    sync::Mutex,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let app: Router = Router::new().route("/", routing::get(root));
    run_server(app).await
}

async fn run_lambda(r: Router) -> anyhow::Result<()> {
    let rr = Mutex::new(r);
    lambda_http::run(lambda_http::service_fn(
        |req: Request<lambda_http::Body>| async {
            let req = Arc::new(req).clone();
            let ax_body = match req.body() {
                lambda_http::Body::Text(t) => axum::body::Body::from(t.clone()),
                _ => axum::body::Body::empty(),
            };
            let ax_req = Request::builder()
                .method(req.method().as_str())
                .uri(req.uri())
                .header(
                    "Content-Type",
                    req.headers().get("Content-Type").context("content-type")?,
                )
                .body(ax_body)?;
            let ax_res = rr.lock().await.call(ax_req).await?;
            let b = ax_res.body();
            let lam_res = lambda_http::Response::builder()
                .status(ax_res.status())
                .body(lambda_http::Body::Text(String::from("hello")))?;
            Ok(lam_res)
        },
    ))
    .await
    .map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(())
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
