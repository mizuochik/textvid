mod di;
mod infra;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let d = di::DI::new();
    if std::env::var("AWS_LAMBDA_RUNTIME_API").is_ok() {
        d.lambda.run().await
    } else {
        d.server.serve().await
    }
}
