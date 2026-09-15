//! Model configuration, protocol definitions, and profile resolution.

/// Default model identifier for search nodes (`gemma-4-26b-a4b-it`).
pub const MODEL_SEARCH: &str = "gemma-4-26b-a4b-it";

/// Default fallback model identifier for search nodes (`gemini-2.5-flash`).
pub const MODEL_SEARCH_FALLBACK: &str = "gemini-2.5-flash";

/// Default model identifier for report synthesis (`gemini-3.5-flash-lite`).
pub const MODEL_REPORT: &str = "gemini-3.5-flash-lite";

/// Default model identifier for interface creation (`gemini-3.5-flash-lite`).
pub const MODEL_CREATE_INTERFACE: &str = "gemini-3.5-flash-lite";

/// Default model identifier for report validation (`gemini-3.5-flash-lite`).
pub const MODEL_VALIDATE: &str = "gemini-3.5-flash-lite";

/// Default model identifier for translation (`gemini-3.5-flash-lite`).
pub const MODEL_TRANSLATE: &str = "gemini-3.5-flash-lite";

// Backward-compatibility aliases
pub const MODEL_GEMMA_4: &str = MODEL_SEARCH;
pub const MODEL_FALLBACK_FLASH: &str = MODEL_SEARCH_FALLBACK;
pub const MODEL_GEMINI_3_5_FLASH_LITE: &str = MODEL_REPORT;

/// Resolves the model to use for search nodes, checking the `MODEL_SEARCH` environment variable.
#[must_use]
pub fn get_model_search() -> String {
    std::env::var("MODEL_SEARCH").unwrap_or_else(|_| MODEL_SEARCH.to_string())
}

/// Resolves the model to use as search fallback, checking `MODEL_SEARCH_FALLBACK` or `GEMINI_FALLBACK_MODEL`.
#[must_use]
pub fn get_model_search_fallback() -> String {
    std::env::var("MODEL_SEARCH_FALLBACK")
        .or_else(|_| std::env::var("GEMINI_FALLBACK_MODEL"))
        .unwrap_or_else(|_| MODEL_SEARCH_FALLBACK.to_string())
}

/// Resolves the model to use for report synthesis, checking the `MODEL_REPORT` environment variable.
#[must_use]
pub fn get_model_report() -> String {
    std::env::var("MODEL_REPORT").unwrap_or_else(|_| MODEL_REPORT.to_string())
}

/// Resolves the model to use for interface creation, checking the `MODEL_CREATE_INTERFACE` environment variable.
#[must_use]
pub fn get_model_create_interface() -> String {
    std::env::var("MODEL_CREATE_INTERFACE").unwrap_or_else(|_| MODEL_CREATE_INTERFACE.to_string())
}

/// Resolves the model to use for report validation, checking the `MODEL_VALIDATE` environment variable.
#[must_use]
pub fn get_model_validate() -> String {
    std::env::var("MODEL_VALIDATE").unwrap_or_else(|_| MODEL_VALIDATE.to_string())
}

/// Resolves the model to use for translation, checking the `MODEL_TRANSLATE` environment variable.
#[must_use]
pub fn get_model_translate() -> String {
    std::env::var("MODEL_TRANSLATE").unwrap_or_else(|_| MODEL_TRANSLATE.to_string())
}

/// Optional API configuration overrides.
#[derive(Debug, Clone)]
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

/// API communication protocol supported by Google generative endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiProtocol {
    /// Traditional `GenerateContent` endpoint (`v1beta/models/{model}:generateContent`)
    GenerateContent,
    /// Modern Interactions API endpoint (`v1beta/interactions`)
    Interactions,
}

/// Thinking capabilities supported by a model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThinkingSupport {
    /// Supports `thinkingConfig` with `thinkingLevel` (e.g. Gemini 3.x, Gemma 4).
    Level,
    /// Does not support `thinkingLevel` (e.g. Gemini 2.5, Gemini 1.5).
    None,
}

/// Configuration profile resolved for a model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelProfile {
    pub protocol: ApiProtocol,
    pub thinking_support: ThinkingSupport,
}

impl ModelProfile {
    /// Resolves the communication protocol and thinking support for a given model.
    #[must_use]
    pub fn for_model(model: &str) -> Self {
        let lower = model.to_lowercase();

        let is_interaction = lower.contains("gemini-2.5")
            || lower.contains("interactions")
            || std::env::var("FORCE_INTERACTIONS_API")
                .is_ok_and(|v| v == "1" || v.to_lowercase() == "true");

        let protocol = if is_interaction {
            ApiProtocol::Interactions
        } else {
            ApiProtocol::GenerateContent
        };

        let thinking_support = if lower.contains("gemini-2.5")
            || lower.contains("gemini-1.5")
            || is_interaction
        {
            ThinkingSupport::None
        } else {
            ThinkingSupport::Level
        };

        Self {
            protocol,
            thinking_support,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_constants() {
        assert_eq!(MODEL_SEARCH, "gemma-4-26b-a4b-it");
        assert_eq!(MODEL_SEARCH_FALLBACK, "gemini-2.5-flash");
        assert_eq!(MODEL_REPORT, "gemini-3.5-flash-lite");
        assert_eq!(MODEL_CREATE_INTERFACE, "gemini-3.5-flash-lite");
        assert_eq!(MODEL_VALIDATE, "gemini-3.5-flash-lite");
        assert_eq!(MODEL_TRANSLATE, "gemini-3.5-flash-lite");

        // Backward compatibility constants
        assert_eq!(MODEL_GEMMA_4, "gemma-4-26b-a4b-it");
        assert_eq!(MODEL_FALLBACK_FLASH, "gemini-2.5-flash");
        assert_eq!(MODEL_GEMINI_3_5_FLASH_LITE, "gemini-3.5-flash-lite");
    }

    #[test]
    fn test_model_resolvers_defaults() {
        assert_eq!(get_model_search(), MODEL_SEARCH);
        assert_eq!(get_model_search_fallback(), MODEL_SEARCH_FALLBACK);
        assert_eq!(get_model_report(), MODEL_REPORT);
        assert_eq!(get_model_create_interface(), MODEL_CREATE_INTERFACE);
        assert_eq!(get_model_validate(), MODEL_VALIDATE);
        assert_eq!(get_model_translate(), MODEL_TRANSLATE);
    }

    #[test]
    fn test_model_profile_resolution() {
        let flash_2_5 = ModelProfile::for_model("gemini-2.5-flash");
        assert_eq!(flash_2_5.protocol, ApiProtocol::Interactions);
        assert_eq!(flash_2_5.thinking_support, ThinkingSupport::None);

        let prefixed_flash = ModelProfile::for_model("models/gemini-2.5-flash");
        assert_eq!(prefixed_flash.protocol, ApiProtocol::Interactions);
        assert_eq!(prefixed_flash.thinking_support, ThinkingSupport::None);

        let gemma = ModelProfile::for_model("gemma-4-26b-a4b-it");
        assert_eq!(gemma.protocol, ApiProtocol::GenerateContent);
        assert_eq!(gemma.thinking_support, ThinkingSupport::Level);

        let lite = ModelProfile::for_model("gemini-3.5-flash-lite");
        assert_eq!(lite.protocol, ApiProtocol::GenerateContent);
        assert_eq!(lite.thinking_support, ThinkingSupport::Level);
    }
}

