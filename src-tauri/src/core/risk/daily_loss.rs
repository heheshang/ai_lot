//! Daily Loss Limit Rule
//!
//! Stops trading when daily losses exceed a configured threshold.

use crate::core::risk::rule::{RiskRule, RiskContext, RiskRuleConfig, RiskAction};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chrono::{Utc, Timelike, Datelike};

/// Daily loss limit rule parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyLossLimitParams {
    /// Maximum daily loss amount (in base currency, e.g., USDT)
    pub max_daily_loss: f64,
    /// Reset hour (0-23) - when to reset the daily loss counter
    pub reset_hour: u8,
    /// Reset minute (0-59)
    pub reset_minute: u8,
}

impl Default for DailyLossLimitParams {
    fn default() -> Self {
        Self {
            max_daily_loss: 1000.0,
            reset_hour: 0,
            reset_minute: 0,
        }
    }
}

/// Daily loss limit rule
///
/// Monitors daily trading losses and triggers when they exceed the threshold.
/// The daily counter resets at the configured time each day.
pub struct DailyLossLimitRule {
    config: RiskRuleConfig,
    params: DailyLossLimitParams,
    // Track daily losses per instance: (instance_id -> (date_str, accumulated_loss))
    daily_losses: Arc<RwLock<HashMap<String, (String, f64)>>>,
}

impl DailyLossLimitRule {
    /// Create a new daily loss limit rule
    pub fn new(params: DailyLossLimitParams, action: RiskAction) -> Self {
        Self {
            config: RiskRuleConfig {
                enabled: true,
                action,
                notify_methods: vec!["log".to_string()],
            },
            params,
            daily_losses: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a default rule with warning action
    pub fn default_with_warning() -> Self {
        Self::new(DailyLossLimitParams::default(), RiskAction::Notify)
    }

    /// Create a default rule with stop strategy action
    pub fn default_with_stop() -> Self {
        Self::new(DailyLossLimitParams::default(), RiskAction::PauseStrategy)
    }

    /// Get the current date key for tracking
    fn get_date_key() -> String {
        let now = Utc::now();
        format!("{}-{}-{}", now.year(), now.month(), now.day())
    }

    /// Check if we need to reset the daily counter
    fn should_reset(&self) -> bool {
        let now = Utc::now();
        now.hour() as u8 == self.params.reset_hour && now.minute() as u8 == self.params.reset_minute && now.second() < 10
    }

    /// Get or initialize daily loss for an instance
    fn get_daily_loss(&self, instance_id: &str) -> f64 {
        let mut losses = self.daily_losses.write().unwrap();
        let current_date = Self::get_date_key();

        if let Some((date, loss)) = losses.get(instance_id) {
            if *date == current_date {
                *loss
            } else {
                // New day, reset
                losses.insert(instance_id.to_string(), (current_date, 0.0));
                0.0
            }
        } else {
            losses.insert(instance_id.to_string(), (current_date, 0.0));
            0.0
        }
    }

    /// Update daily loss for an instance
    fn update_daily_loss(&self, instance_id: &str, loss: f64) {
        let mut losses = self.daily_losses.write().unwrap();
        let current_date = Self::get_date_key();

        let current_loss = if let Some((date, existing_loss)) = losses.get(instance_id) {
            if *date == current_date {
                *existing_loss
            } else {
                0.0
            }
        } else {
            0.0
        };

        losses.insert(instance_id.to_string(), (current_date, current_loss + loss));
    }

    /// Reset daily loss for an instance
    pub fn reset_daily_loss(&self, instance_id: &str) {
        let mut losses = self.daily_losses.write().unwrap();
        let current_date = Self::get_date_key();
        losses.insert(instance_id.to_string(), (current_date, 0.0));
    }

    /// Get all daily losses
    pub fn get_all_losses(&self) -> HashMap<String, (String, f64)> {
        self.daily_losses.read().unwrap().clone()
    }
}

#[async_trait]
impl RiskRule for DailyLossLimitRule {
    fn name(&self) -> &str {
        "daily_loss_limit"
    }

    fn description(&self) -> &str {
        "Stops trading when daily losses exceed the configured threshold"
    }

    async fn check(&self, context: &RiskContext) -> Result<bool> {
        if !self.config.enabled {
            return Ok(false);
        }

        // Reset if needed
        if self.should_reset() {
            self.reset_daily_loss(&context.instance_id);
        }

        // Get current daily loss
        let daily_loss = self.get_daily_loss(&context.instance_id);

        // Add current day's P&L (negative value means loss)
        let current_day_pnl = context.today_pnl;
        if current_day_pnl < 0.0 {
            self.update_daily_loss(&context.instance_id, current_day_pnl.abs());
            let updated_loss = self.get_daily_loss(&context.instance_id);

            // Check if threshold exceeded
            Ok(updated_loss >= self.params.max_daily_loss)
        } else {
            // No losses today
            Ok(false)
        }
    }

    fn config(&self) -> &RiskRuleConfig {
        &self.config
    }

    fn update_config(&mut self, config: RiskRuleConfig) -> Result<()> {
        self.config = config;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_loss_tracking() {
        let rule = DailyLossLimitRule::new(
            DailyLossLimitParams {
                max_daily_loss: 500.0,
                ..Default::default()
            },
            RiskAction::Warning,
        );

        let instance_id = "test-instance";

        // Initial state
        assert_eq!(rule.get_daily_loss(instance_id), 0.0);

        // Add some loss
        rule.update_daily_loss(instance_id, 200.0);
        assert_eq!(rule.get_daily_loss(instance_id), 200.0);

        // Add more loss
        rule.update_daily_loss(instance_id, 150.0);
        assert_eq!(rule.get_daily_loss(instance_id), 350.0);

        // Reset
        rule.reset_daily_loss(instance_id);
        assert_eq!(rule.get_daily_loss(instance_id), 0.0);
    }

    #[test]
    fn test_date_key_format() {
        let key = DailyLossLimitRule::get_date_key();
        assert!(key.contains('-'));
        let parts: Vec<&str> = key.split('-').collect();
        assert_eq!(parts.len(), 3);
    }

    #[test]
    fn test_default_params() {
        let params = DailyLossLimitParams::default();
        assert_eq!(params.max_daily_loss, 1000.0);
        assert_eq!(params.reset_hour, 0);
        assert_eq!(params.reset_minute, 0);
    }
}
