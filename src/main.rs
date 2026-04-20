//! Main entry point for the AI Report Generator
//!
//! Starts the Axum web server and auto report scheduler.

use ai_report_generator::{AppState, create_pool, create_router, db, start_auto_report_scheduler};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    info!("🚀 Starting AI Report Generator (Rust)...");

    // Create database connection pool
    let pool = create_pool().await?;
    info!("✅ Database connection pool created");

    // Initialize database schema
    db::init::init_db(&pool).await?;

    // Create application state
    let state = AppState::new(pool.clone());

    // Start auto report scheduler if enabled
    if start_auto_report_scheduler(pool.clone()) {
        info!("✅ Auto report scheduler started");
    } else {
        info!("ℹ️ Auto report scheduler not started (check environment variables)");
    }

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create router with API routes and static file serving
    let app = create_router(state)
        .nest_service("/static", ServeDir::new("src/static"))
        .layer(cors);

    // Get bind address from environment or use default
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::new(host.parse()?, port);
    info!("🌐 Server listening on http://{}", addr);

    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(addr).await?;

    // Serve the application
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("👋 Server shut down gracefully");
    Ok(())
}

/// Graceful shutdown handler.
async fn shutdown_signal() {
    if let Err(e) = tokio::signal::ctrl_c().await {
        tracing::error!("Failed to install CTRL+C signal handler: {}", e);
    }
    info!("🛑 Shutdown signal received");
}
