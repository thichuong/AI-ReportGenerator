//! Price & technical indicators search node
use crate::workflow::nodes::utils::{call_gemma_api, is_rate_limit_error};
use crate::workflow::{prompts, state::ReportState};
use tracing::{error, info};

/// Searches price and technical indicators using Google Search via Gemma API.
///
/// # Errors
///
/// Returns an error if the API call fails or state transition errors occur.
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

    match call_gemma_api(
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
