//! Macro deep node
use crate::workflow::nodes::utils::{call_gemini_api, is_rate_limit_error};
use crate::workflow::{prompts, state::ReportState};
use tracing::{error, info};

/// Researches macro-economic data using Gemini API.
///
/// # Errors
///
/// Returns an error if the API call fails or state transition errors occur.
pub async fn research_macro(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!("[{}] Step 2b: Research Macro & Sentiment", session_id);

    if state.rate_limit_stop {
        return Ok(state);
    }

    let prompt = prompts::process_placeholders(prompts::research_macro::MACRO_PROMPT);
    let full_prompt = if let Some(ref data) = state.realtime_data {
        prompt.replace("{{REAL_TIME_DATA}}", data)
    } else {
        prompt.replace("{{REAL_TIME_DATA}}", r#"{"notice": "No data"}"#)
    };

    match call_gemini_api(
        &state.api_key,
        &full_prompt,
        session_id,
        "macro",
        true,
        false,
        None,
    )
    .await
    {
        Ok(response) => {
            info!("[{}] Macro research completed", session_id);
            state.macro_analysis_content = Some(response);
            state.success = true;
        }
        Err(e) => {
            let error_msg = format!("Macro API call failed: {e}");
            error!("[{}] {}", session_id, error_msg);
            if is_rate_limit_error(&e.to_string()) {
                state.rate_limit_stop = true;
            }
            state.add_error(&error_msg);
            state.success = false;
        }
    }
    Ok(state)
}
