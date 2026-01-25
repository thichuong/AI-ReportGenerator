//! Progress tracking for workflow execution
//!
//! Equivalent to `app/services/progress_tracker.py`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Progress data for a workflow session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressData {
    pub session_id: String,
    pub status: String,
    pub current_step: u32,
    pub total_steps: u32,
    pub step_name: String,
    pub details: String,
    pub error: Option<String>,
    pub report_id: Option<i32>,
    pub completed: bool,
}

impl ProgressData {
    pub fn new(session_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            status: "starting".to_string(),
            current_step: 0,
            total_steps: 9,
            step_name: "Initializing".to_string(),
            details: String::new(),
            error: None,
            report_id: None,
            completed: false,
        }
    }
}

/// Thread-safe progress tracker.
#[derive(Debug, Default)]
pub struct ProgressTracker {
    sessions: HashMap<String, ProgressData>,
}

impl ProgressTracker {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    /// Starts tracking progress for a new session.
    pub fn start_progress(&mut self, session_id: &str) {
        let progress = ProgressData::new(session_id);
        self.sessions.insert(session_id.to_string(), progress);
    }

    /// Updates progress for a session.
    pub fn update_step(&mut self, session_id: &str, step: u32, step_name: &str, details: &str) {
        if let Some(progress) = self.sessions.get_mut(session_id) {
            progress.current_step = step;
            progress.step_name = step_name.to_string();
            progress.details = details.to_string();
            progress.status = "in_progress".to_string();
        }
    }

    /// Marks a session as completed.
    pub fn complete_progress(&mut self, session_id: &str, report_id: Option<i32>) {
        if let Some(progress) = self.sessions.get_mut(session_id) {
            progress.completed = true;
            progress.status = "completed".to_string();
            progress.report_id = report_id;
            progress.step_name = "Done".to_string();
        }
    }

    /// Marks a session as failed.
    pub fn error_progress(&mut self, session_id: &str, error: &str) {
        if let Some(progress) = self.sessions.get_mut(session_id) {
            progress.status = "error".to_string();
            progress.error = Some(error.to_string());
            progress.completed = true;
        }
    }

    /// Gets progress for a session.
    pub fn get_progress(&self, session_id: &str) -> Option<ProgressData> {
        self.sessions.get(session_id).cloned()
    }

    /// Removes old completed sessions (cleanup).
    pub fn cleanup_old_sessions(&mut self, max_sessions: usize) {
        if self.sessions.len() > max_sessions {
            // Remove completed sessions first
            let completed: Vec<String> = self
                .sessions
                .iter()
                .filter(|(_, p)| p.completed)
                .map(|(k, _)| k.clone())
                .collect();

            for session_id in completed
                .into_iter()
                .take(self.sessions.len() - max_sessions)
            {
                self.sessions.remove(&session_id);
            }
        }
    }
}
