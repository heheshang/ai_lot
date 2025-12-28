//! Drawdown limit risk rule
//!
//! This rule monitors equity drawdown and triggers protective actions
//! when the drawdown exceeds a specified threshold.

use super::rule::{RiskContext, RiskRule, RiskRuleConfig};
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;

/// Drawdown limit risk rule
///
/// Monitors equity drawdown from peak equity and triggers when
/// the drawdown percentage exceeds the configured limit.
pub struct DrawdownLimitRule {
    config: RiskRuleConfig,
    /// Maximum drawdown percentage allowed
    max_drawdown_pct: f64,
    /// Peak equity tracking per strategy instance
    peak_equity: HashMap<String, f64>,
}

impl DrawdownLimitRule {
    /// Creates a new drawdown limit rule
    ///
    /// # Arguments
    /// * `max_drawdown_pct` - Maximum drawdown percentage (e.g., 10.0 for 10%)
    pub fn new(max_drawdown_pct: f64) -> Self {
        Self {
            config: RiskRuleConfig {
                enabled: true,
                action: super::rule::RiskAction::ClosePositions,
                notify_methods: vec!["dingtalk".to_string(), "email".to_string()],
            },
            max_drawdown_pct,
            peak_equity: HashMap::new(),
        }
    }

    /// Creates a new drawdown limit rule with custom configuration
    ///
    /// # Arguments
    /// * `max_drawdown_pct` - Maximum drawdown percentage
    /// * `config` - Custom risk rule configuration
    pub fn with_config(max_drawdown_pct: f64, config: RiskRuleConfig) -> Self {
        Self {
            config,
            max_drawdown_pct,
            peak_equity: HashMap::new(),
        }
    }

    /// Calculates drawdown percentage
    ///
    /// # Arguments
    /// * `current_equity` - Current total equity
    /// * `peak` - Peak equity recorded
    ///
    /// # Returns
    /// Drawdown as a percentage (0-100)
    fn calculate_drawdown(&self, current_equity: f64, peak: f64) -> f64 {
        if peak <= 0.0 {
            return 0.0;
        }
        if current_equity < 0.0 {
            // Negative equity means 100%+ drawdown
            return 100.0;
        }
        let drawdown = ((peak - current_equity) / peak) * 100.0;
        drawdown.max(0.0)
    }

    /// Updates peak equity for a strategy instance
    ///
    /// # Arguments
    /// * `instance_id` - Strategy instance identifier
    /// * `current_equity` - Current total equity
    pub fn update_peak(&mut self, instance_id: &str, current_equity: f64) {
        let peak = self
            .peak_equity
            .entry(instance_id.to_string())
            .or_insert(current_equity);
        *peak = (*peak).max(current_equity);
    }

    /// Gets current peak equity for an instance
    ///
    /// # Arguments
    /// * `instance_id` - Strategy instance identifier
    ///
    /// # Returns
    /// Peak equity value, or None if not tracked
    pub fn get_peak(&self, instance_id: &str) -> Option<f64> {
        self.peak_equity.get(instance_id).copied()
    }

    /// Gets the current max drawdown percentage
    pub fn max_drawdown_pct(&self) -> f64 {
        self.max_drawdown_pct
    }

    /// Sets a new max drawdown percentage
    pub fn set_max_drawdown_pct(&mut self, value: f64) {
        self.max_drawdown_pct = value;
    }

    /// Calculates total equity from balance and positions
    ///
    /// # Arguments
    /// * `context` - Risk context containing positions and balance
    ///
    /// # Returns
    /// Total equity (balance + position value)
    fn calculate_total_equity(&self, context: &RiskContext) -> f64 {
        let position_value: f64 = context
            .positions
            .iter()
            .map(|p| p.quantity * p.entry_price + p.unrealized_pnl)
            .sum();
        context.balance + position_value
    }
}

#[async_trait]
impl RiskRule for DrawdownLimitRule {
    fn name(&self) -> &str {
        "drawdown_limit"
    }

    fn description(&self) -> &str {
        "Limits maximum drawdown percentage and triggers protective actions when exceeded"
    }

    async fn check(&self, context: &RiskContext) -> Result<bool> {
        // Calculate current total equity
        let current_equity = self.calculate_total_equity(context);

        // Get or initialize peak equity
        let peak = self
            .peak_equity
            .get(&context.instance_id)
            .copied()
            .unwrap_or(current_equity);

        // Calculate drawdown percentage
        let drawdown = self.calculate_drawdown(current_equity, peak);

        // Check if limit is exceeded
        if drawdown > self.max_drawdown_pct {
            log::warn!(
                "Drawdown limit exceeded: {:.2}% > {:.2}% (equity: {:.2}, peak: {:.2})",
                drawdown,
                self.max_drawdown_pct,
                current_equity,
                peak
            );
            return Ok(true);
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
    use super::super::rule::RiskAction;
    use super::*;
    use crate::core::trade::types::Position;

    fn create_test_context(instance_id: &str, balance: f64, positions: Vec<Position>) -> RiskContext {
        RiskContext {
            positions,
            orders: vec![],
            balance,
            today_pnl: 0.0,
            instance_id: instance_id.to_string(),
        }
    }

    fn create_test_position(
        symbol: &str,
        side: &str,
        quantity: f64,
        entry_price: f64,
        unrealized_pnl: f64,
    ) -> Position {
        Position {
            id: format!("pos_{}", symbol),
            symbol: symbol.to_string(),
            side: side.to_string(),
            quantity,
            entry_price,
            current_price: Some(entry_price),
            unrealized_pnl,
            realized_pnl: 0.0,
            opened_at: chrono::Utc::now().timestamp(),
        }
    }

    #[test]
    fn test_drawdown_calculation() {
        let rule = DrawdownLimitRule::new(10.0);

        // No drawdown
        assert_eq!(rule.calculate_drawdown(1000.0, 1000.0), 0.0);

        // 10% drawdown
        assert_eq!(rule.calculate_drawdown(900.0, 1000.0), 10.0);

        // 50% drawdown
        assert_eq!(rule.calculate_drawdown(500.0, 1000.0), 50.0);

        // 100% drawdown
        assert_eq!(rule.calculate_drawdown(0.0, 1000.0), 100.0);

        // Peak is zero
        assert_eq!(rule.calculate_drawdown(100.0, 0.0), 0.0);

        // Negative equity
        assert_eq!(rule.calculate_drawdown(-100.0, 1000.0), 100.0);
    }

    #[test]
    fn test_update_peak() {
        let mut rule = DrawdownLimitRule::new(10.0);

        // First update sets the peak
        rule.update_peak("instance1", 1000.0);
        assert_eq!(rule.get_peak("instance1"), Some(1000.0));

        // Lower value doesn't change peak
        rule.update_peak("instance1", 900.0);
        assert_eq!(rule.get_peak("instance1"), Some(1000.0));

        // Higher value updates peak
        rule.update_peak("instance1", 1100.0);
        assert_eq!(rule.get_peak("instance1"), Some(1100.0));

        // Different instance
        rule.update_peak("instance2", 500.0);
        assert_eq!(rule.get_peak("instance1"), Some(1100.0));
        assert_eq!(rule.get_peak("instance2"), Some(500.0));
    }

    #[test]
    fn test_calculate_total_equity() {
        let rule = DrawdownLimitRule::new(10.0);

        // No positions
        let context = create_test_context("inst1", 1000.0, vec![]);
        assert_eq!(rule.calculate_total_equity(&context), 1000.0);

        // With positions
        let positions = vec![
            create_test_position("BTC", "long", 1.0, 50000.0, 1000.0),
            create_test_position("ETH", "long", 10.0, 3000.0, -500.0),
        ];
        let context = create_test_context("inst1", 1000.0, positions.clone());
        
        // Equity = balance + position values + unrealized PnL
        // = 1000 + (1*50000 + 10*3000) + (1000 - 500)
        // = 1000 + 80000 + 500 = 81500
        assert_eq!(rule.calculate_total_equity(&context), 81500.0);
    }

    #[tokio::test]
    async fn test_check_within_limit() {
        let mut rule = DrawdownLimitRule::new(10.0);

        // Set initial peak
        rule.update_peak("instance1", 10000.0);

        // Current equity is 9500 (5% drawdown)
        let context = create_test_context("instance1", 9500.0, vec![]);

        let triggered = rule.check(&context).await.unwrap();
        assert!(!triggered, "Should not trigger at 5% drawdown");
    }

    #[tokio::test]
    async fn test_check_exceeds_limit() {
        let mut rule = DrawdownLimitRule::new(10.0);

        // Set initial peak
        rule.update_peak("instance1", 10000.0);

        // Current equity is 8500 (15% drawdown)
        let context = create_test_context("instance1", 8500.0, vec![]);

        let triggered = rule.check(&context).await.unwrap();
        assert!(triggered, "Should trigger at 15% drawdown");
    }

    #[tokio::test]
    async fn test_check_at_limit_boundary() {
        let mut rule = DrawdownLimitRule::new(10.0);

        // Set initial peak
        rule.update_peak("instance1", 10000.0);

        // Current equity is exactly at 10% drawdown
        let context = create_test_context("instance1", 9000.0, vec![]);

        let triggered = rule.check(&context).await.unwrap();
        assert!(!triggered, "Should not trigger at exactly 10% drawdown");
    }

    #[tokio::test]
    async fn test_check_with_positions() {
        let mut rule = DrawdownLimitRule::new(20.0);

        // Set peak with positions
        let positions = vec![create_test_position("BTC", "long", 1.0, 50000.0, 0.0)];
        let context = create_test_context("instance1", 10000.0, positions.clone());
        rule.update_peak("instance1", rule.calculate_total_equity(&context));

        // Now with loss
        let positions_with_loss = vec![create_test_position("BTC", "long", 1.0, 50000.0, -10000.0)];
        let context_loss = create_test_context("instance1", 10000.0, positions_with_loss);

        // Total equity = 10000 + 50000 - 10000 = 50000
        // Peak was 60000, so drawdown = (60000-50000)/60000 = 16.67%
        let triggered = rule.check(&context_loss).await.unwrap();
        assert!(!triggered, "Should not trigger at 16.67% drawdown when limit is 20%");
    }

    #[test]
    fn test_new_rule_config() {
        let rule = DrawdownLimitRule::new(15.0);

        assert_eq!(rule.max_drawdown_pct(), 15.0);
        assert!(rule.config().enabled);
        matches!(rule.config().action, RiskAction::ClosePositions);
        assert_eq!(rule.config().notify_methods.len(), 2);
    }

    #[test]
    fn test_with_custom_config() {
        let config = RiskRuleConfig {
            enabled: false,
            action: RiskAction::Notify,
            notify_methods: vec!["email".to_string()],
        };

        let rule = DrawdownLimitRule::with_config(5.0, config.clone());

        assert_eq!(rule.max_drawdown_pct(), 5.0);
        assert!(!rule.config().enabled);
        matches!(rule.config().action, RiskAction::Notify);
        assert_eq!(rule.config().notify_methods.len(), 1);
    }

    #[test]
    fn test_setters() {
        let mut rule = DrawdownLimitRule::new(10.0);

        rule.set_max_drawdown_pct(25.0);
        assert_eq!(rule.max_drawdown_pct(), 25.0);

        let new_config = RiskRuleConfig {
            enabled: false,
            action: RiskAction::PauseStrategy,
            notify_methods: vec![],
        };

        rule.update_config(new_config.clone()).unwrap();
        assert!(!rule.config().enabled);
        matches!(rule.config().action, RiskAction::PauseStrategy);
    }

    #[test]
    fn test_name_and_description() {
        let rule = DrawdownLimitRule::new(10.0);

        assert_eq!(rule.name(), "drawdown_limit");
        assert!(!rule.description().is_empty());
    }

    #[tokio::test]
    async fn test_multiple_instances() {
        let mut rule = DrawdownLimitRule::new(10.0);

        // Set different peaks for different instances
        rule.update_peak("instance1", 10000.0);
        rule.update_peak("instance2", 20000.0);

        // Instance 1: 15% drawdown
        let context1 = create_test_context("instance1", 8500.0, vec![]);
        assert!(rule.check(&context1).await.unwrap());

        // Instance 2: 5% drawdown
        let context2 = create_test_context("instance2", 19000.0, vec![]);
        assert!(!rule.check(&context2).await.unwrap());
    }
}
