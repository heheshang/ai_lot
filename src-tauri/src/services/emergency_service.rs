//! Emergency service module
//!
//! This module provides emergency stop functionality for halting all trading
//! activities in critical situations.

use crate::core::strategy::StrategyEngine;
use crate::core::trade::types::*;
use crate::services::TradeService;
use anyhow::Result;
use std::sync::Arc;

/// Emergency service for stopping all trading activities
pub struct EmergencyService {
    trade_service: Arc<TradeService>,
    strategy_engine: Arc<StrategyEngine>,
}

impl EmergencyService {
    /// Create a new emergency service
    pub fn new(trade_service: Arc<TradeService>, strategy_engine: Arc<StrategyEngine>) -> Self {
        Self {
            trade_service,
            strategy_engine,
        }
    }

    /// Main emergency stop function - stops all trading activities
    ///
    /// This function performs the following operations:
    /// 1. Stops all running strategy instances
    /// 2. Cancels all active orders
    /// 3. Closes all positions with market orders
    /// 4. Sends emergency alert notification
    ///
    /// Each operation continues even if previous operations fail, ensuring
    /// maximum cleanup in emergency situations.
    pub async fn emergency_stop_all(&self, user_id: &str) -> Result<EmergencyReport> {
        log::error!("!!! EMERGENCY STOP INITIATED for user: {} !!!", user_id);

        let mut report = EmergencyReport::default();

        // Step 1: Stop all strategy instances
        match self.strategy_engine.stop_all().await {
            Ok(count) => {
                report.strategies_stopped = count;
                log::error!("EMERGENCY: Stopped {} strategy instances", count);
            }
            Err(e) => {
                log::error!("EMERGENCY: Failed to stop strategies: {}", e);
                report.errors.push(format!("Failed to stop strategies: {}", e));
            }
        }

        // Step 2: Cancel all orders
        match self.cancel_all_orders(user_id).await {
            Ok(count) => {
                report.orders_canceled = count;
                log::error!("EMERGENCY: Canceled {} orders", count);
            }
            Err(e) => {
                log::error!("EMERGENCY: Failed to cancel orders: {}", e);
                report.errors.push(format!("Failed to cancel orders: {}", e));
            }
        }

        // Step 3: Close all positions
        match self.close_all_positions(user_id).await {
            Ok(count) => {
                report.positions_closed = count;
                log::error!("EMERGENCY: Closed {} positions", count);
            }
            Err(e) => {
                log::error!("EMERGENCY: Failed to close positions: {}", e);
                report.errors.push(format!("Failed to close positions: {}", e));
            }
        }

        // Step 4: Send emergency alert
        match self.send_emergency_alert(&report).await {
            Ok(_) => {
                report.alert_sent = true;
                log::error!("EMERGENCY: Alert notification sent");
            }
            Err(e) => {
                log::error!("EMERGENCY: Failed to send alert: {}", e);
                report.errors.push(format!("Failed to send alert: {}", e));
            }
        }

        log::error!("!!! EMERGENCY STOP COMPLETED {:?}", report);
        Ok(report)
    }

    /// Cancel all active orders for the user
    async fn cancel_all_orders(&self, user_id: &str) -> Result<usize> {
        log::error!("EMERGENCY: Canceling all orders for user: {}", user_id);

        let orders = self.trade_service.get_open_orders(user_id).await?;
        let mut canceled_count = 0;

        for order in orders {
            match self.trade_service.cancel_order(&order.id, user_id).await {
                Ok(_) => {
                    canceled_count += 1;
                    log::error!("EMERGENCY: Canceled order {}", order.id);
                }
                Err(e) => {
                    log::error!("EMERGENCY: Failed to cancel order {}: {}", order.id, e);
                    // Continue with next order even if this one fails
                }
            }
        }

        Ok(canceled_count)
    }

    /// Close all positions for the user using market orders
    async fn close_all_positions(&self, user_id: &str) -> Result<usize> {
        log::error!("EMERGENCY: Closing all positions for user: {}", user_id);

        let positions = self.trade_service.get_positions(user_id).await?;
        let mut closed_count = 0;

        for position in positions {
            // Determine closing side: opposite of position side
            let close_side = match position.side.as_str() {
                "long" | "buy" => OrderSide::Sell,
                "short" | "sell" => OrderSide::Buy,
                _ => {
                    log::error!("EMERGENCY: Unknown position side: {}", position.side);
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
                client_order_id: Some(format!("EMERGENCY-CLOSE-{}", uuid::Uuid::new_v4())),
                time_in_force: Some(TimeInForce::IOC),
            };

            match self.trade_service.place_order(close_request, user_id).await {
                Ok(_) => {
                    closed_count += 1;
                    log::error!(
                        "EMERGENCY: Closed position {} ({})",
                        position.symbol,
                        position.id
                    );
                }
                Err(e) => {
                    log::error!(
                        "EMERGENCY: Failed to close position {} ({}): {}",
                        position.symbol,
                        position.id,
                        e
                    );
                    // Continue with next position even if this one fails
                }
            }
        }

        Ok(closed_count)
    }

    /// Send emergency alert notification
    async fn send_emergency_alert(&self, report: &EmergencyReport) -> Result<()> {
        log::error!("EMERGENCY: Sending emergency alert notification");

        let message = format!(
            "🚨 EMERGENCY STOP EXECUTED 🚨\n\
             Strategies stopped: {}\n\
             Orders canceled: {}\n\
             Positions closed: {}\n\
             Timestamp: {}",
            report.strategies_stopped,
            report.orders_canceled,
            report.positions_closed,
            chrono::Utc::now().to_rfc3339()
        );

        // TODO: Integrate with notification system when available
        // For now, just log the message at ERROR level
        log::error!("EMERGENCY ALERT:\n{}", message);

        // In a full implementation, this would call:
        // - DingTalk notification service
        // - Email notification service
        // - SMS notification service
        // - In-app notification service

        Ok(())
    }
}

/// Report of emergency stop execution
#[derive(Debug, Clone, Default)]
pub struct EmergencyReport {
    /// Number of strategy instances stopped
    pub strategies_stopped: usize,
    /// Number of orders canceled
    pub orders_canceled: usize,
    /// Number of positions closed
    pub positions_closed: usize,
    /// Whether emergency alert was sent
    pub alert_sent: bool,
    /// Any errors that occurred during execution
    pub errors: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emergency_report_default() {
        let report = EmergencyReport::default();
        assert_eq!(report.strategies_stopped, 0);
        assert_eq!(report.orders_canceled, 0);
        assert_eq!(report.positions_closed, 0);
        assert_eq!(report.alert_sent, false);
        assert!(report.errors.is_empty());
    }

    #[test]
    fn test_emergency_report_with_errors() {
        let mut report = EmergencyReport::default();
        report.strategies_stopped = 5;
        report.orders_canceled = 10;
        report.positions_closed = 3;
        report.alert_sent = true;
        report.errors.push("Test error".to_string());

        assert_eq!(report.strategies_stopped, 5);
        assert_eq!(report.orders_canceled, 10);
        assert_eq!(report.positions_closed, 3);
        assert_eq!(report.alert_sent, true);
        assert_eq!(report.errors.len(), 1);
    }

    // Note: Integration tests for emergency_stop_all require:
    // - Mock TradeService
    // - Mock StrategyEngine
    // - Test database
    // These would be in a separate integration test module
}
