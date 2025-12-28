//! Bybit Exchange implementation
//!
//! Bybit API Documentation: https://bybit-exchange.github.io/docs/v5/intro
//!
//! Key differences from Binance:
//! - Uses HMAC-SHA256 signature with timestamp
//! - WebSocket streams use different channel format
//! - Supports both spot and derivatives trading
//! - API v5 is the current version

use super::super::types::*;
use super::r#trait::{Exchange, ExchangeName};
use async_trait::async_trait;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex, RwLock};
use tokio_tungstenite::tungstenite::Message;
use futures_util::{StreamExt, SinkExt};

const REST_API_BASE: &str = "https://api.bybit.com";
const WS_API_PUBLIC: &str = "wss://stream.bybit.com/v5/public/spot";
const WS_API_PRIVATE: &str = "wss://stream.bybit.com/v5/private";

/// Bybit-specific error codes
#[derive(Debug)]
pub enum BybitError {
    AuthenticationFailed,
    InvalidSignature,
    RateLimited,
    InsufficientFunds,
    InvalidOrder,
    UnknownError(String),
}

impl std::fmt::Display for BybitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AuthenticationFailed => write!(f, "Authentication failed"),
            Self::InvalidSignature => write!(f, "Invalid signature"),
            Self::RateLimited => write!(f, "Rate limited"),
            Self::InsufficientFunds => write!(f, "Insufficient funds"),
            Self::InvalidOrder => write!(f, "Invalid order"),
            Self::UnknownError(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for BybitError {}

/// Bybit API client with signature support
struct BybitClient {
    api_key: String,
    api_secret: String,
    client: Client,
    is_testnet: bool,
}

impl BybitClient {
    fn new(api_key: String, api_secret: String, is_testnet: bool) -> Self {
        Self {
            api_key,
            api_secret,
            client: Client::new(),
            is_testnet,
        }
    }

    /// Generate Bybit API signature
    /// Bybit uses: timestamp + api_key + recv_window + query_string
    fn sign_request(&self, timestamp: &str, params: &str) -> String {
        use hmac::Mac;
        use sha2::Sha256;

        let message = format!("{}{}{}{}", timestamp, self.api_key, "5000", params);
        let mut mac = hmac::Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }

    /// Get base URL based on testnet setting
    fn base_url(&self) -> &str {
        if self.is_testnet {
            "https://api-testnet.bybit.com" // Bybit testnet URL
        } else {
            REST_API_BASE
        }
    }

    /// Make authenticated GET request
    async fn get_signed(&self, path: &str, params: &str) -> Result<Value> {
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let sign = self.sign_request(&timestamp, params);

        let url = if params.is_empty() {
            format!("{}{}", self.base_url(), path)
        } else {
            format!("{}{}?{}", self.base_url(), path, params)
        };

        let response = self.client
            .get(&url)
            .header("X-BAPI-API-KEY", &self.api_key)
            .header("X-BAPI-SIGN", sign)
            .header("X-BAPI-TIMESTAMP", timestamp)
            .header("X-BAPI-RECV-WINDOW", "5000")
            .send()
            .await?;

        let json: Value = response.json().await?;

        // Check Bybit response format
        if json["retCode"] != 0 {
            return Err(anyhow!("Bybit API error: {}", json["retMsg"]));
        }

        Ok(json)
    }

    /// Make authenticated POST request
    async fn post_signed(&self, path: &str, body: &str) -> Result<Value> {
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let sign = self.sign_request(&timestamp, body);

        let response = self.client
            .post(format!("{}{}", self.base_url(), path))
            .header("X-BAPI-API-KEY", &self.api_key)
            .header("X-BAPI-SIGN", sign)
            .header("X-BAPI-TIMESTAMP", timestamp)
            .header("X-BAPI-RECV-WINDOW", "5000")
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        let json: Value = response.json().await?;

        if json["retCode"] != 0 {
            return Err(anyhow!("Bybit API error: {}", json["retMsg"]));
        }

        Ok(json)
    }
}

/// Bybit Exchange implementation
pub struct BybitExchange {
    api_key: Option<String>,
    api_secret: Option<String>,
    is_testnet: bool,
    client: Client,
    ticker_tx: broadcast::Sender<Ticker>,
    kline_tx: broadcast::Sender<Kline>,
    order_tx: broadcast::Sender<Order>,
    connection_state: Arc<RwLock<bool>>,
    ws_task_handles: Arc<Mutex<Vec<tokio::task::JoinHandle<()>>>>,
}

impl BybitExchange {
    pub fn new(
        api_key: Option<String>,
        api_secret: Option<String>,
        _passphrase: Option<String>, // Bybit doesn't use passphrase
    ) -> Self {
        let (ticker_tx, _) = broadcast::channel(1000);
        let (kline_tx, _) = broadcast::channel(1000);
        let (order_tx, _) = broadcast::channel(1000);

        Self {
            api_key,
            api_secret,
            is_testnet: false,
            client: Client::new(),
            ticker_tx,
            kline_tx,
            order_tx,
            connection_state: Arc::new(RwLock::new(false)),
            ws_task_handles: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Set testnet mode
    pub fn with_testnet(mut self, is_testnet: bool) -> Self {
        self.is_testnet = is_testnet;
        self
    }

    /// Create REST client
    fn rest_client(&self) -> Result<BybitClient> {
        let api_key = self.api_key.clone()
            .ok_or_else(|| anyhow!("API key not configured"))?;
        let api_secret = self.api_secret.clone()
            .ok_or_else(|| anyhow!("API secret not configured"))?;

        Ok(BybitClient::new(api_key, api_secret, self.is_testnet))
    }

    /// Bybit uses same format as Binance (BTCUSDT)
    fn normalize_symbol(&self, symbol: &str) -> String {
        symbol.to_uppercase()
    }

    /// Convert Interval to Bybit kline format
    fn interval_to_bybit(&self, interval: Interval) -> &'static str {
        match interval {
            Interval::OneMinute => "1",
            Interval::FiveMinutes => "5",
            Interval::FifteenMinutes => "15",
            Interval::ThirtyMinutes => "30",
            Interval::OneHour => "60",
            Interval::FourHours => "240",
            Interval::OneDay => "D",
        }
    }

    /// Parse Bybit ticker response
    fn parse_ticker(&self, json: &Value, symbol: &str) -> Result<Ticker> {
        let data = &json["result"];

        Ok(Ticker {
            symbol: self.normalize_symbol(symbol),
            price: data["lastPrice"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            price_change: data["price24hPcnt"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            price_change_percent: data["price24hPcnt"].as_str().unwrap_or("0").parse().unwrap_or(0.0) * 100.0,
            high_24h: data["highPrice24h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            low_24h: data["lowPrice24h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            volume_24h: data["volume24h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            timestamp: chrono::Utc::now().timestamp_millis(),
        })
    }

    /// Parse Bybit kline response
    fn parse_kline(&self, item: &Value, symbol: &str, interval: Interval) -> Result<Kline> {
        let data = item.as_object()
            .ok_or_else(|| anyhow!("Invalid kline data"))?;

        Ok(Kline {
            symbol: self.normalize_symbol(symbol),
            timeframe: interval.as_str().to_string(),
            timestamp: data.get("start").and_then(|v| v.as_i64()).unwrap_or(0),
            open: data["open"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            high: data["high"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            low: data["low"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            close: data["close"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            volume: data["volume"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            quote_volume: data["turnover"].as_str().and_then(|s| s.parse().ok()),
        })
    }

    /// Convert OrderType to Bybit format
    fn order_type_to_bybit(order_type: OrderType) -> &'static str {
        match order_type {
            OrderType::Market => "MARKET",
            OrderType::Limit => "LIMIT",
            OrderType::StopLoss => "CONDITIONAL_MARKET",
            OrderType::StopLimit => "CONDITIONAL_LIMIT",
            OrderType::OCO => "OCO",
        }
    }

    /// Convert OrderSide to Bybit format
    fn side_to_bybit(side: OrderSide) -> &'static str {
        match side {
            OrderSide::Buy => "Buy",
            OrderSide::Sell => "Sell",
        }
    }

    /// Parse Bybit order response
    fn parse_order(&self, json: &Value, request: &OrderRequest) -> Result<Order> {
        let data = &json["result"];

        Ok(Order {
            id: data["orderId"].as_str().unwrap_or("").to_string(),
            exchange_order_id: Some(data["orderId"].as_str().unwrap_or("").to_string()),
            client_order_id: data["orderLinkId"].as_str().map(|s| s.to_string()),
            symbol: self.normalize_symbol(&request.symbol),
            side: request.side,
            order_type: request.order_type,
            price: request.price,
            quantity: request.quantity,
            filled_quantity: data["cumExecQty"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            avg_price: data["avgPrice"].as_str().and_then(|s| s.parse().ok()),
            status: self.parse_order_state(data["orderStatus"].as_str().unwrap_or("Created")),
            commission: 0.0,
            created_at: chrono::Utc::now().timestamp_millis(),
            filled_at: None,
        })
    }

    /// Parse Bybit order state
    fn parse_order_state(&self, state: &str) -> OrderState {
        match state {
            "New" | "PartiallyFilled" => OrderState::Open,
            "Filled" => OrderState::Filled,
            "Cancelled" => OrderState::Canceled,
            "Rejected" => OrderState::Rejected,
            _ => OrderState::Pending,
        }
    }

    /// Static helper to normalize symbol
    fn normalize_symbol_static(symbol: &str) -> String {
        symbol.to_uppercase()
    }

    /// Static helper to parse order state
    fn parse_order_state_static(state: &str) -> OrderState {
        match state {
            "New" | "PartiallyFilled" => OrderState::Open,
            "Filled" => OrderState::Filled,
            "Cancelled" => OrderState::Canceled,
            "Rejected" => OrderState::Rejected,
            _ => OrderState::Pending,
        }
    }

    /// Parse WebSocket ticker message
    fn parse_ws_ticker_static(data: &Value) -> Result<Ticker> {
        let symbol = data["s"].as_str().unwrap_or("");
        let symbol = Self::normalize_symbol_static(symbol);

        Ok(Ticker {
            symbol,
            price: data["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0), // Last price
            price_change: 0.0,
            price_change_percent: data["b24"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            high_24h: data["h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            low_24h: data["l"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            volume_24h: data["v"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            timestamp: chrono::Utc::now().timestamp_millis(),
        })
    }

    /// Parse WebSocket kline message
    fn parse_ws_kline_static(data: &Value, interval: Interval) -> Result<Kline> {
        // Bybit kline format: nested object with symbol as key
        if let Some(symbol) = data.as_object().and_then(|o| o.keys().next()) {
            if let Some(kline_data) = data.get(symbol) {
                let arr = kline_data.as_array();
                if let Some(arr) = arr {
                    if let Some(item) = arr.first() {
                        return Ok(Kline {
                            symbol: Self::normalize_symbol_static(symbol),
                            timeframe: interval.as_str().to_string(),
                            timestamp: item.get("t").and_then(|v| v.as_i64()).unwrap_or(0),
                            open: item.get("o").and_then(|v| v.as_str()).unwrap_or("0").parse().unwrap_or(0.0),
                            high: item.get("h").and_then(|v| v.as_str()).unwrap_or("0").parse().unwrap_or(0.0),
                            low: item.get("l").and_then(|v| v.as_str()).unwrap_or("0").parse().unwrap_or(0.0),
                            close: item.get("c").and_then(|v| v.as_str()).unwrap_or("0").parse().unwrap_or(0.0),
                            volume: item.get("v").and_then(|v| v.as_str()).unwrap_or("0").parse().unwrap_or(0.0),
                            quote_volume: None,
                        });
                    }
                }
            }
        }

        // Fallback: try direct array format
        let arr = data.as_array()
            .ok_or_else(|| anyhow!("Invalid kline data"))?;

        Ok(Kline {
            symbol: String::new(),
            timeframe: interval.as_str().to_string(),
            timestamp: arr.get(0).and_then(|v| v.as_i64()).unwrap_or(0),
            open: arr.get(1).and_then(|v| v.as_str()).unwrap_or("0").parse().unwrap_or(0.0),
            high: arr.get(2).and_then(|v| v.as_str()).unwrap_or("0").parse().unwrap_or(0.0),
            low: arr.get(3).and_then(|v| v.as_str()).unwrap_or("0").parse().unwrap_or(0.0),
            close: arr.get(4).and_then(|v| v.as_str()).unwrap_or("0").parse().unwrap_or(0.0),
            volume: arr.get(5).and_then(|v| v.as_str()).unwrap_or("0").parse().unwrap_or(0.0),
            quote_volume: arr.get(6).and_then(|v| v.as_str()).and_then(|s| s.parse().ok()),
        })
    }

    /// Parse WebSocket order message
    fn parse_ws_order_from_value(data: &Value) -> Result<Order> {
        let symbol = data["symbol"].as_str().unwrap_or("");
        let symbol = Self::normalize_symbol_static(symbol);

        Ok(Order {
            id: data["orderId"].as_str().unwrap_or("").to_string(),
            exchange_order_id: Some(data["orderId"].as_str().unwrap_or("").to_string()),
            client_order_id: data["orderLinkId"].as_str().map(|s| s.to_string()),
            symbol,
            side: match data["side"].as_str().unwrap_or("") {
                "Buy" => OrderSide::Buy,
                "Sell" => OrderSide::Sell,
                _ => OrderSide::Buy,
            },
            order_type: match data["orderType"].as_str().unwrap_or("") {
                "MARKET" => OrderType::Market,
                "LIMIT" => OrderType::Limit,
                _ => OrderType::Market,
            },
            price: data["price"].as_str().and_then(|s| s.parse().ok()),
            quantity: data["qty"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            filled_quantity: data["cumExecQty"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            avg_price: data["avgPrice"].as_str().and_then(|s| s.parse().ok()),
            status: Self::parse_order_state_static(data["orderStatus"].as_str().unwrap_or("")),
            commission: 0.0,
            created_at: data["createdTime"].as_i64().unwrap_or(0),
            filled_at: None,
        })
    }

    /// Start public WebSocket for tickers and klines
    async fn ws_public_loop(&self, channels: Vec<String>, interval: Option<Interval>) -> Result<()> {
        Self::ws_public_loop_impl(
            &self.ticker_tx,
            &self.kline_tx,
            channels,
            interval,
        ).await
    }

    /// Implementation of public WebSocket loop
    async fn ws_public_loop_impl(
        ticker_tx: &broadcast::Sender<Ticker>,
        kline_tx: &broadcast::Sender<Kline>,
        channels: Vec<String>,
        interval: Option<Interval>,
    ) -> Result<()> {
        log::info!("Connecting to Bybit public WebSocket: {:?}", channels);

        let (ws_stream, _) = tokio_tungstenite::connect_async(WS_API_PUBLIC).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // Subscribe to channels
        for channel in channels {
            let sub_msg = serde_json::json!({
                "op": "subscribe",
                "args": [channel]
            });
            ws_sender.send(Message::Text(sub_msg.to_string())).await?;
            log::info!("Subscribed to Bybit channel: {}", channel);
        }

        // Message loop
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(json) = serde_json::from_str::<Value>(&text) {
                        // Handle data messages
                        if let Some(topic) = json.get("topic").and_then(|t| t.as_str()) {
                            if let Some(data) = json.get("data") {
                                match topic {
                                    t if t.starts_with("tickers") => {
                                        if let Ok(ticker) = Self::parse_ws_ticker_static(data) {
                                            let _ = ticker_tx.send(ticker);
                                        }
                                    }
                                    t if t.starts_with("kline") => {
                                        if let Some(interval_val) = interval {
                                            if let Ok(kline) = Self::parse_ws_kline_static(data, interval_val) {
                                                let _ = kline_tx.send(kline);
                                            }
                                        }
                                    }
                                    _ => {
                                        log::debug!("Unhandled Bybit topic: {}", topic);
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(Message::Ping(data)) => {
                    let _ = ws_sender.send(Message::Pong(data)).await;
                }
                Ok(Message::Close(_)) => {
                    log::info!("Bybit WebSocket connection closed");
                    break;
                }
                Ok(Message::Pong(_)) => {}
                Err(e) => {
                    log::error!("Bybit WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Static helper for ticker WebSocket loop
    async fn ws_public_loop_helper_ticker(
        channels: Vec<String>,
        tx_ticker: broadcast::Sender<Ticker>,
    ) -> Result<()> {
        let kline_tx = broadcast::channel(1).0;
        Self::ws_public_loop_impl(&tx_ticker, &kline_tx, channels, None).await
    }

    /// Static helper for kline WebSocket loop
    async fn ws_public_loop_helper_kline(
        channels: Vec<String>,
        interval: Interval,
        tx_kline: broadcast::Sender<Kline>,
    ) -> Result<()> {
        let ticker_tx = broadcast::channel(1).0;
        Self::ws_public_loop_impl(&ticker_tx, &tx_kline, channels, Some(interval)).await
    }
}

#[async_trait]
impl Exchange for BybitExchange {
    fn name(&self) -> ExchangeName {
        ExchangeName::Bybit
    }

    fn is_connected(&self) -> bool {
        self.connection_state
            .try_read()
            .map(|guard| *guard)
            .unwrap_or(false)
    }

    async fn connect(&self) -> Result<()> {
        *self.connection_state.write().await = true;
        log::info!("Bybit Exchange connected (testnet: {})", self.is_testnet);
        Ok(())
    }

    async fn disconnect(&self) -> Result<()> {
        *self.connection_state.write().await = false;

        // Stop all WebSocket tasks
        let mut handles = self.ws_task_handles.lock().await;
        for handle in handles.drain(..) {
            handle.abort();
        }

        log::info!("Bybit Exchange disconnected");
        Ok(())
    }

    async fn get_ticker(&self, symbol: &str) -> Result<Ticker> {
        let bybit_symbol = self.normalize_symbol(symbol);
        let path = format!("/v5/market/tickers?category=spot&symbol={}", bybit_symbol);

        let response = self.client.get(&format!("{}{}", REST_API_BASE, path))
            .send()
            .await?
            .json::<Value>()
            .await?;

        if response["retCode"] != 0 {
            return Err(anyhow!("Bybit ticker error: {}", response["retMsg"]));
        }

        self.parse_ticker(&response, symbol)
    }

    async fn get_klines(
        &self,
        symbol: &str,
        interval: Interval,
        limit: usize,
    ) -> Result<Vec<Kline>> {
        let bybit_symbol = self.normalize_symbol(symbol);
        let bybit_interval = self.interval_to_bybit(interval);

        let path = format!(
            "/v5/market/kline?category=spot&symbol={}&interval={}&limit={}",
            bybit_symbol, bybit_interval, limit
        );

        let response = self.client.get(&format!("{}{}", REST_API_BASE, path))
            .send()
            .await?
            .json::<Value>()
            .await?;

        if response["retCode"] != 0 {
            return Err(anyhow!("Bybit klines error: {}", response["retMsg"]));
        }

        let data = response["result"]["list"].as_array()
            .ok_or_else(|| anyhow!("Invalid klines response"))?;

        data.iter()
            .map(|item| self.parse_kline(item, symbol, interval))
            .collect()
    }

    async fn subscribe_ticker(&self, symbols: Vec<String>) -> Result<()> {
        if symbols.is_empty() {
            return Ok(());
        }

        let bybit_symbols: Vec<String> = symbols.iter()
            .map(|s| self.normalize_symbol(s))
            .collect();

        log::info!("Subscribing to Bybit tickers: {:?}", bybit_symbols);

        let channels: Vec<String> = bybit_symbols.iter()
            .map(|s| format!("tickers.{}", s))
            .collect();

        let tx_ticker = self.ticker_tx.clone();
        let connection_state = self.connection_state.clone();

        let handle = tokio::spawn(async move {
            log::info!("Bybit ticker WebSocket task started");

            let mut retry_count = 0;
            const MAX_RETRIES: u32 = 5;

            while *connection_state.read().await {
                if retry_count >= MAX_RETRIES {
                    log::error!("Bybit ticker WebSocket max retries reached");
                    break;
                }

                match Self::ws_public_loop_helper_ticker(channels.clone(), tx_ticker.clone()).await {
                    Ok(_) => {
                        retry_count = 0;
                    }
                    Err(e) => {
                        retry_count += 1;
                        log::error!("Bybit ticker WebSocket error (retry {}): {}", retry_count, e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }

            log::info!("Bybit ticker WebSocket task stopped");
        });

        let mut handles = self.ws_task_handles.lock().await;
        handles.push(handle);

        Ok(())
    }

    async fn subscribe_kline(&self, symbols: Vec<String>, interval: Interval) -> Result<()> {
        if symbols.is_empty() {
            return Ok(());
        }

        let bybit_symbols: Vec<String> = symbols.iter()
            .map(|s| self.normalize_symbol(s))
            .collect();

        let bybit_interval = self.interval_to_bybit(interval);

        log::info!("Subscribing to Bybit klines: {:?} ({})", bybit_symbols, bybit_interval);

        let channels: Vec<String> = bybit_symbols.iter()
            .map(|s| format!("kline.{}.{}", bybit_interval, s))
            .collect();

        let tx_kline = self.kline_tx.clone();
        let connection_state = self.connection_state.clone();

        let handle = tokio::spawn(async move {
            log::info!("Bybit kline WebSocket task started");

            let mut retry_count = 0;
            const MAX_RETRIES: u32 = 5;

            while *connection_state.read().await {
                if retry_count >= MAX_RETRIES {
                    log::error!("Bybit kline WebSocket max retries reached");
                    break;
                }

                match Self::ws_public_loop_helper_kline(channels.clone(), interval, tx_kline.clone()).await {
                    Ok(_) => {
                        retry_count = 0;
                    }
                    Err(e) => {
                        retry_count += 1;
                        log::error!("Bybit kline WebSocket error (retry {}): {}", retry_count, e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }

            log::info!("Bybit kline WebSocket task stopped");
        });

        let mut handles = self.ws_task_handles.lock().await;
        handles.push(handle);

        Ok(())
    }

    fn ticker_stream(&self) -> broadcast::Receiver<Ticker> {
        self.ticker_tx.subscribe()
    }

    fn kline_stream(&self) -> broadcast::Receiver<Kline> {
        self.kline_tx.subscribe()
    }

    fn order_stream(&self) -> broadcast::Receiver<Order> {
        self.order_tx.subscribe()
    }

    async fn subscribe_user_data(&self) -> Result<()> {
        // TODO: Implement Bybit private WebSocket for order updates
        log::warn!("Bybit user data stream not yet implemented");
        Ok(())
    }

    async fn place_order(&self, request: &OrderRequest) -> Result<Order> {
        let client = self.rest_client()?;

        let bybit_symbol = self.normalize_symbol(&request.symbol);

        // Build order request body
        let body = serde_json::json!({
            "category": "spot",
            "symbol": bybit_symbol,
            "side": Self::side_to_bybit(request.side),
            "orderType": Self::order_type_to_bybit(request.order_type),
            "qty": request.quantity.to_string(),
            "timeInForce": "GTC"
        });

        let mut body_map = body.as_object().unwrap().clone();
        if let Some(price) = request.price {
            body_map.insert("price".to_string(), serde_json::json!(price.to_string()));
        }

        let body_str = serde_json::to_string(&body_map)?;
        let response = client.post_signed("/v5/order/create", &body_str).await?;

        self.parse_order(&response, request)
    }

    async fn cancel_order(&self, order_id: &str) -> Result<()> {
        let client = self.rest_client()?;

        let body = serde_json::json!({
            "category": "spot",
            "orderId": order_id,
        });

        let body_str = serde_json::to_string(&body)?;
        client.post_signed("/v5/order/cancel", &body_str).await?;

        Ok(())
    }

    async fn get_order(&self, order_id: &str) -> Result<Order> {
        let client = self.rest_client()?;

        let body = serde_json::json!({
            "category": "spot",
            "orderId": order_id,
        });

        let body_str = serde_json::to_string(&body)?;
        let response = client.post_signed("/v5/order/query", &body_str).await?;

        // Parse response - need proper OrderRequest context
        todo!("Bybit get_order parsing - need context for symbol/side/price")
    }

    async fn get_open_orders(&self, symbol: Option<&str>) -> Result<Vec<Order>> {
        let client = self.rest_client()?;

        let mut body = serde_json::json!({
            "category": "spot",
        });

        if let Some(sym) = symbol {
            body["symbol"] = serde_json::json!(self.normalize_symbol(sym));
        }

        let body_str = serde_json::to_string(&body)?;
        let response = client.post_signed("/v5/order/open", &body_str).await?;

        // Parse response
        todo!("Bybit get_open_orders parsing")
    }

    async fn get_balance(&self) -> Result<Vec<Balance>> {
        let client = self.rest_client()?;

        let body = serde_json::json!({
            "accountType": "SPOT",
        });
        let body_str = serde_json::to_string(&body)?;
        let response = client.post_signed("/v5/account/wallet-balance", &body_str).await?;

        let data = response["result"]["list"].as_array()
            .ok_or_else(|| anyhow!("Invalid balance response"))?;

        let balances: Vec<Balance> = data.iter()
            .filter_map(|account| {
                let coin_list = account["coin"].as_array();
                if let Some(coins) = coin_list {
                    Some(coins.iter().filter_map(|item| {
                        let wallet_balance = item["walletBalance"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                        if wallet_balance > 0.0 {
                            Some(Balance {
                                asset: item["coin"].as_str().unwrap_or("").to_string(),
                                free: wallet_balance,
                                locked: item["walletBalance"].as_str().unwrap_or("0").parse().unwrap_or(0.0) - wallet_balance,
                                total: wallet_balance,
                            })
                        } else {
                            None
                        }
                    }).collect::<Vec<Balance>>())
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        Ok(balances)
    }

    async fn get_positions(&self) -> Result<Vec<Position>> {
        // Bybit SPOT doesn't have position concept like derivatives
        // Return empty for spot trading
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bybit_symbol_conversion() {
        let exchange = BybitExchange::new(None, None, None);

        // Normalize symbol (same as Binance)
        assert_eq!(exchange.normalize_symbol("BTCUSDT"), "BTCUSDT");
        assert_eq!(exchange.normalize_symbol("ETHUSDT"), "ETHUSDT");
    }

    #[test]
    fn test_interval_conversion() {
        let exchange = BybitExchange::new(None, None, None);

        assert_eq!(exchange.interval_to_bybit(Interval::OneMinute), "1");
        assert_eq!(exchange.interval_to_bybit(Interval::OneHour), "60");
        assert_eq!(exchange.interval_to_bybit(Interval::OneDay), "D");
    }

    #[test]
    fn test_order_type_conversion() {
        assert_eq!(BybitExchange::order_type_to_bybit(OrderType::Market), "MARKET");
        assert_eq!(BybitExchange::order_type_to_bybit(OrderType::Limit), "LIMIT");
    }

    #[test]
    fn test_side_conversion() {
        assert_eq!(BybitExchange::side_to_bybit(OrderSide::Buy), "Buy");
        assert_eq!(BybitExchange::side_to_bybit(OrderSide::Sell), "Sell");
    }

    #[test]
    fn test_order_state_parsing() {
        let exchange = BybitExchange::new(None, None, None);

        assert_eq!(exchange.parse_order_state("New"), OrderState::Open);
        assert_eq!(exchange.parse_order_state("Filled"), OrderState::Filled);
        assert_eq!(exchange.parse_order_state("Cancelled"), OrderState::Canceled);
    }
}
