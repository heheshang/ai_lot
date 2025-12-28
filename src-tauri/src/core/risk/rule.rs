//! Risk rule trait and types
//!
//! Defines the core trait for risk management rules and associated types.

use crate::core::trade::types::{Position, Order};
use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Risk rule trait
///
/// All risk management rules must implement this trait.
/// Rules are checked periodically against the current trading context.
#[async_trait]
pub trait RiskRule: Send + Sync {
    /// Returns the rule name
    ///
    /// This should be a unique identifier for the rule type.
    fn name(&self) -> &str;

    /// Returns the rule description
    ///
    /// Should explain what this rule checks and why it's important.
    fn description(&self) -> &str;

    /// Checks if the risk rule is triggered
    ///
    /// # Arguments
    /// * `context` - The current risk context including positions, orders, and balance
    ///
    /// # Returns
    /// * `Ok(true)` - Rule is triggered and risk action should be taken
    /// * `Ok(false)` - Rule is not triggered
    /// * `Err(_)` - Error occurred during checking
    async fn check(&self, context: &RiskContext) -> Result<bool>;

    /// Returns the rule configuration
    fn config(&self) -> &RiskRuleConfig;

    /// Updates the rule configuration
    ///
    /// # Arguments
    /// * `config` - New configuration to apply
    ///
    /// # Returns
    /// * `Ok(())` - Configuration updated successfully
    /// * `Err(_)` - Failed to update configuration
    fn update_config(&mut self, config: RiskRuleConfig) -> Result<()>;
}

/// Risk check context
///
/// Contains all the information needed to evaluate risk rules.
#[derive(Clone, Debug)]
pub struct RiskContext {
    /// Current positions
    pub positions: Vec<Position>,
    /// Active orders
    pub orders: Vec<Order>,
    /// Account balance
    pub balance: f64,
    /// Today's profit and loss
    pub today_pnl: f64,
    /// Strategy instance ID
    pub instance_id: String,
}

impl RiskContext {
    /// Creates a new risk context
    ///
    /// # Arguments
    /// * `positions` - Current positions
    /// * `orders` - Active orders
    /// * `balance` - Account balance
    /// * `today_pnl` - Today's P&L
    /// * `instance_id` - Strategy instance ID
    #[must_use]
    pub const fn new(
        positions: Vec<Position>,
        orders: Vec<Order>,
        balance: f64,
        today_pnl: f64,
        instance_id: String,
    ) -> Self {
        Self {
            positions,
            orders,
            balance,
            today_pnl,
            instance_id,
        }
    }

    /// Creates an empty risk context
    ///
    /// Useful for testing or initial state.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            positions: Vec::new(),
            orders: Vec::new(),
            balance: 0.0,
            today_pnl: 0.0,
            instance_id: String::new(),
        }
    }

    /// Calculates total position value
    ///
    /// Sums the value of all positions (quantity * entry price).
    #[must_use]
    pub fn total_position_value(&self) -> f64 {
        self.positions
            .iter()
            .map(|p| p.quantity * p.entry_price)
            .sum()
    }

    /// Calculates total unrealized PnL
    ///
    /// Sums the unrealized PnL from all positions.
    #[must_use]
    pub fn total_unrealized_pnl(&self) -> f64 {
        self.positions.iter().map(|p| p.unrealized_pnl).sum()
    }

    /// Gets the number of active positions
    #[must_use]
    pub fn position_count(&self) -> usize {
        self.positions.len()
    }

    /// Gets the number of active orders
    #[must_use]
    pub fn order_count(&self) -> usize {
        self.orders.len()
    }
}

/// Risk rule configuration
///
/// Defines how a risk rule behaves when triggered.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskRuleConfig {
    /// Whether the rule is enabled
    ///
    /// Disabled rules are still checked but won't trigger actions.
    pub enabled: bool,
    /// Action to take when rule is triggered
    pub action: RiskAction,
    /// Notification methods to use
    ///
    /// Supported values: "dingtalk", "email", "log"
    pub notify_methods: Vec<String>,
}

impl RiskRuleConfig {
    /// Creates a new risk rule configuration
    ///
    /// # Arguments
    /// * `enabled` - Whether the rule is enabled
    /// * `action` - Action to take when triggered
    /// * `notify_methods` - Notification methods to use
    #[must_use]
    pub fn new(enabled: bool, action: RiskAction, notify_methods: Vec<String>) -> Self {
        Self {
            enabled,
            action,
            notify_methods,
        }
    }

    /// Creates a default configuration with logging only
    #[must_use]
    pub fn log_only() -> Self {
        Self {
            enabled: true,
            action: RiskAction::LogOnly,
            notify_methods: vec!["log".to_string()],
        }
    }

    /// Creates a default configuration with notification
    #[must_use]
    pub fn notify_only(notify_methods: Vec<String>) -> Self {
        Self {
            enabled: true,
            action: RiskAction::Notify,
            notify_methods,
        }
    }
}

impl Default for RiskRuleConfig {
    fn default() -> Self {
        Self::log_only()
    }
}

/// Risk action to take when a rule is triggered
///
/// Defines the severity of response to a risk rule being triggered.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskAction {
    /// Only log the trigger
    ///
    /// Lowest severity - records the event but takes no action.
    LogOnly,

    /// Send notification
    ///
    /// Low severity - sends notification but doesn't affect trading.
    Notify,

    /// Pause the strategy
    ///
    /// Medium severity - stops new orders but keeps positions open.
    PauseStrategy,

    /// Close all positions
    ///
    /// High severity - liquidates all positions immediately.
    ClosePositions,

    /// Emergency stop
    ///
    /// Highest severity - stops everything and closes all positions.
    EmergencyStop,
}

impl RiskAction {
    /// Returns true if this action stops trading
    #[must_use]
    pub const fn stops_trading(&self) -> bool {
        matches!(
            self,
            Self::PauseStrategy | Self::ClosePositions | Self::EmergencyStop
        )
    }

    /// Returns true if this action closes positions
    #[must_use]
    pub const fn closes_positions(&self) -> bool {
        matches!(self, Self::ClosePositions | Self::EmergencyStop)
    }

    /// Returns the severity level (0-4)
    #[must_use]
    pub const fn severity(&self) -> u8 {
        match self {
            Self::LogOnly => 0,
            Self::Notify => 1,
            Self::PauseStrategy => 2,
            Self::ClosePositions => 3,
            Self::EmergencyStop => 4,
        }
    }
}

impl std::fmt::Display for RiskAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LogOnly => write!(f, "log_only"),
            Self::Notify => write!(f, "notify"),
            Self::PauseStrategy => write!(f, "pause_strategy"),
            Self::ClosePositions => write!(f, "close_positions"),
            Self::EmergencyStop => write!(f, "emergency_stop"),
        }
    }
}

impl std::str::FromStr for RiskAction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "log_only" | "log-only" | "logonly" => Ok(Self::LogOnly),
            "notify" => Ok(Self::Notify),
            "pause_strategy" | "pause-strategy" | "pausestrategy" => Ok(Self::PauseStrategy),
            "close_positions" | "close-positions" | "closepositions" => Ok(Self::ClosePositions),
            "emergency_stop" | "emergency-stop" | "emergencystop" => Ok(Self::EmergencyStop),
            _ => anyhow::bail!("Invalid risk action: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_context_empty() {
        let ctx = RiskContext::empty();
        assert_eq!(ctx.position_count(), 0);
        assert_eq!(ctx.order_count(), 0);
        assert_eq!(ctx.total_position_value(), 0.0);
        assert_eq!(ctx.total_unrealized_pnl(), 0.0);
    }

    #[test]
    fn test_risk_context_calculations() {
        let positions = vec![
            Position {
                id: "1".to_string(),
                symbol: "BTCUSDT".to_string(),
                side: "long".to_string(),
                quantity: 1.0,
                entry_price: 50000.0,
                current_price: Some(51000.0),
                unrealized_pnl: 1000.0,
                realized_pnl: 0.0,
                opened_at: 1234567890,
            },
            Position {
                id: "2".to_string(),
                symbol: "ETHUSDT".to_string(),
                side: "long".to_string(),
                quantity: 10.0,
                entry_price: 3000.0,
                current_price: Some(3100.0),
                unrealized_pnl: 1000.0,
                realized_pnl: 0.0,
                opened_at: 1234567891,
            },
        ];

        let ctx = RiskContext::new(
            positions.clone(),
            vec![],
            10000.0,
            500.0,
            "test".to_string(),
        );

        assert_eq!(ctx.position_count(), 2);
        assert_eq!(ctx.total_position_value(), 80000.0); // 1*50000 + 10*3000
        assert_eq!(ctx.total_unrealized_pnl(), 2000.0); // 1000 + 1000
    }

    #[test]
    fn test_risk_rule_config_default() {
        let config = RiskRuleConfig::default();
        assert!(config.enabled);
        assert_eq!(config.action, RiskAction::LogOnly);
        assert_eq!(config.notify_methods, vec!["log".to_string()]);
    }

    #[test]
    fn test_risk_action_severity() {
        assert_eq!(RiskAction::LogOnly.severity(), 0);
        assert_eq!(RiskAction::Notify.severity(), 1);
        assert_eq!(RiskAction::PauseStrategy.severity(), 2);
        assert_eq!(RiskAction::ClosePositions.severity(), 3);
        assert_eq!(RiskAction::EmergencyStop.severity(), 4);
    }

    #[test]
    fn test_risk_action_stops_trading() {
        assert!(!RiskAction::LogOnly.stops_trading());
        assert!(!RiskAction::Notify.stops_trading());
        assert!(RiskAction::PauseStrategy.stops_trading());
        assert!(RiskAction::ClosePositions.stops_trading());
        assert!(RiskAction::EmergencyStop.stops_trading());
    }

    #[test]
    fn test_risk_action_closes_positions() {
        assert!(!RiskAction::LogOnly.closes_positions());
        assert!(!RiskAction::Notify.closes_positions());
        assert!(!RiskAction::PauseStrategy.closes_positions());
        assert!(RiskAction::ClosePositions.closes_positions());
        assert!(RiskAction::EmergencyStop.closes_positions());
    }

    #[test]
    fn test_risk_action_from_str() {
        assert_eq!(
            "notify".parse::<RiskAction>().unwrap(),
            RiskAction::Notify
        );
        assert_eq!(
            "pause_strategy".parse::<RiskAction>().unwrap(),
            RiskAction::PauseStrategy
        );
        assert_eq!(
            "emergency-stop".parse::<RiskAction>().unwrap(),
            RiskAction::EmergencyStop
        );
        assert!("invalid".parse::<RiskAction>().is_err());
    }

    #[test]
    fn test_risk_action_display() {
        assert_eq!(format!("{}", RiskAction::Notify), "notify");
        assert_eq!(format!("{}", RiskAction::EmergencyStop), "emergency_stop");
    }
}
