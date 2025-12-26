use super::super::types::*;
use async_trait::async_trait;
use anyhow::Result;
use tokio::sync::broadcast;

/// 交易所名称
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExchangeName {
    Binance,
    OKX,
    Bybit,
}

/// 交易所统一接口
#[async_trait]
pub trait Exchange: Send + Sync {
    // ========== 元数据 ==========
    fn name(&self) -> ExchangeName;
    fn is_connected(&self) -> bool;

    // ========== 连接管理 ==========
    async fn connect(&self) -> Result<()>;
    async fn disconnect(&self) -> Result<()>;

    // ========== 行情数据 (REST) ==========
    async fn get_ticker(&self, symbol: &str) -> Result<Ticker>;
    async fn get_klines(
        &self,
        symbol: &str,
        interval: Interval,
        limit: usize,
    ) -> Result<Vec<Kline>>;

    // ========== 行情数据 (WebSocket订阅) ==========
    async fn subscribe_ticker(&self, symbols: Vec<String>) -> Result<()>;
    async fn subscribe_kline(
        &self,
        symbols: Vec<String>,
        interval: Interval,
    ) -> Result<()>;

    // ========== 事件流 ==========
    fn ticker_stream(&self) -> broadcast::Receiver<Ticker>;
    fn kline_stream(&self) -> broadcast::Receiver<Kline>;
}
