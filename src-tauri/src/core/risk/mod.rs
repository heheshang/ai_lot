//! Risk management module
//!
//! This module provides risk monitoring and control functionality including
//! rule-based risk checks, alerting, and emergency stop capabilities.

pub mod rule;
pub mod drawdown_limit;
pub mod monitor;
pub mod position_limit;

pub use rule::{RiskRule, RiskContext, RiskRuleConfig, RiskAction};
pub use drawdown_limit::DrawdownLimitRule;
pub use monitor::RiskMonitor;
pub use position_limit::PositionLimitRule;
