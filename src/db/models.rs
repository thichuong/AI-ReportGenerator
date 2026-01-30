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

/// New article for insertion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewArticle {
    pub title: String,
    pub content: Option<String>,
    pub summary: Option<String>,
    pub symbol: Option<String>,
    pub report_type: Option<String>,
    pub is_published: Option<bool>,
}

impl Article {
    /// Inserts a new article into the database.
    pub async fn insert(pool: &sqlx::PgPool, article: NewArticle) -> Result<Self, sqlx::Error> {
        let result = sqlx::query_as::<_, Article>(
            r#"
            INSERT INTO articles (title, content, summary, symbol, report_type, is_published, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, COALESCE($6, false), NOW(), NOW())
            RETURNING id, title, content, summary, symbol, report_type, is_published, created_at, updated_at
            "#,
        )
        .bind(&article.title)
        .bind(&article.content)
        .bind(&article.summary)
        .bind(&article.symbol)
        .bind(&article.report_type)
        .bind(&article.is_published)
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    /// Finds an article by ID.
    pub async fn find_by_id(pool: &sqlx::PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query_as::<_, Article>(
            "SELECT id, title, content, summary, symbol, report_type, is_published, created_at, updated_at FROM articles WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }

    /// Lists articles with pagination.
    pub async fn list(pool: &sqlx::PgPool, limit: i64, offset: i64) -> Result<Vec<Self>, sqlx::Error> {
        let result = sqlx::query_as::<_, Article>(
            "SELECT id, title, content, summary, symbol, report_type, is_published, created_at, updated_at FROM articles ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        Ok(result)
    }

    /// Finds the latest article by symbol.
    pub async fn find_latest_by_symbol(pool: &sqlx::PgPool, symbol: &str) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query_as::<_, Article>(
            "SELECT id, title, content, summary, symbol, report_type, is_published, created_at, updated_at FROM articles WHERE symbol = $1 ORDER BY created_at DESC LIMIT 1",
        )
        .bind(symbol)
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }
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
