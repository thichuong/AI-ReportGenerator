//! Equivalent to the previous Python implementation in `app/`.

use crate::workflow::state::ReportState;
use multi_tier_cache::StreamingBackend;
use tracing::{error, info, warn};
use std::env;

/// Prepares data and initializes the workflow state.
///
/// - Validates API key
/// - Reads prompts from environment variables (with file fallback)
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

    // Prepare data completed
    state.current_attempt = 0;

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

/// Fetches real-time market data from Redis Stream using multi-tier-cache.
async fn get_realtime_data() -> Result<Option<String>, anyhow::Error> {
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    info!(
        "📡 Connecting to Redis: {}",
        &redis_url[..redis_url.len().min(30)]
    );

    // Use multi-tier-cache RedisStreams
    let streams = match multi_tier_cache::redis_streams::RedisStreams::new(&redis_url).await {
        Ok(s) => {
            info!("✅ Redis connection established");
            s
        }
        Err(e) => {
            warn!("❌ Failed to connect to Redis: {}", e);
            return Ok(None);
        }
    };

    // Read latest entry from 'market_data_stream'
    info!("📥 Reading from stream 'market_data_stream'...");
    match streams.stream_read_latest("market_data_stream", 1).await {
        Ok(entries) => {
            if let Some(entry) = entries.first() {
                info!("✅ Got stream entry with ID: {}", entry.0);

                // Find 'data' field in entry
                for (field, value) in &entry.1 {
                    if field == "data" {
                        info!("📊 Data field found, length: {} bytes", value.len());

                        // Log preview of data
                        let preview = if value.len() > 200 {
                            format!("{}...", &value[..200])
                        } else {
                            value.clone()
                        };
                        info!("📋 Data preview: {}", preview);

                        return Ok(Some(value.clone()));
                    }
                }

                warn!(
                    "⚠️ No 'data' field found in entry. Available fields: {:?}",
                    entry.1.iter().map(|(k, _)| k.as_str()).collect::<Vec<_>>()
                );
            } else {
                warn!("⚠️ No entries found in 'market_data_stream'");
            }
            Ok(None)
        }
        Err(e) => {
            warn!("❌ Failed to read from stream: {}", e);
            Ok(None)
        }
    }
}
