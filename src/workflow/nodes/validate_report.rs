//! Validate report node
//!
//! Validates the research content quality.
//! Equivalent to `app/services/workflow_nodes/validate_report.py`

use crate::workflow::state::ReportState;
use tracing::{info, warn};

/// Validates the research report content.
///
/// Checks if the research content meets quality requirements.
pub async fn validate_report(mut state: ReportState) -> Result<ReportState, anyhow::Error> {
    let session_id = &state.session_id.clone();
    info!("[{}] Step 3: Validate report", session_id);

    // Check rate limit flag
    if state.rate_limit_stop {
        warn!(
            "[{}] Skipping validation - rate limit flag is set",
            session_id
        );
        state.validation_result = Some("SKIP".to_string());
        return Ok(state);
    }

    // Get research content
    let research_content = match &state.research_content {
        Some(content) if !content.is_empty() => content,
        _ => {
            warn!("[{}] No research content to validate", session_id);
            state.validation_result = Some("FAIL".to_string());
            state.add_error("No research content available for validation");
            return Ok(state);
        }
    };

    // Perform validation checks
    let validation_result = check_report_validation(research_content);

    info!("[{}] Validation result: {}", session_id, validation_result);
    state.validation_result = Some(validation_result);

    Ok(state)
}

/// Checks report validation based on content quality.
///
/// Returns "PASS", "FAIL", or "UNKNOWN".
fn check_report_validation(content: &str) -> String {
    let content_lower = content.to_lowercase();

    // Check for explicit validation markers
    if content_lower.contains("validation: pass")
        || content_lower.contains("validation_result: pass")
        || content_lower.contains("✅ pass")
    {
        return "PASS".to_string();
    }

    if content_lower.contains("validation: fail")
        || content_lower.contains("validation_result: fail")
        || content_lower.contains("❌ fail")
    {
        return "FAIL".to_string();
    }

    // Basic quality checks
    let word_count = content.split_whitespace().count();

    // Must have minimum content length
    if word_count < 100 {
        return "FAIL".to_string();
    }

    // Check for required sections (basic heuristic)
    let has_analysis = content_lower.contains("phân tích")
        || content_lower.contains("analysis")
        || content_lower.contains("đánh giá");
    let has_data = content_lower.contains("dữ liệu")
        || content_lower.contains("data")
        || content_lower.contains("thị trường");

    if has_analysis && has_data {
        return "PASS".to_string();
    }

    "UNKNOWN".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_pass_explicit() {
        let content = "Some content with Validation: PASS marker";
        assert_eq!(check_report_validation(content), "PASS");
    }

    #[test]
    fn test_validation_fail_short_content() {
        let content = "Too short";
        assert_eq!(check_report_validation(content), "FAIL");
    }

    #[test]
    fn test_validation_pass_quality() {
        let content = "Đây là bản phân tích thị trường với dữ liệu chi tiết. ".repeat(20);
        assert_eq!(check_report_validation(&content), "PASS");
    }
}
