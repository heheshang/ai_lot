//! Volatility Limit Rule
//!
//! Stops trading when market volatility exceeds a configured threshold.

use crate::core::risk::rule::{RiskRule, RiskContext, RiskRuleConfig, RiskAction};
use crate::core::trade::types::Kline;
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};

/// Volatility limit rule parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolatilityLimitParams {
    /// Maximum ATR ratio (ATR / price) allowed
    pub max_atr_ratio: f64,
    /// ATR period for volatility calculation
    pub atr_period: usize,
    /// Number of klines to track for volatility
    pub history_size: usize,
}

impl Default for VolatilityLimitParams {
    fn default() -> Self {
        Self {
            max_atr_ratio: 0.02, // 2%
            atr_period: 14,
            history_size: 100,
        }
    }
}

/// Volatility limit rule
///
/// Monitors market volatility using ATR and stops trading when volatility is too high.
pub struct VolatilityLimitRule {
    config: RiskRuleConfig,
    params: VolatilityLimitParams,
    // Track klines per symbol: (symbol -> kline_history)
    kline_history: Arc<RwLock<HashMap<String, VecDeque<Kline>>>>,
}

impl VolatilityLimitRule {
    /// Create a new volatility limit rule
    pub fn new(params: VolatilityLimitParams, action: RiskAction) -> Self {
        Self {
            config: RiskRuleConfig {
                enabled: true,
                action,
                notify_methods: vec!["log".to_string()],
            },
            params,
            kline_history: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a default rule with warning action
    pub fn default_with_warning() -> Self {
        Self::new(VolatilityLimitParams::default(), RiskAction::Notify)
    }

    /// Create a default rule with stop strategy action
    pub fn default_with_stop() -> Self {
        Self::new(VolatilityLimitParams::default(), RiskAction::PauseStrategy)
    }

    /// Update kline history with new data
    pub fn update_klines(&self, symbol: &str, klines: Vec<Kline>) {
        let mut history = self.kline_history.write().unwrap();

        let symbol_history = history.entry(symbol.to_string()).or_insert_with(|| {
            VecDeque::with_capacity(self.params.history_size)
        });

        for kline in klines {
            // Add new kline
            symbol_history.push_back(kline);

            // Remove old klines if over limit
            while symbol_history.len() > self.params.history_size {
                symbol_history.pop_front();
            }
        }
    }

    /// Calculate ATR for a symbol
    fn calculate_atr(&self, symbol: &str) -> Option<f64> {
        let history = self.kline_history.read().unwrap();
        let klines = history.get(symbol)?;

        if klines.len() < self.params.atr_period + 1 {
            return None;
        }

        let mut true_ranges = Vec::new();

        for i in 1..klines.len() {
            let prev = &klines[i - 1];
            let curr = klines.get(i)?;

            let high_low = curr.high - curr.low;
            let high_close = (curr.high - prev.close).abs();
            let low_close = (curr.low - prev.close).abs();

            true_ranges.push(high_low.max(high_close).max(low_close));
        }

        // Calculate ATR using simple moving average
        let atr_sum: f64 = true_ranges.iter().take(self.params.atr_period).sum();
        Some(atr_sum / self.params.atr_period as f64)
    }

    /// Get current volatility as ATR ratio
    pub fn get_volatility(&self, symbol: &str) -> Option<f64> {
        let history = self.kline_history.read().unwrap();
        let klines = history.get(symbol)?;

        if let Some(last_kline) = klines.back() {
            let atr = self.calculate_atr(symbol)?;
            Some(atr / last_kline.close)
        } else {
            None
        }
    }

    /// Check if volatility is too high for a symbol
    pub fn is_volatility_high(&self, symbol: &str) -> bool {
        if let Some(volatility) = self.get_volatility(symbol) {
            volatility >= self.params.max_atr_ratio
        } else {
            false
        }
    }

    /// Clear kline history for a symbol
    pub fn clear_history(&self, symbol: &str) {
        let mut history = self.kline_history.write().unwrap();
        history.remove(symbol);
    }

    /// Get number of symbols tracked
    pub fn tracked_symbols_count(&self) -> usize {
        self.kline_history.read().unwrap().len()
    }
}

#[async_trait]
impl RiskRule for VolatilityLimitRule {
    fn name(&self) -> &str {
        "volatility_limit"
    }

    fn description(&self) -> &str {
        "Stops trading when market volatility exceeds the configured threshold"
    }

    async fn check(&self, context: &RiskContext) -> Result<bool> {
        if !self.config.enabled {
            return Ok(false);
        }

        // Check volatility for each symbol in positions
        for position in &context.positions {
            if self.is_volatility_high(&position.symbol) {
                log::warn!(
                    "Volatility too high for {}: {:.2}%",
                    position.symbol,
                    self.get_volatility(&position.symbol).unwrap_or(0.0) * 100.0
                );
                return Ok(true);
            }
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

    fn create_test_kline(timestamp: i64, close: f64, range: f64) -> Kline {
        Kline {
            symbol: "BTCUSDT".to_string(),
            timeframe: "1h".to_string(),
            timestamp,
            open: close,
            high: close + range / 2.0,
            low: close - range / 2.0,
            close,
            volume: 1000.0,
            quote_volume: None,
        }
    }

    #[test]
    fn test_volatility_calculation() {
        let rule = VolatilityLimitRule::new(
            VolatilityLimitParams {
                max_atr_ratio: 0.03, // 3%
                atr_period: 14,
                history_size: 20,
            },
            RiskAction::LogOnly,
        );

        let symbol = "BTCUSDT";
        let base_price = 50000.0;

        // Create low volatility klines (1% range)
        let mut klines = Vec::new();
        for i in 0..20 {
            klines.push(create_test_kline(i * 3600, base_price, 500.0));
        }

        rule.update_klines(symbol, klines);

        // Should not be triggered with low volatility
        assert!(!rule.is_volatility_high(symbol));

        // Add high volatility klines (5% range)
        let mut high_vol_klines = Vec::new();
        for i in 0..20 {
            high_vol_klines.push(create_test_kline(20 * 3600 + i * 3600, base_price, 2500.0));
        }

        rule.update_klines(symbol, high_vol_klines);

        // Should be triggered with high volatility
        assert!(rule.is_volatility_high(symbol));
    }

    #[test]
    fn test_atr_calculation() {
        let rule = VolatilityLimitRule::new(
            VolatilityLimitParams::default(),
            RiskAction::LogOnly
        );

        let symbol = "BTCUSDT";
        let mut klines = Vec::new();

        // Create klines with predictable ranges
        for i in 0..20 {
            klines.push(create_test_kline(i * 3600, 50000.0 + i as f64 * 10.0, 500.0));
        }

        rule.update_klines(symbol, klines);

        let atr = rule.calculate_atr(symbol);
        assert!(atr.is_some());
        assert!(atr.unwrap() > 0.0);
    }

    #[test]
    fn test_history_management() {
        let rule = VolatilityLimitRule::new(
            VolatilityLimitParams {
                history_size: 5,
                ..Default::default()
            },
            RiskAction::LogOnly,
        );

        let symbol = "BTCUSDT";
        let mut klines = Vec::new();

        // Add more klines than history_size
        for i in 0..10 {
            klines.push(create_test_kline(i * 3600, 50000.0, 500.0));
        }

        rule.update_klines(symbol, klines);

        // History should be limited
        let history = rule.kline_history.read().unwrap();
        assert_eq!(history.get(symbol).unwrap().len(), 5);

        // Clear history
        rule.clear_history(symbol);
        assert_eq!(rule.tracked_symbols_count(), 0);
    }
}
