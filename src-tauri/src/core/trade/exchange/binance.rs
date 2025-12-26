use super::super::types::*;
use super::r#trait::{Exchange, ExchangeName};
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
    connection_state: Arc<RwLock<bool>>,
    ws_task_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

impl BinanceExchange {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        let (ticker_tx, _) = broadcast::channel(1000);
        let (kline_tx, _) = broadcast::channel(1000);

        Self {
            api_key,
            api_secret,
            client: Client::new(),
            ticker_tx,
            kline_tx,
            connection_state: Arc::new(RwLock::new(false)),
            ws_task_handle: Arc::new(Mutex::new(None)),
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
            quote_volume: k["q"].as_str().map(|s| s.parse().ok()).flatten(),
        })
    }

    /// WebSocket message loop for combined streams
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

        let handle = tokio::spawn(async move {
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
            quote_volume: k["q"].as_str().map(|s| s.parse().ok()).flatten(),
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
                    quote_volume: item[7].as_str().map(|s| s.parse().ok()).flatten(),
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
}
