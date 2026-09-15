//! Upcoming events calendar search node
use crate::workflow::nodes::utils::call_search_api;
use crate::workflow::{prompts, state::ReportState};
use tracing::{info, warn};

/// Searches upcoming 7-day economic and crypto calendar using Google Search via Search API.
///
/// # Errors
///
/// Returns an error if state transition errors occur.
pub async fn search_events_calendar(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!("[{}] Step 2.10: Search 7-Day Events Calendar", session_id);

    if state.rate_limit_stop {
        return Ok(state);
    }

    let prompt = prompts::process_placeholders(
        prompts::search_events_calendar::SEARCH_EVENTS_CALENDAR_PROMPT,
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
        "search_events_calendar",
        true,
        false,
        None,
    )
    .await
    {
        Ok(response) => {
            info!("[{}] Events calendar search completed", session_id);
            state.search_events_calendar = Some(response);
            state.success = true;
        }
        Err(e) => {
            let error_msg = format!("Search events calendar API call failed: {e}");
            warn!(
                "[{}] {} - applying partial degradation (fallback to N/A)",
                session_id, error_msg
            );
            state.search_events_calendar =
                Some("N/A - 7-day events calendar temporarily unavailable".to_string());
            state.add_error(&format!("[DEGRADED] {error_msg}"));
            state.success = true;
        }
    }
    Ok(state)
}
