//! Macroeconomic & Central Bank search node
use crate::workflow::nodes::utils::{call_gemma_api, is_rate_limit_error};
use crate::workflow::{prompts, state::ReportState};
use tracing::{error, info};

/// Searches macroeconomic indicators and central bank policies using Google Search via Gemma API.
///
/// # Errors
///
/// Returns an error if the API call fails or state transition errors occur.
pub async fn search_macro_economy(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!(
        "[{}] Step 2.7: Search Macro Economy & Fed Policy",
        session_id
    );

    if state.rate_limit_stop {
        return Ok(state);
    }

    let prompt =
        prompts::process_placeholders(prompts::search_macro_economy::SEARCH_MACRO_ECONOMY_PROMPT);
    let full_prompt = if let Some(ref data) = state.realtime_data {
        prompt.replace("{{REAL_TIME_DATA}}", data)
    } else {
        prompt.replace("{{REAL_TIME_DATA}}", r#"{"notice": "No data"}"#)
    };

    match call_gemma_api(
        &state.api_key,
        &full_prompt,
        session_id,
        "search_macro_economy",
        true,
        false,
        None,
    )
    .await
    {
        Ok(response) => {
            info!("[{}] Macro economy search completed", session_id);
            state.search_macro_economy = Some(response);
            state.success = true;
        }
        Err(e) => {
            let error_msg = format!("Search macro economy API call failed: {e}");
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
