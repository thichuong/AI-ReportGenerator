//! Prepare data node
//!
//! Initializes data and reads prompts from environment.
//! Equivalent to `app/services/workflow_nodes/prepare_data.py`

use crate::workflow::state::ReportState;
use chrono::Utc;
use std::env;
use tracing::{error, info};

/// Prepares data and initializes the workflow state.
///
/// - Validates API key
/// - Reads prompts from environment variables
/// - Fetches real-time data from Redis (if available)
pub async fn prepare_data(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id;
    info!("[{}] Step 1: Preparing data", session_id);

    // Validate API key
    if state.api_key.is_empty() {
        let error_msg = "API key is invalid or empty";
        error!("[{}] {}", session_id, error_msg);
        state.add_error(error_msg);
        state.success = false;
        return Ok(state);
    }

    // Read prompts from environment variables
    state.research_analysis_prompt = get_prompt_from_env("COMBINED_RESEARCH_VALIDATION_PROMPT");
    state.data_validation_prompt = get_prompt_from_env("DATA_VALIDATION_PROMPT");
    state.create_report_prompt = get_prompt_from_env("CREATE_REPORT_PROMPT");

    // Validate required prompts
    if state.research_analysis_prompt.is_none() {
        let error_msg = "Cannot read combined research validation prompt from environment";
        error!("[{}] {}", session_id, error_msg);
        state.add_error(error_msg);
        state.success = false;
        return Ok(state);
    }

    // Replace date placeholders in prompts
    if let Some(ref mut prompt) = state.research_analysis_prompt {
        *prompt = replace_date_placeholders(prompt);
    }

    // Fetch real-time data from Redis
    info!("[{}] Fetching real-time data...", session_id);
    match get_realtime_data().await {
        Ok(Some(data)) => {
            info!("[{}] Real-time data cached successfully", session_id);
            state.realtime_data = Some(data);
        }
        Ok(None) => {
            info!(
                "[{}] No real-time data available, will use fallback",
                session_id
            );
        }
        Err(e) => {
            info!("[{}] Failed to fetch real-time data: {}", session_id, e);
        }
    }

    state.current_attempt = 0;
    state.success = true;
    info!("[{}] Data preparation completed", session_id);

    Ok(state)
}

/// Gets prompt from environment variable.
fn get_prompt_from_env(name: &str) -> Option<String> {
    env::var(name).ok().filter(|s| !s.is_empty())
}

/// Replaces date placeholders in prompt text.
fn replace_date_placeholders(text: &str) -> String {
    let now = Utc::now();
    text.replace("{{DATE}}", &now.format("%Y-%m-%d").to_string())
        .replace("{{YEAR}}", &now.format("%Y").to_string())
        .replace("{{MONTH}}", &now.format("%m").to_string())
        .replace("{{DAY}}", &now.format("%d").to_string())
}

/// Fetches real-time market data from Redis.
async fn get_realtime_data() -> Result<Option<String>, anyhow::Error> {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());

    let client = redis::Client::open(redis_url)?;
    let mut con = client.get_multiplexed_async_connection().await?;

    // Read from Redis Stream 'market_data_stream'
    let result: redis::RedisResult<Vec<redis::streams::StreamReadReply>> = redis::cmd("XREAD")
        .arg("COUNT")
        .arg(1)
        .arg("STREAMS")
        .arg("market_data_stream")
        .arg("0")
        .query_async(&mut con)
        .await;

    match result {
        Ok(streams) => {
            if let Some(stream) = streams.first() {
                if let Some(entry) = stream.keys.first().and_then(|k| k.ids.first()) {
                    // Extract data from stream entry
                    if let Some(data) = entry.map.get("data") {
                        if let redis::Value::BulkString(bytes) = data {
                            return Ok(Some(String::from_utf8_lossy(bytes).to_string()));
                        }
                    }
                }
            }
            Ok(None)
        }
        Err(_) => Ok(None),
    }
}
