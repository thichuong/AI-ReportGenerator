//! Report Synthesizer Node
use crate::workflow::nodes::utils::{call_gemini_flash_lite_api, is_rate_limit_error};
use crate::workflow::{prompts, state::ReportState};
use tracing::{error, info};

/// Generates the initial report content using Gemini 3.5 Flash Lite API.
///
/// # Errors
///
/// Returns an error if the API call fails or state transition errors occur.
pub async fn report_writer(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!("[{}] Step 2c: Report Synthesizer", session_id);

    if state.rate_limit_stop {
        return Ok(state);
    }

    let price_technicals = state
        .search_price_technicals
        .as_deref()
        .unwrap_or("No Technical Price Data");
    let btcd_altcoins = state
        .search_btcd_altcoins
        .as_deref()
        .unwrap_or("No BTC Dominance Data");
    let etf_flows = state
        .search_etf_flows
        .as_deref()
        .unwrap_or("No ETF Flow Data");
    let whale_onchain = state
        .search_whale_onchain
        .as_deref()
        .unwrap_or("No Whale On-Chain Data");
    let corporate_treasury = state
        .search_corporate_treasury
        .as_deref()
        .unwrap_or("No Corporate Treasury Data");
    let fear_greed = state
        .search_fear_greed
        .as_deref()
        .unwrap_or("No Fear & Greed Data");
    let macro_economy = state
        .search_macro_economy
        .as_deref()
        .unwrap_or("No Macro Economy Data");
    let regulatory_legal = state
        .search_regulatory_legal
        .as_deref()
        .unwrap_or("No Regulatory/Legal Data");
    let breaking_news = state
        .search_breaking_news
        .as_deref()
        .unwrap_or("No Breaking News Data");
    let events_calendar = state
        .search_events_calendar
        .as_deref()
        .unwrap_or("No Events Calendar Data");

    let prompt = prompts::process_placeholders(prompts::report_writer::WRITER_PROMPT);
    let mut full_prompt = prompt
        .replace("{{SEARCH_PRICE_TECHNICALS}}", price_technicals)
        .replace("{{SEARCH_BTCD_ALTCOINS}}", btcd_altcoins)
        .replace("{{SEARCH_ETF_FLOWS}}", etf_flows)
        .replace("{{SEARCH_WHALE_ONCHAIN}}", whale_onchain)
        .replace("{{SEARCH_CORPORATE_TREASURY}}", corporate_treasury)
        .replace("{{SEARCH_FEAR_GREED}}", fear_greed)
        .replace("{{SEARCH_MACRO_ECONOMY}}", macro_economy)
        .replace("{{SEARCH_REGULATORY_LEGAL}}", regulatory_legal)
        .replace("{{SEARCH_BREAKING_NEWS}}", breaking_news)
        .replace("{{SEARCH_EVENTS_CALENDAR}}", events_calendar);

    full_prompt = if let Some(ref data) = state.realtime_data {
        full_prompt.replace("{{REAL_TIME_DATA}}", data)
    } else {
        full_prompt.replace("{{REAL_TIME_DATA}}", r#"{"notice": "No data"}"#)
    };

    // No google search needed for the synthesizer, uses Gemini 3.5 Flash Lite
    match call_gemini_flash_lite_api(
        &state.api_key,
        &full_prompt,
        session_id,
        "writer",
        false,
        None,
    )
    .await
    {
        Ok(response) => {
            info!("[{}] Report synthesis completed", session_id);
            // Assign to research_content to maintain compatibility with downstream nodes
            state.research_content = Some(response);
            state.success = true;
        }
        Err(e) => {
            let error_msg = format!("Writer API call failed: {e}");
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
