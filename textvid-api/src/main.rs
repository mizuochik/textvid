mod di;
mod infra;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    di::DI::new().server.serve().await
}
