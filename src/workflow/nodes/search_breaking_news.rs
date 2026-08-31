//! Breaking news & institutional views search node
use crate::workflow::nodes::utils::{call_gemma_api, is_rate_limit_error};
use crate::workflow::{prompts, state::ReportState};
use tracing::{error, info};

/// Searches 24h breaking news and institutional views using Google Search via Gemma API.
///
/// # Errors
///
/// Returns an error if the API call fails or state transition errors occur.
pub async fn search_breaking_news(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!(
        "[{}] Step 2.9: Search Breaking News & Institutions",
        session_id
    );

    if state.rate_limit_stop {
        return Ok(state);
    }

    let prompt =
        prompts::process_placeholders(prompts::search_breaking_news::SEARCH_BREAKING_NEWS_PROMPT);
    let full_prompt = if let Some(ref data) = state.realtime_data {
        prompt.replace("{{REAL_TIME_DATA}}", data)
    } else {
        prompt.replace("{{REAL_TIME_DATA}}", r#"{"notice": "No data"}"#)
    };

    match call_gemma_api(
        &state.api_key,
        &full_prompt,
        session_id,
        "search_breaking_news",
        true,
        false,
        None,
    )
    .await
    {
        Ok(response) => {
            info!("[{}] Breaking news search completed", session_id);
            state.search_breaking_news = Some(response);
            state.success = true;
        }
        Err(e) => {
            let error_msg = format!("Search breaking news API call failed: {e}");
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
