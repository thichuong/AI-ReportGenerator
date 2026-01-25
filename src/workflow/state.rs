//! Report state definition
//!
//! Equivalent to `ReportState` in `app/services/workflow_nodes/base.py`

use serde::{Deserialize, Serialize};

/// State schema for report generation workflow.
///
/// This struct holds all the data that flows through the workflow nodes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportState {
    /// Unique session identifier
    pub session_id: String,
    /// Gemini API key
    pub api_key: String,
    /// Maximum retry attempts for validation
    pub max_attempts: u32,
    /// Current attempt number
    pub current_attempt: u32,

    // Prompts
    pub research_analysis_prompt: Option<String>,
    pub data_validation_prompt: Option<String>,
    pub create_report_prompt: Option<String>,

    // Generated contents
    pub research_content: Option<String>,
    pub report_content: Option<String>,
    pub html_content: Option<String>,
    pub css_content: Option<String>,
    pub js_content: Option<String>,
    pub html_content_en: Option<String>,
    pub js_content_en: Option<String>,

    // Validation
    pub validation_result: Option<String>,

    // State flags
    pub success: bool,
    pub error_messages: Vec<String>,
    pub rate_limit_stop: bool,

    // Retry counters
    pub interface_attempt: u32,
    pub html_attempt: u32,
    pub js_attempt: u32,
    pub css_attempt: u32,

    // Result
    pub report_id: Option<i32>,

    // Cached data
    pub realtime_data: Option<String>,

    // Timestamps
    pub created_at: Option<String>,
}

impl ReportState {
    /// Creates a new ReportState with initial values.
    pub fn new(session_id: &str, api_key: &str, max_attempts: u32) -> Self {
        Self {
            session_id: session_id.to_string(),
            api_key: api_key.to_string(),
            max_attempts,
            current_attempt: 0,

            research_analysis_prompt: None,
            data_validation_prompt: None,
            create_report_prompt: None,

            research_content: None,
            report_content: None,
            html_content: None,
            css_content: None,
            js_content: None,
            html_content_en: None,
            js_content_en: None,

            validation_result: None,

            success: false,
            error_messages: Vec::new(),
            rate_limit_stop: false,

            interface_attempt: 0,
            html_attempt: 0,
            js_attempt: 0,
            css_attempt: 0,

            report_id: None,
            realtime_data: None,
            created_at: Some(chrono::Utc::now().to_rfc3339()),
        }
    }

    /// Adds an error message to the state.
    pub fn add_error(&mut self, message: &str) {
        self.error_messages.push(message.to_string());
    }

    /// Checks if there are any errors.
    pub fn has_errors(&self) -> bool {
        !self.error_messages.is_empty()
    }
}

impl Default for ReportState {
    fn default() -> Self {
        Self::new("", "", 3)
    }
}
