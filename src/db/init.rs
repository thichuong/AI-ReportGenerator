//! Database initialization
//!
//! Handles schema creation on startup.

use sqlx::PgPool;
use tracing::info;

/// Initializes the database schema.
///
/// Creates tables if they do not exist.
pub async fn init_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    info!("Checking database schema...");

    // Create articles table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS articles (
            id SERIAL PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT,
            summary TEXT,
            symbol VARCHAR(20),
            report_type VARCHAR(50),
            is_published BOOLEAN NOT NULL DEFAULT FALSE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ
        );
        "#,
    )
    .execute(pool)
    .await?;
    info!("✅ Table 'articles' checked/created");

    // Create crypto_report table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS crypto_report (
            id SERIAL PRIMARY KEY,
            html_content TEXT NOT NULL,
            css_content TEXT,
            js_content TEXT,
            html_content_en TEXT,
            js_content_en TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        "#,
    )
    .execute(pool)
    .await?;
    info!("✅ Table 'crypto_report' checked/created");

    info!("Database initialized successfully");
    Ok(())
}
