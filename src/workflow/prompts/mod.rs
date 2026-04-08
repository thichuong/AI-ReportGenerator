//! Prompts module
//!
//! Contains all AI prompts used in the workflow.
//! Previously loaded from .md files in prompt_envs/.

pub mod interface;
pub mod report_validator;
pub mod report_writer;
pub mod research_macro;
pub mod research_tech;
pub mod translation;

use chrono::Utc;

/// Replaces common placeholders in prompt text.
pub fn process_placeholders(text: &str) -> String {
    let now = Utc::now();
    text.replace("<<@day>>", &now.format("%d").to_string())
        .replace("<<@month>>", &now.format("%m").to_string())
        .replace("<<@year>>", &now.format("%Y").to_string())
        .replace("{{DATE}}", &now.format("%Y-%m-%d").to_string())
        .replace("{{YEAR}}", &now.format("%Y").to_string())
        .replace("{{MONTH}}", &now.format("%m").to_string())
        .replace("{{DAY}}", &now.format("%d").to_string())
}
