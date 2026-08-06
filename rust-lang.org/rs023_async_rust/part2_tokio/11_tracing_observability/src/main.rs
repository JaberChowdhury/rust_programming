// CONCEPT: Structured logging and distributed tracing.

use tracing::{debug, info, instrument};

#[instrument]
async fn do_work(id: u32) {
    debug!("Starting work for {}", id);
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    info!("Finished work for {}", id);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("App starting up");
    do_work(42).await;
    info!("App shutting down");
}
// EXERCISE:
// 1. Add fields to #[instrument].
