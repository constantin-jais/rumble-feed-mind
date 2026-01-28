//! FeedMind API Server

use std::net::SocketAddr;
use axum::Router;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod config;
mod error;
mod routes;
mod state;
mod middleware;
mod extractors;

use config::AppConfig;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "feedmind_api=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    // Load configuration
    let config = AppConfig::load()?;
    info!("Configuration loaded");

    // Create app state
    let state = AppState::new(&config).await?;
    info!("App state initialized");

    // Build router
    let app = build_router(state);

    // Start server
    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    info!(%addr, "Starting server");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn build_router(state: AppState) -> Router {
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any) // TODO: Restrict in production
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(routes::health::router())
        .merge(routes::auth::router())
        .merge(routes::feeds::router())
        .merge(routes::folders::router())
        .merge(routes::articles::router())
        .merge(routes::categories::router())
        .merge(routes::rules::router())
        .merge(routes::tags::router())
        .merge(routes::opml::router())
        .merge(routes::billing::router())
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}
