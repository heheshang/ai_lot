//! Trade commands for Tauri
//!
//! This module provides Tauri command handlers for trading operations.

use crate::core::response::{ApiResponse, ApiError};
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
) -> Result<ApiResponse<Order>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] trade_place_order called: user_id={}", request_id, user_id);

    // Convert request to internal format
    let order_request: OrderRequest = request.try_into()
        .map_err(|e| format!("Invalid request: {}", e))?;

    // Get TradeService and place the order
    let trade_service = db.get_trade_service().await;
    match trade_service.place_order(order_request, &user_id).await {
        Ok(order) => {
            log::info!("[{}] Order placed successfully: {}", request_id, order.id);
            Ok(ApiResponse::success(order).with_request_id(request_id))
        }
        Err(e) => {
            log::error!("[{}] Failed to place order: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed(format!("下单失败: {}", e))).with_request_id(request_id))
        }
    }
}

/// Cancel an existing order
#[tauri::command]
pub async fn trade_cancel_order(
    db: State<'_, Database>,
    user_id: String,
    order_id: String,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] trade_cancel_order called: user_id={}, order_id={}", request_id, user_id, order_id);

    let trade_service = db.get_trade_service().await;
    match trade_service.cancel_order(&order_id, &user_id).await {
        Ok(()) => {
            log::info!("[{}] Order canceled successfully: {}", request_id, order_id);
            Ok(ApiResponse::success_empty().with_request_id(request_id))
        }
        Err(e) => {
            log::error!("[{}] Failed to cancel order: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed(format!("撤单失败: {}", e))).with_request_id(request_id))
        }
    }
}

/// Get order by ID
#[tauri::command]
pub async fn trade_get_order(
    db: State<'_, Database>,
    user_id: String,
    order_id: String,
) -> Result<ApiResponse<Order>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] trade_get_order called: user_id={}, order_id={}", request_id, user_id, order_id);

    let trade_service = db.get_trade_service().await;
    match trade_service.get_order(&order_id, &user_id).await {
        Ok(order) => Ok(ApiResponse::success(order).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to get order: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::not_found("订单")).with_request_id(request_id))
        }
    }
}

/// Get orders with optional filters
#[tauri::command]
pub async fn trade_get_orders(
    db: State<'_, Database>,
    user_id: String,
    symbol: Option<String>,
    status: Option<String>,
    limit: Option<usize>,
) -> Result<ApiResponse<Vec<Order>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let limit = limit.unwrap_or(100);
    log::info!(
        "[{}] trade_get_orders called: user_id={}, symbol={:?}, status={:?}, limit={}",
        request_id, user_id, symbol, status, limit
    );

    let trade_service = db.get_trade_service().await;
    let order_status = status.as_ref().and_then(|s| s.parse().ok());
    match trade_service.get_orders(&user_id, symbol.as_deref(), order_status, limit).await {
        Ok(orders) => Ok(ApiResponse::success(orders).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to get orders: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("查询订单失败")).with_request_id(request_id))
        }
    }
}

/// Get all open orders
#[tauri::command]
pub async fn trade_get_open_orders(
    db: State<'_, Database>,
    user_id: String,
) -> Result<ApiResponse<Vec<Order>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] trade_get_open_orders called: user_id={}", request_id, user_id);

    let trade_service = db.get_trade_service().await;
    match trade_service.get_open_orders(&user_id).await {
        Ok(orders) => Ok(ApiResponse::success(orders).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to get open orders: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("查询挂单失败")).with_request_id(request_id))
        }
    }
}

/// Sync order status from exchange
#[tauri::command]
pub async fn trade_sync_order_status(
    db: State<'_, Database>,
    user_id: String,
    order_id: String,
) -> Result<ApiResponse<Order>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!(
        "[{}] trade_sync_order_status called: user_id={}, order_id={}",
        request_id, user_id, order_id
    );

    let trade_service = db.get_trade_service().await;
    match trade_service.sync_order_status(&order_id, &user_id).await {
        Ok(order) => Ok(ApiResponse::success(order).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to sync order: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("同步订单失败")).with_request_id(request_id))
        }
    }
}

/// Get current positions
#[tauri::command]
pub async fn trade_get_positions(
    db: State<'_, Database>,
    user_id: String,
) -> Result<ApiResponse<Vec<Position>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] trade_get_positions called: user_id={}", request_id, user_id);

    let trade_service = db.get_trade_service().await;
    match trade_service.get_positions(&user_id).await {
        Ok(positions) => Ok(ApiResponse::success(positions).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to get positions: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("查询持仓失败")).with_request_id(request_id))
        }
    }
}

/// Get account balance
#[tauri::command]
pub async fn trade_get_balance(
    db: State<'_, Database>,
) -> Result<ApiResponse<Vec<Balance>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] trade_get_balance called", request_id);

    let trade_service = db.get_trade_service().await;
    match trade_service.get_balance().await {
        Ok(balance) => Ok(ApiResponse::success(balance).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to get balance: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("查询余额失败")).with_request_id(request_id))
        }
    }
}

/// Cancel all open orders
#[tauri::command]
pub async fn trade_cancel_all_orders(
    db: State<'_, Database>,
    user_id: String,
    symbol: Option<String>,
) -> Result<ApiResponse<usize>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!(
        "[{}] trade_cancel_all_orders called: user_id={}, symbol={:?}",
        request_id, user_id, symbol
    );

    let trade_service = db.get_trade_service().await;

    // Get all open orders and cancel them one by one
    let open_orders = match trade_service.get_open_orders(&user_id).await {
        Ok(orders) => orders,
        Err(e) => {
            log::error!("[{}] Failed to get open orders: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("查询挂单失败")).with_request_id(request_id));
        }
    };

    let mut canceled_count = 0;
    for order in open_orders {
        if symbol.as_ref().is_none_or(|s| order.symbol == *s)
            && trade_service.cancel_order(&order.id, &user_id).await.is_ok()
        {
            canceled_count += 1;
        }
    }

    log::info!("[{}] Canceled {} orders", request_id, canceled_count);
    Ok(ApiResponse::success(canceled_count).with_request_id(request_id))
}

/// Close a position
#[tauri::command]
pub async fn trade_close_position(
    db: State<'_, Database>,
    user_id: String,
    symbol: String,
    side: String,
    quantity: Option<f64>,
) -> Result<ApiResponse<f64>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!(
        "[{}] trade_close_position called: user_id={}, symbol={}, side={}, quantity={:?}",
        request_id, user_id, symbol, side, quantity
    );

    let trade_service = db.get_trade_service().await;

    // Get current positions
    let positions = match trade_service.get_positions(&user_id).await {
        Ok(pos) => pos,
        Err(e) => {
            log::error!("[{}] Failed to get positions: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("查询持仓失败")).with_request_id(request_id));
        }
    };

    // Find the position to close
    let position = match positions
        .iter()
        .find(|p| p.symbol == symbol && p.side.to_string().to_lowercase() == side.to_lowercase())
    {
        Some(p) => p,
        None => {
            return Ok(ApiResponse::error(ApiError::not_found("持仓")).with_request_id(request_id));
        }
    };

    // Determine quantity to close
    let close_qty = quantity.unwrap_or(position.quantity);

    if close_qty > position.quantity {
        return Ok(ApiResponse::error(ApiError::invalid_parameter("quantity")).with_request_id(request_id));
    }

    // Create opposite side order to close position
    let close_side_str = match position.side.to_lowercase().as_str() {
        "buy" => "sell",
        "sell" => "buy",
        _ => return Ok(ApiResponse::error(ApiError::invalid_parameter("side")).with_request_id(request_id)),
    };

    let order_side: crate::core::trade::types::OrderSide = match close_side_str.parse() {
        Ok(s) => s,
        Err(e) => return Ok(ApiResponse::error(ApiError::invalid_parameter(format!("Invalid side: {}", e))).with_request_id(request_id)),
    };

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
    match trade_service.place_order(order_request, &user_id).await {
        Ok(_) => {
            log::info!("[{}] Position closed successfully", request_id);
            Ok(ApiResponse::success(0.0).with_request_id(request_id))
        }
        Err(e) => {
            log::error!("[{}] Failed to close position: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("平仓失败")).with_request_id(request_id))
        }
    }
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
