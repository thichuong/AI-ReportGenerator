//! Database module
//!
//! Provides connection pool and model definitions.
//! Equivalent to `app/db/session.py` and `app/models/`

pub mod init;
pub mod models;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use tracing::info;

/// Creates a PostgreSQL connection pool.
///
/// Reads `DATABASE_URL` from environment variables.
/// Uses lazy connection to avoid blocking startup.
pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "postgresql://localhost/ai_report".to_string());

    info!(
        "Creating database pool for: {}...",
        &database_url[..database_url.len().min(50)]
    );

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .min_connections(0) // Don't require immediate connections
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect_lazy(&database_url)?;

    info!("Database pool created (lazy connection)");
    Ok(pool)
}

/// Gets a database session (connection from pool).
///
/// This is the Rust equivalent of Python's `get_db()` dependency.
pub fn get_db(pool: &PgPool) -> &PgPool {
    pool
}
