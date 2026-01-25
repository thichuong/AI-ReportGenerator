//! Generate report content node
//!
//! Generates the final report content from research.
//! Equivalent to `app/services/workflow_nodes/generate_report_content.py`

use crate::workflow::state::ReportState;
use tracing::{error, info};

/// Generates the report content from research.
pub async fn generate_content(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!("[{}] Step 4: Generate report content", session_id);

    // Check rate limit flag
    if state.rate_limit_stop {
        error!(
            "[{}] Skipping content generation - rate limit flag is set",
            session_id
        );
        return Ok(state);
    }

    // Get research content
    let research_content = match &state.research_content {
        Some(content) => content.clone(),
        None => {
            let error_msg = "Research content is missing";
            error!("[{}] {}", session_id, error_msg);
            state.add_error(error_msg);
            state.success = false;
            return Ok(state);
        }
    };

    // Get create report prompt
    let prompt = match &state.create_report_prompt {
        Some(p) => p.clone(),
        None => {
            // Use a default prompt if not set
            "Generate a professional crypto market report based on the following research:"
                .to_string()
        }
    };

    // Build full prompt
    let full_prompt = format!("{}\n\n## Research Content:\n{}", prompt, research_content);

    // Call Gemini API
    match call_gemini_api(&state.api_key, &full_prompt).await {
        Ok(response) => {
            info!("[{}] Report content generated successfully", session_id);
            state.report_content = Some(response);
            state.success = true;
        }
        Err(e) => {
            let error_msg = format!("Content generation failed: {}", e);
            error!("[{}] {}", session_id, error_msg);

            if is_rate_limit_error(&e.to_string()) {
                state.rate_limit_stop = true;
                state.add_error("Rate limit encountered");
            } else {
                state.add_error(&error_msg);
            }
            state.success = false;
        }
    }

    Ok(state)
}

/// Calls Gemini API with the given prompt.
async fn call_gemini_api(api_key: &str, prompt: &str) -> Result<String, anyhow::Error> {
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
            "temperature": 0.7,
            "maxOutputTokens": 8192
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

    let text = json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to extract text from API response"))?;

    Ok(text.to_string())
}

/// Checks if the error message indicates a rate limit.
fn is_rate_limit_error(error: &str) -> bool {
    let error_lower = error.to_lowercase();
    error_lower.contains("429")
        || error_lower.contains("rate limit")
        || error_lower.contains("quota")
}
