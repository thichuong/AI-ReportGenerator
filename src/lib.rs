//! AI Report Generator - Rust Implementation
//!
//! This crate provides the core functionality for:
//! - Database operations (PostgreSQL)
//! - Workflow orchestration for AI report generation
//! - Auto report scheduler
//! - REST API (Axum)

pub mod api;
pub mod db;
pub mod scheduler;
pub mod workflow;

pub use api::{create_router, AppState};
pub use db::create_pool;
pub use scheduler::start_auto_report_scheduler;
pub use workflow::run_workflow;
