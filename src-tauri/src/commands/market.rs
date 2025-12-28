use crate::core::trade::types::*;
use crate::services::MarketService;
use std::sync::Arc;
use tauri::State;

/// 订阅行情 - WebSocket 实时订阅
#[tauri::command]
pub async fn market_subscribe_ticker(
    market_service: State<'_, Arc<MarketService>>,
    symbols: Vec<String>,
) -> Result<(), String> {
    log::info!("market_subscribe_ticker called with symbols: {:?}", symbols);

    market_service
        .subscribe_ticker(symbols)
        .await
        .map_err(|e| e.to_string())
}

/// 获取K线数据 - 支持缓存和数据库
#[tauri::command]
pub async fn market_get_klines(
    market_service: State<'_, Arc<MarketService>>,
    symbol: String,
    interval: String,
    limit: usize,
) -> Result<Vec<Kline>, String> {
    log::info!("market_get_klines called: symbol={}, interval={}, limit={}", symbol, interval, limit);

    market_service
        .get_klines(&symbol, &interval, limit)
        .await
        .map_err(|e| e.to_string())
}

/// 获取交易对列表
#[tauri::command]
pub async fn market_get_symbols(
    market_service: State<'_, Arc<MarketService>>,
) -> Result<Vec<String>, String> {
    log::info!("market_get_symbols called");

    // Return common trading pairs
    // In the future, this could be fetched from the exchange
    Ok(vec![
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
    ])
}

/// 获取当前市场状态
#[tauri::command]
pub async fn market_get_status(
    market_service: State<'_, Arc<MarketService>>,
) -> Result<MarketStatus, String> {
    log::info!("market_get_status called");

    let exchanges = market_service.list_exchanges().await;

    Ok(MarketStatus {
        connected: !exchanges.is_empty(),
        exchanges_count: exchanges.len(),
        subscriptions_count: 0, // TODO: Track actual subscription count
        last_update: Some(chrono::Utc::now().timestamp_millis()),
    })
}

/// 取消订阅行情
#[tauri::command]
pub async fn market_unsubscribe_ticker(
    market_service: State<'_, Arc<MarketService>>,
    symbols: Vec<String>,
) -> Result<(), String> {
    log::info!("market_unsubscribe_ticker called with symbols: {:?}", symbols);

    // TODO: Implement actual unsubscribe logic
    // For now, this is a placeholder
    Ok(())
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MarketStatus {
    pub connected: bool,
    pub exchanges_count: usize,
    pub subscriptions_count: usize,
    pub last_update: Option<i64>,
}
