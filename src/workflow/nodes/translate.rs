//! Translate content node
//!
//! Translates content to English.
//! Equivalent to `app/services/workflow_nodes/translate_content.py`

use crate::workflow::nodes::utils::process_mathjax;
use crate::workflow::{prompts, state::ReportState};
use tracing::{error, info, warn};

/// Translates the report content to English.
pub async fn translate(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!("[{}] Step 7: Translate content", session_id);

    // Check rate limit flag
    if state.rate_limit_stop {
        warn!(
            "[{}] Skipping translation - rate limit flag is set",
            session_id
        );
        return Ok(state);
    }

    // Get API key
    let api_key = state.api_key.clone();

    // Translate HTML content using translate_html prompt
    if let Some(ref html) = state.html_content
        && !html.trim().is_empty()
    {
        let prompt = prompts::translation::TRANSLATE_HTML_PROMPT.replace("{content}", html);

        match translate_with_prompt(&api_key, &prompt, session_id, "html").await {
            Ok((translated, is_rate_limit)) => {
                if is_rate_limit {
                    error!("[{}] Rate limit error while translating HTML", session_id);
                    state.rate_limit_stop = true;
                    state.add_error("Rate limit error when translating HTML");
                    return Ok(state);
                }
                if let Some(content) = translated {
                    info!(
                        "[{}] HTML translated successfully - {} chars",
                        session_id,
                        content.len()
                    );
                    state.html_content_en = Some(content);
                }
            }
            Err(e) => {
                warn!("[{}] HTML translation failed: {}", session_id, e);
            }
        }
    }

    // Translate JS content using translate_js prompt
    if let Some(ref js) = state.js_content
        && !js.trim().is_empty()
    {
        let prompt = prompts::translation::TRANSLATE_JS_PROMPT.replace("{js_content}", js);

        match translate_with_prompt(&api_key, &prompt, session_id, "js").await {
            Ok((translated, is_rate_limit)) => {
                if is_rate_limit {
                    error!("[{}] Rate limit error while translating JS", session_id);
                    state.rate_limit_stop = true;
                    state.add_error("Rate limit error when translating JS");
                    return Ok(state);
                }
                if let Some(content) = translated {
                    info!(
                        "[{}] JS translated successfully - {} chars",
                        session_id,
                        content.len()
                    );
                    state.js_content_en = Some(content);
                }
            }
            Err(e) => {
                warn!("[{}] JS translation failed: {}", session_id, e);
            }
        }
    }

    // Apply MathJax processing after translation to save tokens
    if let Some(ref html) = state.html_content {
        state.html_content = Some(process_mathjax(html));
    }
    if let Some(ref html_en) = state.html_content_en {
        state.html_content_en = Some(process_mathjax(html_en));
    }

    info!("[{}] Translation completed", session_id);
    state.success = true;
    Ok(state)
}

/// Translates text using the given prompt.
/// Returns (Option<translated_content>, is_rate_limit_error)
async fn translate_with_prompt(
    api_key: &str,
    prompt: &str,
    session_id: &str,
    suffix: &str,
) -> Result<(Option<String>, bool), anyhow::Error> {
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
        "generationConfig": {
            "temperature": 0.1,
            "maxOutputTokens": 32768,
            "thinkingConfig": {
                "thinkingLevel": "MINIMAL"
            }
        }
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let status = response.status();

    if !status.is_success() {
        let error_text = response.text().await.unwrap_or_default();

        // Check for rate limit error
        if is_rate_limit_error(&error_text) || status.as_u16() == 429 {
            return Ok((None, true));
        }

        return Err(anyhow::anyhow!(
            "Translation API failed with status {}: {}",
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
        let debug_json_path = format!("debug_translate_{}_full_{}.json", suffix, session_id);
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

    // Extract ALL text parts from the response (to handle multi-part Thinking + Result)
    let mut full_text = String::new();
    if let Some(parts) = json["candidates"][0]["content"]["parts"].as_array() {
        for part in parts {
            if let Some(part_text) = part["text"].as_str() {
                full_text.push_str(part_text);
            }
        }
    }

    if !full_text.is_empty() {
        // DEBUG: Save response to file
        if debug_enabled {
            let debug_path = format!("debug_translate_{}_{}.txt", suffix, session_id);
            if let Err(e) = std::fs::write(&debug_path, &full_text) {
                error!("[{}] Failed to write debug file: {}", session_id, e);
            } else {
                info!("[{}] Saved debug response to {}", session_id, debug_path);
            }
        }

        let mut cleaned = full_text.trim().to_string();

        // Remove markdown code blocks if present
        if cleaned.starts_with("```") {
            let lines: Vec<&str> = cleaned.lines().collect();
            if lines.len() > 2 {
                cleaned = lines[1..lines.len() - 1].join("\n");
            }
        }

        if !cleaned.is_empty() {
            return Ok((Some(cleaned), false));
        }
    }

    Ok((None, false))
}

/// Checks if the error message indicates a rate limit.
fn is_rate_limit_error(error: &str) -> bool {
    let error_lower = error.to_lowercase();
    error_lower.contains("429")
        || error_lower.contains("rate limit")
        || error_lower.contains("quota")
        || error_lower.contains("resource exhausted")
}
