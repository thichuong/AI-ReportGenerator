//! Validate report node
//!
//! Validates the research content quality.
//! Equivalent to `app/services/workflow_nodes/validate_report.py`

use crate::workflow::state::ReportState;
use regex::Regex;
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
    
    // Update success status based on validation
    if state.validation_result.as_deref() == Some("PASS") {
         // Keep success=true (default/previous state check might be needed if false)
    } else if state.validation_result.as_deref() == Some("FAIL") {
         state.success = false;
    }

    Ok(state)
}

/// Checks report validation based on content quality.
///
/// Returns "PASS", "FAIL", or "UNKNOWN".
fn check_report_validation(content: &str) -> String {
    let content_lower = content.to_lowercase();

    // Check for Vietnamese validation markers (match Python)
    if content.contains("KẾT QUẢ KIỂM TRA: PASS") || content.contains("KẾT QUẢ KIỂM TRA:  PASS")
    {
        return "PASS".to_string();
    }

    if content.contains("KẾT QUẢ KIỂM TRA: FAIL") || content.contains("KẾT QUẢ KIỂM TRA:  FAIL")
    {
        return "FAIL".to_string();
    }

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

    // Fallback quality validation (Match Python logic)
    // 1. Length check (> 2000 chars roughly matches Python check)
    if content.len() < 2000 {
        return "FAIL".to_string();
    }

    // 2. Element checks
    let mut quality_score = 0;
    
    // has_btc
    if content_lower.contains("bitcoin") || content_lower.contains("btc") {
        quality_score += 1;
    }
    
    // has_analysis
    if content_lower.contains("phân tích") || content_lower.contains("analysis") || content_lower.contains("thị trường") || content_lower.contains("market") {
        quality_score += 1;
    }
    
    // has_numbers (regex)
    if let Ok(re) = Regex::new(r"\d+\.?\d*\s*%|\$\d+") {
        if re.is_match(content) {
            quality_score += 1;
        }
    }
    
    // has_fng
    if content_lower.contains("fear") || content_lower.contains("greed") || content_lower.contains("sợ hãi") || content_lower.contains("tham lam") {
        quality_score += 1;
    }
    
    // has_validation_table
    if content.contains("Bảng Đối chiếu") || content.contains("Validation Summary") || content.contains("| Dữ liệu") || content.contains("| BTC Price") {
        quality_score += 1;
    }
    
    if quality_score >= 4 {
        return "PASS".to_string();
    }

    "FAIL".to_string()
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
