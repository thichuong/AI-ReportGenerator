//! Database models
//!
//! Equivalent to `app/models/__init__.py`

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Article model for investment reports.
///
/// Equivalent to Python's `Article` SQLAlchemy model.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub content: Option<String>,
    pub summary: Option<String>,
    /// Stock symbol (e.g., VNM, FPT)
    pub symbol: Option<String>,
    /// Report type (e.g., daily, weekly)
    pub report_type: Option<String>,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// Crypto report model for AI-generated content.
///
/// Equivalent to Python's `CryptoReport` SQLAlchemy model.
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct CryptoReport {
    pub id: i32,
    pub html_content: String,
    pub css_content: Option<String>,
    pub js_content: Option<String>,
    /// English HTML content (translated)
    pub html_content_en: Option<String>,
    /// English JS content (translated)
    pub js_content_en: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// New crypto report for insertion (without id).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCryptoReport {
    pub html_content: String,
    pub css_content: Option<String>,
    pub js_content: Option<String>,
    pub html_content_en: Option<String>,
    pub js_content_en: Option<String>,
}

impl CryptoReport {
    /// Inserts a new crypto report into the database.
    pub async fn insert(pool: &sqlx::PgPool, report: NewCryptoReport) -> Result<Self, sqlx::Error> {
        let result = sqlx::query_as::<_, CryptoReport>(
            r#"
            INSERT INTO crypto_report (html_content, css_content, js_content, html_content_en, js_content_en, created_at)
            VALUES ($1, $2, $3, $4, $5, NOW())
            RETURNING id, html_content, css_content, js_content, html_content_en, js_content_en, created_at
            "#,
        )
        .bind(&report.html_content)
        .bind(&report.css_content)
        .bind(&report.js_content)
        .bind(&report.html_content_en)
        .bind(&report.js_content_en)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    /// Finds a crypto report by ID.
    pub async fn find_by_id(pool: &sqlx::PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query_as::<_, CryptoReport>(
            "SELECT id, html_content, css_content, js_content, html_content_en, js_content_en, created_at FROM crypto_report WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    /// Gets the latest crypto report.
    pub async fn find_latest(pool: &sqlx::PgPool) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query_as::<_, CryptoReport>(
            "SELECT id, html_content, css_content, js_content, html_content_en, js_content_en, created_at FROM crypto_report ORDER BY created_at DESC LIMIT 1",
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }
}
