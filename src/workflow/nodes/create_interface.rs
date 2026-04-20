//! Create interface node
//!
//! Creates HTML/CSS/JS interface for the report.
//! Equivalent to `app/services/workflow_nodes/create_interface.py`

use crate::workflow::nodes::utils::{ApiConfig, call_gemini_api, is_rate_limit_error};
use crate::workflow::{prompts, state::ReportState};
use tracing::{error, info};

/// Creates the HTML/CSS/JS interface for the report.
/// Creates the report interface using Gemini API.
///
/// # Errors
///
/// Returns an error if the API call fails or state transition errors occur.
pub async fn create_interface(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id;
    info!(
        "[{}] Step 5: Create interface (attempt {})",
        session_id,
        state.interface_attempt + 1
    );

    // Check rate limit flag
    if state.rate_limit_stop {
        error!(
            "[{}] Skipping interface creation - rate limit flag is set",
            session_id
        );
        return Ok(state);
    }

    // Get report content (prefer report_content, fallback to research_content)
    let report_md = state
        .report_content
        .as_ref()
        .or(state.research_content.as_ref())
        .cloned()
        .unwrap_or_default();

    if report_md.is_empty() {
        let error_msg = "No report content to create interface";
        error!("[{}] {}", session_id, error_msg);
        state.add_error(error_msg);
        state.success = false;
        return Ok(state);
    }

    // Get create_report prompt (hardcoded)
    let create_report_prompt = prompts::interface::CREATE_INTERFACE_PROMPT;

    let full_prompt = format!(
        "{create_report_prompt}\n\n---\n\n**NỘI DUNG BÁO CÁO CẦN XỬ LÝ:**\n\n{report_md}"
    );

    state.interface_attempt += 1;

    // Interface generation benefits from lower thinking level but slightly higher temperature
    let config = ApiConfig {
        temperature: 0.5,
        thinking_level: "MINIMAL".to_string(),
        ..Default::default()
    };

    // Call shared Gemini API utility
    match call_gemini_api(
        &state.api_key,
        &full_prompt,
        session_id,
        "interface",
        false,
        false,
        Some(config),
    )
    .await
    {
        Ok(response) => {
            info!("[{}] Interface code generated", session_id);
            state.interface_content = Some(response);
            state.success = true;
        }
        Err(e) => {
            let error_msg = format!("Interface creation failed: {e}");
            error!("[{}] {}", session_id, error_msg);

            if is_rate_limit_error(&e.to_string()) {
                state.rate_limit_stop = true;
                state.add_error("Rate limit encountered");
            } else {
                state.add_error(&error_msg);
            }
            state.success = false;
        }
    }

    Ok(state)
}
