//! Prepare data node
//!
//! Initializes data and reads prompts from environment or file fallback.
//! Equivalent to `app/services/workflow_nodes/prepare_data.py`

use crate::workflow::state::ReportState;
use chrono::Utc;
use regex::Regex;
use std::{env, fs, path::Path};
use tracing::{error, info, warn};

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

    // Read prompts from environment with file fallback
    state.research_analysis_prompt = get_prompt("combined_research_validation");
    state.data_validation_prompt = get_prompt("data_validation");
    state.create_report_prompt = get_prompt("create_report");
    state.generate_report_prompt = get_prompt("generate_report");
    state.translate_html_prompt = get_prompt("translate_html");
    state.translate_js_prompt = get_prompt("translate_js");

    // Validate required prompts
    if state.research_analysis_prompt.is_none() {
        let error_msg = "Cannot read combined research validation prompt";
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

/// Gets prompt with fallback: env var → file.
///
/// Lookup order:
/// 1. Environment variable: `prompt_name` (as-is)
/// 2. File: `prompt_envs/.env.prompt_{name}`
fn get_prompt(prompt_name: &str) -> Option<String> {
    // First try environment variable
    if let Some(content) = get_prompt_from_env(prompt_name) {
        info!("Loaded prompt '{}' from environment", prompt_name);
        return Some(process_prompt_placeholders(&content));
    }

    // Fallback: read from file
    if let Some(content) = read_prompt_from_file(prompt_name) {
        info!("Loaded prompt '{}' from file fallback", prompt_name);
        return Some(process_prompt_placeholders(&content));
    }

    warn!("Could not load prompt '{}' from env or file", prompt_name);
    None
}

/// Gets prompt from environment variable.
fn get_prompt_from_env(name: &str) -> Option<String> {
    env::var(name).ok().filter(|s| !s.is_empty())
}

/// Reads prompt from file in `prompt_envs/` directory.
///
/// Tries both:
/// - `prompt_envs/.env.prompt_{name}` (env style)
/// - `prompt_envs/prompt_{name}.md` (markdown style)
fn read_prompt_from_file(prompt_name: &str) -> Option<String> {
    // Try to find prompt_envs directory relative to working directory
    let possible_paths = vec![
        format!("prompt_envs/.env.prompt_{}", prompt_name),
        format!("prompt_envs/prompt_{}.md", prompt_name),
        format!("../prompt_envs/.env.prompt_{}", prompt_name),
        format!("../prompt_envs/prompt_{}.md", prompt_name),
    ];

    for path_str in possible_paths {
        let path = Path::new(&path_str);
        if path.exists() {
            match fs::read_to_string(path) {
                Ok(content) if !content.is_empty() => {
                    info!("Read prompt from file: {}", path_str);
                    return Some(content);
                }
                Ok(_) => {
                    warn!("File {} is empty", path_str);
                }
                Err(e) => {
                    warn!("Failed to read {}: {}", path_str, e);
                }
            }
        }
    }

    None
}

/// Processes placeholders in prompt content.
///
/// Handles:
/// - `{{ @css_root }}` - replaces with CSS root content
fn process_prompt_placeholders(content: &str) -> String {
    let mut result = content.to_string();

    // Replace {{ @css_root }} placeholder
    if result.contains("{{ @css_root }}") {
        let css_root = read_css_root().unwrap_or_default();
        result = result.replace("{{ @css_root }}", &css_root);
    }

    result
}

/// Reads CSS :root content from colors.css file.
fn read_css_root() -> Option<String> {
    let possible_paths = vec![
        "app/static/css/colors.css",
        "prompt_envs/colors.css",
        "../app/static/css/colors.css",
    ];

    for path_str in possible_paths {
        let path = Path::new(path_str);
        if path.exists() {
            if let Ok(content) = fs::read_to_string(path) {
                // Extract :root { ... } content
                if let Ok(regex) = Regex::new(r":root\s*\{([^}]+)\}") {
                    if let Some(captures) = regex.captures(&content) {
                        if let Some(root_content) = captures.get(1) {
                            return Some(root_content.as_str().trim().to_string());
                        }
                    }
                }
            }
        }
    }

    None
}

/// Replaces date placeholders in prompt text.
fn replace_date_placeholders(text: &str) -> String {
    let now = Utc::now();
    text.replace("<<@day>>", &now.format("%d").to_string())
        .replace("<<@month>>", &now.format("%m").to_string())
        .replace("<<@year>>", &now.format("%Y").to_string())
        .replace("{{DATE}}", &now.format("%Y-%m-%d").to_string())
        .replace("{{YEAR}}", &now.format("%Y").to_string())
        .replace("{{MONTH}}", &now.format("%m").to_string())
        .replace("{{DAY}}", &now.format("%d").to_string())
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
