//! Spot Bitcoin ETF Flows search node
use crate::workflow::nodes::utils::call_gemma_api;
use crate::workflow::{prompts, state::ReportState};
use tracing::{info, warn};

/// Searches Spot Bitcoin ETF flows using Google Search via Gemma API.
///
/// # Errors
///
/// Returns an error if state transition errors occur.
pub async fn search_etf_flows(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!("[{}] Step 2.3: Search Spot Bitcoin ETF Flows", session_id);

    if state.rate_limit_stop {
        return Ok(state);
    }

    let prompt = prompts::process_placeholders(prompts::search_etf_flows::SEARCH_ETF_FLOWS_PROMPT);
    let full_prompt = if let Some(ref data) = state.realtime_data {
        prompt.replace("{{REAL_TIME_DATA}}", data)
    } else {
        prompt.replace("{{REAL_TIME_DATA}}", r#"{"notice": "No data"}"#)
    };

    match call_gemma_api(
        &state.api_key,
        &full_prompt,
        session_id,
        "search_etf_flows",
        true,
        false,
        None,
    )
    .await
    {
        Ok(response) => {
            info!("[{}] Spot Bitcoin ETF flows search completed", session_id);
            state.search_etf_flows = Some(response);
            state.success = true;
        }
        Err(e) => {
            let error_msg = format!("Search ETF flows API call failed: {e}");
            warn!(
                "[{}] {} - applying partial degradation (fallback to N/A)",
                session_id, error_msg
            );
            state.search_etf_flows =
                Some("N/A - Spot Bitcoin ETF flows data temporarily unavailable".to_string());
            state.add_error(&format!("[DEGRADED] {error_msg}"));
            state.success = true;
        }
    }
    Ok(state)
}
