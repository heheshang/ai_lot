use crate::services::MarketService;
use crate::core::trade::types::*;
use crate::infrastructure::Database;
use tauri::State;

/// 订阅行情
#[tauri::command]
pub async fn market_subscribe_ticker(
    db: State<'_, Database>,
    symbols: Vec<String>,
) -> Result<(), String> {
    log::info!("market_subscribe_ticker called with symbols: {:?}", symbols);

    // Note: MarketService should be managed as Tauri state
    // For now, we'll return a placeholder error
    // In production, MarketService would be added to app.manage() during setup

    // This is a simplified implementation
    // The full implementation would require MarketService to be in Tauri state
    Err("MarketService not initialized. This feature requires MarketService to be registered in Tauri state.".to_string())
}

/// 获取K线数据
#[tauri::command]
pub async fn market_get_klines(
    db: State<'_, Database>,
    symbol: String,
    interval: String,
    limit: usize,
) -> Result<Vec<Kline>, String> {
    log::info!("market_get_klines called: symbol={}, interval={}, limit={}", symbol, interval, limit);

    // Note: MarketService should be managed as Tauri state
    // For now, we'll return a placeholder
    // In production, MarketService would be added to app.manage() during setup

    // This is a simplified implementation that returns empty data
    // The full implementation would require MarketService to be in Tauri state
    Ok(vec![])
}

/// 获取交易对列表
#[tauri::command]
pub async fn market_get_symbols(
    db: State<'_, Database>,
) -> Result<Vec<String>, String> {
    log::info!("market_get_symbols called");

    // Return common trading pairs
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
    ])
}

/// 获取当前市场状态
#[tauri::command]
pub async fn market_get_status(
    db: State<'_, Database>,
) -> Result<MarketStatus, String> {
    log::info!("market_get_status called");

    Ok(MarketStatus {
        connected: false,
        exchanges_count: 0,
        subscriptions_count: 0,
        last_update: None,
    })
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MarketStatus {
    pub connected: bool,
    pub exchanges_count: usize,
    pub subscriptions_count: usize,
    pub last_update: Option<i64>,
}

/// 取消订阅行情
#[tauri::command]
pub async fn market_unsubscribe_ticker(
    db: State<'_, Database>,
    symbols: Vec<String>,
) -> Result<(), String> {
    log::info!("market_unsubscribe_ticker called with symbols: {:?}", symbols);

    // Placeholder for unsubscribe functionality
    Ok(())
}
