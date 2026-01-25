//! Auto report scheduler module
//!
//! Provides scheduled report generation.
//! Equivalent to `app/services/auto_report_scheduler.py`

use chrono::{NaiveTime, Timelike, Utc};
use chrono_tz::Asia::Ho_Chi_Minh;
use sqlx::PgPool;
use std::env;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};

use crate::workflow;

/// Auto report scheduler that runs at specified times.
pub struct AutoReportScheduler {
    schedule_times: Vec<NaiveTime>,
    api_key: String,
    pool: PgPool,
}

impl AutoReportScheduler {
    /// Creates a new scheduler if configuration is valid.
    ///
    /// Returns `None` if:
    /// - `GEMINI_API_KEY` is not set
    /// - `ENABLE_AUTO_REPORT_SCHEDULER` is not "true"
    pub fn new(pool: PgPool) -> Option<Self> {
        let api_key = env::var("GEMINI_API_KEY").ok()?;

        let enable = env::var("ENABLE_AUTO_REPORT_SCHEDULER")
            .map(|v| v.to_lowercase() == "true")
            .unwrap_or(false);

        if !enable {
            info!("Auto report scheduler disabled (ENABLE_AUTO_REPORT_SCHEDULER != true)");
            return None;
        }

        let schedule_times = Self::parse_schedule_times();
        info!("Scheduler times: {:?}", schedule_times);

        Some(Self {
            schedule_times,
            api_key,
            pool,
        })
    }

    /// Parses schedule times from environment or uses defaults.
    ///
    /// Format: "HH:MM,HH:MM,..." (e.g., "07:30,19:00")
    /// Default: ["07:30", "19:00"] (Vietnam time)
    fn parse_schedule_times() -> Vec<NaiveTime> {
        env::var("AUTO_REPORT_SCHEDULE_TIMES")
            .map(|times| {
                times
                    .split(',')
                    .filter_map(|t| NaiveTime::parse_from_str(t.trim(), "%H:%M").ok())
                    .collect()
            })
            .unwrap_or_else(|_| {
                vec![
                    NaiveTime::from_hms_opt(7, 30, 0).expect("Valid time"),
                    NaiveTime::from_hms_opt(19, 0, 0).expect("Valid time"),
                ]
            })
    }

    /// Runs the scheduler loop indefinitely.
    pub async fn run(&self) {
        info!(
            "🎯 Auto report scheduler started with times: {:?} (GMT+7)",
            self.schedule_times
        );

        loop {
            // Calculate next run time
            let _now = Utc::now().with_timezone(&Ho_Chi_Minh);
            let next_run = self.get_next_run_time();
            let wait_duration = self.calculate_wait_duration(next_run);

            info!(
                "⏰ Next scheduled run at: {} (in {:?})",
                next_run.format("%Y-%m-%d %H:%M:%S %Z"),
                wait_duration
            );

            // Wait until next scheduled time
            sleep(wait_duration).await;

            // Verify we're still within tolerance (5 minutes)
            if !self.is_within_schedule_tolerance(&Utc::now().with_timezone(&Ho_Chi_Minh)) {
                warn!("Skipping run - not within scheduled time window");
                continue;
            }

            info!("🚀 Starting scheduled report generation...");
            let start = std::time::Instant::now();

            // Run report generation
            match self.generate_report().await {
                Ok(report_id) => {
                    let duration = start.elapsed();
                    info!(
                        "✅ Report #{} generated successfully in {:?}",
                        report_id, duration
                    );
                }
                Err(e) => {
                    let duration = start.elapsed();
                    error!("❌ Report generation failed in {:?}: {}", duration, e);
                    info!("ℹ️ Will retry at next scheduled time");
                }
            }
        }
    }

    /// Gets the next scheduled run time.
    fn get_next_run_time(&self) -> chrono::DateTime<chrono_tz::Tz> {
        let now = Utc::now().with_timezone(&Ho_Chi_Minh);
        let today = now.date_naive();

        // Find next time slot today
        for time in &self.schedule_times {
            let scheduled = today.and_time(*time);
            let scheduled = scheduled
                .and_local_timezone(Ho_Chi_Minh)
                .single()
                .expect("Valid timezone");

            if scheduled > now {
                return scheduled;
            }
        }

        // No more slots today, use first slot tomorrow
        let tomorrow = today.succ_opt().expect("Valid date");
        let first_time = self.schedule_times.first().expect("At least one time");
        let scheduled = tomorrow.and_time(*first_time);
        scheduled
            .and_local_timezone(Ho_Chi_Minh)
            .single()
            .expect("Valid timezone")
    }

    /// Calculates wait duration until target time.
    fn calculate_wait_duration(&self, target: chrono::DateTime<chrono_tz::Tz>) -> Duration {
        let now = Utc::now().with_timezone(&Ho_Chi_Minh);
        let diff = target.signed_duration_since(now);

        if diff.num_seconds() > 0 {
            Duration::from_secs(diff.num_seconds() as u64)
        } else {
            Duration::from_secs(0)
        }
    }

    /// Checks if current time is within 5-minute tolerance of any scheduled time.
    fn is_within_schedule_tolerance(&self, now: &chrono::DateTime<chrono_tz::Tz>) -> bool {
        let current_time = now.time();
        const TOLERANCE_SECONDS: i64 = 300; // 5 minutes

        for scheduled_time in &self.schedule_times {
            let diff = (current_time.num_seconds_from_midnight() as i64)
                - (scheduled_time.num_seconds_from_midnight() as i64);

            if diff.abs() <= TOLERANCE_SECONDS {
                return true;
            }
        }

        false
    }

    /// Generates a report using the workflow.
    async fn generate_report(&self) -> Result<i32, anyhow::Error> {
        let max_attempts = env::var("MAX_REPORT_ATTEMPTS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(3);

        let result = workflow::run_workflow(&self.pool, &self.api_key, max_attempts).await?;

        result
            .report_id
            .ok_or_else(|| anyhow::anyhow!("No report ID generated"))
    }
}

/// Starts the auto report scheduler in a background task.
///
/// Returns `true` if scheduler was started, `false` otherwise.
pub fn start_auto_report_scheduler(pool: PgPool) -> bool {
    if let Some(scheduler) = AutoReportScheduler::new(pool) {
        tokio::spawn(async move {
            scheduler.run().await;
        });
        true
    } else {
        false
    }
}

/// Creates a manual report immediately.
///
/// Equivalent to Python's `create_manual_report()`.
pub async fn create_manual_report(pool: &PgPool) -> Result<i32, anyhow::Error> {
    let api_key =
        env::var("GEMINI_API_KEY").map_err(|_| anyhow::anyhow!("GEMINI_API_KEY not set"))?;

    let max_attempts = env::var("MAX_REPORT_ATTEMPTS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(3);

    info!("🚀 Manual report generation started...");
    let start = std::time::Instant::now();

    let result = workflow::run_workflow(pool, &api_key, max_attempts).await?;
    let duration = start.elapsed();

    match result.report_id {
        Some(id) => {
            info!("✅ Manual report #{} created in {:?}", id, duration);
            Ok(id)
        }
        None => {
            error!("❌ Manual report failed in {:?}", duration);
            Err(anyhow::anyhow!(
                "Report generation failed: {:?}",
                result.error_messages
            ))
        }
    }
}
