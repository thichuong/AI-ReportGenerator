//! API request handlers
//!
//! Equivalent to `app/routers/reports.py`

use super::{progress::ProgressData, AppState};
use crate::db::models::CryptoReport;
use crate::scheduler::create_manual_report;
use crate::workflow;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Serialize;
use serde_json::json;
use std::env;
use uuid::Uuid;

/// Response for health check.
#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

/// Response for generate report.
#[derive(Serialize)]
pub struct GenerateResponse {
    pub success: bool,
    pub message: String,
    pub session_id: String,
}

/// Response for scheduler status.
#[derive(Serialize)]
pub struct SchedulerStatusResponse {
    pub scheduler_enabled: bool,
    pub api_key_configured: bool,
    pub max_attempts: u32,
}

/// Health check endpoint.
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
    })
}

/// Root redirect to dashboard.
pub async fn root_redirect() -> axum::response::Redirect {
    axum::response::Redirect::permanent("/static/index.html")
}

/// Generates a report in the background.
pub async fn generate_auto_report(
    State(state): State<AppState>,
) -> Result<Json<GenerateResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Check API key
    let api_key = env::var("GEMINI_API_KEY").map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "error": "GEMINI_API_KEY not configured"
            })),
        )
    })?;

    // Generate session ID
    let session_id = Uuid::new_v4().to_string();

    // Start progress tracking
    {
        let mut tracker = state.progress_tracker.write().await;
        tracker.start_progress(&session_id);
    }

    // Clone state for background task
    let pool = state.pool.clone();
    let progress_tracker = state.progress_tracker.clone();
    let session_id_clone = session_id.clone();

    // Spawn background task
    tokio::spawn(async move {
        let max_attempts = env::var("MAX_REPORT_ATTEMPTS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3);

        match workflow::run_workflow(&pool, &api_key, max_attempts).await {
            Ok(result) => {
                let mut tracker = progress_tracker.write().await;
                if result.success {
                    tracker.complete_progress(&session_id_clone, result.report_id);
                    tracing::info!(
                        "Workflow completed successfully for session {}",
                        session_id_clone
                    );
                } else {
                    let error = result.error_messages.join(", ");
                    tracker.error_progress(&session_id_clone, &error);
                    tracing::error!(
                        "Workflow failed for session {}: {}",
                        session_id_clone,
                        error
                    );
                }
            }
            Err(e) => {
                let mut tracker = progress_tracker.write().await;
                tracker.error_progress(&session_id_clone, &e.to_string());
                tracing::error!("Workflow error for session {}: {}", session_id_clone, e);
            }
        }
    });

    Ok(Json(GenerateResponse {
        success: true,
        message: "Đã bắt đầu tạo báo cáo, theo dõi tiến độ qua API".to_string(),
        session_id,
    }))
}

/// Gets progress for a session.
pub async fn get_progress(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> Result<Json<ProgressData>, (StatusCode, Json<serde_json::Value>)> {
    let tracker = state.progress_tracker.read().await;

    tracker.get_progress(&session_id).map(Json).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "error": "Session not found"
            })),
        )
    })
}

/// Generates a report synchronously (blocking).
pub async fn manual_generate(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match create_manual_report(&state.pool).await {
        Ok(report_id) => Ok(Json(json!({
            "success": true,
            "message": format!("Báo cáo #{} được tạo thành công", report_id),
            "report_id": report_id
        }))),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": e.to_string()
            })),
        )),
    }
}

/// Gets scheduler status.
pub async fn scheduler_status() -> Json<SchedulerStatusResponse> {
    Json(SchedulerStatusResponse {
        scheduler_enabled: env::var("ENABLE_AUTO_REPORT_SCHEDULER")
            .map(|v| v.to_lowercase() == "true")
            .unwrap_or(false),
        api_key_configured: env::var("GEMINI_API_KEY").is_ok(),
        max_attempts: env::var("MAX_REPORT_ATTEMPTS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3),
    })
}

/// Gets the latest report.
pub async fn get_latest_report(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    match CryptoReport::find_latest(&state.pool).await {
        Ok(Some(report)) => Ok(Json(json!({
            "success": true,
            "report": {
                "id": report.id,
                "created_at": report.created_at,
                "has_html": !report.html_content.is_empty(),
                "has_css": report.css_content.is_some(),
                "has_js": report.js_content.is_some(),
                "has_english": report.html_content_en.is_some()
            }
        }))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "error": "No reports found"
            })),
        )),
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "error": e.to_string()
            })),
        )),
    }
}
