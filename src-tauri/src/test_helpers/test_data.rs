//! Test data generators for risk management tests
//!
//! This module provides helper functions to generate test positions, orders,
//! and risk contexts for testing.

use crate::core::risk::RiskContext;
use crate::core::trade::types::*;
use chrono::Utc;

/// Create a test position
pub fn create_test_position(
    symbol: &str,
    side: &str,
    quantity: f64,
    entry_price: f64,
    unrealized_pnl: f64,
) -> Position {
    Position {
        id: format!("pos_{}", uuid::Uuid::new_v4()),
        symbol: symbol.to_string(),
        side: side.to_string(),
        quantity,
        entry_price,
        current_price: Some(entry_price),
        unrealized_pnl,
        realized_pnl: 0.0,
        opened_at: Utc::now().timestamp(),
    }
}

/// Create a test order
pub fn create_test_order(
    symbol: &str,
    side: OrderSide,
    order_type: OrderType,
    quantity: f64,
    price: Option<f64>,
) -> Order {
    Order {
        id: format!("order_{}", uuid::Uuid::new_v4()),
        exchange_order_id: None,
        client_order_id: Some(format!("client_{}", uuid::Uuid::new_v4())),
        symbol: symbol.to_string(),
        side,
        order_type,
        price,
        quantity,
        filled_quantity: 0.0,
        avg_price: None,
        status: OrderState::Open,
        commission: 0.0,
        created_at: Utc::now().timestamp(),
        filled_at: None,
    }
}

/// Create a test risk context
pub fn create_test_context(
    positions: Vec<Position>,
    orders: Vec<Order>,
    balance: f64,
    today_pnl: f64,
    instance_id: &str,
) -> RiskContext {
    RiskContext {
        positions,
        orders,
        balance,
        today_pnl,
        instance_id: instance_id.to_string(),
    }
}

/// Create a default test context with minimal data
pub fn create_default_context() -> RiskContext {
    RiskContext {
        positions: vec![],
        orders: vec![],
        balance: 10000.0,
        today_pnl: 0.0,
        instance_id: "test_instance".to_string(),
    }
}

/// Create test positions for BTC and ETH
pub fn create_crypto_positions() -> Vec<Position> {
    vec![
        create_test_position("BTCUSDT", "long", 1.0, 50000.0, 1000.0),
        create_test_position("ETHUSDT", "long", 10.0, 3000.0, 500.0),
    ]
}

/// Create a large position that would trigger position limit rule
pub fn create_large_position() -> Position {
    create_test_position("BTCUSDT", "long", 100.0, 50000.0, 0.0)
}

/// Create balanced long and short positions
pub fn create_balanced_positions() -> Vec<Position> {
    vec![
        create_test_position("BTCUSDT", "long", 1.0, 50000.0, 0.0),
        create_test_position("ETHUSDT", "short", 10.0, 3000.0, 0.0),
    ]
}

/// Create imbalanced positions (80% long)
pub fn create_imbalanced_positions() -> Vec<Position> {
    vec![
        create_test_position("BTCUSDT", "long", 2.0, 50000.0, 0.0),
        create_test_position("ETHUSDT", "long", 4.0, 3000.0, 0.0),
        create_test_position("SOLUSDT", "short", 1.0, 100.0, 0.0),
    ]
}

/// Create test orders
pub fn create_test_orders() -> Vec<Order> {
    vec![
        create_test_order("BTCUSDT", OrderSide::Buy, OrderType::Limit, 1.0, Some(49000.0)),
        create_test_order("ETHUSDT", OrderSide::Sell, OrderType::Limit, 5.0, Some(3100.0)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_position() {
        let pos = create_test_position("BTCUSDT", "long", 1.0, 50000.0, 1000.0);

        assert_eq!(pos.symbol, "BTCUSDT");
        assert_eq!(pos.side, "long");
        assert_eq!(pos.quantity, 1.0);
        assert_eq!(pos.entry_price, 50000.0);
        assert_eq!(pos.unrealized_pnl, 1000.0);
    }

    #[test]
    fn test_create_test_order() {
        let order = create_test_order(
            "BTCUSDT",
            OrderSide::Buy,
            OrderType::Limit,
            1.0,
            Some(49000.0),
        );

        assert_eq!(order.symbol, "BTCUSDT");
        assert_eq!(order.side, OrderSide::Buy);
        assert_eq!(order.order_type, OrderType::Limit);
        assert_eq!(order.quantity, 1.0);
        assert_eq!(order.price, Some(49000.0));
        assert_eq!(order.status, OrderState::Open);
    }

    #[test]
    fn test_create_test_context() {
        let positions = create_crypto_positions();
        let orders = create_test_orders();
        let context = create_test_context(
            positions.clone(),
            orders.clone(),
            10000.0,
            500.0,
            "test_instance",
        );

        assert_eq!(context.positions.len(), 2);
        assert_eq!(context.orders.len(), 2);
        assert_eq!(context.balance, 10000.0);
        assert_eq!(context.today_pnl, 500.0);
        assert_eq!(context.instance_id, "test_instance");
    }

    #[test]
    fn test_create_default_context() {
        let context = create_default_context();

        assert!(context.positions.is_empty());
        assert!(context.orders.is_empty());
        assert_eq!(context.balance, 10000.0);
        assert_eq!(context.today_pnl, 0.0);
        assert_eq!(context.instance_id, "test_instance");
    }

    #[test]
    fn test_create_crypto_positions() {
        let positions = create_crypto_positions();

        assert_eq!(positions.len(), 2);
        assert_eq!(positions[0].symbol, "BTCUSDT");
        assert_eq!(positions[1].symbol, "ETHUSDT");
    }

    #[test]
    fn test_create_balanced_positions() {
        let positions = create_balanced_positions();

        assert_eq!(positions.len(), 2);
        assert_eq!(positions[0].side, "long");
        assert_eq!(positions[1].side, "short");
    }

    #[test]
    fn test_create_imbalanced_positions() {
        let positions = create_imbalanced_positions();

        assert_eq!(positions.len(), 3);
        // Should have 2 long and 1 short
        let long_count = positions.iter().filter(|p| p.side == "long").count();
        let short_count = positions.iter().filter(|p| p.side == "short").count();
        assert_eq!(long_count, 2);
        assert_eq!(short_count, 1);
    }
}
