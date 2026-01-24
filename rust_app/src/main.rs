use axum::{routing::get, Router};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1. Load environment variables
    dotenv().ok();

    // 2. Setup tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🚀 Starting AI Report Generator (Rust)...");

    // 3. Connect to Database
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Note: We use PgPoolOptions for Postgres. If SQLite is needed, we'd use SqlitePoolOptions.
    // Based on analysis, the project supports both but optimized for Postgres deploy.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to database: {}", e))?;

    tracing::info!("✅ Database connected successfully");

    // 4. Build application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(pool);

    // 5. Run server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "healthy"
}
