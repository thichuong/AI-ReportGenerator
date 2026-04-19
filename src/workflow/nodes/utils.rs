//! Utility functions for workflow nodes
use regex::Regex;
use std::sync::OnceLock;
use tracing::{error, info};

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn get_client() -> &'static reqwest::Client {
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("Failed to create reqwest client")
    })
}

/// Optional API configuration overrides
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

/// Calls Gemini API with the given prompt.
pub async fn call_gemini_api(
    api_key: &str,
    prompt: &str,
    session_id: &str,
    node_name: &str,
    enable_search: bool,
    json_mode: bool,
    config_override: Option<ApiConfig>,
) -> Result<String, anyhow::Error> {
    let client = get_client();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemma-4-31b-it:generateContent?key={}",
        api_key
    );

    let config_val = config_override.unwrap_or_default();

    let mut config = serde_json::json!({
        "temperature": config_val.temperature,
        "maxOutputTokens": config_val.max_output_tokens,
        "thinkingConfig": {
            "thinkingLevel": config_val.thinking_level
        }
    });

    if json_mode {
        config["responseMimeType"] = serde_json::Value::String("application/json".to_string());
        // For JSON mode, lower temperature to reduce hallucination
        config["temperature"] =
            serde_json::Value::Number(serde_json::Number::from_f64(0.1).unwrap());
    }

    let mut body = serde_json::json!({
        "contents": [{
            "parts": [{ "text": prompt }]
        }],
        "generationConfig": config
    });

    if enable_search {
        body["tools"] = serde_json::json!([{ "googleSearch": {} }]);
    }

    let mut retries = 0;
    const MAX_RETRIES: u32 = 3;

    loop {
        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = response.status();

        if status.is_success() {
            let json: serde_json::Value = response.json().await?;

            let debug_enabled = std::env::var("DEBUG")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(false);

            if debug_enabled {
                let debug_json_path = format!("debug_{}_full_{}.json", node_name, session_id);
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

            let mut full_text = String::new();
            if let Some(parts) = json["candidates"][0]["content"]["parts"].as_array() {
                for part in parts {
                    if let Some(part_text) = part["text"].as_str() {
                        full_text.push_str(part_text);
                    }
                }
            }

            if full_text.is_empty() {
                return Err(anyhow::anyhow!("Failed to extract text from AI response"));
            }

            return Ok(full_text);
        } else if status.as_u16() == 500 && retries < MAX_RETRIES {
            retries += 1;
            error!(
                "[{}] Gemini API 500 Internal Server Error (Attempt {}/{}). Retrying in 5s...",
                session_id, retries, MAX_RETRIES
            );
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            continue;
        } else {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "API request failed with status {}: {}",
                status,
                error_text
            ));
        }
    }
}

/// Checks if the error message indicates a rate limit.
pub fn is_rate_limit_error(error: &str) -> bool {
    let error_lower = error.to_lowercase();
    error_lower.contains("429")
        || error_lower.contains("rate limit")
        || error_lower.contains("quota")
        || error_lower.contains("resource exhausted")
}

static MATHJAX_REPLACEMENTS: OnceLock<Vec<(Regex, &'static str)>> = OnceLock::new();

fn get_mathjax_replacements() -> &'static Vec<(Regex, &'static str)> {
    MATHJAX_REPLACEMENTS.get_or_init(|| {
        vec![
            (
                Regex::new(r"\$\s*\\rightarrow\s*\$").expect("Invalid regex: rightarrow"),
                r#"<i class="fas fa-arrow-right mx-1"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\leftarrow\s*\$").expect("Invalid regex: leftarrow"),
                r#"<i class="fas fa-arrow-left mx-1"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\Rightarrow\s*\$").expect("Invalid regex: Rightarrow"),
                r#"<i class="fas fa-arrow-right mx-1"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\Leftarrow\s*\$").expect("Invalid regex: Leftarrow"),
                r#"<i class="fas fa-arrow-left mx-1"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\uparrow\s*\$").expect("Invalid regex: uparrow"),
                r#"<i class="fas fa-arrow-up mx-1 text-green-500"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\downarrow\s*\$").expect("Invalid regex: downarrow"),
                r#"<i class="fas fa-arrow-down mx-1 text-red-500"></i>"#,
            ),
            (
                Regex::new(r"\$\s*\\pm\s*\$").expect("Invalid regex: pm"),
                "±",
            ),
            (
                Regex::new(r"\$\s*\\ge(q)?\s*\$").expect("Invalid regex: ge"),
                "≥",
            ),
            (
                Regex::new(r"\$\s*\\le(q)?\s*\$").expect("Invalid regex: le"),
                "≤",
            ),
            (
                Regex::new(r"\$\s*\\approx\s*\$").expect("Invalid regex: approx"),
                "≈",
            ),
            (
                Regex::new(r"\$\s*\\dots\s*\$").expect("Invalid regex: dots"),
                "…",
            ),
            (
                Regex::new(r"\$\s*\\text\{([^}]+)\}\s*\$").expect("Invalid regex: text"),
                "$1",
            ),
        ]
    })
}

/// Processes content to convert common symbols to MathJax/LaTeX equivalents.
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
