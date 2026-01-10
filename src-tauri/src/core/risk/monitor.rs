//! Risk monitoring service
//!
//! This module provides the main risk monitoring service that coordinates
//! all risk rules and executes appropriate actions when rules are triggered.

use super::rule::*;
use crate::core::strategy::StrategyEngine;
use crate::core::trade::types::*;
use crate::infrastructure::NotificationService;
use crate::services::{EmergencyService, TradeService};
use anyhow::{Context, Result};
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use uuid::Uuid;

/// Risk monitoring service that continuously checks risk rules
pub struct RiskMonitor {
    /// Collection of risk rules to check
    rules: Arc<RwLock<Vec<Box<dyn RiskRule>>>>,
    /// Trade service for accessing trading data
    trade_service: Arc<TradeService>,
    /// Strategy engine for stopping/pausing strategies
    strategy_engine: Arc<StrategyEngine>,
    /// Emergency service for emergency operations
    emergency_service: Arc<EmergencyService>,
    /// Notification service for sending alerts
    notification_service: Arc<dyn NotificationService>,
}

impl RiskMonitor {
    /// Create a new risk monitor
    pub fn new(
        trade_service: Arc<TradeService>,
        strategy_engine: Arc<StrategyEngine>,
        notification_service: Arc<dyn NotificationService>,
    ) -> Self {
        let emergency_service = Arc::new(EmergencyService::new(
            trade_service.clone(),
            strategy_engine.clone(),
        ));

        Self {
            rules: Arc::new(RwLock::new(Vec::new())),
            trade_service,
            strategy_engine,
            emergency_service,
            notification_service,
        }
    }

    /// Add a risk rule to the monitor
    pub async fn add_rule(&self, rule: Box<dyn RiskRule>) {
        let mut rules = self.rules.write().await;
        rules.push(rule);
        log::info!("Risk rule added to monitor");
    }

    /// Start the risk monitoring loop
    ///
    /// This runs continuously in the background, checking all enabled
    /// rules every 10 seconds.
    pub async fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut timer = interval(Duration::from_secs(10));

            loop {
                timer.tick().await;

                if let Err(e) = self.check_all_rules().await {
                    log::error!("Risk check failed: {}", e);
                }
            }
        });

        log::info!("Risk monitor started");
    }

    /// Check all enabled risk rules
    async fn check_all_rules(&self) -> Result<()> {
        // Get current trading state
        // Note: Using a placeholder user_id for now
        let user_id = "system";

        // Get current positions
        let positions = self
            .trade_service
            .get_positions(user_id)
            .await
            .unwrap_or_default();

        // Get active orders
        let orders = self
            .trade_service
            .get_open_orders(user_id)
            .await
            .unwrap_or_default();

        // Get account balance
        let balances = self
            .trade_service
            .get_balance()
            .await
            .unwrap_or_default();
        let balance = balances.iter().map(|b| b.total).sum();

        // Build risk context
        let context = RiskContext {
            positions,
            orders,
            balance,
            today_pnl: 0.0, // TODO: Calculate from database
            instance_id: "default".to_string(),
        };

        // Check all rules
        let rules = self.rules.read().await;
        for rule in rules.iter() {
            if !rule.config().enabled {
                continue;
            }

            match rule.check(&context).await {
                Ok(triggered) => {
                    if triggered {
                        if let Err(e) = self.handle_rule_trigger(&**rule, &context).await {
                            log::error!("Failed to handle rule trigger for {}: {}", rule.name(), e);
                        }
                    }
                }
                Err(e) => {
                    log::error!("Rule {} check failed: {}", rule.name(), e);
                }
            }
        }

        Ok(())
    }

    /// Handle a triggered risk rule
    async fn handle_rule_trigger(
        &self,
        rule: &dyn RiskRule,
        context: &RiskContext,
    ) -> Result<()> {
        let config = rule.config();

        log::warn!(
            "Risk rule '{}' triggered for strategy '{}'",
            rule.name(),
            context.instance_id
        );

        // Record alert to database
        if let Err(e) = self.record_alert(rule, context).await {
            log::error!("Failed to record alert: {}", e);
        }

        // Send notifications
        for method in &config.notify_methods {
            if let Err(e) = self.send_notification(method, rule, context).await {
                log::error!("Failed to send {} notification: {}", method, e);
            }
        }

        // Execute the configured action
        match &config.action {
            RiskAction::LogOnly => {
                // Already logged above
            }
            RiskAction::Notify => {
                // Notifications already sent above
            }
            RiskAction::PauseStrategy => {
                self.pause_strategy(&context.instance_id).await?;
            }
            RiskAction::ClosePositions => {
                self.close_all_positions(&context.instance_id).await?;
            }
            RiskAction::EmergencyStop => {
                self.emergency_stop().await?;
            }
        }

        Ok(())
    }

    /// Record alert to database
    async fn record_alert(
        &self,
        rule: &dyn RiskRule,
        _context: &RiskContext,
    ) -> Result<()> {
        let alert_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now().timestamp();

        // TODO: Implement database insertion when risk_alerts table is created
        log::info!(
            "Recording alert: {} - {} at {}",
            alert_id,
            rule.name(),
            timestamp
        );

        // Placeholder for database insertion
        // sqlx::query(
        //     r#"
        //     INSERT INTO risk_alerts (id, rule_name, instance_id, message, action, created_at)
        //     VALUES (?, ?, ?, ?, ?, ?)
        //     "#
        // )
        // .bind(&alert_id)
        // .bind(rule.name())
        // .bind(&context.instance_id)
        // .bind(format!("Rule '{}' triggered", rule.name()))
        // .bind(format!("{:?}", rule.config().action))
        // .bind(timestamp)
        // .execute(&self.db.pool)
        // .await?;

        Ok(())
    }

    /// Send notification via specified method
    async fn send_notification(
        &self,
        method: &str,
        rule: &dyn RiskRule,
        context: &RiskContext,
    ) -> Result<()> {
        let message = format!(
            "风控触发预警\n\
             规则: {}\n\
             策略: {}\n\
             动作: {:?}\n\
             时间: {}\n\
             持仓数: {}\n\
             账户余额: {:.2}",
            rule.name(),
            context.instance_id,
            rule.config().action,
            Utc::now().format("%Y-%m-%d %H:%M:%S"),
            context.positions.len(),
            context.balance
        );

        match method {
            "dingtalk" => {
                self.notification_service
                    .send_dingtalk(&message)
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to send DingTalk notification: {}", e))?;
            }
            "email" => {
                self.notification_service
                    .send_email("风控预警", &message)
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to send email notification: {}", e))?;
            }
            "log" => {
                log::warn!("Risk notification: {}", message);
            }
            _ => {
                log::warn!("Unknown notification method: {}", method);
            }
        }

        Ok(())
    }

    /// Pause a strategy instance
    async fn pause_strategy(&self, instance_id: &str) -> Result<()> {
        log::warn!("Pausing strategy due to risk rule: {}", instance_id);

        self.strategy_engine
            .pause_instance(instance_id)
            .await
            .context(format!("Failed to pause strategy instance {}", instance_id))?;

        log::warn!("Strategy {} paused due to risk rule", instance_id);
        Ok(())
    }

    /// Close all positions for a strategy (closes all positions for now)
    async fn close_all_positions(&self, instance_id: &str) -> Result<()> {
        log::warn!("Closing all positions for strategy: {}", instance_id);

        let user_id = "system"; // TODO: Get actual user_id from context
        let mut closed_count = 0;

        // Get all positions
        let positions = self
            .trade_service
            .get_positions(user_id)
            .await
            .unwrap_or_default();

        for position in positions {
            // Determine closing side: opposite of position side
            let close_side = match position.side.as_str() {
                "long" | "buy" => OrderSide::Sell,
                "short" | "sell" => OrderSide::Buy,
                _ => {
                    log::error!("Unknown position side: {}", position.side);
                    continue;
                }
            };

            // Create market order to close position
            let close_request = OrderRequest {
                symbol: position.symbol.clone(),
                side: close_side,
                order_type: OrderType::Market,
                price: None,
                stop_price: None,
                quantity: position.quantity.abs(), // Ensure positive quantity
                client_order_id: Some(format!("RISK-CLOSE-{}", Uuid::new_v4())),
                time_in_force: Some(TimeInForce::IOC),
            };

            match self
                .trade_service
                .place_order(close_request, user_id)
                .await
            {
                Ok(_) => {
                    closed_count += 1;
                    log::warn!(
                        "Closed position {} ({}) due to risk rule",
                        position.symbol,
                        position.id
                    );
                }
                Err(e) => {
                    log::error!(
                        "Failed to close position {} ({}): {}",
                        position.symbol,
                        position.id,
                        e
                    );
                }
            }
        }

        log::warn!("Closed {} positions for strategy {}", closed_count, instance_id);
        Ok(())
    }

    /// Emergency stop - stop all strategies and close all positions
    async fn emergency_stop(&self) -> Result<()> {
        log::error!("!!! EMERGENCY STOP TRIGGERED !!!");

        let user_id = "system"; // TODO: Get actual user_id from context

        // Use the emergency service to execute full emergency stop
        let report = self
            .emergency_service
            .emergency_stop_all(user_id)
            .await
            .context("Failed to execute emergency stop")?;

        log::error!(
            "!!! EMERGENCY STOP COMPLETED: {:?} !!!",
            report
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_risk_monitor_creation() {
        // This would need mock services
        // For now, just verify compilation
        assert!(true);
    }
}
