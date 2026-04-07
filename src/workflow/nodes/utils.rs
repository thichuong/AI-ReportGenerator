//! Utility functions for workflow nodes
use tracing::{error, info};

/// Calls Gemini API with the given prompt.
pub async fn call_gemini_api(api_key: &str, prompt: &str, session_id: &str, node_name: &str, enable_search: bool, json_mode: bool) -> Result<String, anyhow::Error> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemma-4-31b-it:generateContent?key={}",
        api_key
    );

    let mut config = serde_json::json!({
        "temperature": 0.3,
        "maxOutputTokens": 32768,
        "thinkingConfig": {
            "thinkingLevel": "HIGH"
        }
    });

    if json_mode {
        config["responseMimeType"] = serde_json::Value::String("application/json".to_string());
        // For JSON mode, lower temperature to reduce hallucination
        config["temperature"] = serde_json::Value::Number(serde_json::Number::from_f64(0.1).unwrap());
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

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(anyhow::anyhow!(
            "API request failed with status {}: {}",
            status,
            error_text
        ));
    }

    let json: serde_json::Value = response.json().await?;
    
    let debug_enabled = std::env::var("DEBUG")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false);

    if debug_enabled {
        let debug_json_path = format!("debug_{}_full_{}.json", node_name, session_id);
        if let Err(e) = std::fs::write(&debug_json_path, serde_json::to_string_pretty(&json).unwrap_or_default()) {
            error!("[{}] Failed to write debug JSON: {}", session_id, e);
        } else {
            info!("[{}] Saved full debug JSON to {}", session_id, debug_json_path);
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

    Ok(full_text)
}

/// Checks if the error message indicates a rate limit.
pub fn is_rate_limit_error(error: &str) -> bool {
    let error_lower = error.to_lowercase();
    error_lower.contains("429")
        || error_lower.contains("rate limit")
        || error_lower.contains("quota")
        || error_lower.contains("resource exhausted")
}
