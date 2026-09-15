//! Whale & On-chain metrics search node
use crate::workflow::nodes::utils::call_search_api;
use crate::workflow::{prompts, state::ReportState};
use tracing::{info, warn};

/// Searches whale activities and on-chain metrics using Google Search via Search API.
///
/// # Errors
///
/// Returns an error if state transition errors occur.
pub async fn search_whale_onchain(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!("[{}] Step 2.4: Search Whale & On-chain Data", session_id);

    if state.rate_limit_stop {
        return Ok(state);
    }

    let prompt =
        prompts::process_placeholders(prompts::search_whale_onchain::SEARCH_WHALE_ONCHAIN_PROMPT);
    let full_prompt = if let Some(ref data) = state.realtime_data {
        prompt.replace("{{REAL_TIME_DATA}}", data)
    } else {
        prompt.replace("{{REAL_TIME_DATA}}", r#"{"notice": "No data"}"#)
    };

    match call_search_api(
        &state.api_key,
        &full_prompt,
        session_id,
        "search_whale_onchain",
        true,
        false,
        None,
    )
    .await
    {
        Ok(response) => {
            info!("[{}] Whale & on-chain search completed", session_id);
            state.search_whale_onchain = Some(response);
            state.success = true;
        }
        Err(e) => {
            let error_msg = format!("Search whale on-chain API call failed: {e}");
            warn!(
                "[{}] {} - applying partial degradation (fallback to N/A)",
                session_id, error_msg
            );
            state.search_whale_onchain =
                Some("N/A - Whale & On-Chain data temporarily unavailable".to_string());
            state.add_error(&format!("[DEGRADED] {error_msg}"));
            state.success = true;
        }
    }
    Ok(state)
}
