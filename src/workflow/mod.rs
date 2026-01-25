//! Workflow module
//!
//! Provides AI report generation workflow orchestration.
//! Equivalent to `app/services/report_workflow.py`

pub mod nodes;
pub mod routing;
pub mod state;

use crate::db::models::{CryptoReport, NewCryptoReport};
use routing::{should_retry_interface_or_continue, should_retry_or_continue, RoutingDecision};
use sqlx::PgPool;
use state::ReportState;
use tracing::{error, info};
use uuid::Uuid;

/// Runs the complete report generation workflow.
///
/// This is equivalent to Python's `generate_auto_research_report_langgraph_v2`.
pub async fn run_workflow(
    pool: &PgPool,
    api_key: &str,
    max_attempts: u32,
) -> Result<ReportState, anyhow::Error> {
    let session_id = Uuid::new_v4().to_string();
    let mut state = ReportState::new(&session_id, api_key, max_attempts);

    info!("[{}] Starting report workflow v2", session_id);

    // Step 1: Prepare data
    state = nodes::prepare_data(state).await?;
    if !state.success {
        error!("[{}] Prepare data failed", session_id);
        return Ok(state);
    }

    // Research and validation loop
    loop {
        // Step 2: Research deep
        state = nodes::research_deep(state).await?;

        // Step 3: Validate report
        state = nodes::validate_report(state).await?;

        // Routing decision
        match should_retry_or_continue(&state) {
            RoutingDecision::Continue => {
                info!("[{}] Validation PASSED, continuing...", session_id);
                break;
            }
            RoutingDecision::Retry => {
                state.current_attempt += 1;
                info!(
                    "[{}] Validation FAILED, retrying ({}/{})",
                    session_id, state.current_attempt, state.max_attempts
                );
                continue;
            }
            RoutingDecision::End => {
                error!("[{}] Max attempts reached, ending workflow", session_id);
                return Ok(state);
            }
            _ => unreachable!(),
        }
    }

    // Step 4: Generate report content
    state = nodes::generate_content(state).await?;

    // Interface creation loop
    loop {
        // Step 5: Create interface
        state = nodes::create_interface(state).await?;

        // Step 6: Extract code
        state = nodes::extract_code(state).await?;

        match should_retry_interface_or_continue(&state) {
            RoutingDecision::Continue => {
                info!("[{}] Interface created successfully", session_id);
                break;
            }
            RoutingDecision::RetryInterface => {
                state.interface_attempt += 1;
                info!(
                    "[{}] Interface creation failed, retrying ({}/3)",
                    session_id, state.interface_attempt
                );
                continue;
            }
            RoutingDecision::End => {
                error!("[{}] Max interface attempts reached", session_id);
                return Ok(state);
            }
            _ => unreachable!(),
        }
    }

    // Step 7: Translate content
    state = nodes::translate(state).await?;

    // Step 8: Save to database
    state = save_to_database(pool, state).await?;

    info!(
        "[{}] Workflow completed. Report ID: {:?}",
        session_id, state.report_id
    );
    Ok(state)
}

/// Saves the report to database.
///
/// Equivalent to `save_database_node` in Python.
async fn save_to_database(
    pool: &PgPool,
    mut state: ReportState,
) -> Result<ReportState, anyhow::Error> {
    // Check rate limit flag
    if state.rate_limit_stop {
        error!(
            "[{}] Skipping save_database - rate limit flag is set",
            state.session_id
        );
        state.success = false;
        state
            .error_messages
            .push("Skipped save due to rate limit".to_string());
        return Ok(state);
    }

    // Validate required fields
    let html_content = match &state.html_content {
        Some(content) if !content.trim().is_empty() => content.clone(),
        _ => {
            state.success = false;
            state
                .error_messages
                .push("HTML content is missing or empty".to_string());
            return Ok(state);
        }
    };

    let new_report = NewCryptoReport {
        html_content,
        css_content: state.css_content.clone(),
        js_content: state.js_content.clone(),
        html_content_en: state.html_content_en.clone(),
        js_content_en: state.js_content_en.clone(),
    };

    match CryptoReport::insert(pool, new_report).await {
        Ok(report) => {
            info!(
                "[{}] Report saved to database with ID: {}",
                state.session_id, report.id
            );
            state.report_id = Some(report.id);
            state.success = true;
        }
        Err(e) => {
            error!("[{}] Failed to save report: {}", state.session_id, e);
            state.success = false;
            state
                .error_messages
                .push(format!("Database save error: {}", e));
        }
    }

    Ok(state)
}
