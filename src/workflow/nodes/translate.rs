//! Translate content node
//!
//! Translates content to English.
//! Equivalent to `app/services/workflow_nodes/translate_content.py`

use crate::workflow::state::ReportState;
use tracing::{info, warn};

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
        // Copy original content as fallback
        state.html_content_en = state.html_content.clone();
        state.js_content_en = state.js_content.clone();
        return Ok(state);
    }

    // Translate HTML content
    if let Some(ref html) = state.html_content {
        match translate_text(&state.api_key, html).await {
            Ok(translated) => {
                info!("[{}] HTML translated successfully", session_id);
                state.html_content_en = Some(translated);
            }
            Err(e) => {
                warn!("[{}] HTML translation failed: {}", session_id, e);
                // Use original as fallback
                state.html_content_en = state.html_content.clone();
            }
        }
    }

    // Translate JS content (only text strings)
    if let Some(ref js) = state.js_content {
        match translate_text(&state.api_key, js).await {
            Ok(translated) => {
                info!("[{}] JS translated successfully", session_id);
                state.js_content_en = Some(translated);
            }
            Err(e) => {
                warn!("[{}] JS translation failed: {}", session_id, e);
                // Use original as fallback
                state.js_content_en = state.js_content.clone();
            }
        }
    }

    state.success = true;
    Ok(state)
}

/// Translates text from Vietnamese to English using Gemini.
async fn translate_text(api_key: &str, text: &str) -> Result<String, anyhow::Error> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
        api_key
    );

    let prompt = format!(
        r#"Translate the following Vietnamese text to English. 
Keep all HTML tags, CSS, and JavaScript code structure intact.
Only translate the human-readable text content.

Text to translate:
{}

Return ONLY the translated text without any explanation."#,
        text
    );

    let body = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": prompt
            }]
        }],
        "generationConfig": {
            "temperature": 0.3,
            "maxOutputTokens": 16384
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
            "Translation API failed with status {}: {}",
            status,
            error_text
        ));
    }

    let json: serde_json::Value = response.json().await?;

    let text = json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to extract translated text"))?;

    Ok(text.to_string())
}
