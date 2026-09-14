//! Utility functions for workflow nodes
use regex::Regex;
use std::sync::OnceLock;
use tracing::{error, info, warn};

#[allow(clippy::expect_used)]
fn get_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        let timeout_secs = std::env::var("HTTP_REQUEST_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(300); // 5 minutes default

        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to create reqwest client")
    })
}

static SENSITIVE_REGEXES: OnceLock<(Regex, Regex, Regex)> = OnceLock::new();

/// Masks sensitive query params (e.g. `key=***`), headers, and URL credentials.
///
/// # Panics
///
/// Panics if the internal regex patterns fail to compile (only possible if hardcoded pattern is invalid).
#[allow(clippy::expect_used)]
#[must_use]
pub fn mask_sensitive_data(input: &str) -> String {
    let (key_re, header_re, url_auth_re) = SENSITIVE_REGEXES.get_or_init(|| {
        (
            Regex::new(r#"(?i)(key=)[^&\s"'`\)]+"#).expect("Valid regex for key param"),
            Regex::new(r#"(?i)(x-goog-api-key:\s*)[^\s"'`]+"#).expect("Valid regex for api key header"),
            Regex::new(r"(?i)(://[^:\s/]+:)[^@\s/]+(@)").expect("Valid regex for url auth"),
        )
    });

    let s1 = key_re.replace_all(input, "${1}***");
    let s2 = header_re.replace_all(&s1, "${1}***");
    url_auth_re.replace_all(&s2, "${1}***${2}").to_string()
}

/// Calculates exponential backoff delay with jitter.
///
/// Delay sequence:
/// - Attempt 1: ~2s
/// - Attempt 2: ~5s
/// - Attempt 3: ~10s
/// - Attempt 4: ~15s
/// - Attempt 5: ~30s
/// - Attempt 6: ~45s
/// - Attempt 7..10: ~60s (capped at 60s base)
///
/// Adds +0% to +25% jitter to prevent thundering herd.
#[must_use]
pub fn calculate_backoff_delay(attempt: u32) -> std::time::Duration {
    let base_secs: u64 = match attempt {
        1 => 2,
        2 => 5,
        3 => 10,
        4 => 15,
        5 => 30,
        6 => 45,
        _ => 60,
    };

    let base_ms = base_secs * 1000;
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.subsec_nanos());
    let jitter_pct = u64::from(nanos % 26); // 0..25%
    let total_ms = base_ms + (base_ms * jitter_pct) / 100;
    std::time::Duration::from_millis(total_ms)
}

/// Optional API configuration overrides
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub temperature: f64,
    pub thinking_level: String,
    pub max_output_tokens: u32,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            temperature: 0.3,
            thinking_level: "HIGH".to_string(),
            max_output_tokens: 32768,
        }
    }
}

fn build_gemini_request_body(
    prompt: &str,
    enable_search: bool,
    json_mode: bool,
    config_override: Option<ApiConfig>,
) -> serde_json::Value {
    let config_val = config_override.unwrap_or_default();

    let mut config = serde_json::json!({
        "temperature": config_val.temperature,
        "maxOutputTokens": config_val.max_output_tokens,
        "thinkingConfig": {
            "thinkingLevel": config_val.thinking_level
        }
    });

    if json_mode && let Some(obj) = config.as_object_mut() {
        obj.insert(
            "responseMimeType".to_string(),
            serde_json::Value::String("application/json".to_string()),
        );
        // For JSON mode, lower temperature to reduce hallucination
        if let Some(num) = serde_json::Number::from_f64(0.1) {
            obj.insert("temperature".to_string(), serde_json::Value::Number(num));
        }
    }

    let mut body = serde_json::json!({
        "contents": [{
            "parts": [{ "text": prompt }]
        }],
        "generationConfig": config
    });

    if enable_search && let Some(obj) = body.as_object_mut() {
        obj.insert(
            "tools".to_string(),
            serde_json::json!([{ "googleSearch": {} }]),
        );
    }

    body
}

fn extract_text_from_response(json: &serde_json::Value) -> Option<String> {
    let mut full_text = String::new();
    let parts = json
        .get("candidates")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("content"))
        .and_then(|c| c.get("parts"))
        .and_then(|p| p.as_array());

    if let Some(parts_arr) = parts {
        for part in parts_arr {
            if let Some(part_text) = part.get("text").and_then(|t| t.as_str()) {
                full_text.push_str(part_text);
            }
        }
    }

    if full_text.is_empty() {
        None
    } else {
        Some(full_text)
    }
}

/// Model identifier for Gemma 4 (used for research nodes with search tool enabled).
pub const MODEL_GEMMA_4: &str = "gemma-4-31b-it";

/// Fallback model identifier for search nodes if Gemma 4 fails after 10 retries.
pub const MODEL_FALLBACK_FLASH: &str = "gemini-2.5-flash";

/// Model identifier for Gemini 3.5 Flash Lite (used for synthesis, validation, interface, translation).
pub const MODEL_GEMINI_3_5_FLASH_LITE: &str = "gemini-3.5-flash-lite";

/// Low-level function to call Google Generative Language API with a specific model.
///
/// # Errors
///
/// Returns an error if:
/// - The API request fails and all retries are exhausted.
/// - The API response cannot be parsed.
/// - No text can be extracted from the AI response.
#[allow(clippy::too_many_arguments)]
pub async fn call_model_api(
    model: &str,
    api_key: &str,
    prompt: &str,
    session_id: &str,
    node_name: &str,
    enable_search: bool,
    json_mode: bool,
    config_override: Option<ApiConfig>,
    max_retries: u32,
) -> Result<String, anyhow::Error> {
    let client = get_client();
    // Do not pass API key in query parameters to prevent leaking in reqwest error messages.
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent"
    );

    let body = build_gemini_request_body(prompt, enable_search, json_mode, config_override);

    let debug_enabled = std::env::var("DEBUG").map_or(true, |v| v.to_lowercase() == "true");

    if debug_enabled {
        info!(
            "[{}] Calling {} API ({}). Body size: {} bytes",
            session_id,
            model,
            node_name,
            serde_json::to_string(&body).unwrap_or_default().len()
        );
    }

    let mut retries = 0;

    loop {
        let response_result = client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("x-goog-api-key", api_key)
            .json(&body)
            .send()
            .await;

        let response = match response_result {
            Ok(resp) => resp,
            Err(e) if retries < max_retries => {
                retries += 1;
                let delay = calculate_backoff_delay(retries);
                let masked_err = mask_sensitive_data(&e.to_string());
                error!(
                    "[{}] Request failed for node {} with model {} (Attempt {}/{}): {}. Retrying in {}s...",
                    session_id, node_name, model, retries, max_retries, masked_err, delay.as_secs()
                );
                tokio::time::sleep(delay).await;
                continue;
            }
            Err(e) => {
                let masked_err = mask_sensitive_data(&e.to_string());
                error!(
                    "[{}] Request failed for node {} with model {}: {}. URL: {}",
                    session_id, node_name, model, masked_err, url
                );
                return Err(anyhow::anyhow!(
                    "Request failed after {retries} retries: {masked_err}"
                ));
            }
        };

        let status = response.status();

        if status.is_success() {
            let json: serde_json::Value = response.json().await?;

            let debug_enabled = std::env::var("DEBUG").map_or(true, |v| v.to_lowercase() == "true");

            if debug_enabled {
                let debug_json_path = format!("debug_{node_name}_full_{session_id}.json");
                if let Err(e) = std::fs::write(
                    &debug_json_path,
                    serde_json::to_string_pretty(&json).unwrap_or_default(),
                ) {
                    error!("[{}] Failed to write debug JSON: {}", session_id, e);
                } else {
                    info!(
                        "[{}] Saved full debug JSON to {}",
                        session_id, debug_json_path
                    );
                }
            }

            return extract_text_from_response(&json)
                .ok_or_else(|| anyhow::anyhow!("Failed to extract text from AI response"));
        } else if (status.is_server_error() || status.as_u16() == 429) && retries < max_retries {
            retries += 1;
            let delay = calculate_backoff_delay(retries);
            error!(
                "[{}] {} API status {} for node {} (Attempt {}/{}). Retrying in {}s...",
                session_id, model, status, node_name, retries, max_retries, delay.as_secs()
            );
            tokio::time::sleep(delay).await;
            continue;
        }

        let raw_error_text = response.text().await.unwrap_or_default();
        let masked_error_text = mask_sensitive_data(&raw_error_text);
        return Err(anyhow::anyhow!(
            "API request to {model} failed with status {status}: {masked_error_text}"
        ));
    }
}

/// Calls Gemma 4 API (`gemma-4-31b-it`) with up to 10 retries.
/// If all 10 retries are exhausted, falls back to Gemini 2.5 Flash (or model in `GEMINI_FALLBACK_MODEL`) for 1 attempt.
///
/// # Errors
///
/// Returns an error if both primary and fallback API requests fail or responses cannot be parsed.
#[allow(clippy::too_many_arguments)]
pub async fn call_gemma_api(
    api_key: &str,
    prompt: &str,
    session_id: &str,
    node_name: &str,
    enable_search: bool,
    json_mode: bool,
    config_override: Option<ApiConfig>,
) -> Result<String, anyhow::Error> {
    let primary_result = call_model_api(
        MODEL_GEMMA_4,
        api_key,
        prompt,
        session_id,
        node_name,
        enable_search,
        json_mode,
        config_override.clone(),
        10,
    )
    .await;

    match primary_result {
        Ok(response) => Ok(response),
        Err(primary_err) => {
            let fallback_model = std::env::var("GEMINI_FALLBACK_MODEL")
                .unwrap_or_else(|_| MODEL_FALLBACK_FLASH.to_string());

            warn!(
                "[{}] Node {} primary model {} failed after 10 retries: {}. Switching to fallback model {} (1 attempt)...",
                session_id, node_name, MODEL_GEMMA_4, primary_err, fallback_model
            );

            match call_model_api(
                &fallback_model,
                api_key,
                prompt,
                session_id,
                node_name,
                enable_search,
                json_mode,
                config_override,
                1,
            )
            .await
            {
                Ok(fallback_response) => {
                    info!(
                        "[{}] Node {} fallback to {} SUCCEEDED!",
                        session_id, node_name, fallback_model
                    );
                    Ok(fallback_response)
                }
                Err(fallback_err) => {
                    error!(
                        "[{}] Node {} fallback to {} also failed: {}",
                        session_id, node_name, fallback_model, fallback_err
                    );
                    Err(anyhow::anyhow!(
                        "Primary model ({MODEL_GEMMA_4}) failed after 10 retries: {primary_err}. Fallback ({fallback_model}) also failed: {fallback_err}"
                    ))
                }
            }
        }
    }
}

/// Calls Gemini 3.5 Flash Lite API (`gemini-3.5-flash-lite`), used for synthesis, validation, interface creation, and translation.
///
/// # Errors
///
/// Returns an error if the API request fails or response cannot be parsed.
pub async fn call_gemini_flash_lite_api(
    api_key: &str,
    prompt: &str,
    session_id: &str,
    node_name: &str,
    json_mode: bool,
    config_override: Option<ApiConfig>,
) -> Result<String, anyhow::Error> {
    call_model_api(
        MODEL_GEMINI_3_5_FLASH_LITE,
        api_key,
        prompt,
        session_id,
        node_name,
        false,
        json_mode,
        config_override,
        10,
    )
    .await
}

/// Checks if the error message indicates a rate limit.
#[must_use]
pub fn is_rate_limit_error(error: &str) -> bool {
    let error_lower = error.to_lowercase();
    error_lower.contains("429")
        || error_lower.contains("rate limit")
        || error_lower.contains("quota")
        || error_lower.contains("resource exhausted")
}

static MATHJAX_REPLACEMENTS: OnceLock<Vec<(Regex, &'static str)>> = OnceLock::new();

#[allow(clippy::expect_used)]
fn get_mathjax_replacements() -> &'static Vec<(Regex, &'static str)> {
    MATHJAX_REPLACEMENTS.get_or_init(|| {
        vec![
            // Arrow combinations (Trend indicators)
            (
                Regex::new(r"\$\s*\\uparrow\s*\\rightarrow\s*\$").expect("Invalid regex: up-right"),
                r#"<i class="fas fa-arrow-up mx-1 text-green-500"></i><i class="fas fa-arrow-right mx-1"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\downarrow\s*\\rightarrow\s*\$").expect("Invalid regex: down-right"),
                r#"<i class="fas fa-arrow-down mx-1 text-red-500"></i><i class="fas fa-arrow-right mx-1"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\rightarrow\s*\\uparrow\s*\$").expect("Invalid regex: right-up"),
                r#"<i class="fas fa-arrow-right mx-1"></i><i class="fas fa-arrow-up mx-1 text-green-500"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\rightarrow\s*\\downarrow\s*\$").expect("Invalid regex: right-down"),
                r#"<i class="fas fa-arrow-right mx-1"></i><i class="fas fa-arrow-down mx-1 text-red-500"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\uparrow\s*\\uparrow\s*\$").expect("Invalid regex: double-up"),
                r#"<i class="fas fa-arrow-up mx-1 text-green-500"></i><i class="fas fa-arrow-up mx-1 text-green-500"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\downarrow\s*\\downarrow\s*\$").expect("Invalid regex: double-down"),
                r#"<i class="fas fa-arrow-down mx-1 text-red-500"></i><i class="fas fa-arrow-down mx-1 text-red-500"></i>"#,
            ),
            // Single Right arrow
            (
                Regex::new(r"\$\s*\\rightarrow\s*\$").expect("Invalid regex: rightarrow"),
                r#"<i class="fas fa-arrow-right mx-1"></i>"#,
            ),
            // Single Left arrow
            (
                Regex::new(r"\$\s*\\leftarrow\s*\$").expect("Invalid regex: leftarrow"),
                r#"<i class="fas fa-arrow-left mx-1"></i>"#,
            ),
            // Right double arrow
            (
                Regex::new(r"\$\s*\\Rightarrow\s*\$").expect("Invalid regex: Rightarrow"),
                r#"<i class="fas fa-arrow-right mx-1"></i>"#,
            ),
            // Left double arrow
            (
                Regex::new(r"\$\s*\\Leftarrow\s*\$").expect("Invalid regex: Leftarrow"),
                r#"<i class="fas fa-arrow-left mx-1"></i>"#,
            ),
            // Up arrow (Bullish)
            (
                Regex::new(r"\$\s*\\uparrow\s*\$").expect("Invalid regex: uparrow"),
                r#"<i class="fas fa-arrow-up mx-1 text-green-500"></i>"#,
            ),
            // Down arrow (Bearish)
            (
                Regex::new(r"\$\s*\\downarrow\s*\$").expect("Invalid regex: downarrow"),
                r#"<i class="fas fa-arrow-down mx-1 text-red-500"></i>"#,
            ),
            // Plus-minus sign
            (
                Regex::new(r"\$\s*\\pm\s*\$").expect("Invalid regex: pm"),
                "±",
            ),
            // Greater than or equal to
            (
                Regex::new(r"\$\s*\\ge(q)?\s*\$").expect("Invalid regex: ge"),
                "≥",
            ),
            // Less than or equal to
            (
                Regex::new(r"\$\s*\\le(q)?\s*\$").expect("Invalid regex: le"),
                "≤",
            ),
            // Approximation
            (
                Regex::new(r"\$\s*\\approx\s*\$").expect("Invalid regex: approx"),
                "≈",
            ),
            // Ellipsis
            (
                Regex::new(r"\$\s*\\dots\s*\$").expect("Invalid regex: dots"),
                "…",
            ),
            // Inline text in MathJax
            (
                Regex::new(r"\$\s*\\text\{([^}]+)\}\s*\$").expect("Invalid regex: text"),
                "$1",
            ),
        ]
    })
}

/// Processes content to convert common symbols to MathJax/LaTeX equivalents.
///
/// This is used to replace backend-side technical symbols with LaTeX/MathJax or
/// `FontAwesome` icons for consistent rendering across platforms.
#[must_use]
pub fn process_mathjax(content: &str) -> String {
    let replacements = get_mathjax_replacements();
    let mut result = std::borrow::Cow::Borrowed(content);

    for (re, rep) in replacements {
        if re.is_match(&result) {
            result = std::borrow::Cow::Owned(re.replace_all(&result, *rep).into_owned());
        }
    }

    result.into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_sensitive_data_key_param() {
        let input = "error sending request for url (https://generativelanguage.googleapis.com/v1beta/models/gemini:generateContent?key=AIzaSyABC123xyz): connection reset";
        let masked = mask_sensitive_data(input);
        assert!(!masked.contains("AIzaSyABC123xyz"));
        assert!(masked.contains("key=***"));
    }

    #[test]
    fn test_mask_sensitive_data_multiple_params() {
        let input = "https://example.com/api?foo=bar&key=SECRET_KEY_123&test=1";
        let masked = mask_sensitive_data(input);
        assert!(!masked.contains("SECRET_KEY_123"));
        assert!(masked.contains("key=***"));
        assert!(masked.contains("foo=bar"));
        assert!(masked.contains("&test=1"));
    }

    #[test]
    fn test_mask_sensitive_data_header() {
        let input = "Request headers: x-goog-api-key: AIzaSyTopSecretKey\nContent-Type: application/json";
        let masked = mask_sensitive_data(input);
        assert!(!masked.contains("AIzaSyTopSecretKey"));
        assert!(masked.contains("x-goog-api-key: ***"));
    }

    #[test]
    fn test_mask_sensitive_data_url_auth() {
        let input = "Connecting to redis://default:superpassword123@redis-node:6379/0";
        let masked = mask_sensitive_data(input);
        assert!(!masked.contains("superpassword123"));
        assert!(masked.contains("redis://default:***@redis-node:6379/0"));
    }

    #[test]
    #[allow(clippy::duration_suboptimal_units)]
    fn test_calculate_backoff_delay_ranges() {
        let d1 = calculate_backoff_delay(1);
        assert!(d1 >= std::time::Duration::from_secs(2));
        assert!(d1 <= std::time::Duration::from_millis(2600));

        let d2 = calculate_backoff_delay(2);
        assert!(d2 >= std::time::Duration::from_secs(5));
        assert!(d2 <= std::time::Duration::from_millis(6500));

        let d5 = calculate_backoff_delay(5);
        assert!(d5 >= std::time::Duration::from_secs(30));
        assert!(d5 <= std::time::Duration::from_secs(38));

        let d10 = calculate_backoff_delay(10);
        assert!(d10 >= std::time::Duration::from_secs(60));
        assert!(d10 <= std::time::Duration::from_secs(76));
    }

    #[test]
    fn test_model_constants() {
        assert_eq!(MODEL_GEMMA_4, "gemma-4-31b-it");
        assert_eq!(MODEL_FALLBACK_FLASH, "gemini-2.5-flash");
        assert_eq!(MODEL_GEMINI_3_5_FLASH_LITE, "gemini-3.5-flash-lite");
    }
}
