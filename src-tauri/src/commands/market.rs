//! 市场数据命令
//!
//! 提供市场数据订阅、K线获取、交易对查询等功能

use crate::core::response::{ApiResponse, ApiError};
use crate::core::validation::{validate_symbol, validate_interval, validate_limit};
use crate::core::trade::types::*;
use crate::services::MarketService;
use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

/// 订阅行情 - WebSocket 实时订阅
///
/// # 参数
/// - `symbols`: 要订阅的交易对列表
///
/// # 返回
/// 成功时返回空响应，失败时返回错误信息
#[tauri::command]
pub async fn market_subscribe_ticker(
    market_service: State<'_, Arc<MarketService>>,
    symbols: Vec<String>,
) -> Result<ApiResponse<()>, String> {
    let request_id = Uuid::new_v4().to_string();
    log::info!("[{}] market_subscribe_ticker called with symbols: {:?}", request_id, symbols);

    // 验证输入
    if symbols.is_empty() {
        log::warn!("[{}] Empty symbols list", request_id);
        return Ok(ApiResponse::error(ApiError::missing_parameter("symbols")));
    }

    // 验证每个交易对格式
    for symbol in &symbols {
        if let Err(e) = validate_symbol(symbol) {
            log::warn!("[{}] Invalid symbol '{}': {}", request_id, symbol, e.message);
            return Ok(ApiResponse::error(e));
        }
    }

    // 执行订阅逻辑
    market_service
        .subscribe_ticker(symbols.clone())
        .await
        .map_err(|e| {
            log::error!("[{}] Failed to subscribe: {}", request_id, e);
            ApiError::operation_failed("订阅行情失败").to_string()
        })?;

    log::info!("[{}] Successfully subscribed to {} symbols", request_id, symbols.len());
    Ok(ApiResponse::success(()).with_request_id(request_id))
}

/// 获取K线数据 - 支持缓存和数据库
///
/// # 参数
/// - `symbol`: 交易对符号 (如: BTCUSDT)
/// - `interval`: K线间隔 (如: 1m, 5m, 1h, 1d)
/// - `limit`: 返回数据条数 (1-1000)
///
/// # 返回
/// 返回K线数据数组
#[tauri::command]
pub async fn market_get_klines(
    market_service: State<'_, Arc<MarketService>>,
    symbol: String,
    interval: String,
    limit: usize,
) -> Result<ApiResponse<Vec<Kline>>, String> {
    let request_id = Uuid::new_v4().to_string();
    log::info!(
        "[{}] market_get_klines called: symbol={}, interval={}, limit={}",
        request_id,
        symbol,
        interval,
        limit
    );

    // 验证symbol
    if let Err(e) = validate_symbol(&symbol) {
        log::warn!("[{}] Invalid symbol '{}': {}", request_id, symbol, e.message);
        return Ok(ApiResponse::error(e));
    }

    // 验证interval
    if let Err(e) = validate_interval(&interval) {
        log::warn!("[{}] Invalid interval '{}': {}", request_id, interval, e.message);
        return Ok(ApiResponse::error(e));
    }

    // 验证并规范化limit
    let limit = match validate_limit(limit) {
        Ok(l) => l,
        Err(e) => {
            log::warn!("[{}] Invalid limit '{}': {}", request_id, limit, e.message);
            return Ok(ApiResponse::error(e));
        }
    };

    // 获取K线数据
    let klines = market_service
        .get_klines(&symbol, &interval, limit)
        .await
        .map_err(|e| {
            log::error!("[{}] Failed to get klines: {}", request_id, e);
            ApiError::database_error(format!("获取K线数据失败: {}", e)).to_string()
        })?;

    log::debug!("[{}] Returning {} klines for {} {}", request_id, klines.len(), symbol, interval);
    Ok(ApiResponse::success(klines).with_request_id(request_id))
}

/// 获取交易对列表
///
/// # 返回
/// 返回支持的交易对列表
#[tauri::command]
pub async fn market_get_symbols(
    _market_service: State<'_, Arc<MarketService>>,
) -> Result<ApiResponse<Vec<String>>, String> {
    let request_id = Uuid::new_v4().to_string();
    log::info!("[{}] market_get_symbols called", request_id);

    // TODO: 从配置或数据库获取
    // 当前返回硬编码的常用交易对
    let symbols = vec![
        "BTCUSDT".to_string(),
        "ETHUSDT".to_string(),
        "BNBUSDT".to_string(),
        "SOLUSDT".to_string(),
        "XRPUSDT".to_string(),
        "ADAUSDT".to_string(),
        "DOGEUSDT".to_string(),
        "DOTUSDT".to_string(),
        "MATICUSDT".to_string(),
        "LINKUSDT".to_string(),
        "AVAXUSDT".to_string(),
        "UNIUSDT".to_string(),
    ];

    log::debug!("[{}] Returning {} symbols", request_id, symbols.len());
    Ok(ApiResponse::success(symbols).with_request_id(request_id))
}

/// 获取当前市场状态
///
/// # 返回
/// 返回市场连接状态、交易所数量、订阅数量等信息
#[tauri::command]
pub async fn market_get_status(
    market_service: State<'_, Arc<MarketService>>,
) -> Result<ApiResponse<MarketStatus>, String> {
    let request_id = Uuid::new_v4().to_string();
    log::info!("[{}] market_get_status called", request_id);

    let exchanges = market_service.list_exchanges().await;
    let exchanges_str: Vec<String> = exchanges.iter().map(|e| format!("{:?}", e)).collect();

    let status = MarketStatus {
        connected: !exchanges.is_empty(),
        exchanges_count: exchanges.len(),
        exchanges: exchanges_str,
        subscriptions_count: 0, // TODO: Track actual subscription count
        last_update: Some(chrono::Utc::now().timestamp_millis()),
    };

    log::debug!(
        "[{}] Market status: connected={}, exchanges={}, subscriptions={}",
        request_id,
        status.connected,
        status.exchanges_count,
        status.subscriptions_count
    );

    Ok(ApiResponse::success(status).with_request_id(request_id))
}

/// 取消订阅行情
///
/// # 参数
/// - `symbols`: 要取消订阅的交易对列表
///
/// # 注意
/// 当前为占位实现，实际取消订阅逻辑待实现
#[tauri::command]
pub async fn market_unsubscribe_ticker(
    _market_service: State<'_, Arc<MarketService>>,
    symbols: Vec<String>,
) -> Result<ApiResponse<()>, String> {
    let request_id = Uuid::new_v4().to_string();
    log::info!("[{}] market_unsubscribe_ticker called with symbols: {:?}", request_id, symbols);

    // 验证输入
    if symbols.is_empty() {
        log::warn!("[{}] Empty symbols list", request_id);
        return Ok(ApiResponse::error(ApiError::missing_parameter("symbols")));
    }

    // 验证每个交易对格式
    for symbol in &symbols {
        if let Err(e) = validate_symbol(symbol) {
            log::warn!("[{}] Invalid symbol '{}': {}", request_id, symbol, e.message);
            return Ok(ApiResponse::error(e));
        }
    }

    // TODO: 实现实际的取消订阅逻辑
    log::info!("[{}] Unsubscribe requested for {} symbols (placeholder)", request_id, symbols.len());
    Ok(ApiResponse::success(()).with_request_id(request_id))
}

/// 市场状态
#[derive(Debug, Clone, serde::Serialize)]
pub struct MarketStatus {
    /// 是否已连接到交易所
    pub connected: bool,
    /// 已连接的交易所数量
    pub exchanges_count: usize,
    /// 已连接的交易所列表
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exchanges: Vec<String>,
    /// 当前订阅的数量
    pub subscriptions_count: usize,
    /// 最后更新时间
    pub last_update: Option<i64>,
}
