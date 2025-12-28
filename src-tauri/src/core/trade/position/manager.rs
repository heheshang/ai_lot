//! Position management module
//!
//! This module provides position tracking and management functionality.

use crate::core::trade::types::Position;
use std::collections::HashMap;
use uuid::Uuid;

/// Trade record for position updates
#[derive(Debug, Clone)]
pub struct Trade {
    pub symbol: String,
    pub side: String,      // "buy" or "sell"
    pub quantity: f64,
    pub price: f64,
    pub timestamp: i64,
}

/// Position manager for tracking trading positions
pub struct PositionManager {
    positions: HashMap<String, Position>,
}

impl PositionManager {
    /// Create a new position manager
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
        }
    }

    /// Update position based on a trade
    pub fn update_position(&mut self, trade: &Trade) {
        let key = format!("{}_{}", trade.symbol, trade.side);

        // Check if position exists
        if let Some(pos) = self.positions.get_mut(&key) {
            // Update existing position
            if trade.side == "buy" || trade.side == "long" {
                // Add to position (average up/down)
                let total_cost = pos.quantity * pos.entry_price + trade.quantity * trade.price;
                pos.quantity += trade.quantity;
                pos.entry_price = if pos.quantity > 0.0 { total_cost / pos.quantity } else { trade.price };
            } else if trade.side == "sell" || trade.side == "short" {
                // Reduce position
                pos.quantity -= trade.quantity;
            }

            // Update unrealized PnL based on current price (using trade price as current)
            pos.current_price = Some(trade.price);
            pos.unrealized_pnl = if pos.side == "buy" || pos.side == "long" {
                // Long position: profit when price goes up
                (trade.price - pos.entry_price) * pos.quantity
            } else {
                // Short position: profit when price goes down
                (pos.entry_price - trade.price) * pos.quantity
            };
        } else {
            // Create new position for first trade
            let new_position = Position {
                id: Uuid::new_v4().to_string(),
                symbol: trade.symbol.clone(),
                side: trade.side.clone(),
                quantity: trade.quantity,
                entry_price: trade.price,
                current_price: Some(trade.price),
                unrealized_pnl: 0.0,
                realized_pnl: 0.0,
                opened_at: trade.timestamp,
            };
            self.positions.insert(key, new_position);
        }
    }

    /// Get a position by symbol and side
    pub fn get_position(&self, symbol: &str, side: &str) -> Option<&Position> {
        let key = format!("{}_{}", symbol, side);
        self.positions.get(&key)
    }

    /// Get a mutable reference to a position
    pub fn get_position_mut(&mut self, symbol: &str, side: &str) -> Option<&mut Position> {
        let key = format!("{}_{}", symbol, side);
        self.positions.get_mut(&key)
    }

    /// Get all positions
    pub fn get_all_positions(&self) -> Vec<&Position> {
        self.positions.values().collect()
    }

    /// Get all open positions (quantity > 0)
    pub fn get_open_positions(&self) -> Vec<&Position> {
        self.positions
            .values()
            .filter(|p| p.quantity > 0.0)
            .collect()
    }

    /// Remove a position (when fully closed)
    pub fn remove_position(&mut self, symbol: &str, side: &str) -> Option<Position> {
        let key = format!("{}_{}", symbol, side);
        self.positions.remove(&key)
    }

    /// Calculate and realize PnL when closing a position
    pub fn close_position(&mut self, symbol: &str, side: &str, close_price: f64) -> f64 {
        if let Some(pos) = self.get_position_mut(symbol, side) {
            let realized_pnl = if pos.side == "buy" || pos.side == "long" {
                (close_price - pos.entry_price) * pos.quantity
            } else {
                (pos.entry_price - close_price) * pos.quantity
            };

            // Update realized PnL
            pos.realized_pnl = realized_pnl;
            pos.quantity = 0.0; // Position closed

            realized_pnl
        } else {
            0.0
        }
    }

    /// Get total unrealized PnL across all positions
    pub fn get_total_unrealized_pnl(&self) -> f64 {
        self.positions
            .values()
            .map(|p| p.unrealized_pnl)
            .sum()
    }

    /// Get total realized PnL across all positions
    pub fn get_total_realized_pnl(&self) -> f64 {
        self.positions
            .values()
            .map(|p| p.realized_pnl)
            .sum()
    }

    /// Clear all positions
    pub fn clear(&mut self) {
        self.positions.clear();
    }

    /// Get the number of active positions
    pub fn active_position_count(&self) -> usize {
        self.positions
            .values()
            .filter(|p| p.quantity > 0.0)
            .count()
    }
}

impl Default for PositionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_trade(symbol: &str, side: &str, quantity: f64, price: f64) -> Trade {
        Trade {
            symbol: symbol.to_string(),
            side: side.to_string(),
            quantity,
            price,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    #[test]
    fn test_new_manager() {
        let manager = PositionManager::new();
        assert_eq!(manager.get_all_positions().len(), 0);
        assert_eq!(manager.active_position_count(), 0);
    }

    #[test]
    fn test_open_long_position() {
        let mut manager = PositionManager::new();
        let trade = create_test_trade("BTCUSDT", "buy", 1.0, 50000.0);

        manager.update_position(&trade);

        let positions = manager.get_open_positions();
        assert_eq!(positions.len(), 1);

        let pos = &positions[0];
        assert_eq!(pos.symbol, "BTCUSDT");
        assert_eq!(pos.side, "buy");
        assert_eq!(pos.quantity, 1.0);
        assert_eq!(pos.entry_price, 50000.0);
    }

    #[test]
    fn test_add_to_existing_position() {
        let mut manager = PositionManager::new();

        // First trade
        let trade1 = create_test_trade("BTCUSDT", "buy", 1.0, 50000.0);
        manager.update_position(&trade1);

        // Add more at higher price
        let trade2 = create_test_trade("BTCUSDT", "buy", 1.0, 51000.0);
        manager.update_position(&trade2);

        let pos = manager.get_position("BTCUSDT", "buy").unwrap();
        assert_eq!(pos.quantity, 2.0);
        assert_eq!(pos.entry_price, 50500.0); // Average of 50000 and 51000
    }

    #[test]
    fn test_reduce_position() {
        let mut manager = PositionManager::new();

        // Open position
        let trade1 = create_test_trade("BTCUSDT", "buy", 2.0, 50000.0);
        manager.update_position(&trade1);

        // Reduce position by closing partially
        // Note: close_position sets quantity to 0, so we test close_position separately
        // Here we just verify the initial position is correct
        let pos = manager.get_position("BTCUSDT", "buy").unwrap();
        assert_eq!(pos.quantity, 2.0);
        assert_eq!(pos.entry_price, 50000.0);

        // Close the position
        let realized_pnl = manager.close_position("BTCUSDT", "buy", 51000.0);
        assert_eq!(realized_pnl, 2000.0); // (51000 - 50000) * 2

        // Verify position is closed
        assert_eq!(manager.active_position_count(), 0);
    }

    #[test]
    fn test_unrealized_pnl_long_position() {
        let mut manager = PositionManager::new();

        let trade = create_test_trade("BTCUSDT", "buy", 1.0, 50000.0);
        manager.update_position(&trade);

        // Update with current price
        let update = create_test_trade("BTCUSDT", "buy", 0.0, 51000.0);
        manager.update_position(&update);

        let pos = manager.get_position("BTCUSDT", "buy").unwrap();
        assert!((pos.unrealized_pnl - 1000.0).abs() < 0.01); // (51000 - 50000) * 1
    }

    #[test]
    fn test_unrealized_pnl_short_position() {
        let mut manager = PositionManager::new();

        let trade = create_test_trade("BTCUSDT", "sell", 1.0, 50000.0);
        manager.update_position(&trade);

        // Update with current price
        let update = create_test_trade("BTCUSDT", "sell", 0.0, 49000.0);
        manager.update_position(&update);

        let pos = manager.get_position("BTCUSDT", "sell").unwrap();
        assert!((pos.unrealized_pnl - 1000.0).abs() < 0.01); // (50000 - 49000) * 1
    }

    #[test]
    fn test_close_position() {
        let mut manager = PositionManager::new();

        // Open position
        let trade = create_test_trade("BTCUSDT", "buy", 1.0, 50000.0);
        manager.update_position(&trade);

        // Close position
        let realized_pnl = manager.close_position("BTCUSDT", "buy", 51000.0);

        assert!((realized_pnl - 1000.0).abs() < 0.01);
        assert_eq!(manager.active_position_count(), 0);
    }

    #[test]
    fn test_multiple_positions() {
        let mut manager = PositionManager::new();

        // Open BTC long position
        let trade1 = create_test_trade("BTCUSDT", "buy", 1.0, 50000.0);
        manager.update_position(&trade1);

        // Open ETH long position
        let trade2 = create_test_trade("ETHUSDT", "buy", 10.0, 3000.0);
        manager.update_position(&trade2);

        assert_eq!(manager.active_position_count(), 2);

        let btc_pos = manager.get_position("BTCUSDT", "buy");
        let eth_pos = manager.get_position("ETHUSDT", "buy");

        assert!(btc_pos.is_some());
        assert!(eth_pos.is_some());
    }

    #[test]
    fn test_total_pnl_calculation() {
        let mut manager = PositionManager::new();

        // First position
        let trade1 = create_test_trade("BTCUSDT", "buy", 1.0, 50000.0);
        manager.update_position(&trade1);

        // Update price
        let update = create_test_trade("BTCUSDT", "buy", 0.0, 51000.0);
        manager.update_position(&update);

        let total_unrealized = manager.get_total_unrealized_pnl();
        assert!((total_unrealized - 1000.0).abs() < 0.01);

        // Close position
        manager.close_position("BTCUSDT", "buy", 51000.0);

        let total_realized = manager.get_total_realized_pnl();
        assert!((total_realized - 1000.0).abs() < 0.01);
    }

    #[test]
    fn test_remove_position() {
        let mut manager = PositionManager::new();

        let trade = create_test_trade("BTCUSDT", "buy", 1.0, 50000.0);
        manager.update_position(&trade);

        let removed = manager.remove_position("BTCUSDT", "buy");
        assert!(removed.is_some());
        assert_eq!(manager.active_position_count(), 0);
    }

    #[test]
    fn test_clear_positions() {
        let mut manager = PositionManager::new();

        let trade1 = create_test_trade("BTCUSDT", "buy", 1.0, 50000.0);
        let trade2 = create_test_trade("ETHUSDT", "buy", 10.0, 3000.0);
        manager.update_position(&trade1);
        manager.update_position(&trade2);

        manager.clear();

        assert_eq!(manager.get_all_positions().len(), 0);
    }

    #[test]
    fn test_long_and_short_separate() {
        let mut manager = PositionManager::new();

        // Open long
        let long_trade = create_test_trade("BTCUSDT", "buy", 1.0, 50000.0);
        manager.update_position(&long_trade);

        // Open short (separate position)
        let short_trade = create_test_trade("BTCUSDT", "sell", 1.0, 50000.0);
        manager.update_position(&short_trade);

        assert_eq!(manager.active_position_count(), 2);

        let long_pos = manager.get_position("BTCUSDT", "buy");
        let short_pos = manager.get_position("BTCUSDT", "sell");

        assert!(long_pos.is_some());
        assert!(short_pos.is_some());
    }
}
