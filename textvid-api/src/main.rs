use anyhow;
use lambda_http;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    lambda_http::run(lambda_http::service_fn(handler))
        .await
        .map_err(|e| anyhow::anyhow!("lambda: {}", e))?;
    Ok(())
}

async fn handler(
    _req: lambda_http::Request,
) -> Result<impl lambda_http::IntoResponse, lambda_http::Error> {
    Ok(lambda_http::Response::builder()
        .status(200)
        .body("hoge")
        .unwrap())
}
