use super::super::types::*;
use async_trait::async_trait;
use anyhow::Result;
use tokio::sync::broadcast;
use std::fmt;

/// 交易所名称
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExchangeName {
    Binance,
    OKX,
    Bybit,
}

impl ExchangeName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Binance => "binance",
            Self::OKX => "okx",
            Self::Bybit => "bybit",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "binance" => Some(Self::Binance),
            "okx" => Some(Self::OKX),
            "bybit" => Some(Self::Bybit),
            _ => None,
        }
    }
}

impl fmt::Display for ExchangeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<String> for ExchangeName {
    fn from(s: String) -> Self {
        Self::parse(s.as_str()).unwrap_or(Self::Binance)
    }
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
    fn order_stream(&self) -> broadcast::Receiver<Order>;

    // ========== 用户数据流订阅 ==========
    /// Subscribe to user data stream for real-time order/account updates
    async fn subscribe_user_data(&self) -> Result<()>;

    // ========== 交易操作 ==========
    /// Place a new order
    async fn place_order(&self, request: &OrderRequest) -> Result<Order>;

    /// Cancel an existing order
    async fn cancel_order(&self, order_id: &str) -> Result<()>;

    /// Get order details by order ID
    async fn get_order(&self, order_id: &str) -> Result<Order>;

    /// Get all open orders
    async fn get_open_orders(&self, symbol: Option<&str>) -> Result<Vec<Order>>;

    /// Get current account balance
    async fn get_balance(&self) -> Result<Vec<Balance>>;

    /// Get current positions
    async fn get_positions(&self) -> Result<Vec<Position>>;
}
