//! Validate report node
//!
//! Validates the research content quality via AI Validator.

use crate::workflow::nodes::utils::{call_gemini_api, is_rate_limit_error};
use crate::workflow::{prompts, state::ReportState};
use tracing::{error, info, warn};

/// Validates the research report content.
/// Validates the generated report for quality and consistency.
///
/// # Errors
///
/// Returns an error if the validation API call fails or state transition errors occur.
pub async fn validate_report(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!("[{}] Step 3: Validate report via AI Validator", session_id);

    // Check rate limit flag
    if state.rate_limit_stop {
        warn!(
            "[{}] Skipping validation - rate limit flag is set",
            session_id
        );
        state.validation_result = Some("SKIP".to_string());
        return Ok(state);
    }

    // Get research content
    let research_content = match &state.research_content {
        Some(content) if !content.is_empty() => content.clone(),
        _ => {
            warn!("[{}] No research content to validate", session_id);
            state.validation_result = Some("FAIL".to_string());
            state.add_error("No research content available for validation");
            return Ok(state);
        }
    };

    let prompt = prompts::process_placeholders(prompts::report_validator::VALIDATOR_PROMPT);
    let mut full_prompt = prompt.replace("{{REPORT_CONTENT}}", &research_content);
    full_prompt = if let Some(ref data) = state.realtime_data {
        full_prompt.replace("{{REAL_TIME_DATA}}", data)
    } else {
        full_prompt.replace(
            "{{REAL_TIME_DATA}}",
            r#"{"notice": "No realtime data available for validation"}"#,
        )
    };

    // Call API with JSON format enabled
    match call_gemini_api(
        &state.api_key,
        &full_prompt,
        session_id,
        "validator",
        false,
        true,
        None,
    )
    .await
    {
        Ok(json_response) => {
            info!("[{}] AI Validation query completed", session_id);

            // Clean up potentially malformed json (e.g. wrapped in ```json)
            let cleaned_json = json_response
                .trim()
                .trim_start_matches("```json")
                .trim_start_matches("```")
                .trim_end_matches("```")
                .trim();

            // Try to parse the JSON
            let parsed: Result<serde_json::Value, _> = serde_json::from_str(cleaned_json);
            match parsed {
                Ok(val) => {
                    let status = val
                        .get("status")
                        .and_then(|s| s.as_str())
                        .unwrap_or("UNKNOWN")
                        .to_uppercase();
                    let reasoning = val
                        .get("reasoning")
                        .and_then(|r| r.as_str())
                        .unwrap_or("No reasoning provided");

                    info!(
                        "[{}] Validation Status: {}, Reasoning: {}",
                        session_id, status, reasoning
                    );
                    state.validation_result = Some(status.clone());

                    if status == "FAIL" {
                        state.success = false;
                        state.add_error(&format!("Validation failed: {reasoning}"));
                    }
                }
                Err(e) => {
                    warn!(
                        "[{}] Failed to parse validation JSON: {}, raw: {}",
                        session_id, e, json_response
                    );
                    // Fallback using string matching just in case JSON parsing fails
                    if json_response.to_uppercase().contains("\"PASS\"") {
                        state.validation_result = Some("PASS".to_string());
                    } else {
                        state.validation_result = Some("FAIL".to_string());
                        state.success = false;
                        state.add_error("Validation JSON format error");
                    }
                }
            }
        }
        Err(e) => {
            let error_msg = format!("Validator API call failed: {e}");
            error!("[{}] {}", session_id, error_msg);
            if is_rate_limit_error(&e.to_string()) {
                state.rate_limit_stop = true;
            }
            state.validation_result = Some("FAIL".to_string());
            state.add_error(&error_msg);
            state.success = false;
        }
    }

    Ok(state)
}
