//! Price & technical indicators search node
use crate::workflow::nodes::utils::call_search_api;
use crate::workflow::{prompts, state::ReportState};
use tracing::{info, warn};

/// Searches price and technical indicators using Google Search via Search API.
///
/// # Errors
///
/// Returns an error if state transition errors occur.
pub async fn search_price_technicals(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!("[{}] Step 2.1: Search Price & Technicals", session_id);

    if state.rate_limit_stop {
        return Ok(state);
    }

    let prompt = prompts::process_placeholders(
        prompts::search_price_technicals::SEARCH_PRICE_TECHNICALS_PROMPT,
    );
    let full_prompt = if let Some(ref data) = state.realtime_data {
        prompt.replace("{{REAL_TIME_DATA}}", data)
    } else {
        prompt.replace("{{REAL_TIME_DATA}}", r#"{"notice": "No data"}"#)
    };

    match call_search_api(
        &state.api_key,
        &full_prompt,
        session_id,
        "search_price_technicals",
        true,
        false,
        None,
    )
    .await
    {
        Ok(response) => {
            info!("[{}] Price & technicals search completed", session_id);
            state.search_price_technicals = Some(response);
            state.success = true;
        }
        Err(e) => {
            let error_msg = format!("Search price technicals API call failed: {e}");
            warn!(
                "[{}] {} - applying partial degradation (fallback to N/A & realtime data)",
                session_id, error_msg
            );
            state.search_price_technicals = Some(
                "N/A - Technical price search unavailable. System will use realtime data.".to_string(),
            );
            state.add_error(&format!("[DEGRADED] {error_msg}"));
            state.success = true;
        }
    }
    Ok(state)
}
