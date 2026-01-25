//! Create interface node
//!
//! Creates HTML/CSS/JS interface for the report.
//! Equivalent to `app/services/workflow_nodes/create_interface.py`

use crate::workflow::state::ReportState;
use tracing::{error, info};

/// Creates the HTML/CSS/JS interface for the report.
pub async fn create_interface(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!(
        "[{}] Step 5: Create interface (attempt {})",
        session_id,
        state.interface_attempt + 1
    );

    // Check rate limit flag
    if state.rate_limit_stop {
        error!(
            "[{}] Skipping interface creation - rate limit flag is set",
            session_id
        );
        return Ok(state);
    }

    // Get report content
    let report_content = match &state.report_content {
        Some(content) => content.clone(),
        None => {
            let error_msg = "Report content is missing";
            error!("[{}] {}", session_id, error_msg);
            state.add_error(error_msg);
            state.success = false;
            return Ok(state);
        }
    };

    // Build interface creation prompt
    let prompt = format!(
        r#"Create a complete, responsive HTML page for a crypto market report.

## Requirements:
1. Use modern CSS with dark theme
2. Include interactive elements with JavaScript
3. Make it mobile-responsive
4. Use professional design with gradients and animations

## Report Content:
{}

## Output Format:
Return the complete code in three separate code blocks:
```html
<!-- HTML code here -->
```

```css
/* CSS code here */
```

```javascript
// JavaScript code here
```"#,
        report_content
    );

    // Call Gemini API
    match call_gemini_api(&state.api_key, &prompt).await {
        Ok(response) => {
            info!("[{}] Interface code generated", session_id);
            state.report_content = Some(response);
            state.success = true;
        }
        Err(e) => {
            let error_msg = format!("Interface creation failed: {}", e);
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
