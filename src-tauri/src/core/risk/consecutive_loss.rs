//! Consecutive Loss Limit Rule
//!
//! Stops trading after a configured number of consecutive losing trades.

use crate::core::risk::rule::{RiskRule, RiskContext, RiskRuleConfig, RiskAction};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chrono::Utc;

/// Consecutive loss limit rule parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsecutiveLossLimitParams {
    /// Maximum number of consecutive losses allowed
    pub max_consecutive_losses: usize,
    /// Minimum loss amount to count as a loss (filters out tiny losses)
    pub min_loss_threshold: f64,
    /// Cooling period in seconds before allowing new trades after trigger
    pub cooling_period_seconds: u64,
}

impl Default for ConsecutiveLossLimitParams {
    fn default() -> Self {
        Self {
            max_consecutive_losses: 3,
            min_loss_threshold: 1.0,
            cooling_period_seconds: 3600, // 1 hour
        }
    }
}

/// Trade outcome tracking
///
/// Reserved for future use in detailed trade analysis
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
struct TradeOutcome {
    pnl: f64,
    timestamp: i64,
}

/// Consecutive loss limit rule
///
/// Tracks consecutive losing trades and stops trading when the limit is reached.
pub struct ConsecutiveLossLimitRule {
    config: RiskRuleConfig,
    params: ConsecutiveLossLimitParams,
    // Track consecutive losses per instance: (instance_id -> (count, last_trade_time, triggered_time))
    consecutive_data: Arc<RwLock<HashMap<String, (usize, i64, Option<i64>)>>>,
}

impl ConsecutiveLossLimitRule {
    /// Create a new consecutive loss limit rule
    pub fn new(params: ConsecutiveLossLimitParams, action: RiskAction) -> Self {
        Self {
            config: RiskRuleConfig {
                enabled: true,
                action,
                notify_methods: vec!["log".to_string()],
            },
            params,
            consecutive_data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a default rule with warning action
    pub fn default_with_warning() -> Self {
        Self::new(ConsecutiveLossLimitParams::default(), RiskAction::Notify)
    }

    /// Create a default rule with stop strategy action
    pub fn default_with_stop() -> Self {
        Self::new(ConsecutiveLossLimitParams::default(), RiskAction::PauseStrategy)
    }

    /// Record a trade outcome and update consecutive loss count
    pub fn record_trade(&self, instance_id: &str, pnl: f64) {
        let mut data = self.consecutive_data.write().unwrap();
        let now = Utc::now().timestamp();

        if let Some((count, last_time, triggered_time)) = data.get_mut(instance_id) {
            // Check if we're still in cooling period
            if let Some(triggered) = *triggered_time {
                if now - triggered < self.params.cooling_period_seconds as i64 {
                    // Still in cooling period, don't update count
                    return;
                }
                // Cooling period over, reset
                *triggered_time = None;
            }

            if pnl < -self.params.min_loss_threshold {
                // Another loss
                *count += 1;
            } else if pnl > 0.0 {
                // A win, reset the count
                *count = 0;
            }
            // Small losses around threshold don't affect count
            *last_time = now;
        } else {
            // First trade for this instance
            let count = if pnl < -self.params.min_loss_threshold { 1 } else { 0 };
            data.insert(instance_id.to_string(), (count, now, None));
        }
    }

    /// Get consecutive loss count for an instance
    pub fn get_consecutive_count(&self, instance_id: &str) -> usize {
        self.consecutive_data
            .read()
            .unwrap()
            .get(instance_id)
            .map(|(count, _, _)| *count)
            .unwrap_or(0)
    }

    /// Check if instance is in cooling period
    pub fn is_in_cooling_period(&self, instance_id: &str) -> bool {
        let now = Utc::now().timestamp();
        self.consecutive_data
            .read()
            .unwrap()
            .get(instance_id)
            .map(|(_, _, triggered)| {
                if let Some(triggered) = triggered {
                    now - triggered < self.params.cooling_period_seconds as i64
                } else {
                    false
                }
            })
            .unwrap_or(false)
    }

    /// Manually reset consecutive loss count for an instance
    pub fn reset_count(&self, instance_id: &str) {
        let mut data = self.consecutive_data.write().unwrap();
        if let Some((count, _, triggered)) = data.get_mut(instance_id) {
            *count = 0;
            *triggered = None;
        }
    }

    /// Get time remaining in cooling period (returns 0 if not in cooling)
    pub fn cooling_time_remaining(&self, instance_id: &str) -> u64 {
        let now = Utc::now().timestamp();
        self.consecutive_data
            .read()
            .unwrap()
            .get(instance_id)
            .map(|(_, _, triggered)| {
                if let Some(triggered) = triggered {
                    let elapsed = now - triggered;
                    let cooling_period = self.params.cooling_period_seconds as i64;
                    if elapsed < cooling_period {
                        (cooling_period - elapsed) as u64
                    } else {
                        0
                    }
                } else {
                    0
                }
            })
            .unwrap_or(0)
    }
}

#[async_trait]
impl RiskRule for ConsecutiveLossLimitRule {
    fn name(&self) -> &str {
        "consecutive_loss_limit"
    }

    fn description(&self) -> &str {
        "Stops trading after a configured number of consecutive losing trades"
    }

    async fn check(&self, context: &RiskContext) -> Result<bool> {
        if !self.config.enabled {
            return Ok(false);
        }

        // Update count based on today's P&L
        // In a real implementation, this would track each trade individually
        let _count = self.get_consecutive_count(&context.instance_id);

        // Update with today's result
        if context.today_pnl < -self.params.min_loss_threshold {
            self.record_trade(&context.instance_id, context.today_pnl);
            let updated_count = self.get_consecutive_count(&context.instance_id);

            // Check if we've hit the limit
            if updated_count >= self.params.max_consecutive_losses {
                // Mark as triggered
                let mut data = self.consecutive_data.write().unwrap();
                let now = Utc::now().timestamp();
                if let Some((_count, _last_time, triggered)) = data.get_mut(&context.instance_id) {
                    if triggered.is_none() {
                        *triggered = Some(now);
                    }
                }
                return Ok(true);
            }
        } else if context.today_pnl > 0.0 {
            // Win, reset count
            self.reset_count(&context.instance_id);
        }

        Ok(false)
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
    fn test_consecutive_loss_tracking() {
        let rule = ConsecutiveLossLimitRule::new(
            ConsecutiveLossLimitParams {
                max_consecutive_losses: 3,
                min_loss_threshold: 1.0,
                cooling_period_seconds: 60,
            },
            RiskAction::LogOnly,
        );

        let instance_id = "test-instance";

        // Initial state
        assert_eq!(rule.get_consecutive_count(instance_id), 0);

        // First loss
        rule.record_trade(instance_id, -10.0);
        assert_eq!(rule.get_consecutive_count(instance_id), 1);

        // Second loss
        rule.record_trade(instance_id, -5.0);
        assert_eq!(rule.get_consecutive_count(instance_id), 2);

        // Third loss - should trigger
        rule.record_trade(instance_id, -8.0);
        assert_eq!(rule.get_consecutive_count(instance_id), 3);
        assert!(rule.is_in_cooling_period(instance_id));

        // Reset
        rule.reset_count(instance_id);
        assert_eq!(rule.get_consecutive_count(instance_id), 0);
        assert!(!rule.is_in_cooling_period(instance_id));
    }

    #[test]
    fn test_win_resets_count() {
        let rule = ConsecutiveLossLimitRule::new(
            ConsecutiveLossLimitParams::default(),
            RiskAction::LogOnly,
        );

        let instance_id = "test-instance";

        // Two losses
        rule.record_trade(instance_id, -10.0);
        rule.record_trade(instance_id, -5.0);
        assert_eq!(rule.get_consecutive_count(instance_id), 2);

        // A win resets the count
        rule.record_trade(instance_id, 20.0);
        assert_eq!(rule.get_consecutive_count(instance_id), 0);
    }

    #[test]
    fn test_small_loss_ignored() {
        let rule = ConsecutiveLossLimitRule::new(
            ConsecutiveLossLimitParams {
                min_loss_threshold: 10.0,
                ..Default::default()
            },
            RiskAction::LogOnly,
        );

        let instance_id = "test-instance";

        // Small loss below threshold
        rule.record_trade(instance_id, -5.0);
        assert_eq!(rule.get_consecutive_count(instance_id), 0);

        // Loss above threshold
        rule.record_trade(instance_id, -15.0);
        assert_eq!(rule.get_consecutive_count(instance_id), 1);
    }
}
