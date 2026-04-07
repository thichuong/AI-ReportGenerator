//! Tech deep node
use crate::workflow::{prompts, state::ReportState};
use crate::workflow::nodes::utils::{call_gemini_api, is_rate_limit_error};
use tracing::{error, info};

pub async fn research_tech(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!("[{}] Step 2a: Research Tech & On-chain", session_id);

    if state.rate_limit_stop { return Ok(state); }

    let prompt = prompts::process_placeholders(prompts::research_tech::TECH_PROMPT);
    let full_prompt = if let Some(ref data) = state.realtime_data {
        prompt.replace("{{REAL_TIME_DATA}}", data)
    } else {
        prompt.replace("{{REAL_TIME_DATA}}", r#"{"notice": "No data"}"#)
    };

    match call_gemini_api(&state.api_key, &full_prompt, session_id, "tech", true, false).await {
        Ok(response) => {
            info!("[{}] Tech research completed", session_id);
            state.tech_analysis_content = Some(response);
            state.success = true;
        }
        Err(e) => {
            let error_msg = format!("Tech API call failed: {}", e);
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
