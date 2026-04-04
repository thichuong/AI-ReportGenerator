//! Research deep node
//!
//! Performs deep research using Gemini AI.
//! Equivalent to `app/services/workflow_nodes/research_deep.py`

use crate::workflow::state::ReportState;
use tracing::{error, info};

/// Performs deep research using AI.
///
/// Calls Gemini API with the research prompt and real-time data.
pub async fn research_deep(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!("[{}] Step 2: Research deep", session_id);

    // Check rate limit flag
    if state.rate_limit_stop {
        error!(
            "[{}] Skipping research - rate limit flag is set",
            session_id
        );
        return Ok(state);
    }

    // Get the research prompt
    let prompt = match &state.research_analysis_prompt {
        Some(p) => p.clone(),
        None => {
            let error_msg = "Research prompt is missing";
            error!("[{}] {}", session_id, error_msg);
            state.add_error(error_msg);
            state.success = false;
            return Ok(state);
        }
    };

    // Build the full prompt with real-time data (inject via placeholder)
    let full_prompt = if let Some(ref data) = state.realtime_data {
        info!("[{}] Injecting real-time data into prompt", session_id);
        prompt.replace("{{REAL_TIME_DATA}}", data)
    } else {
        info!("[{}] No real-time data, using fallback message", session_id);
        prompt.replace(
            "{{REAL_TIME_DATA}}",
            r#"{"notice": "Real-time data không khả dụng, sử dụng Google Search để lấy dữ liệu mới nhất"}"#,
        )
    };

    // Call Gemini API
    match call_gemini_api(&state.api_key, &full_prompt, session_id).await {
        Ok(response) => {
            info!("[{}] Research completed successfully", session_id);

            // DEBUG: Save response to file
            let debug_enabled = std::env::var("DEBUG")
                .map(|v| v.to_lowercase() == "true")
                .unwrap_or(false);

            if debug_enabled {
                let debug_path = format!("debug_research_{}.txt", session_id);
                if let Err(e) = std::fs::write(&debug_path, &response) {
                    error!("[{}] Failed to write debug file: {}", session_id, e);
                } else {
                    info!("[{}] Saved debug response to {}", session_id, debug_path);
                }
            }
            state.research_content = Some(response.clone());

            // Check for validation result
            let validation_result = if response.contains("KẾT QUẢ KIỂM TRA: PASS")
                || response.contains("validation_result: PASS")
            {
                "PASS"
            } else if response.contains("KẾT QUẢ KIỂM TRA: FAIL")
                || response.contains("validation_result: FAIL")
            {
                "FAIL"
            } else {
                "UNKNOWN"
            };

            state.validation_result = Some(validation_result.to_string());

            if validation_result == "FAIL" {
                state.success = false;
                info!("[{}] Research validation FAILED", session_id);
            } else {
                state.success = true; // PASS or UNKNOWN treated as success for now (match Python UNKNOWN behavior)
                info!(
                    "[{}] Research validation result: {}",
                    session_id, validation_result
                );
            }
        }
        Err(e) => {
            let error_msg = format!("Research API call failed: {}", e);
            error!("[{}] {}", session_id, error_msg);

            // Check for rate limit error
            if is_rate_limit_error(&e.to_string()) {
                state.rate_limit_stop = true;
                state.add_error("Rate limit encountered - stopping workflow");
            } else {
                state.add_error(&error_msg);
            }
            state.success = false;
        }
    }

    Ok(state)
}

/// Calls Gemini API with the given prompt.
async fn call_gemini_api(api_key: &str, prompt: &str, session_id: &str) -> Result<String, anyhow::Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemma-4-31b-it:generateContent?key={}",
        api_key
    );

    let body = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }],
        "tools": [{
            "googleSearch": {}
        }],
        "generationConfig": {
            "temperature": 0.7,
            "maxOutputTokens": 32768,
            "thinkingConfig": {
                "thinkingLevel": "HIGH"
            }
        }
    });

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
    
    // DEBUG: Save full JSON to file
    let debug_enabled = std::env::var("DEBUG")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false);

    if debug_enabled {
        let debug_json_path = format!("debug_research_full_{}.json", session_id);
        if let Err(e) = std::fs::write(&debug_json_path, serde_json::to_string_pretty(&json).unwrap_or_default()) {
            error!("[{}] Failed to write debug JSON: {}", session_id, e);
        } else {
            info!("[{}] Saved full debug JSON to {}", session_id, debug_json_path);
        }
    }
    
    // Extract ALL text parts from the response (to handle multi-part Thinking + Result)
    let mut full_text = String::new();
    if let Some(parts) = json["candidates"][0]["content"]["parts"].as_array() {
        for part in parts {
            if let Some(part_text) = part["text"].as_str() {
                full_text.push_str(part_text);
            }
        }
    }

    if full_text.is_empty() {
        return Err(anyhow::anyhow!("Failed to extract text from AI response (empty or missing parts)"));
    }

    // Parse validation result (simple check to match Python logic)
    if full_text.contains("KẾT QUẢ KIỂM TRA: PASS") || full_text.contains("validation_result: PASS") {
        // state.validation_result set in caller, but we return text here
    }

    Ok(full_text)
}

/// Checks if the error message indicates a rate limit.
fn is_rate_limit_error(error: &str) -> bool {
    let error_lower = error.to_lowercase();
    error_lower.contains("429")
        || error_lower.contains("rate limit")
        || error_lower.contains("quota")
        || error_lower.contains("resource exhausted")
}
