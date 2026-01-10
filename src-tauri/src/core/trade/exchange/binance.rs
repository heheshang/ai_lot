use super::super::types::*;
use super::r#trait::{Exchange, ExchangeName};
use super::client::BinanceClient;
use async_trait::async_trait;
use anyhow::{anyhow, Result};
use futures_util::{StreamExt, SinkExt};
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex, RwLock};
use tokio_tungstenite::tungstenite::Message;

const REST_API_BASE: &str = "https://api.binance.com";
const WS_API_BASE: &str = "wss://stream.binance.com:9443/ws";

pub struct BinanceExchange {
    api_key: Option<String>,
    api_secret: Option<String>,
    client: Client,
    ticker_tx: broadcast::Sender<Ticker>,
    kline_tx: broadcast::Sender<Kline>,
    order_tx: broadcast::Sender<Order>,
    connection_state: Arc<RwLock<bool>>,
    ws_task_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    listen_key: Arc<Mutex<Option<String>>>,
}

impl BinanceExchange {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        let (ticker_tx, _) = broadcast::channel(1000);
        let (kline_tx, _) = broadcast::channel(1000);
        let (order_tx, _) = broadcast::channel(1000);

        Self {
            api_key,
            api_secret,
            client: Client::new(),
            ticker_tx,
            kline_tx,
            order_tx,
            connection_state: Arc::new(RwLock::new(false)),
            ws_task_handle: Arc::new(Mutex::new(None)),
            listen_key: Arc::new(Mutex::new(None)),
        }
    }

    async fn get(&self, path: &str) -> Result<Value> {
        let url = format!("{}{}", REST_API_BASE, path);
        let response: reqwest::Response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("HTTP error: {}", response.status()));
        }

        let json = response.json().await?;
        Ok(json)
    }

    /// Parse ticker data from Binance WebSocket 24hrTicker event
    ///
    /// # Note
    /// This method is reserved for future WebSocket implementation.
    /// Currently not exposed through the Exchange trait.
    #[allow(dead_code)]
    fn parse_ticker(&self, json: &Value) -> Result<Ticker> {
        Ok(Ticker {
            symbol: json["s"].as_str().unwrap_or("").to_string(),
            price: json["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            price_change: json["p"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            price_change_percent: json["P"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            high_24h: json["h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            low_24h: json["l"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            volume_24h: json["v"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            timestamp: json["E"].as_i64().unwrap_or(0),
        })
    }

    /// Parse kline data from Binance WebSocket kline event
    ///
    /// # Note
    /// This method is reserved for future WebSocket implementation.
    #[allow(dead_code)]
    fn parse_kline(&self, json: &Value, interval: Interval) -> Result<Kline> {
        let k = &json["k"];
        Ok(Kline {
            symbol: json["s"].as_str().unwrap_or("").to_string(),
            timeframe: interval.as_str().to_string(),
            timestamp: k["t"].as_i64().unwrap_or(0),
            open: k["o"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            high: k["h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            low: k["l"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            close: k["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            volume: k["v"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            quote_volume: k["q"].as_str().and_then(|s| s.parse().ok()),
        })
    }

    /// WebSocket message loop for combined streams
    ///
    /// # Note
    /// Reserved for future WebSocket streaming implementation.
    #[allow(dead_code)]
    async fn ws_loop(&self, streams: String, interval: Option<Interval>) -> Result<()> {
        let url = format!("{}/{}", WS_API_BASE, streams);
        log::info!("Connecting to WebSocket: {}", url);

        let (ws_stream, _) = tokio_tungstenite::connect_async(&url).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        log::info!("WebSocket connected for streams: {}", streams);

        // Receive message loop
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(json) = serde_json::from_str::<Value>(&text) {
                        // Check for event type
                        if let Some(event) = json.get("e").and_then(|e| e.as_str()) {
                            match event {
                                "24hrTicker" => {
                                    if let Ok(ticker) = self.parse_ticker(&json) {
                                        let _ = self.ticker_tx.send(ticker);
                                    }
                                }
                                "kline" => {
                                    if let Some(interval) = interval {
                                        if let Ok(kline) = self.parse_kline(&json, interval) {
                                            let _ = self.kline_tx.send(kline);
                                        }
                                    }
                                }
                                _ => {
                                    log::debug!("Unhandled WebSocket event: {}", event);
                                }
                            }
                        }
                    }
                }
                Ok(Message::Ping(data)) => {
                    // Respond to ping with pong
                    if let Err(e) = ws_sender.send(Message::Pong(data)).await {
                        log::error!("Failed to send pong: {}", e);
                        break;
                    }
                }
                Ok(Message::Close(_)) => {
                    log::info!("WebSocket connection closed");
                    break;
                }
                Ok(Message::Pong(_)) => {
                    // Pong received, connection is alive
                }
                Err(e) => {
                    log::error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Start a generic WebSocket connection for a given stream
    async fn start_ws_stream(&self, streams: String, interval: Option<Interval>) -> Result<()> {
        let tx_ticker = self.ticker_tx.clone();
        let tx_kline = self.kline_tx.clone();
        let connection_state = self.connection_state.clone();
        let stream_name = streams.clone();

        let _handle = tokio::spawn(async move {
            log::info!("WebSocket task started for stream: {}", stream_name);

            // Retry logic for WebSocket connection
            let mut retry_count = 0;
            const MAX_RETRIES: u32 = 5;

            loop {
                // Check if we should stop
                let is_connected = *connection_state.read().await;
                if !is_connected {
                    log::info!("WebSocket task stopping (disconnected)");
                    break;
                }

                // Attempt to connect
                let url = format!("{}/{}", WS_API_BASE, stream_name);
                match tokio_tungstenite::connect_async(&url).await {
                    Ok((ws_stream, _)) => {
                        log::info!("WebSocket connected: {}", stream_name);
                        retry_count = 0; // Reset retry count on success

                        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

                        // Message loop
                        while let Some(msg) = ws_receiver.next().await {
                            match msg {
                                Ok(Message::Text(text)) => {
                                    if let Ok(json) = serde_json::from_str::<Value>(&text) {
                                        if let Some(event) = json.get("e").and_then(|e| e.as_str()) {
                                            match event {
                                                "24hrTicker" => {
                                                    if let Ok(ticker) = Self::parse_ticker_static(&json) {
                                                        let _ = tx_ticker.send(ticker);
                                                    }
                                                }
                                                "kline" => {
                                                    if let Some(interval_val) = interval {
                                                        if let Ok(kline) = Self::parse_kline_static(&json, interval_val) {
                                                            let _ = tx_kline.send(kline);
                                                        }
                                                    }
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                                Ok(Message::Ping(data)) => {
                                    let _ = ws_sender.send(Message::Pong(data)).await;
                                }
                                Ok(Message::Close(_)) => {
                                    log::warn!("WebSocket closed, will retry");
                                    break;
                                }
                                Err(e) => {
                                    log::error!("WebSocket error: {}", e);
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("WebSocket connection failed: {}", e);
                        retry_count += 1;

                        if retry_count >= MAX_RETRIES {
                            log::error!("Max retries reached, giving up");
                            break;
                        }

                        // Exponential backoff
                        let delay = tokio::time::Duration::from_secs(2u64.pow(retry_count.min(5)));
                        log::info!("Retrying in {} seconds...", delay.as_secs());
                        tokio::time::sleep(delay).await;
                    }
                }
            }

            log::info!("WebSocket task ended for stream: {}", stream_name);
        });

        Ok(())
    }

    /// Static version of parse_ticker for use in spawned tasks
    fn parse_ticker_static(json: &Value) -> Result<Ticker> {
        Ok(Ticker {
            symbol: json["s"].as_str().unwrap_or("").to_string(),
            price: json["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            price_change: json["p"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            price_change_percent: json["P"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            high_24h: json["h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            low_24h: json["l"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            volume_24h: json["v"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            timestamp: json["E"].as_i64().unwrap_or(0),
        })
    }

    /// Static version of parse_kline for use in spawned tasks
    fn parse_kline_static(json: &Value, interval: Interval) -> Result<Kline> {
        let k = &json["k"];
        Ok(Kline {
            symbol: json["s"].as_str().unwrap_or("").to_string(),
            timeframe: interval.as_str().to_string(),
            timestamp: k["t"].as_i64().unwrap_or(0),
            open: k["o"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            high: k["h"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            low: k["l"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            close: k["c"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            volume: k["v"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            quote_volume: k["q"].as_str().and_then(|s| s.parse().ok()),
        })
    }

    // ========== REST API Helper methods ==========

    /// 创建 Binance REST 客户端
    fn rest_client(&self) -> Result<BinanceClient> {
        let api_key = self.api_key.clone()
            .ok_or_else(|| anyhow!("API key not configured"))?;
        let api_secret = self.api_secret.clone()
            .ok_or_else(|| anyhow!("API secret not configured"))?;

        // TODO: 从配置读取 testnet 设置
        Ok(BinanceClient::new(api_key, api_secret, false))
    }

    /// 解析订单状态
    fn parse_order_state(status: &str) -> OrderState {
        match status {
            "NEW" => OrderState::Open,
            "PARTIALLY_FILLED" => OrderState::PartiallyFilled,
            "FILLED" => OrderState::Filled,
            "CANCELED" => OrderState::Canceled,
            "PENDING_CANCEL" => OrderState::Pending,
            "REJECTED" => OrderState::Rejected,
            "EXPIRED" => OrderState::Canceled,
            _ => OrderState::Pending,
        }
    }

    /// 从 Binance API 响应解析订单
    fn parse_order(&self, json: &Value, request: &OrderRequest) -> Result<Order> {
        Ok(Order {
            id: json["orderId"].as_str().unwrap_or_else(|| json["clientOrderId"].as_str().unwrap_or("")).to_string(),
            exchange_order_id: Some(json["orderId"].as_str().unwrap_or("").to_string()),
            client_order_id: json["clientOrderId"].as_str().map(|s| s.to_string()),
            symbol: json["symbol"].as_str().unwrap_or(&request.symbol).to_string(),
            side: request.side,
            order_type: request.order_type,
            price: request.price,
            quantity: request.quantity,
            filled_quantity: json["executedQty"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            avg_price: json["avgPrice"].as_str().and_then(|s: &str| s.parse().ok())
                .or_else(|| json["cummulativeQuoteQty"].as_str().and_then(|q: &str| {
                    let filled = json["executedQty"].as_str().unwrap_or("0").parse::<f64>().unwrap_or(0.0);
                    let quote = q.parse::<f64>().unwrap_or(0.0);
                    if filled > 0.0 { Some(quote / filled) } else { None }
                })),
            status: Self::parse_order_state(json["status"].as_str().unwrap_or("UNKNOWN")),
            commission: 0.0, // 需要从交易历史获取
            created_at: json["time"].as_i64().unwrap_or(0),
            filled_at: json["updateTime"].as_i64(),
        })
    }

    // ========== User Data Stream methods ==========

    /// 创建用户数据流 listenKey
    async fn create_listen_key(&self) -> Result<String> {
        let client = self.rest_client()?;

        let response = client.post_signed("/api/v3/userDataStream", &[]).await
            .map_err(|e| anyhow!("Failed to create listen key: {}", e))?;

        response["listenKey"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Invalid listen key response"))
    }

    /// 保持 listenKey 活跃（每30分钟调用一次）
    ///
    /// # Note
    /// Reserved for future user data stream implementation.
    #[allow(dead_code)]
    async fn keepalive_listen_key(&self) -> Result<()> {
        let client = self.rest_client()?;
        let guard = self.listen_key.lock().await;
        let listen_key = guard.as_ref()
            .ok_or_else(|| anyhow!("No listen key available"))?;

        let endpoint = format!("/api/v3/userDataStream?listenKey={}", listen_key);
        let _ = client.put_signed(&endpoint, &[]).await
            .map_err(|e| anyhow!("Failed to keepalive listen key: {}", e))?;

        Ok(())
    }

    /// 用户数据流 WebSocket 循环
    ///
    /// # Note
    /// Reserved for future user data stream implementation.
    #[allow(dead_code)]
    async fn user_data_stream_loop(&self) -> Result<()> {
        let listen_key = {
            let guard = self.listen_key.lock().await;
            guard.clone().ok_or_else(|| anyhow!("No listen key"))?
        };

        let url = format!("{}/ws/{}", WS_API_BASE, listen_key);
        log::info!("Connecting to user data stream: {}", url);

        let (ws_stream, _) = tokio_tungstenite::connect_async(&url).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        log::info!("User data stream connected");

        // 启动 keep-alive 任务（定时刷新 listenKey）
        let connection_state = self.connection_state.clone();
        let listen_key_ref = self.listen_key.clone();
        let api_key = self.api_key.clone();
        let api_secret = self.api_secret.clone();

        let keepalive_handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1800)); // 30分钟
            loop {
                interval.tick().await;

                let is_connected = *connection_state.read().await;
                if !is_connected {
                    log::info!("Keep-alive task stopping");
                    break;
                }

                // 保持 listenKey 有效（通过 REST API）
                if let (Some(key), Some(secret), Some(lk)) = (&api_key, &api_secret, listen_key_ref.lock().await.as_ref()) {
                    let client = BinanceClient::new(key.clone(), secret.clone(), false);
                    let endpoint = format!("/api/v3/userDataStream?listenKey={}", lk);
                    if let Err(e) = client.put_signed(&endpoint, &[]).await {
                        log::error!("Failed to keepalive listen key: {}", e);
                    } else {
                        log::debug!("Listen key keepalive sent");
                    }
                }
            }
        });

        // 消息处理循环
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(json) = serde_json::from_str::<Value>(&text) {
                        if let Some(event_type) = json.get("e").and_then(|e| e.as_str()) {
                            match event_type {
                                "executionReport" => {
                                    if let Ok(order) = self.parse_execution_report(&json) {
                                        let _ = self.order_tx.send(order);
                                    }
                                }
                                "account" => {
                                    // 账户更新事件
                                    log::debug!("Account update: {}", json);
                                }
                                "outboundAccountPosition" => {
                                    // 账户持仓更新
                                    log::debug!("Account position update: {}", json);
                                }
                                _ => {
                                    log::debug!("Unhandled user data event: {}", event_type);
                                }
                            }
                        }
                    }
                }
                Ok(Message::Ping(data)) => {
                    let _ = ws_sender.send(Message::Pong(data)).await;
                }
                Ok(Message::Close(_)) => {
                    log::warn!("User data stream connection closed");
                    break;
                }
                Err(e) => {
                    log::error!("User data stream error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        keepalive_handle.abort();
        Ok(())
    }

    /// 解析执行报告事件
    ///
    /// # Note
    /// Reserved for future order event parsing.
    #[allow(dead_code)]
    fn parse_execution_report(&self, json: &Value) -> Result<Order> {
        Self::parse_execution_report_static(json)
    }

    /// 静态版本的 executionReport 解析器（用于 spawned tasks）
    fn parse_execution_report_static(json: &Value) -> Result<Order> {
        // Binance executionReport event field mapping:
        // s = symbol, S = side, o = order type, p = price, q = quantity
        // i = order id, c = client order id, z = filled quantity
        // X = order status, ap = average price, n = commission
        // T = trade time, E = event time

        let side = match json.get("S").and_then(|s| s.as_str()) {
            Some("BUY") => OrderSide::Buy,
            Some("SELL") => OrderSide::Sell,
            _ => return Err(anyhow!("Invalid order side")),
        };

        let order_type = match json.get("o").and_then(|o| o.as_str()) {
            Some("MARKET") => OrderType::Market,
            Some("LIMIT") => OrderType::Limit,
            Some("STOP_LOSS_LIMIT") => OrderType::StopLimit,
            Some("TAKE_PROFIT_LIMIT") => OrderType::StopLimit,
            _ => OrderType::Limit,
        };

        let price = json.get("p").and_then(|p| p.as_f64());
        let quantity = json.get("q").and_then(|q| q.as_f64()).unwrap_or(0.0);
        let filled_quantity = json.get("z").and_then(|z| z.as_f64()).unwrap_or(0.0);

        Ok(Order {
            id: json.get("i").and_then(|i| i.as_i64()).map(|i| i.to_string()).unwrap_or_default(),
            exchange_order_id: json.get("i").and_then(|i| i.as_i64()).map(|i| i.to_string()),
            client_order_id: json.get("c").and_then(|c| c.as_str()).map(|s| s.to_string()),
            symbol: json.get("s").and_then(|s| s.as_str()).unwrap_or("").to_string(),
            side,
            order_type,
            price,
            quantity,
            filled_quantity,
            avg_price: json.get("ap").and_then(|ap| ap.as_f64()),
            status: Self::parse_order_state(json.get("X").and_then(|x| x.as_str()).unwrap_or("NEW")),
            commission: json.get("n").and_then(|n| n.as_f64()).unwrap_or(0.0),
            created_at: json.get("T").and_then(|t| t.as_i64()).unwrap_or(0),
            filled_at: json.get("T").and_then(|t| t.as_i64()),
        })
    }
}

#[async_trait]
impl Exchange for BinanceExchange {
    fn name(&self) -> ExchangeName {
        ExchangeName::Binance
    }

    fn is_connected(&self) -> bool {
        self.connection_state
            .try_read()
            .map(|guard| *guard)
            .unwrap_or(false)
    }

    async fn connect(&self) -> Result<()> {
        *self.connection_state.write().await = true;
        log::info!("BinanceExchange connected");
        Ok(())
    }

    async fn disconnect(&self) -> Result<()> {
        *self.connection_state.write().await = false;

        // Stop WebSocket task if running
        let mut handle_guard = self.ws_task_handle.lock().await;
        if let Some(handle) = handle_guard.take() {
            handle.abort();
            log::info!("WebSocket task stopped");
        }

        log::info!("BinanceExchange disconnected");
        Ok(())
    }

    async fn get_ticker(&self, symbol: &str) -> Result<Ticker> {
        let path = format!("/ticker/24hr?symbol={}", symbol.to_uppercase());
        let json = self.get(&path).await?;

        Ok(Ticker {
            symbol: json["symbol"].as_str().unwrap().to_string(),
            price: json["lastPrice"].as_str().unwrap().parse().unwrap_or(0.0),
            price_change: json["priceChange"].as_str().unwrap().parse().unwrap_or(0.0),
            price_change_percent: json["priceChangePercent"].as_str().unwrap().parse().unwrap_or(0.0),
            high_24h: json["highPrice"].as_str().unwrap().parse().unwrap_or(0.0),
            low_24h: json["lowPrice"].as_str().unwrap().parse().unwrap_or(0.0),
            volume_24h: json["volume"].as_str().unwrap().parse().unwrap_or(0.0),
            timestamp: json["closeTime"].as_i64().unwrap_or(0),
        })
    }

    async fn get_klines(
        &self,
        symbol: &str,
        interval: Interval,
        limit: usize,
    ) -> Result<Vec<Kline>> {
        let path = format!(
            "/klines?symbol={}&interval={}&limit={}",
            symbol.to_uppercase(),
            interval.as_str(),
            limit
        );
        let json = self.get(&path).await?;

        let klines = json
            .as_array()
            .ok_or_else(|| anyhow!("Invalid response"))?
            .iter()
            .map(|item| -> Result<Kline> {
                Ok(Kline {
                    symbol: symbol.to_uppercase(),
                    timeframe: interval.as_str().to_string(),
                    timestamp: item[0].as_i64().unwrap(),
                    open: item[1].as_str().unwrap().parse().unwrap_or(0.0),
                    high: item[2].as_str().unwrap().parse().unwrap_or(0.0),
                    low: item[3].as_str().unwrap().parse().unwrap_or(0.0),
                    close: item[4].as_str().unwrap().parse().unwrap_or(0.0),
                    volume: item[5].as_str().unwrap().parse().unwrap_or(0.0),
                    quote_volume: item[7].as_str().and_then(|s| s.parse().ok()),
                })
            })
            .collect::<Result<Vec<Kline>>>()?;

        Ok(klines)
    }

    async fn subscribe_ticker(&self, symbols: Vec<String>) -> Result<()> {
        if symbols.is_empty() {
            return Ok(());
        }

        // Build subscription streams
        let streams: Vec<String> = symbols
            .iter()
            .map(|s| format!("{}@ticker", s.to_lowercase()))
            .collect();

        let stream = streams.join("/");

        log::info!("Subscribing to ticker streams: {}", stream);

        // Start WebSocket stream
        self.start_ws_stream(stream, None).await?;

        Ok(())
    }

    async fn subscribe_kline(&self, symbols: Vec<String>, interval: Interval) -> Result<()> {
        if symbols.is_empty() {
            return Ok(());
        }

        // Build subscription streams
        let streams: Vec<String> = symbols
            .iter()
            .map(|s| format!("{}@kline_{}", s.to_lowercase(), interval.as_str()))
            .collect();

        let stream = streams.join("/");

        log::info!("Subscribing to kline streams: {}", stream);

        // Start WebSocket stream
        self.start_ws_stream(stream, Some(interval)).await?;

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
        // 如果还没有 listen key，创建一个
        let mut guard = self.listen_key.lock().await;
        if guard.is_none() {
            let listen_key = self.create_listen_key().await?;
            *guard = Some(listen_key);
        }
        drop(guard);

        // 获取 API 密钥用于 keep-alive
        let api_key = self.api_key.clone()
            .ok_or_else(|| anyhow!("API key not configured"))?;
        let api_secret = self.api_secret.clone()
            .ok_or_else(|| anyhow!("API secret not configured"))?;

        // 启动用户数据流循环
        let connection_state = self.connection_state.clone();
        let listen_key_ref = self.listen_key.clone();
        let order_tx = self.order_tx.clone();

        let handle = tokio::spawn(async move {
            // 重新创建 WebSocket 连接需要的组件
            let url = {
                let guard = listen_key_ref.lock().await;
                let listen_key = guard.as_ref().unwrap();
                format!("{}/ws/{}", WS_API_BASE, listen_key)
            };

            log::info!("Starting user data stream: {}", url);

            let (ws_stream, _) = match tokio_tungstenite::connect_async(&url).await {
                Ok(stream) => stream,
                Err(e) => {
                    log::error!("Failed to connect to user data stream: {}", e);
                    return;
                }
            };

            let (mut ws_sender, mut ws_receiver) = ws_stream.split();

            // 启动 keep-alive 任务
            let connection_state_ka = connection_state.clone();
            let listen_key_ref_ka = listen_key_ref.clone();
            let api_key_ka = api_key.clone();
            let api_secret_ka = api_secret.clone();

            let keepalive_handle = tokio::spawn(async move {
                let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1800)); // 30分钟
                loop {
                    interval.tick().await;

                    let is_connected = *connection_state_ka.read().await;
                    if !is_connected {
                        log::info!("Keep-alive task stopping");
                        break;
                    }

                    // 保持 listenKey 有效
                    if let Some(lk) = listen_key_ref_ka.lock().await.as_ref() {
                        let client = BinanceClient::new(api_key_ka.clone(), api_secret_ka.clone(), false);
                        let endpoint = format!("/api/v3/userDataStream?listenKey={}", lk);
                        if let Err(e) = client.put_signed(&endpoint, &[]).await {
                            log::error!("Failed to keepalive listen key: {}", e);
                        } else {
                            log::debug!("Listen key keepalive sent");
                        }
                    }
                }
            });

            // 消息处理循环
            while let Some(msg) = ws_receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Ok(json) = serde_json::from_str::<Value>(&text) {
                            if let Some(event_type) = json.get("e").and_then(|e| e.as_str()) {
                                match event_type {
                                    "executionReport" => {
                                        if let Ok(order) = Self::parse_execution_report_static(&json) {
                                            let _ = order_tx.send(order);
                                        }
                                    }
                                    "account" => {
                                        log::debug!("Account update: {}", json);
                                    }
                                    "outboundAccountPosition" => {
                                        log::debug!("Account position update: {}", json);
                                    }
                                    _ => {
                                        log::debug!("Unhandled user data event: {}", event_type);
                                    }
                                }
                            }
                        }
                    }
                    Ok(Message::Ping(data)) => {
                        let _ = ws_sender.send(Message::Pong(data)).await;
                    }
                    Ok(Message::Close(_)) => {
                        log::warn!("User data stream connection closed");
                        break;
                    }
                    Err(e) => {
                        log::error!("User data stream error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }

            keepalive_handle.abort();
        });

        // 保存 task handle
        let mut ws_handle = self.ws_task_handle.lock().await;
        *ws_handle = Some(handle);

        Ok(())
    }

    // ========== Trading operations ==========

    async fn place_order(&self, request: &OrderRequest) -> Result<Order> {
        let client = self.rest_client()?;

        let symbol = request.symbol.to_uppercase();
        let side = request.side.to_string().to_uppercase();
        let order_type = request.order_type.to_string().to_uppercase();
        let quantity = request.quantity.to_string();
        let price_str = request.price.map(|p| p.to_string());

        let mut params = vec![
            ("symbol", symbol.as_str()),
            ("side", side.as_str()),
            ("type", order_type.as_str()),
            ("quantity", quantity.as_str()),
        ];

        // 添加可选参数
        if let Some(ref price) = price_str {
            params.push(("price", price.as_str()));
        }

        // MARKET 订单不需要 timeInForce
        if request.order_type != OrderType::Market {
            params.push(("timeInForce", "GTC"));
        }

        let response = client.post_signed("/api/v3/order", &params).await
            .map_err(|e| anyhow!("Place order failed: {}", e))?;

        self.parse_order(&response, request)
    }

    async fn cancel_order(&self, order_id: &str) -> Result<()> {
        let client = self.rest_client()?;

        let params = vec![("orderId", order_id)];

        client.delete_signed("/api/v3/order", &params).await
            .map_err(|e| anyhow!("Cancel order failed: {}", e))?;

        Ok(())
    }

    async fn get_order(&self, order_id: &str) -> Result<Order> {
        let client = self.rest_client()?;

        let params = vec![("orderId", order_id)];

        let response = client.get_signed("/api/v3/order", &params).await
            .map_err(|e| anyhow!("Get order failed: {}", e))?;

        let dummy_request = OrderRequest {
            symbol: response["symbol"].as_str().unwrap_or("").to_string(),
            side: match response["side"].as_str().unwrap_or("") {
                "BUY" => OrderSide::Buy,
                "SELL" => OrderSide::Sell,
                _ => OrderSide::Buy,
            },
            order_type: match response["type"].as_str().unwrap_or("") {
                "LIMIT" => OrderType::Limit,
                "MARKET" => OrderType::Market,
                "STOP_LOSS_LIMIT" => OrderType::StopLimit,
                _ => OrderType::Limit,
            },
            price: response["price"].as_str().and_then(|s| s.parse().ok()),
            stop_price: None,
            quantity: response["origQty"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
            client_order_id: None,
            time_in_force: None,
        };

        self.parse_order(&response, &dummy_request)
    }

    async fn get_open_orders(&self, symbol: Option<&str>) -> Result<Vec<Order>> {
        let client = self.rest_client()?;

        let params = if let Some(sym) = symbol {
            vec![("symbol", sym)]
        } else {
            vec![]
        };

        let response = client.get_signed("/api/v3/openOrders", &params).await
            .map_err(|e| anyhow!("Get open orders failed: {}", e))?;

        let orders = response.as_array()
            .ok_or_else(|| anyhow!("Invalid response format"))?
            .iter()
            .map(|item| {
                let dummy_request = OrderRequest {
                    symbol: item["symbol"].as_str().unwrap_or("").to_string(),
                    side: match item["side"].as_str().unwrap_or("") {
                        "BUY" => OrderSide::Buy,
                        "SELL" => OrderSide::Sell,
                        _ => OrderSide::Buy,
                    },
                    order_type: match item["type"].as_str().unwrap_or("") {
                        "LIMIT" => OrderType::Limit,
                        "MARKET" => OrderType::Market,
                        "STOP_LOSS_LIMIT" => OrderType::StopLimit,
                        _ => OrderType::Limit,
                    },
                    price: item["price"].as_str().and_then(|s| s.parse().ok()),
                    stop_price: None,
                    quantity: item["origQty"].as_str().unwrap_or("0").parse().unwrap_or(0.0),
                    client_order_id: None,
                    time_in_force: None,
                };
                self.parse_order(item, &dummy_request)
            })
            .collect::<Result<Vec<Order>>>()?;

        Ok(orders)
    }

    async fn get_balance(&self) -> Result<Vec<Balance>> {
        let client = self.rest_client()?;

        let response = client.get_signed("/api/v3/account", &[]).await
            .map_err(|e| anyhow!("Get balance failed: {}", e))?;

        let balances = response["balances"].as_array()
            .ok_or_else(|| anyhow!("Invalid balances format"))?
            .iter()
            .map(|item| -> Result<Balance> {
                let free = item["free"].as_str().unwrap_or("0").parse().unwrap_or(0.0);
                let locked = item["locked"].as_str().unwrap_or("0").parse().unwrap_or(0.0);

                if (free + locked) > 0.0 {
                    Ok(Balance {
                        asset: item["asset"].as_str().unwrap_or("").to_string(),
                        free,
                        locked,
                        total: free + locked,
                    })
                } else {
                    Err(anyhow!("Skip zero balance"))
                }
            })
            .filter_map(|r: Result<Balance>| r.ok())
            .collect();

        Ok(balances)
    }

    async fn get_positions(&self) -> Result<Vec<Position>> {
        // Binance SPOT 没有持仓概念
        Ok(Vec::new())
    }
}
