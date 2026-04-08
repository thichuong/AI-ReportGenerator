//! Report Synthesizer Node
use crate::workflow::{prompts, state::ReportState};
use crate::workflow::nodes::utils::{call_gemini_api, is_rate_limit_error};
use tracing::{error, info};

pub async fn report_writer(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!("[{}] Step 2c: Report Synthesizer", session_id);

    if state.rate_limit_stop { return Ok(state); }

    let tech_content = state.tech_analysis_content.as_deref().unwrap_or("No Technical Data");
    let macro_content = state.macro_analysis_content.as_deref().unwrap_or("No Macro Data");

    let prompt = prompts::process_placeholders(prompts::report_writer::WRITER_PROMPT);
    let mut full_prompt = prompt.replace("{{TECH_CONTENT}}", tech_content)
                                .replace("{{MACRO_CONTENT}}", macro_content);
                                
    full_prompt = if let Some(ref data) = state.realtime_data {
        full_prompt.replace("{{REAL_TIME_DATA}}", data)
    } else {
        full_prompt.replace("{{REAL_TIME_DATA}}", r#"{"notice": "No data"}"#)
    };

    // No google search needed for the synthesizer
    match call_gemini_api(&state.api_key, &full_prompt, session_id, "writer", false, false, None).await {
        Ok(response) => {
            info!("[{}] Report synthesis completed", session_id);
            // Assign to research_content to maintain compatibility with downstream nodes
            state.research_content = Some(response);
            state.success = true;
        }
        Err(e) => {
            let error_msg = format!("Writer API call failed: {}", e);
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
