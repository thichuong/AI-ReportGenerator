//! Workflow nodes
//!
//! Individual node implementations for the report generation workflow.
//! Equivalent to `app/services/workflow_nodes/`

mod create_interface;
mod extract_code;
mod prepare_data;
mod report_writer;
mod search_breaking_news;
mod search_btcd_altcoins;
mod search_corporate_treasury;
mod search_etf_flows;
mod search_events_calendar;
mod search_fear_greed;
mod search_macro_economy;
mod search_price_technicals;
mod search_regulatory_legal;
mod search_whale_onchain;
mod translate;
pub mod utils;
mod validate_report;

pub use create_interface::create_interface;
pub use extract_code::extract_code;
pub use prepare_data::prepare_data;
pub use report_writer::report_writer;
pub use search_breaking_news::search_breaking_news;
pub use search_btcd_altcoins::search_btcd_altcoins;
pub use search_corporate_treasury::search_corporate_treasury;
pub use search_etf_flows::search_etf_flows;
pub use search_events_calendar::search_events_calendar;
pub use search_fear_greed::search_fear_greed;
pub use search_macro_economy::search_macro_economy;
pub use search_price_technicals::search_price_technicals;
pub use search_regulatory_legal::search_regulatory_legal;
pub use search_whale_onchain::search_whale_onchain;
pub use translate::translate;
pub use validate_report::validate_report;
