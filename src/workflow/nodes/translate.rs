//! Translate content node
//!
//! Translates content to English.
//! Equivalent to `app/services/workflow_nodes/translate_content.py`

use crate::workflow::state::ReportState;
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
    if let Some(ref html) = state.html_content {
        if !html.trim().is_empty() {
            let prompt = match &state.translate_html_prompt {
                Some(p) => p.replace("{content}", html),
                None => {
                    warn!(
                        "[{}] translate_html prompt not found, using default",
                        session_id
                    );
                    format!(
                        "Translate the following HTML content from Vietnamese to English.\n\
                         Keep all HTML tags intact. Only translate the text content.\n\n\
                         {}\n\nReturn ONLY the translated HTML without explanation.",
                        html
                    )
                }
            };

            match translate_with_prompt(&api_key, &prompt).await {
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
    }

    // Translate JS content using translate_js prompt
    if let Some(ref js) = state.js_content {
        if !js.trim().is_empty() {
            let prompt = match &state.translate_js_prompt {
                Some(p) => p.replace("{js_content}", js),
                None => {
                    warn!(
                        "[{}] translate_js prompt not found, using default",
                        session_id
                    );
                    format!(
                        "Translate the following JavaScript content from Vietnamese to English.\n\
                         Keep all JavaScript code intact. Only translate string literals and comments.\n\n\
                         {}\n\nReturn ONLY the translated JavaScript without explanation.",
                        js
                    )
                }
            };

            match translate_with_prompt(&api_key, &prompt).await {
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
) -> Result<(Option<String>, bool), anyhow::Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
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
            "maxOutputTokens": 65536,
            "thinkingConfig": {
                "includeThoughts": false,
                "thinkingBudget": 2048
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

    // Extract and clean text
    if let Some(text) = json["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        let mut cleaned = text.trim().to_string();

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
