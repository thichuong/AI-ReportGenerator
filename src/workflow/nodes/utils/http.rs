//! HTTP client and security utilities for workflow nodes.

use regex::Regex;
use std::sync::OnceLock;

/// Returns a lazily initialized static `reqwest::Client`.
///
/// # Panics
///
/// Panics if the reqwest client builder fails to initialize.
#[allow(clippy::expect_used)]
#[must_use]
pub fn get_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        let timeout_secs = std::env::var("HTTP_REQUEST_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(300); // 5 minutes default

        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to create reqwest client")
    })
}

static SENSITIVE_REGEXES: OnceLock<(Regex, Regex, Regex)> = OnceLock::new();

/// Masks sensitive query params (e.g. `key=***`), headers, and URL credentials.
///
/// # Panics
///
/// Panics if the internal regex patterns fail to compile (only possible if hardcoded pattern is invalid).
#[allow(clippy::expect_used)]
#[must_use]
pub fn mask_sensitive_data(input: &str) -> String {
    let (key_re, header_re, url_auth_re) = SENSITIVE_REGEXES.get_or_init(|| {
        (
            Regex::new(r#"(?i)(key=)[^&\s"'`\)]+"#).expect("Valid regex for key param"),
            Regex::new(r#"(?i)(x-goog-api-key:\s*)[^\s"'`]+"#).expect("Valid regex for api key header"),
            Regex::new(r"(?i)(://[^:\s/]+:)[^@\s/]+(@)").expect("Valid regex for url auth"),
        )
    });

    let s1 = key_re.replace_all(input, "${1}***");
    let s2 = header_re.replace_all(&s1, "${1}***");
    url_auth_re.replace_all(&s2, "${1}***${2}").to_string()
}

/// Calculates exponential backoff delay with jitter.
///
/// Delay sequence:
/// - Attempt 1: ~2s
/// - Attempt 2: ~5s
/// - Attempt 3: ~10s
/// - Attempt 4: ~15s
/// - Attempt 5: ~30s
/// - Attempt 6: ~45s
/// - Attempt 7..10: ~60s (capped at 60s base)
///
/// Adds +0% to +25% jitter to prevent thundering herd.
#[must_use]
pub fn calculate_backoff_delay(attempt: u32) -> std::time::Duration {
    let base_secs: u64 = match attempt {
        1 => 2,
        2 => 5,
        3 => 10,
        4 => 15,
        5 => 30,
        6 => 45,
        _ => 60,
    };

    let base_ms = base_secs * 1000;
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.subsec_nanos());
    let jitter_pct = u64::from(nanos % 26); // 0..25%
    let total_ms = base_ms + (base_ms * jitter_pct) / 100;
    std::time::Duration::from_millis(total_ms)
}

/// Checks if the error message indicates a rate limit.
#[must_use]
pub fn is_rate_limit_error(error: &str) -> bool {
    let error_lower = error.to_lowercase();
    error_lower.contains("429")
        || error_lower.contains("rate limit")
        || error_lower.contains("quota")
        || error_lower.contains("resource exhausted")
        || error_lower.contains("resource_exhausted")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_sensitive_data_key_param() {
        let input = "error sending request for url (https://generativelanguage.googleapis.com/v1beta/models/gemini:generateContent?key=AIzaSyABC123xyz): connection reset";
        let masked = mask_sensitive_data(input);
        assert!(!masked.contains("AIzaSyABC123xyz"));
        assert!(masked.contains("key=***"));
    }

    #[test]
    fn test_mask_sensitive_data_multiple_params() {
        let input = "https://example.com/api?foo=bar&key=SECRET_KEY_123&test=1";
        let masked = mask_sensitive_data(input);
        assert!(!masked.contains("SECRET_KEY_123"));
        assert!(masked.contains("key=***"));
        assert!(masked.contains("foo=bar"));
        assert!(masked.contains("&test=1"));
    }

    #[test]
    fn test_mask_sensitive_data_header() {
        let input = "Request headers: x-goog-api-key: AIzaSyTopSecretKey\nContent-Type: application/json";
        let masked = mask_sensitive_data(input);
        assert!(!masked.contains("AIzaSyTopSecretKey"));
        assert!(masked.contains("x-goog-api-key: ***"));
    }

    #[test]
    fn test_mask_sensitive_data_url_auth() {
        let input = "Connecting to redis://default:superpassword123@redis-node:6379/0";
        let masked = mask_sensitive_data(input);
        assert!(!masked.contains("superpassword123"));
        assert!(masked.contains("redis://default:***@redis-node:6379/0"));
    }

    #[test]
    #[allow(clippy::duration_suboptimal_units)]
    fn test_calculate_backoff_delay_ranges() {
        let d1 = calculate_backoff_delay(1);
        assert!(d1 >= std::time::Duration::from_secs(2));
        assert!(d1 <= std::time::Duration::from_millis(2600));

        let d2 = calculate_backoff_delay(2);
        assert!(d2 >= std::time::Duration::from_secs(5));
        assert!(d2 <= std::time::Duration::from_millis(6500));

        let d5 = calculate_backoff_delay(5);
        assert!(d5 >= std::time::Duration::from_secs(30));
        assert!(d5 <= std::time::Duration::from_secs(38));

        let d10 = calculate_backoff_delay(10);
        assert!(d10 >= std::time::Duration::from_secs(60));
        assert!(d10 <= std::time::Duration::from_secs(76));
    }

    #[test]
    fn test_is_rate_limit_error() {
        assert!(is_rate_limit_error("HTTP 429 Too Many Requests"));
        assert!(is_rate_limit_error("Quota exceeded for quota metric"));
        assert!(is_rate_limit_error("RESOURCE_EXHAUSTED"));
        assert!(!is_rate_limit_error("HTTP 500 Internal Error"));
    }
}
