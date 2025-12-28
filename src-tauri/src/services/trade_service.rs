//! Trade service module
//!
//! This module provides trading functionality including order management,
//! position tracking, and account operations.

use crate::core::trade::exchange::Exchange;
use crate::core::trade::types::*;
use crate::core::trade::position::PositionManager;
use crate::core::trade::order::OrderStateMachine;
use crate::core::{AppError, AppResult};
use crate::infrastructure::Database;
use chrono::Utc;
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Trade service for managing orders and positions
pub struct TradeService {
    exchange: Arc<dyn Exchange>,
    db: Database,
    position_manager: Arc<RwLock<PositionManager>>,
}

impl TradeService {
    /// Create a new trade service
    pub fn new(exchange: Arc<dyn Exchange>, db: Database) -> Self {
        Self {
            exchange,
            db,
            position_manager: Arc::new(RwLock::new(PositionManager::new())),
        }
    }

    /// Place a new order
    pub async fn place_order(&self, request: OrderRequest, user_id: &str) -> AppResult<Order> {
        // Validate order request
        self.validate_order_request(&request)?;

        // Create order ID
        let order_id = Uuid::new_v4().to_string();

        // Call exchange to place the order
        let mut order = self.exchange.place_order(&request).await?;

        // Update order with our ID and user info
        order.id = order_id.clone();
        order.client_order_id = request.client_order_id;

        // Save to database
        self.save_order_to_db(&order, user_id).await?;

        // Publish order event
        self.publish_order_event(&order).await;

        Ok(order)
    }

    /// Cancel an existing order
    pub async fn cancel_order(&self, order_id: &str, user_id: &str) -> AppResult<()> {
        // Get order from database
        let order = self.get_order_from_db(order_id, user_id).await?;

        // Check if order can be canceled
        let sm = OrderStateMachine::from_state(order.status);
        if !sm.can_transition_to(&OrderState::Canceled) {
            return Err(AppError::validation(format!(
                "Order cannot be canceled: current state is {:?}",
                order.status
            )));
        }

        // Call exchange to cancel the order
        if let Some(exchange_order_id) = &order.exchange_order_id {
            self.exchange.cancel_order(exchange_order_id).await?;
        }

        // Update order status in database
        self.update_order_status(order_id, OrderState::Canceled).await?;

        Ok(())
    }

    /// Get order by ID
    pub async fn get_order(&self, order_id: &str, user_id: &str) -> AppResult<Order> {
        self.get_order_from_db(order_id, user_id).await
    }

    /// Get all orders for a user
    pub async fn get_orders(
        &self,
        user_id: &str,
        symbol: Option<&str>,
        status: Option<OrderState>,
        limit: usize,
    ) -> AppResult<Vec<Order>> {
        let mut query = String::from(
            "SELECT * FROM orders WHERE user_id = ?"
        );

        let _params = [user_id.to_string()];

        if let Some(sym) = symbol {
            query.push_str(&format!(" AND symbol = '{}'", sym));
        }

        if let Some(st) = status {
            query.push_str(&format!(" AND status = '{}'", st));
        }

        query.push_str(&format!(" ORDER BY created_at DESC LIMIT {}", limit));

        let rows = sqlx::query(&query)
            .fetch_all(&self.db.pool)
            .await?;

        let mut orders = Vec::new();
        for row in rows {
            orders.push(self.row_to_order(row)?);
        }

        Ok(orders)
    }

    /// Get all open orders
    pub async fn get_open_orders(&self, user_id: &str) -> AppResult<Vec<Order>> {
        self.get_orders(user_id, None, Some(OrderState::Open), 1000).await
    }

    /// Sync order status from exchange
    pub async fn sync_order_status(&self, order_id: &str, user_id: &str) -> AppResult<Order> {
        let mut order = self.get_order_from_db(order_id, user_id).await?;

        if let Some(exchange_order_id) = &order.exchange_order_id {
            let updated_order = self.exchange.get_order(exchange_order_id).await?;

            // Update status if changed
            if updated_order.status != order.status {
                order.status = updated_order.status;
                order.filled_quantity = updated_order.filled_quantity;
                order.avg_price = updated_order.avg_price;
                order.filled_at = updated_order.filled_at;

                self.update_order_in_db(&order).await?;

                // Update positions if filled
                if matches!(order.status, OrderState::Filled | OrderState::PartiallyFilled) {
                    self.update_positions_from_order(&order).await;
                }
            }
        }

        Ok(order)
    }

    /// Get all positions
    pub async fn get_positions(&self, user_id: &str) -> AppResult<Vec<Position>> {
        let rows = sqlx::query(
            "SELECT * FROM positions WHERE user_id = ? AND quantity > 0 ORDER BY opened_at DESC"
        )
        .bind(user_id)
        .fetch_all(&self.db.pool)
        .await?;

        let mut positions = Vec::new();
        for row in rows {
            positions.push(self.row_to_position(row)?);
        }

        Ok(positions)
    }

    /// Get account balance
    pub async fn get_balance(&self) -> AppResult<Vec<Balance>> {
        self.exchange.get_balance().await
            .map_err(|e| AppError::Exchange(e.to_string()))
    }

    // ========== Private helper methods ==========

    fn validate_order_request(&self, request: &OrderRequest) -> AppResult<()> {
        if request.quantity <= 0.0 {
            return Err(AppError::Validation("Order quantity must be positive".to_string()));
        }

        if matches!(request.order_type, OrderType::Limit | OrderType::StopLimit)
            && (request.price.is_none() || request.price.unwrap() <= 0.0) {
                return Err(AppError::Validation("Limit orders must have a positive price".to_string()));
            }

        if matches!(request.order_type, OrderType::StopLoss | OrderType::StopLimit)
            && (request.stop_price.is_none() || request.stop_price.unwrap() <= 0.0) {
                return Err(AppError::Validation("Stop orders must have a positive stop price".to_string()));
            }

        Ok(())
    }

    async fn save_order_to_db(&self, order: &Order, user_id: &str) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO orders (id, user_id, exchange_order_id, client_order_id,
                               symbol, side, order_type, price, quantity,
                               filled_quantity, avg_price, status, commission,
                               created_at, filled_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&order.id)
        .bind(user_id)
        .bind(&order.exchange_order_id)
        .bind(&order.client_order_id)
        .bind(&order.symbol)
        .bind(order.side.to_string())
        .bind(order.order_type.to_string())
        .bind(order.price)
        .bind(order.quantity)
        .bind(order.filled_quantity)
        .bind(order.avg_price)
        .bind(order.status.to_string())
        .bind(order.commission)
        .bind(order.created_at)
        .bind(order.filled_at)
        .execute(&self.db.pool)
        .await?;

        Ok(())
    }

    async fn get_order_from_db(&self, order_id: &str, user_id: &str) -> AppResult<Order> {
        let row = sqlx::query(
            "SELECT * FROM orders WHERE id = ? AND user_id = ?"
        )
        .bind(order_id)
        .bind(user_id)
        .fetch_optional(&self.db.pool)
        .await?;

        match row {
            Some(r) => self.row_to_order(r),
            None => Err(AppError::Validation(format!("Order not found: {}", order_id))),
        }
    }

    async fn update_order_status(&self, order_id: &str, status: OrderState) -> AppResult<()> {
        sqlx::query(
            "UPDATE orders SET status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(status.to_string())
        .bind(Utc::now().timestamp())
        .bind(order_id)
        .execute(&self.db.pool)
        .await?;

        Ok(())
    }

    async fn update_order_in_db(&self, order: &Order) -> AppResult<()> {
        sqlx::query(
            r#"
            UPDATE orders SET
                filled_quantity = ?,
                avg_price = ?,
                status = ?,
                filled_at = ?,
                updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(order.filled_quantity)
        .bind(order.avg_price)
        .bind(order.status.to_string())
        .bind(order.filled_at)
        .bind(Utc::now().timestamp())
        .bind(&order.id)
        .execute(&self.db.pool)
        .await?;

        Ok(())
    }

    async fn update_positions_from_order(&self, order: &Order) {
        let mut manager = self.position_manager.write().await;

        use crate::core::trade::position::Trade;

        let trade = Trade {
            symbol: order.symbol.clone(),
            side: if order.side == OrderSide::Buy { "buy".to_string() } else { "sell".to_string() },
            quantity: order.filled_quantity,
            price: order.avg_price.unwrap_or(order.price.unwrap_or(0.0)),
            timestamp: order.created_at,
        };

        manager.update_position(&trade);
    }

    async fn publish_order_event(&self, order: &Order) {
        // TODO: Publish to event bus
        log::info!("Order placed: {:?}", order);
    }

    fn row_to_order(&self, row: sqlx::sqlite::SqliteRow) -> AppResult<Order> {
        Ok(Order {
            id: row.try_get("id")?,
            exchange_order_id: row.try_get("exchange_order_id")?,
            client_order_id: row.try_get("client_order_id")?,
            symbol: row.try_get("symbol")?,
            side: row.try_get::<String, _>("side")?.parse()?,
            order_type: row.try_get::<String, _>("order_type")?.parse()?,
            price: row.try_get("price")?,
            quantity: row.try_get("quantity")?,
            filled_quantity: row.try_get("filled_quantity")?,
            avg_price: row.try_get("avg_price")?,
            status: row.try_get::<String, _>("status")?.parse()?,
            commission: row.try_get("commission")?,
            created_at: row.try_get("created_at")?,
            filled_at: row.try_get("filled_at")?,
        })
    }

    fn row_to_position(&self, row: sqlx::sqlite::SqliteRow) -> AppResult<Position> {
        Ok(Position {
            id: row.try_get("id")?,
            symbol: row.try_get("symbol")?,
            side: row.try_get("side")?,
            quantity: row.try_get("quantity")?,
            entry_price: row.try_get("entry_price")?,
            current_price: row.try_get("current_price")?,
            unrealized_pnl: row.try_get("unrealized_pnl")?,
            realized_pnl: row.try_get("realized_pnl")?,
            opened_at: row.try_get("opened_at")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_order_request() {
        let service = create_test_service();

        // Valid market order
        let request = OrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            price: None,
            stop_price: None,
            quantity: 1.0,
            client_order_id: None,
            time_in_force: None,
        };

        assert!(service.validate_order_request(&request).is_ok());

        // Invalid quantity
        let invalid_request = OrderRequest {
            quantity: -1.0,
            ..request.clone()
        };
        assert!(service.validate_order_request(&invalid_request).is_err());

        // Limit order without price
        let invalid_limit = OrderRequest {
            order_type: OrderType::Limit,
            price: None,
            ..request
        };
        assert!(service.validate_order_request(&invalid_limit).is_err());
    }

    fn create_test_service() -> TradeService {
        // This would need a mock exchange and test database
        // For now, just a placeholder
        todo!("Implement test service setup")
    }
}
