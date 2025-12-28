//! OKX Exchange implementation
//!
//! OKX API Documentation: https://www.okx.com/docs-v5/
//!
//! Key differences from Binance:
//! - Requires passphrase for API signature
//! - Uses different authentication (HMAC-SHA256 with timestamp)
//! - WebSocket streams use different channel format
//! - Supports more order types natively

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

const REST_API_BASE: &str = "https://www.okx.com";
const WS_API_PUBLIC: &str = "wss://ws.okx.com:8443/ws/v5/public";
const WS_API_PRIVATE: &str = "wss://ws.okx.com:8443/ws/v5/private";

/// OKX-specific error codes
#[derive(Debug)]
pub enum OkxError {
    AuthenticationFailed,
    InvalidSignature,
    RateLimited,
    InsufficientFunds,
    InvalidOrder,
    UnknownError(String),
}

impl std::fmt::Display for OkxError {
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

impl std::error::Error for OkxError {}

/// OKX API client with signature support
struct OkxClient {
    api_key: String,
    api_secret: String,
    passphrase: String,
    client: Client,
    is_testnet: bool,
}

impl OkxClient {
    fn new(api_key: String, api_secret: String, passphrase: String, is_testnet: bool) -> Self {
        Self {
            api_key,
            api_secret,
            passphrase,
            client: Client::new(),
            is_testnet,
        }
    }

    /// Generate OKX API signature
    /// OKX uses: timestamp + method + requestPath + body
    fn sign_request(&self, timestamp: &str, method: &str, path: &str, body: &str) -> String {
        use hmac::Mac;
        use sha2::Sha256;

        let message = format!("{}{}{}{}", timestamp, method, path, body);
        let mut mac = hmac::Hmac::<Sha256>::new_from_slice(self.api_secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(message.as_bytes());
        let result = mac.finalize();
        hex::encode(result.into_bytes())
    }

    /// Get base URL based on testnet setting
    fn base_url(&self) -> &str {
        if self.is_testnet {
            "https://www.okx.com" // OKX testnet URL
        } else {
            REST_API_BASE
        }
    }

    /// Make authenticated GET request
    async fn get_signed(&self, path: &str) -> Result<Value> {
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let sign = self.sign_request(&timestamp, "GET", path, "");

        let response = self.client
            .get(format!("{}{}", self.base_url(), path))
            .header("OK-ACCESS-KEY", &self.api_key)
            .header("OK-ACCESS-SIGN", sign)
            .header("OK-ACCESS-TIMESTAMP", timestamp)
            .header("OK-ACCESS-PASSPHRASE", &self.passphrase)
            .send()
            .await?;

        let json: Value = response.json().await?;

        // Check OKX response format
        if json["code"] != "0" {
            return Err(anyhow!("OKX API error: {}", json["msg"]));
        }

        Ok(json)
    }

    /// Make authenticated POST request
    async fn post_signed(&self, path: &str, body: &str) -> Result<Value> {
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let sign = self.sign_request(&timestamp, "POST", path, body);

        let response = self.client
            .post(format!("{}{}", self.base_url(), path))
            .header("OK-ACCESS-KEY", &self.api_key)
            .header("OK-ACCESS-SIGN", sign)
            .header("OK-ACCESS-TIMESTAMP", timestamp)
            .header("OK-ACCESS-PASSPHRASE", &self.passphrase)
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        let json: Value = response.json().await?;

        if json["code"] != "0" {
            return Err(anyhow!("OKX API error: {}", json["msg"]));
        }

        Ok(json)
    }

    /// Make authenticated DELETE request
    async fn delete_signed(&self, path: &str, params: &[(&str, &str)]) -> Result<Value> {
        let query_string = if params.is_empty() {
            String::new()
        } else {
            format!("?{}", params.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&"))
        };

        let full_path = if query_string.is_empty() {
            path.to_string()
        } else {
            format!("{}{}", path, query_string)
        };

        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let sign = self.sign_request(&timestamp, "DELETE", &full_path, "");

        let response = self.client
            .delete(format!("{}{}", self.base_url(), full_path))
            .header("OK-ACCESS-KEY", &self.api_key)
            .header("OK-ACCESS-SIGN", sign)
            .header("OK-ACCESS-TIMESTAMP", timestamp)
            .header("OK-ACCESS-PASSPHRASE", &self.passphrase)
            .send()
            .await?;

        let json: Value = response.json().await?;

        if json["code"] != "0" {
            return Err(anyhow!("OKX API error: {}", json["msg"]));
        }

        Ok(json)
    }
}

/// OKX Exchange implementation
pub struct OkxExchange {
    api_key: Option<String>,
    api_secret: Option<String>,
    passphrase: Option<String>,
    is_testnet: bool,
    client: Client,
    ticker_tx: broadcast::Sender<Ticker>,
    kline_tx: broadcast::Sender<Kline>,
    order_tx: broadcast::Sender<Order>,
    connection_state: Arc<RwLock<bool>>,
    ws_task_handles: Arc<Mutex<Vec<tokio::task::JoinHandle<()>>>>,
}

impl OkxExchange {
    pub fn new(
        api_key: Option<String>,
        api_secret: Option<String>,
        passphrase: Option<String>,
    ) -> Self {
        let (ticker_tx, _) = broadcast::channel(1000);
        let (kline_tx, _) = broadcast::channel(1000);
        let (order_tx, _) = broadcast::channel(1000);

        Self {
            api_key,
            api_secret,
            passphrase,
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
    fn rest_client(&self) -> Result<OkxClient> {
        let api_key = self.api_key.clone()
            .ok_or_else(|| anyhow!("API key not configured"))?;
        let api_secret = self.api_secret.clone()
            .ok_or_else(|| anyhow!("API secret not configured"))?;
        let passphrase = self.passphrase.clone()
            .ok_or_else(|| anyhow!("Passphrase not configured"))?;

        Ok(OkxClient::new(api_key, api_secret, passphrase, self.is_testnet))
    }

    /// Convert OKX symbol format (e.g., BTC-USDT to BTCUSDT)
    fn normalize_symbol(&self, symbol: &str) -> String {
        symbol.replace("-", "").to_uppercase()
    }

    /// Convert to OKX symbol format (BTCUSDT to BTC-USDT)
    fn to_okx_symbol(&self, symbol: &str) -> String {
        if symbol.contains("-") {
            symbol.to_uppercase()
        } else {
            // Try to add dash before USDT
            if symbol.ends_with("USDT") {
                format!("{}-USDT", &symbol[..symbol.len() - 4])
            } else if symbol.ends_with("USDC") {
                format!("{}-USDC", &symbol[..symbol.len() - 4])
            } else {
                symbol.to_uppercase()
            }
        }
    }

    /// Convert Interval to OKX bar format
    fn interval_to_okx_bar(&self, interval: Interval) -> &'static str {
        match interval {
            Interval::OneMinute => "1m",
            Interval::FiveMinutes => "5m",
            Interval::FifteenMinutes => "15m",
            Interval::ThirtyMinutes => "30m",
            Interval::OneHour => "1H",
            Interval::FourHours => "4H",
            Interval::OneDay => "1D",
        }
    }

    /// Parse OKX ticker response
    fn parse_ticker(&self, json: &Value, symbol: &str) -> Result<Ticker> {
        let data = &json["data"][0];

        Ok(Ticker {
            symbol: self.normalize_symbol(symbol),
            price: data["last"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            price_change: data["last"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0)
                - data["open24h"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0),
            price_change_percent: data["open24h"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0),
            high_24h: data["high24h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            low_24h: data["low24h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            volume_24h: data["vol24h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            timestamp: chrono::Utc::now().timestamp_millis(),
        })
    }

    /// Parse OKX kline response
    fn parse_kline(&self, item: &Value, symbol: &str, interval: Interval) -> Result<Kline> {
        let arr = item.as_array()
            .ok_or_else(|| anyhow!("Invalid kline data"))?;

        Ok(Kline {
            symbol: self.normalize_symbol(symbol),
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

    /// Convert OrderType to OKX format
    fn order_type_to_okx(order_type: OrderType) -> &'static str {
        match order_type {
            OrderType::Market => "market",
            OrderType::Limit => "limit",
            OrderType::StopLoss => "conditional_market", // OKX uses conditional orders
            OrderType::StopLimit => "conditional_limit",
            OrderType::OCO => "oco",
        }
    }

    /// Convert OrderSide to OKX format
    fn side_to_okx(side: OrderSide) -> &'static str {
        match side {
            OrderSide::Buy => "buy",
            OrderSide::Sell => "sell",
        }
    }

    /// Parse OKX order response
    fn parse_order(&self, json: &Value, request: &OrderRequest) -> Result<Order> {
        let data = &json["data"][0];

        Ok(Order {
            id: data["ordId"].as_str().unwrap_or("").to_string(),
            exchange_order_id: Some(data["ordId"].as_str().unwrap_or("").to_string()),
            client_order_id: data["clOrdId"].as_str().map(|s| s.to_string()),
            symbol: self.normalize_symbol(&request.symbol),
            side: request.side,
            order_type: request.order_type,
            price: request.price,
            quantity: request.quantity,
            filled_quantity: data["fillSz"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            avg_price: data["avgPx"].as_str().and_then(|s| s.parse().ok()),
            status: self.parse_order_state(data["state"].as_str().unwrap_or("live")),
            commission: 0.0,
            created_at: chrono::Utc::now().timestamp_millis(),
            filled_at: None,
        })
    }

    /// Parse OKX order state
    fn parse_order_state(&self, state: &str) -> OrderState {
        match state {
            "live" => OrderState::Open,
            "partially_filled" => OrderState::PartiallyFilled,
            "filled" => OrderState::Filled,
            "canceled" => OrderState::Canceled,
            "mmp" | "failed" => OrderState::Rejected,
            _ => OrderState::Pending,
        }
    }

    /// Start public WebSocket for tickers and klines
    async fn ws_public_loop(&self, channels: Vec<String>, interval: Option<Interval>) -> Result<()> {
        OkxExchange::ws_public_loop_impl(
            &self.ticker_tx,
            &self.kline_tx,
            channels,
            interval,
        ).await
    }

    /// Static helper for public WebSocket loop - ticker only (used in spawned tasks)
    async fn ws_public_loop_helper_ticker(
        channels: Vec<String>,
        tx_ticker: broadcast::Sender<Ticker>,
    ) -> Result<()> {
        let kline_tx = broadcast::channel(1).0; // Not used for ticker-only
        Self::ws_public_loop_impl(&tx_ticker, &kline_tx, channels, None).await
    }

    /// Static helper for public WebSocket loop - kline only (used in spawned tasks)
    async fn ws_public_loop_helper_kline(
        channels: Vec<String>,
        interval: Interval,
        tx_kline: broadcast::Sender<Kline>,
    ) -> Result<()> {
        let ticker_tx = broadcast::channel(1).0; // Not used for kline-only
        Self::ws_public_loop_impl(&ticker_tx, &tx_kline, channels, Some(interval)).await
    }

    /// Static helper for public WebSocket loop with both ticker and kline (used in spawned tasks)
    async fn ws_public_loop_helper(
        channels: Vec<String>,
        interval: Option<Interval>,
        tx_ticker: broadcast::Sender<Ticker>,
    ) -> Result<()> {
        let kline_tx = broadcast::channel(1).0;
        Self::ws_public_loop_impl(&tx_ticker, &kline_tx, channels, interval).await
    }

    /// Implementation of public WebSocket loop
    async fn ws_public_loop_impl(
        ticker_tx: &broadcast::Sender<Ticker>,
        kline_tx: &broadcast::Sender<Kline>,
        channels: Vec<String>,
        interval: Option<Interval>,
    ) -> Result<()> {
        log::info!("Connecting to OKX public WebSocket: {:?}", channels);

        let (ws_stream, _) = tokio_tungstenite::connect_async(WS_API_PUBLIC).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // Subscribe to channels
        for channel in channels {
            let sub_msg = serde_json::json!({
                "op": "subscribe",
                "args": [channel]
            });
            ws_sender.send(Message::Text(sub_msg.to_string())).await?;
            log::info!("Subscribed to OKX channel: {}", channel);
        }

        // Message loop
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(json) = serde_json::from_str::<Value>(&text) {
                        // Handle data messages
                        if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
                            if let Some(arg) = json.get("arg").and_then(|a| a.as_object()) {
                                let channel = arg.get("channel").and_then(|c| c.as_str()).unwrap_or("");

                                match channel {
                                    "tickers" => {
                                        for item in data {
                                            if let Ok(ticker) = Self::parse_ws_ticker_static(item) {
                                                let _ = ticker_tx.send(ticker);
                                            }
                                        }
                                    }
                                    "candle" | "candle1m" | "candle5m" | "candle15m" | "candle30m" | "candle1H" | "candle4H" | "candle1D" => {
                                        if let Some(interval_val) = interval {
                                            for item in data {
                                                if let Ok(kline) = Self::parse_ws_kline_static(item, interval_val) {
                                                    let _ = kline_tx.send(kline);
                                                }
                                            }
                                        }
                                    }
                                    _ => {
                                        log::debug!("Unhandled OKX channel: {}", channel);
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
                    log::info!("OKX WebSocket connection closed");
                    break;
                }
                Ok(Message::Pong(_)) => {}
                Err(e) => {
                    log::error!("OKX WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Start private WebSocket for user data (orders, account)
    async fn ws_private_loop(&self) -> Result<()> {
        OkxExchange::ws_private_loop_impl(
            &self.order_tx,
            self.api_key.clone(),
            self.api_secret.clone(),
            self.passphrase.clone(),
        ).await
    }

    /// Static helper for private WebSocket loop (used in spawned tasks)
    async fn ws_private_loop_helper(
        api_key: &str,
        api_secret: &str,
        passphrase: &str,
        tx_order: broadcast::Sender<Order>,
    ) -> Result<()> {
        OkxExchange::ws_private_loop_impl(
            &tx_order,
            Some(api_key.to_string()),
            Some(api_secret.to_string()),
            Some(passphrase.to_string()),
        ).await
    }

    /// Implementation of private WebSocket loop
    async fn ws_private_loop_impl(
        order_tx: &broadcast::Sender<Order>,
        api_key: Option<String>,
        api_secret: Option<String>,
        passphrase: Option<String>,
    ) -> Result<()> {
        let api_key = api_key.ok_or_else(|| anyhow!("API key not configured"))?;
        let api_secret = api_secret.ok_or_else(|| anyhow!("API secret not configured"))?;
        let passphrase = passphrase.ok_or_else(|| anyhow!("Passphrase not configured"))?;

        log::info!("Connecting to OKX private WebSocket");

        let (ws_stream, _) = tokio_tungstenite::connect_async(WS_API_PRIVATE).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // Create a temporary client for signing
        let temp_client = OkxClient::new(api_key.clone(), api_secret.clone(), passphrase.clone(), false);

        // Login and subscribe
        let timestamp = chrono::Utc::now().timestamp_millis().to_string();
        let sign = temp_client.sign_request(&timestamp, "GET", "/users/self/verify", "");

        let login_msg = serde_json::json!({
            "op": "login",
            "args": [{
                "apiKey": api_key,
                "passphrase": passphrase,
                "timestamp": timestamp,
                "sign": sign
            }]
        });

        ws_sender.send(Message::Text(login_msg.to_string())).await?;

        // Wait for login response
        if let Some(Ok(Message::Text(text))) = ws_receiver.next().await {
            if let Ok(json) = serde_json::from_str::<Value>(&text) {
                if json["event"] == "login" && json["code"] == "0" {
                    log::info!("OKX WebSocket login successful");

                    // Subscribe to orders channel
                    let sub_msg = serde_json::json!({
                        "op": "subscribe",
                        "args": [{"channel": "orders"}]
                    });
                    ws_sender.send(Message::Text(sub_msg.to_string())).await?;
                } else {
                    return Err(anyhow!("OKX WebSocket login failed: {}", json));
                }
            }
        }

        // Message loop
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(json) = serde_json::from_str::<Value>(&text) {
                        if let Some(data) = json.get("data").and_then(|d| d.as_array()) {
                            for item in data {
                                if let Ok(order) = Self::parse_ws_order_from_value(item) {
                                    let _ = order_tx.send(order);
                                }
                            }
                        }
                    }
                }
                Ok(Message::Ping(data)) => {
                    let _ = ws_sender.send(Message::Pong(data)).await;
                }
                Ok(Message::Close(_)) => {
                    log::info!("OKX private WebSocket closed");
                    break;
                }
                Err(e) => {
                    log::error!("OKX private WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Parse WebSocket ticker message
    fn parse_ws_ticker(&self, data: &Value) -> Result<Ticker> {
        Self::parse_ws_ticker_static(data)
    }

    /// Static helper to parse ticker (used in static async functions)
    fn parse_ws_ticker_static(data: &Value) -> Result<Ticker> {
        let inst_id = data["instId"].as_str().unwrap_or("");
        let symbol = Self::normalize_symbol_static(inst_id);

        Ok(Ticker {
            symbol,
            price: data["last"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            price_change: 0.0, // Need to calculate from previous close
            price_change_percent: 0.0,
            high_24h: data["high24h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            low_24h: data["low24h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            volume_24h: data["vol24h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            timestamp: chrono::Utc::now().timestamp_millis(),
        })
    }

    /// Parse WebSocket kline message
    fn parse_ws_kline(&self, data: &Value, interval: Interval) -> Result<Kline> {
        Self::parse_ws_kline_static(data, interval)
    }

    /// Static helper to parse kline (used in static async functions)
    /// OKX WebSocket candle format: [timestamp, open, high, low, close, volume, quote_volume, confirm]
    fn parse_ws_kline_static(data: &Value, interval: Interval) -> Result<Kline> {
        let arr = data.as_array()
            .ok_or_else(|| anyhow!("Invalid kline data"))?;

        // OKX candle WebSocket format: [timestamp, open, high, low, close, volume, ...]
        // Need to get instId from the arg in the message, but we don't have it here
        // For now, use empty symbol - caller should update it
        Ok(Kline {
            symbol: String::new(), // Will be filled by the message handler
            timeframe: interval.as_str().to_string(),
            timestamp: arr.get(0).and_then(|v| v.as_str()).and_then(|s| s.parse().ok()).unwrap_or(0),
            open: arr.get(1).and_then(|v| v.as_str()).unwrap_or("0").parse().unwrap_or(0.0),
            high: arr.get(2).and_then(|v| v.as_str()).unwrap_or("0").parse().unwrap_or(0.0),
            low: arr.get(3).and_then(|v| v.as_str()).unwrap_or("0").parse().unwrap_or(0.0),
            close: arr.get(4).and_then(|v| v.as_str()).unwrap_or("0").parse().unwrap_or(0.0),
            volume: arr.get(5).and_then(|v| v.as_str()).unwrap_or("0").parse().unwrap_or(0.0),
            quote_volume: arr.get(6).and_then(|v| v.as_str()).and_then(|s| s.parse().ok()),
        })
    }

    /// Parse WebSocket order message
    fn parse_ws_order(&self, data: &Value) -> Result<Order> {
        Self::parse_ws_order_from_value(data)
    }

    /// Static helper to parse order from Value (used in static async functions)
    fn parse_ws_order_from_value(data: &Value) -> Result<Order> {
        let inst_id = data["instId"].as_str().unwrap_or("");
        let symbol = Self::normalize_symbol_static(inst_id);

        Ok(Order {
            id: data["ordId"].as_str().unwrap_or("").to_string(),
            exchange_order_id: Some(data["ordId"].as_str().unwrap_or("").to_string()),
            symbol,
            side: match data["side"].as_str().unwrap_or("") {
                "buy" => OrderSide::Buy,
                "sell" => OrderSide::Sell,
                _ => OrderSide::Buy,
            },
            order_type: match data["ordType"].as_str().unwrap_or("") {
                "market" => OrderType::Market,
                "limit" => OrderType::Limit,
                _ => OrderType::Market,
            },
            price: data["px"].as_str().and_then(|s| s.parse().ok()),
            quantity: data["sz"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            filled_quantity: data["fillSz"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            avg_price: data["avgPx"].as_str().and_then(|s| s.parse().ok()),
            status: Self::parse_order_state_static(data["state"].as_str().unwrap_or("")),
            commission: 0.0,
            created_at: data["cTime"].as_i64().unwrap_or(0),
            filled_at: None,
            client_order_id: data["clOrdId"].as_str().map(|s| s.to_string()),
        })
    }

    /// Static helper to normalize symbol (used in static async functions)
    fn normalize_symbol_static(symbol: &str) -> String {
        symbol.replace("-", "").to_uppercase()
    }

    /// Static helper to parse order state (used in static async functions)
    fn parse_order_state_static(state: &str) -> OrderState {
        match state {
            "live" => OrderState::Open,
            "partially_filled" => OrderState::PartiallyFilled,
            "filled" => OrderState::Filled,
            "canceled" => OrderState::Canceled,
            "mmp" | "failed" => OrderState::Rejected,
            _ => OrderState::Pending,
        }
    }
}

#[async_trait]
impl Exchange for OkxExchange {
    fn name(&self) -> ExchangeName {
        ExchangeName::OKX
    }

    fn is_connected(&self) -> bool {
        self.connection_state
            .try_read()
            .map(|guard| *guard)
            .unwrap_or(false)
    }

    async fn connect(&self) -> Result<()> {
        *self.connection_state.write().await = true;
        log::info!("OKX Exchange connected (testnet: {})", self.is_testnet);
        Ok(())
    }

    async fn disconnect(&self) -> Result<()> {
        *self.connection_state.write().await = false;

        // Stop all WebSocket tasks
        let mut handles = self.ws_task_handles.lock().await;
        for handle in handles.drain(..) {
            handle.abort();
        }

        log::info!("OKX Exchange disconnected");
        Ok(())
    }

    async fn get_ticker(&self, symbol: &str) -> Result<Ticker> {
        let okx_symbol = self.to_okx_symbol(symbol);
        let path = format!("/api/v5/market/ticker?instId={}", okx_symbol);

        let response = self.client.get(&format!("{}{}", REST_API_BASE, path))
            .send()
            .await?
            .json::<Value>()
            .await?;

        if response["code"] != "0" {
            return Err(anyhow!("OKX ticker error: {}", response["msg"]));
        }

        self.parse_ticker(&response, symbol)
    }

    async fn get_klines(
        &self,
        symbol: &str,
        interval: Interval,
        limit: usize,
    ) -> Result<Vec<Kline>> {
        let okx_symbol = self.to_okx_symbol(symbol);
        let bar_interval = self.interval_to_okx_bar(interval);

        let path = format!(
            "/api/v5/market/candles?instId={}&bar={}&limit={}",
            okx_symbol, bar_interval, limit
        );

        let response = self.client.get(&format!("{}{}", REST_API_BASE, path))
            .send()
            .await?
            .json::<Value>()
            .await?;

        if response["code"] != "0" {
            return Err(anyhow!("OKX klines error: {}", response["msg"]));
        }

        let data = response["data"].as_array()
            .ok_or_else(|| anyhow!("Invalid klines response"))?;

        data.iter()
            .map(|item| self.parse_kline(item, symbol, interval))
            .collect()
    }

    async fn subscribe_ticker(&self, symbols: Vec<String>) -> Result<()> {
        if symbols.is_empty() {
            return Ok(());
        }

        let okx_symbols: Vec<String> = symbols.iter()
            .map(|s| self.to_okx_symbol(s))
            .collect();

        log::info!("Subscribing to OKX tickers: {:?}", okx_symbols);

        // Build channel subscriptions
        let channels: Vec<String> = okx_symbols.iter()
            .map(|s| format!(r#"{{"channel":"tickers","instId":"{}"}}"#, s))
            .collect();

        // Start WebSocket loop in background task
        let tx_ticker = self.ticker_tx.clone();
        let connection_state = self.connection_state.clone();
        let okx_symbols_clone = okx_symbols.clone();

        let handle = tokio::spawn(async move {
            log::info!("OKX ticker WebSocket task started");

            let mut retry_count = 0;
            const MAX_RETRIES: u32 = 5;

            while *connection_state.read().await {
                if retry_count >= MAX_RETRIES {
                    log::error!("OKX ticker WebSocket max retries reached");
                    break;
                }

                match Self::ws_public_loop_helper_ticker(channels.clone(), tx_ticker.clone()).await {
                    Ok(_) => {
                        retry_count = 0;
                    }
                    Err(e) => {
                        retry_count += 1;
                        log::error!("OKX ticker WebSocket error (retry {}): {}", retry_count, e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }

            log::info!("OKX ticker WebSocket task stopped");
        });

        let mut handles = self.ws_task_handles.lock().await;
        handles.push(handle);

        Ok(())
    }

    async fn subscribe_kline(&self, symbols: Vec<String>, interval: Interval) -> Result<()> {
        if symbols.is_empty() {
            return Ok(());
        }

        let okx_symbols: Vec<String> = symbols.iter()
            .map(|s| self.to_okx_symbol(s))
            .collect();

        let bar_interval = self.interval_to_okx_bar(interval);

        log::info!("Subscribing to OKX klines: {:?} ({})", okx_symbols, bar_interval);

        // Build channel subscriptions for candles
        let channels: Vec<String> = okx_symbols.iter()
            .map(|s| format!(r#"{{"channel":"candle{}","instId":"{}"}}"#, bar_interval, s))
            .collect();

        // Start WebSocket loop in background task
        let tx_kline = self.kline_tx.clone();
        let connection_state = self.connection_state.clone();
        let interval_clone = interval;

        let handle = tokio::spawn(async move {
            log::info!("OKX kline WebSocket task started");

            let mut retry_count = 0;
            const MAX_RETRIES: u32 = 5;

            while *connection_state.read().await {
                if retry_count >= MAX_RETRIES {
                    log::error!("OKX kline WebSocket max retries reached");
                    break;
                }

                match Self::ws_public_loop_helper_kline(channels.clone(), interval_clone, tx_kline.clone()).await {
                    Ok(_) => {
                        retry_count = 0;
                    }
                    Err(e) => {
                        retry_count += 1;
                        log::error!("OKX kline WebSocket error (retry {}): {}", retry_count, e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }

            log::info!("OKX kline WebSocket task stopped");
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
        log::info!("Subscribing to OKX user data stream");

        let tx_order = self.order_tx.clone();
        let connection_state = self.connection_state.clone();
        let api_key = self.api_key.clone();
        let api_secret = self.api_secret.clone();
        let passphrase = self.passphrase.clone();

        if api_key.is_none() || api_secret.is_none() || passphrase.is_none() {
            return Err(anyhow!("API credentials not configured for OKX user data stream"));
        }

        let handle = tokio::spawn(async move {
            log::info!("OKX user data WebSocket task started");

            let mut retry_count = 0;
            const MAX_RETRIES: u32 = 5;

            while *connection_state.read().await {
                if retry_count >= MAX_RETRIES {
                    log::error!("OKX user data WebSocket max retries reached");
                    break;
                }

                match Self::ws_private_loop_helper(
                    api_key.as_ref().unwrap(),
                    api_secret.as_ref().unwrap(),
                    passphrase.as_ref().unwrap(),
                    tx_order.clone(),
                ).await {
                    Ok(_) => {
                        retry_count = 0;
                    }
                    Err(e) => {
                        retry_count += 1;
                        log::error!("OKX user data WebSocket error (retry {}): {}", retry_count, e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                    }
                }
            }

            log::info!("OKX user data WebSocket task stopped");
        });

        let mut handles = self.ws_task_handles.lock().await;
        handles.push(handle);

        Ok(())
    }

    async fn place_order(&self, request: &OrderRequest) -> Result<Order> {
        let client = self.rest_client()?;

        let okx_symbol = self.to_okx_symbol(&request.symbol);
        let td_mode = "cash"; // Trading mode: cash, cross, isolated

        // Build order request body
        let mut body_map = serde_json::Map::new();
        body_map.insert("instId".to_string(), serde_json::json!(okx_symbol));
        body_map.insert("tdMode".to_string(), serde_json::json!(td_mode));
        body_map.insert("side".to_string(), serde_json::json!(Self::side_to_okx(request.side)));
        body_map.insert("ordType".to_string(), serde_json::json!(Self::order_type_to_okx(request.order_type)));
        body_map.insert("sz".to_string(), serde_json::json!(request.quantity.to_string()));

        // Add price for limit orders
        if let Some(price) = request.price {
            body_map.insert("px".to_string(), serde_json::json!(price.to_string()));
        }

        let body_str = serde_json::to_string(&body_map)?;
        let response = client.post_signed("/api/v5/trade/order", &body_str).await?;

        self.parse_order(&response, request)
    }

    async fn cancel_order(&self, order_id: &str) -> Result<()> {
        let client = self.rest_client()?;

        // OKX requires instId to cancel order
        // For now, we'll use a placeholder
        let body = serde_json::json!({
            "ordId": order_id,
        });

        let body_str = serde_json::to_string(&body)?;
        client.post_signed("/api/v5/trade/cancel-order", &body_str).await?;

        Ok(())
    }

    async fn get_order(&self, order_id: &str) -> Result<Order> {
        let client = self.rest_client()?;

        let body = serde_json::json!({
            "ordId": order_id,
        });

        let body_str = serde_json::to_string(&body)?;
        let response = client.post_signed("/api/v5/trade/order", &body_str).await?;

        // Parse response - need proper OrderRequest context
        // For now, return placeholder
        todo!("OKX get_order parsing - need context for symbol/side/price")
    }

    async fn get_open_orders(&self, symbol: Option<&str>) -> Result<Vec<Order>> {
        let client = self.rest_client()?;

        let body = if let Some(sym) = symbol {
            serde_json::json!({
                "instId": self.to_okx_symbol(sym),
            })
        } else {
            serde_json::json!({})
        };

        let body_str = serde_json::to_string(&body)?;
        let response = client.post_signed("/api/v5/trade/orders-pending", &body_str).await?;

        // Parse response
        // For now, return placeholder
        todo!("OKX get_open_orders parsing")
    }

    async fn get_balance(&self) -> Result<Vec<Balance>> {
        let client = self.rest_client()?;

        let body = serde_json::json!({});
        let body_str = serde_json::to_string(&body)?;
        let response = client.get_signed("/api/v5/account/balance").await?;

        let data = response["data"][0]["details"].as_array()
            .ok_or_else(|| anyhow!("Invalid balance response"))?;

        let balances: Vec<Balance> = data.iter()
            .filter_map(|item| {
                let bal = item["bal"].as_str().unwrap_or("0");
                let frozen = item["frozenBal"].as_str().unwrap_or("0");

                let total = bal.parse::<f64>().unwrap_or(0.0)
                    + frozen.parse::<f64>().unwrap_or(0.0);

                if total > 0.0 {
                    Some(Balance {
                        asset: item["ccy"].as_str().unwrap_or("").to_string(),
                        free: bal.parse().unwrap_or(0.0),
                        locked: frozen.parse().unwrap_or(0.0),
                        total,
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(balances)
    }

    async fn get_positions(&self) -> Result<Vec<Position>> {
        let client = self.rest_client()?;

        let body = serde_json::json!({
            "instType": "SPOT",
        });
        let body_str = serde_json::to_string(&body)?;
        let response = client.post_signed("/api/v5/account/positions", &body_str).await?;

        let data = response["data"].as_array()
            .ok_or_else(|| anyhow!("Invalid positions response"))?;

        let positions: Vec<Position> = data.iter()
            .filter_map(|item| {
                let pos = item["pos"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);

                if pos != 0.0 {
                    Some(Position {
                        id: item["posId"].as_str().unwrap_or("").to_string(),
                        symbol: self.normalize_symbol(item["instId"].as_str().unwrap_or("")),
                        side: if item["pos"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0) > 0.0 {
                            "long".to_string()
                        } else {
                            "short".to_string()
                        },
                        quantity: pos.abs(),
                        entry_price: item["avgPx"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        current_price: item["markPx"].as_str().and_then(|s| s.parse().ok()),
                        unrealized_pnl: item["upl"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        realized_pnl: item["realizedPnl"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                        opened_at: item["openTime"].as_str().unwrap_or("0").parse().unwrap_or(0),
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(positions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_okx_symbol_conversion() {
        let exchange = OkxExchange::new(None, None, None);

        // Normalize OKX format to standard
        assert_eq!(exchange.normalize_symbol("BTC-USDT"), "BTCUSDT");
        assert_eq!(exchange.normalize_symbol("ETH-USDT"), "ETHUSDT");

        // Convert standard to OKX format
        assert_eq!(exchange.to_okx_symbol("BTCUSDT"), "BTC-USDT");
        assert_eq!(exchange.to_okx_symbol("ETHUSDT"), "ETH-USDT");
    }

    #[test]
    fn test_interval_conversion() {
        let exchange = OkxExchange::new(None, None, None);

        assert_eq!(exchange.interval_to_okx_bar(Interval::OneMinute), "1m");
        assert_eq!(exchange.interval_to_okx_bar(Interval::OneHour), "1H");
        assert_eq!(exchange.interval_to_okx_bar(Interval::OneDay), "1D");
    }

    #[test]
    fn test_order_type_conversion() {
        assert_eq!(OkxExchange::order_type_to_okx(OrderType::Market), "market");
        assert_eq!(OkxExchange::order_type_to_okx(OrderType::Limit), "limit");
        assert_eq!(OkxExchange::order_type_to_okx(OrderType::StopLoss), "conditional_market");
    }

    #[test]
    fn test_side_conversion() {
        assert_eq!(OkxExchange::side_to_okx(OrderSide::Buy), "buy");
        assert_eq!(OkxExchange::side_to_okx(OrderSide::Sell), "sell");
    }

    #[test]
    fn test_order_state_parsing() {
        let exchange = OkxExchange::new(None, None, None);

        assert_eq!(exchange.parse_order_state("live"), OrderState::Open);
        assert_eq!(exchange.parse_order_state("partially_filled"), OrderState::PartiallyFilled);
        assert_eq!(exchange.parse_order_state("filled"), OrderState::Filled);
        assert_eq!(exchange.parse_order_state("canceled"), OrderState::Canceled);
        assert_eq!(exchange.parse_order_state("failed"), OrderState::Rejected);
    }
}