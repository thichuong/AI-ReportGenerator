//! Translate content node
//!
//! Translates content to English.
//! Equivalent to `app/services/workflow_nodes/translate_content.py`

use crate::workflow::nodes::utils::{
    ApiConfig, call_gemini_api, is_rate_limit_error, process_mathjax,
};
use crate::workflow::{prompts, state::ReportState};
use tracing::{error, info, warn};

/// Translates the report content to English.
///
/// # Errors
///
/// Returns an error if:
/// - The translation API fails for both HTML and JS (non-rate-limit errors).
/// - State transitions fail.
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
/// Returns (Option<`translated_content`>, `is_rate_limit_error`)
async fn translate_with_prompt(
    api_key: &str,
    prompt: &str,
    session_id: &str,
    suffix: &str,
) -> Result<(Option<String>, bool), anyhow::Error> {
    let config = ApiConfig {
        temperature: 0.1,
        thinking_level: "MINIMAL".to_string(),
        ..Default::default()
    };

    let node_name = format!("translate_{suffix}");

    match call_gemini_api(
        api_key,
        prompt,
        session_id,
        &node_name,
        false,
        false,
        Some(config),
    )
    .await
    {
        Ok(response) => {
            let mut cleaned = response.trim().to_string();

            // Remove markdown code blocks if present
            if cleaned.starts_with("```") {
                let lines: Vec<&str> = cleaned.lines().collect();
                if lines.len() >= 3
                    && let Some(middle) = lines.get(1..lines.len() - 1)
                {
                    cleaned = middle.join("\n");
                }
            }

            if cleaned.is_empty() {
                Ok((None, false))
            } else {
                Ok((Some(cleaned), false))
            }
        }
        Err(e) => {
            let err_str = e.to_string();
            if is_rate_limit_error(&err_str) {
                Ok((None, true))
            } else {
                Err(e)
            }
        }
    }
}
