//! FeedMind Background Worker

use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod jobs;
mod queue;
mod scheduler;

use config::WorkerConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "feedmind_worker=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    // Load configuration
    let config = WorkerConfig::load()?;
    info!("Worker configuration loaded");

    // Initialize queue consumer
    let mut consumer = queue::QueueConsumer::new(&config).await?;
    info!("Queue consumer initialized");

    // Start scheduler for periodic tasks
    let scheduler = scheduler::Scheduler::new(&config).await?;
    scheduler.start().await?;
    info!("Scheduler started");

    // Run consumer (blocking)
    info!("Starting worker...");
    consumer.run().await?;

    Ok(())
}
