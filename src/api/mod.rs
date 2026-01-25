//! API module with Axum framework
//!
//! Provides REST API endpoints for report generation.
//! Equivalent to `app/routers/`

pub mod handlers;
pub mod progress;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Application state shared across handlers.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub progress_tracker: Arc<RwLock<progress::ProgressTracker>>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            progress_tracker: Arc::new(RwLock::new(progress::ProgressTracker::new())),
        }
    }
}

/// Creates the API router with all endpoints.
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Root redirect to dashboard
        .route("/", get(handlers::root_redirect))
        // Health check
        .route("/health", get(handlers::health_check))
        // API v1 routes
        .route(
            "/api/v1/generate-auto-report",
            post(handlers::generate_auto_report),
        )
        .route("/api/v1/progress/{session_id}", get(handlers::get_progress))
        .route("/api/v1/manual-generate", post(handlers::manual_generate))
        .route("/api/v1/scheduler/status", get(handlers::scheduler_status))
        .route("/api/v1/reports/latest", get(handlers::get_latest_report))
        .with_state(state)
}
