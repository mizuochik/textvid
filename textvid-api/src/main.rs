use anyhow::anyhow;

mod di;
mod infra;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let d = di::DI::new();
    if std::env::var("AWS_LAMBDA_RUNTIME_API").is_ok() {
        Err(anyhow!("not implemented"))
    } else {
        d.server.serve().await
    }
}
