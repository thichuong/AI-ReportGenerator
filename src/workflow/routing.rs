//! Conditional routing functions for workflow
//!
//! Equivalent to `app/services/workflow_nodes/routing.py`

use super::state::ReportState;

/// Routing decision enum for workflow control flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingDecision {
    /// Retry the current step
    Retry,
    /// Continue to the next step
    Continue,
    /// End the workflow
    End,
    /// Retry interface creation specifically
    RetryInterface,
    /// Retry HTML creation specifically
    RetryHtml,
    /// Retry JS creation specifically
    RetryJs,
    /// Retry CSS creation specifically
    RetryCss,
}

/// Decides next action after validation.
///
/// Returns:
/// - `Continue` if validation passed
/// - `End` if max attempts reached
/// - `Retry` otherwise
pub fn should_retry_or_continue(state: &ReportState) -> RoutingDecision {
    // If validation PASS, continue
    if state.validation_result.as_deref() == Some("PASS") {
        return RoutingDecision::Continue;
    }

    // If max attempts reached, end
    if state.current_attempt >= state.max_attempts {
        return RoutingDecision::End;
    }

    // Otherwise retry
    RoutingDecision::Retry
}

/// Decides next action after extract_code.
///
/// Returns:
/// - `Continue` if extraction successful
/// - `End` if max interface attempts (3) reached
/// - `RetryInterface` otherwise
pub fn should_retry_interface_or_continue(state: &ReportState) -> RoutingDecision {
    // If successful, continue
    if state.success {
        return RoutingDecision::Continue;
    }

    // Check interface attempt limit (max 3)
    if state.interface_attempt >= 3 {
        return RoutingDecision::End;
    }

    // Otherwise retry interface
    RoutingDecision::RetryInterface
}

/// Decides next action after create_html.
pub fn should_retry_html_or_continue(state: &ReportState) -> RoutingDecision {
    if state.success && state.html_content.is_some() {
        return RoutingDecision::Continue;
    }

    if state.html_attempt >= 3 {
        return RoutingDecision::End;
    }

    RoutingDecision::RetryHtml
}

/// Decides next action after create_javascript.
pub fn should_retry_js_or_continue(state: &ReportState) -> RoutingDecision {
    if state.success && state.js_content.is_some() {
        return RoutingDecision::Continue;
    }

    if state.js_attempt >= 3 {
        return RoutingDecision::End;
    }

    RoutingDecision::RetryJs
}

/// Decides next action after create_css.
pub fn should_retry_css_or_continue(state: &ReportState) -> RoutingDecision {
    if state.success && state.css_content.is_some() {
        return RoutingDecision::Continue;
    }

    if state.css_attempt >= 3 {
        return RoutingDecision::End;
    }

    RoutingDecision::RetryCss
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_retry_or_continue_pass() {
        let mut state = ReportState::default();
        state.validation_result = Some("PASS".to_string());

        assert_eq!(should_retry_or_continue(&state), RoutingDecision::Continue);
    }

    #[test]
    fn test_should_retry_or_continue_fail() {
        let mut state = ReportState::default();
        state.validation_result = Some("FAIL".to_string());
        state.current_attempt = 1;
        state.max_attempts = 3;

        assert_eq!(should_retry_or_continue(&state), RoutingDecision::Retry);
    }

    #[test]
    fn test_should_retry_or_continue_max_attempts() {
        let mut state = ReportState::default();
        state.validation_result = Some("FAIL".to_string());
        state.current_attempt = 3;
        state.max_attempts = 3;

        assert_eq!(should_retry_or_continue(&state), RoutingDecision::End);
    }

    #[test]
    fn test_should_retry_interface_success() {
        let mut state = ReportState::default();
        state.success = true;

        assert_eq!(
            should_retry_interface_or_continue(&state),
            RoutingDecision::Continue
        );
    }

    #[test]
    fn test_should_retry_interface_max_attempts() {
        let mut state = ReportState::default();
        state.success = false;
        state.interface_attempt = 3;

        assert_eq!(
            should_retry_interface_or_continue(&state),
            RoutingDecision::End
        );
    }
}
