//! Trade commands for Tauri
//!
//! This module provides Tauri command handlers for trading operations.

use crate::core::trade::types::*;
use crate::infrastructure::Database;
use tauri::State;
use serde::{Deserialize, Serialize};

/// Order request from frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,
    pub quantity: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
}

/// Convert frontend request to internal OrderRequest
impl TryFrom<PlaceOrderRequest> for OrderRequest {
    type Error = String;

    fn try_from(req: PlaceOrderRequest) -> Result<Self, Self::Error> {
        let side = req.side.parse().map_err(|e| format!("Invalid side: {}", e))?;
        let order_type = req.order_type.parse().map_err(|e| format!("Invalid order type: {}", e))?;

        Ok(OrderRequest {
            symbol: req.symbol,
            side,
            order_type,
            price: req.price,
            stop_price: req.stop_price,
            quantity: req.quantity,
            client_order_id: req.client_order_id,
            time_in_force: req.time_in_force.and_then(|t| t.parse().ok()),
        })
    }
}

/// Place a new order
#[tauri::command]
pub async fn trade_place_order(
    db: State<'_, Database>,
    user_id: String,
    request: PlaceOrderRequest,
) -> Result<Order, String> {
    log::info!("trade_place_order called: user_id={}, request={:?}", user_id, request);

    // Convert request to internal format
    let order_request: OrderRequest = request.try_into()?;

    // Get TradeService and place the order
    let trade_service = db.get_trade_service().await;
    trade_service
        .place_order(order_request, &user_id)
        .await
        .map_err(|e| e.to_string())
}

/// Cancel an existing order
#[tauri::command]
pub async fn trade_cancel_order(
    db: State<'_, Database>,
    user_id: String,
    order_id: String,
) -> Result<(), String> {
    log::info!("trade_cancel_order called: user_id={}, order_id={}", user_id, order_id);

    let trade_service = db.get_trade_service().await;
    trade_service
        .cancel_order(&order_id, &user_id)
        .await
        .map_err(|e| e.to_string())
}

/// Get order by ID
#[tauri::command]
pub async fn trade_get_order(
    db: State<'_, Database>,
    user_id: String,
    order_id: String,
) -> Result<Order, String> {
    log::info!("trade_get_order called: user_id={}, order_id={}", user_id, order_id);

    let trade_service = db.get_trade_service().await;
    trade_service
        .get_order(&order_id, &user_id)
        .await
        .map_err(|e| e.to_string())
}

/// Get orders with optional filters
#[tauri::command]
pub async fn trade_get_orders(
    db: State<'_, Database>,
    user_id: String,
    symbol: Option<String>,
    status: Option<String>,
    limit: Option<usize>,
) -> Result<Vec<Order>, String> {
    let limit = limit.unwrap_or(100);
    log::info!(
        "trade_get_orders called: user_id={}, symbol={:?}, status={:?}, limit={}",
        user_id, symbol, status, limit
    );

    let trade_service = db.get_trade_service().await;
    let order_status = status.as_ref().and_then(|s| s.parse().ok());
    trade_service
        .get_orders(&user_id, symbol.as_deref(), order_status, limit)
        .await
        .map_err(|e| e.to_string())
}

/// Get all open orders
#[tauri::command]
pub async fn trade_get_open_orders(
    db: State<'_, Database>,
    user_id: String,
) -> Result<Vec<Order>, String> {
    log::info!("trade_get_open_orders called: user_id={}", user_id);

    let trade_service = db.get_trade_service().await;
    trade_service
        .get_open_orders(&user_id)
        .await
        .map_err(|e| e.to_string())
}

/// Sync order status from exchange
#[tauri::command]
pub async fn trade_sync_order_status(
    db: State<'_, Database>,
    user_id: String,
    order_id: String,
) -> Result<Order, String> {
    log::info!(
        "trade_sync_order_status called: user_id={}, order_id={}",
        user_id, order_id
    );

    let trade_service = db.get_trade_service().await;
    trade_service
        .sync_order_status(&order_id, &user_id)
        .await
        .map_err(|e| e.to_string())
}

/// Get current positions
#[tauri::command]
pub async fn trade_get_positions(
    db: State<'_, Database>,
    user_id: String,
) -> Result<Vec<Position>, String> {
    log::info!("trade_get_positions called: user_id={}", user_id);

    let trade_service = db.get_trade_service().await;
    trade_service
        .get_positions(&user_id)
        .await
        .map_err(|e| e.to_string())
}

/// Get account balance
#[tauri::command]
pub async fn trade_get_balance(
    db: State<'_, Database>,
) -> Result<Vec<Balance>, String> {
    log::info!("trade_get_balance called");

    let trade_service = db.get_trade_service().await;
    trade_service
        .get_balance()
        .await
        .map_err(|e| e.to_string())
}

/// Cancel all open orders
#[tauri::command]
pub async fn trade_cancel_all_orders(
    db: State<'_, Database>,
    user_id: String,
    symbol: Option<String>,
) -> Result<usize, String> {
    log::info!(
        "trade_cancel_all_orders called: user_id={}, symbol={:?}",
        user_id, symbol
    );

    let trade_service = db.get_trade_service().await;

    // Get all open orders and cancel them one by one
    let open_orders = trade_service.get_open_orders(&user_id).await.map_err(|e| e.to_string())?;

    let mut canceled_count = 0;
    for order in open_orders {
        if symbol.as_ref().map_or(true, |s| order.symbol == *s) {
            if trade_service.cancel_order(&order.id, &user_id).await.is_ok() {
                canceled_count += 1;
            }
        }
    }

    Ok(canceled_count)
}

/// Close a position
#[tauri::command]
pub async fn trade_close_position(
    db: State<'_, Database>,
    user_id: String,
    symbol: String,
    side: String,
    quantity: Option<f64>,
) -> Result<f64, String> {
    log::info!(
        "trade_close_position called: user_id={}, symbol={}, side={}, quantity={:?}",
        user_id, symbol, side, quantity
    );

    let trade_service = db.get_trade_service().await;

    // Get current positions
    let positions = trade_service.get_positions(&user_id).await.map_err(|e| e.to_string())?;

    // Find the position to close
    let position = positions
        .iter()
        .find(|p| p.symbol == symbol && p.side.to_string().to_lowercase() == side.to_lowercase())
        .ok_or_else(|| format!("Position not found: {} {}", side, symbol))?;

    // Determine quantity to close
    let close_qty = quantity.unwrap_or(position.quantity);

    if close_qty > position.quantity {
        return Err(format!(
            "Cannot close more than current position: {} > {}",
            close_qty, position.quantity
        ));
    }

    // Create opposite side order to close position
    let close_side_str = match position.side.to_lowercase().as_str() {
        "buy" => "sell",
        "sell" => "buy",
        _ => return Err(format!("Invalid position side: {}", position.side)),
    };

    let order_side: crate::core::trade::types::OrderSide = close_side_str
        .parse()
        .map_err(|e| format!("Invalid side: {}", e))?;

    let order_request = crate::core::trade::types::OrderRequest {
        symbol: symbol.clone(),
        side: order_side,
        order_type: crate::core::trade::types::OrderType::Market,
        price: None,
        stop_price: None,
        quantity: close_qty,
        client_order_id: Some(format!("close_{}", uuid::Uuid::new_v4())),
        time_in_force: None,
    };

    // Place the closing order
    let _order = trade_service
        .place_order(order_request, &user_id)
        .await
        .map_err(|e| e.to_string())?;

    // Calculate realized PnL (simplified - in production would track entry price)
    // For now, return 0 as PnL calculation requires more context
    Ok(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_order_request_conversion() {
        let req = PlaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: "buy".to_string(),
            order_type: "market".to_string(),
            price: None,
            stop_price: None,
            quantity: 1.0,
            client_order_id: None,
            time_in_force: None,
        };

        let order_request: OrderRequest = req.try_into().unwrap();
        assert_eq!(order_request.symbol, "BTCUSDT");
        assert_eq!(order_request.side, OrderSide::Buy);
        assert_eq!(order_request.order_type, OrderType::Market);
    }

    #[test]
    fn test_place_order_request_with_limit() {
        let req = PlaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: "sell".to_string(),
            order_type: "limit".to_string(),
            price: Some(50000.0),
            stop_price: None,
            quantity: 0.5,
            client_order_id: Some("client123".to_string()),
            time_in_force: Some("GTC".to_string()),
        };

        let order_request: OrderRequest = req.try_into().unwrap();
        assert_eq!(order_request.side, OrderSide::Sell);
        assert_eq!(order_request.order_type, OrderType::Limit);
        assert_eq!(order_request.price, Some(50000.0));
        assert_eq!(order_request.time_in_force, Some(TimeInForce::GTC));
    }

    #[test]
    fn test_invalid_side() {
        let req = PlaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: "invalid".to_string(),
            order_type: "market".to_string(),
            price: None,
            stop_price: None,
            quantity: 1.0,
            client_order_id: None,
            time_in_force: None,
        };

        let result: Result<OrderRequest, _> = req.try_into();
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_order_type() {
        let req = PlaceOrderRequest {
            symbol: "BTCUSDT".to_string(),
            side: "buy".to_string(),
            order_type: "invalid".to_string(),
            price: None,
            stop_price: None,
            quantity: 1.0,
            client_order_id: None,
            time_in_force: None,
        };

        let result: Result<OrderRequest, _> = req.try_into();
        assert!(result.is_err());
    }
}
