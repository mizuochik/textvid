use std::future::Future;

use anyhow;
use lambda_http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    run_lambda(handler).await?;
    Ok(())
}

async fn run_lambda<F, R, T>(f: F) -> anyhow::Result<()>
where
    R: lambda_http::IntoResponse,
    F: Fn(lambda_http::Request) -> T,
    T: Future<Output = Result<R, lambda_http::Error>>,
{
    lambda_http::run(lambda_http::service_fn(f))
        .await
        .map_err(|e| anyhow::anyhow!("lambda: {}", e))
}

async fn handler(
    _req: lambda_http::Request,
) -> Result<impl lambda_http::IntoResponse, lambda_http::Error> {
    Ok(lambda_http::Response::builder()
        .status(200)
        .body("hoge")
        .unwrap())
}
