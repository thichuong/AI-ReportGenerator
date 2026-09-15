//! Generative AI model invocation, payload formatting, and response parsing.

use super::http::{calculate_backoff_delay, get_client, mask_sensitive_data};
use super::models::{
    ApiConfig, ApiProtocol, ModelProfile, ThinkingSupport, get_model_create_interface,
    get_model_report, get_model_search, get_model_search_fallback, get_model_translate,
    get_model_validate,
};
use tracing::{error, info, warn};

#[must_use]
pub fn build_gemini_request_body(
    prompt: &str,
    enable_search: bool,
    json_mode: bool,
    config_override: Option<&ApiConfig>,
    thinking_support: ThinkingSupport,
) -> serde_json::Value {
    let temperature = config_override.map_or(0.3, |c| c.temperature);
    let max_output_tokens = config_override.map_or(32768, |c| c.max_output_tokens);
    let thinking_level = config_override.map_or("HIGH", |c| c.thinking_level.as_str());

    let mut config = serde_json::json!({
        "temperature": temperature,
        "maxOutputTokens": max_output_tokens,
    });

    if thinking_support == ThinkingSupport::Level
        && let Some(obj) = config.as_object_mut()
    {
        obj.insert(
            "thinkingConfig".to_string(),
            serde_json::json!({
                "thinkingLevel": thinking_level
            }),
        );
    }

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

#[must_use]
pub fn build_interaction_request_body(
    model: &str,
    prompt: &str,
    enable_search: bool,
    json_mode: bool,
    config_override: Option<&ApiConfig>,
) -> serde_json::Value {
    let model_name = if model.starts_with("models/") {
        model.to_string()
    } else {
        format!("models/{model}")
    };

    let temp = config_override.map_or(1.0, |c| c.temperature);
    let max_tokens = config_override.map_or(65536, |c| c.max_output_tokens);

    let mut generation_config = serde_json::json!({
        "temperature": temp,
        "max_output_tokens": max_tokens,
        "top_p": 0.95,
    });

    if json_mode && let Some(obj) = generation_config.as_object_mut() {
        obj.insert(
            "response_mime_type".to_string(),
            serde_json::Value::String("application/json".to_string()),
        );
        if let Some(num) = serde_json::Number::from_f64(0.1) {
            obj.insert("temperature".to_string(), serde_json::Value::Number(num));
        }
    }

    let mut body = serde_json::json!({
        "model": model_name,
        "input": prompt,
        "generation_config": generation_config,
    });

    if enable_search && let Some(obj) = body.as_object_mut() {
        obj.insert(
            "tools".to_string(),
            serde_json::json!([{ "type": "google_search" }]),
        );
    }

    body
}

#[must_use]
pub fn extract_text_from_response(json: &serde_json::Value) -> Option<String> {
    let mut full_text = String::new();

    // 1. generateContent schema: candidates[0].content.parts[].text
    if let Some(parts_arr) = json
        .get("candidates")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("content"))
        .and_then(|c| c.get("parts"))
        .and_then(|p| p.as_array())
    {
        for part in parts_arr {
            if let Some(part_text) = part.get("text").and_then(|t| t.as_str()) {
                full_text.push_str(part_text);
            }
        }
    }

    // 2. Interactions API schema: top-level "output_text"
    if full_text.is_empty()
        && let Some(txt) = json.get("output_text").and_then(|t| t.as_str())
    {
        let trimmed = txt.trim();
        if !trimmed.is_empty() {
            full_text.push_str(trimmed);
        }
    }

    // 3. Interactions API schema: "steps" timeline with "model_output"
    if full_text.is_empty()
        && let Some(steps) = json.get("steps").and_then(|s| s.as_array())
    {
        for step in steps {
            if step.get("type").and_then(|t| t.as_str()) == Some("model_output") {
                if let Some(content) = step.get("content") {
                    if let Some(arr) = content.as_array() {
                        for item in arr {
                            if let Some(t) = item.get("text").and_then(|s| s.as_str()) {
                                full_text.push_str(t);
                            }
                        }
                    } else if let Some(t) = content.get("text").and_then(|s| s.as_str()) {
                        full_text.push_str(t);
                    } else if let Some(s) = content.as_str() {
                        full_text.push_str(s);
                    }
                } else if let Some(t) = step.get("text").and_then(|s| s.as_str()) {
                    full_text.push_str(t);
                }
            }
        }
    }

    // 4. Interactions API schema: "outputs" array
    if full_text.is_empty()
        && let Some(outputs) = json.get("outputs").and_then(|o| o.as_array())
    {
        for item in outputs {
            if item.get("type").and_then(|t| t.as_str()) == Some("text") {
                if let Some(t) = item.get("text").and_then(|s| s.as_str()) {
                    full_text.push_str(t);
                }
            } else if let Some(t) = item.get("text").and_then(|s| s.as_str()) {
                full_text.push_str(t);
            }
        }
    }

    if full_text.is_empty() {
        None
    } else {
        Some(full_text)
    }
}

/// Low-level function to call Google Generative Language API with a specific model.
///
/// Automatically routes between `GenerateContent` and Interactions API based on the model profile.
///
/// # Errors
///
/// Returns an error if:
/// - The API request fails and all retries are exhausted.
/// - The API response cannot be parsed.
/// - No text can be extracted from the AI response.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
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
    let profile = ModelProfile::for_model(model);

    let (url, body) = match profile.protocol {
        ApiProtocol::Interactions => (
            "https://generativelanguage.googleapis.com/v1beta/interactions".to_string(),
            build_interaction_request_body(
                model,
                prompt,
                enable_search,
                json_mode,
                config_override.as_ref(),
            ),
        ),
        ApiProtocol::GenerateContent => (
            format!(
                "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent"
            ),
            build_gemini_request_body(
                prompt,
                enable_search,
                json_mode,
                config_override.as_ref(),
                profile.thinking_support,
            ),
        ),
    };

    let debug_enabled = std::env::var("DEBUG").map_or(true, |v| v.to_lowercase() == "true");

    if debug_enabled {
        info!(
            "[{}] Calling {} API ({}) via {:?}. Body size: {} bytes",
            session_id,
            model,
            node_name,
            profile.protocol,
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

/// Calls the search API with up to 10 retries using `MODEL_SEARCH` (or `MODEL_SEARCH` env var).
/// If all retries are exhausted, falls back to `MODEL_SEARCH_FALLBACK` (or `MODEL_SEARCH_FALLBACK` env var) for 1 attempt.
///
/// # Errors
///
/// Returns an error if both primary and fallback API requests fail or responses cannot be parsed.
#[allow(clippy::too_many_arguments)]
pub async fn call_search_api(
    api_key: &str,
    prompt: &str,
    session_id: &str,
    node_name: &str,
    enable_search: bool,
    json_mode: bool,
    config_override: Option<ApiConfig>,
) -> Result<String, anyhow::Error> {
    let primary_model = get_model_search();
    let primary_result = call_model_api(
        &primary_model,
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
            let fallback_model = get_model_search_fallback();

            warn!(
                "[{}] Node {} primary model {} failed after 10 retries: {}. Switching to fallback model {} (1 attempt)...",
                session_id, node_name, primary_model, primary_err, fallback_model
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
                        "Primary model ({primary_model}) failed after 10 retries: {primary_err}. Fallback ({fallback_model}) also failed: {fallback_err}"
                    ))
                }
            }
        }
    }
}

/// Backward compatibility alias for `call_search_api`.
///
/// # Errors
///
/// Returns an error if both primary and fallback API requests fail or responses cannot be parsed.
pub async fn call_gemma_api(
    api_key: &str,
    prompt: &str,
    session_id: &str,
    node_name: &str,
    enable_search: bool,
    json_mode: bool,
    config_override: Option<ApiConfig>,
) -> Result<String, anyhow::Error> {
    call_search_api(
        api_key,
        prompt,
        session_id,
        node_name,
        enable_search,
        json_mode,
        config_override,
    )
    .await
}

/// Calls the report synthesis model using `MODEL_REPORT` (or `MODEL_REPORT` env var).
///
/// # Errors
///
/// Returns an error if the API request fails or response cannot be parsed.
pub async fn call_report_api(
    api_key: &str,
    prompt: &str,
    session_id: &str,
    node_name: &str,
    json_mode: bool,
    config_override: Option<ApiConfig>,
) -> Result<String, anyhow::Error> {
    let model = get_model_report();
    call_model_api(
        &model,
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

/// Calls the report validation model using `MODEL_VALIDATE` (or `MODEL_VALIDATE` env var).
///
/// # Errors
///
/// Returns an error if the API request fails or response cannot be parsed.
pub async fn call_validate_api(
    api_key: &str,
    prompt: &str,
    session_id: &str,
    node_name: &str,
    json_mode: bool,
    config_override: Option<ApiConfig>,
) -> Result<String, anyhow::Error> {
    let model = get_model_validate();
    call_model_api(
        &model,
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

/// Calls the UI interface generation model using `MODEL_CREATE_INTERFACE` (or `MODEL_CREATE_INTERFACE` env var).
///
/// # Errors
///
/// Returns an error if the API request fails or response cannot be parsed.
pub async fn call_interface_api(
    api_key: &str,
    prompt: &str,
    session_id: &str,
    node_name: &str,
    json_mode: bool,
    config_override: Option<ApiConfig>,
) -> Result<String, anyhow::Error> {
    let model = get_model_create_interface();
    call_model_api(
        &model,
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

/// Calls the translation model using `MODEL_TRANSLATE` (or `MODEL_TRANSLATE` env var).
///
/// # Errors
///
/// Returns an error if the API request fails or response cannot be parsed.
pub async fn call_translate_api(
    api_key: &str,
    prompt: &str,
    session_id: &str,
    node_name: &str,
    json_mode: bool,
    config_override: Option<ApiConfig>,
) -> Result<String, anyhow::Error> {
    let model = get_model_translate();
    call_model_api(
        &model,
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

/// Backward compatibility alias for `call_report_api` / legacy Gemini Flash Lite caller.
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
    call_report_api(
        api_key,
        prompt,
        session_id,
        node_name,
        json_mode,
        config_override,
    )
    .await
}

#[cfg(test)]
#[allow(clippy::indexing_slicing)]
mod tests {
    use super::*;

    #[test]
    fn test_build_gemini_request_body_thinking_support() {
        let prompt = "test prompt";
        let body_level = build_gemini_request_body(prompt, false, false, None, ThinkingSupport::Level);
        assert!(body_level["generationConfig"].get("thinkingConfig").is_some());

        let body_none = build_gemini_request_body(prompt, false, false, None, ThinkingSupport::None);
        assert!(body_none["generationConfig"].get("thinkingConfig").is_none());
    }

    #[test]
    fn test_build_interaction_request_body() {
        let prompt = "Bitcoin price analysis";
        let body = build_interaction_request_body(
            "gemini-2.5-flash",
            prompt,
            true,
            false,
            None,
        );

        assert_eq!(body["model"], "models/gemini-2.5-flash");
        assert_eq!(body["input"], prompt);
        assert_eq!(body["tools"][0]["type"], "google_search");
        assert_eq!(body["generation_config"]["temperature"], 1.0);
        assert_eq!(body["generation_config"]["max_output_tokens"], 65536);
        assert_eq!(body["generation_config"]["top_p"], 0.95);
        // Verify thinkingConfig is NOT present
        assert!(body["generation_config"].get("thinkingConfig").is_none());
        assert!(body["generation_config"].get("thinking_level").is_none());
    }

    #[test]
    fn test_extract_text_from_response_generate_content() {
        let json = serde_json::json!({
            "candidates": [{
                "content": {
                    "parts": [{ "text": "Generated from generateContent" }]
                }
            }]
        });
        assert_eq!(
            extract_text_from_response(&json),
            Some("Generated from generateContent".to_string())
        );
    }

    #[test]
    fn test_extract_text_from_response_interactions_output_text() {
        let json = serde_json::json!({
            "id": "int_123",
            "output_text": "Generated from output_text property"
        });
        assert_eq!(
            extract_text_from_response(&json),
            Some("Generated from output_text property".to_string())
        );
    }

    #[test]
    fn test_extract_text_from_response_interactions_steps() {
        let json = serde_json::json!({
            "steps": [
                {
                    "type": "google_search_call",
                    "arguments": { "queries": ["bitcoin price"] }
                },
                {
                    "type": "model_output",
                    "content": [
                        { "type": "text", "text": "Generated from steps timeline" }
                    ]
                }
            ]
        });
        assert_eq!(
            extract_text_from_response(&json),
            Some("Generated from steps timeline".to_string())
        );
    }

    #[test]
    fn test_extract_text_from_response_interactions_outputs() {
        let json = serde_json::json!({
            "outputs": [
                { "type": "google_search_call" },
                { "type": "text", "text": "Generated from outputs array" }
            ]
        });
        assert_eq!(
            extract_text_from_response(&json),
            Some("Generated from outputs array".to_string())
        );
    }
}

