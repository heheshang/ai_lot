use crate::core::trade::exchange::{Exchange, ExchangeName, binance::BinanceExchange};
use crate::core::trade::types::*;
use crate::core::trade::converter::{MarketDataConverter, ConverterFactory};
use crate::core::event::EventBus;
use crate::infrastructure::Database;
use crate::infrastructure::cache::{get_klines, insert_klines};
use anyhow::{anyhow, Result};
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use std::time::Instant;

pub struct MarketService {
    exchanges: Arc<RwLock<Vec<Arc<dyn Exchange>>>>,
    event_bus: Arc<EventBus>,
    db: Database,
    ws_handles: Arc<RwLock<Vec<JoinHandle<()>>>>,
}

impl MarketService {
    pub fn new(db: Database) -> Self {
        Self {
            exchanges: Arc::new(RwLock::new(Vec::new())),
            event_bus: Arc::new(EventBus::new()),
            db,
            ws_handles: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get the event bus for this market service
    pub fn event_bus(&self) -> Arc<EventBus> {
        self.event_bus.clone()
    }

    /// Add an exchange to the service
    pub async fn add_exchange(&self, exchange: Arc<dyn Exchange>) {
        let mut exchanges = self.exchanges.write().await;
        exchanges.push(exchange);
    }

    /// Remove an exchange by name
    pub async fn remove_exchange(&self, name: ExchangeName) -> Result<()> {
        let mut exchanges = self.exchanges.write().await;
        let initial_len = exchanges.len();
        exchanges.retain(|e| e.name() != name);

        if exchanges.len() == initial_len {
            return Err(anyhow!("Exchange not found: {:?}", name));
        }

        Ok(())
    }

    /// Get an exchange by name
    pub async fn get_exchange(&self, name: ExchangeName) -> Option<Arc<dyn Exchange>> {
        let exchanges = self.exchanges.read().await;
        exchanges.iter().find(|e| e.name() == name).cloned()
    }

    /// Get list of all registered exchanges
    pub async fn list_exchanges(&self) -> Vec<ExchangeName> {
        let exchanges = self.exchanges.read().await;
        exchanges.iter().map(|e| e.name()).collect()
    }

    /// Initialize with default Binance exchange
    pub async fn init_binance(&self, api_key: Option<String>, api_secret: Option<String>) -> Result<()> {
        let binance = Arc::new(BinanceExchange::new(api_key, api_secret));

        // Connect to the exchange
        binance.connect().await?;

        // Add to exchanges list
        self.add_exchange(binance).await;

        log::info!("Binance exchange initialized");
        Ok(())
    }

    /// Subscribe to ticker updates for given symbols
    pub async fn subscribe_ticker(&self, symbols: Vec<String>) -> Result<()> {
        // Subscribe to all connected exchanges
        let exchanges = self.exchanges.read().await;

        if exchanges.is_empty() {
            return Err(anyhow!("No exchanges available. Call init_binance() first."));
        }

        // Start event forwarding for Binance (if not already started)
        if let Some(exchange) = exchanges.first() {
            if exchange.name() == ExchangeName::Binance {
                // This will spawn forwarding tasks if they don't exist
                let _ = self.start_event_forwarding(ExchangeName::Binance).await;
            }
        }

        for exchange in exchanges.iter() {
            exchange.subscribe_ticker(symbols.clone()).await?;
        }

        log::info!("Subscribed to tickers: {:?}", symbols);
        Ok(())
    }

    /// Subscribe to kline updates for given symbols and interval
    pub async fn subscribe_kline(&self, symbols: Vec<String>, interval: Interval) -> Result<()> {
        let exchanges = self.exchanges.read().await;

        if exchanges.is_empty() {
            return Err(anyhow!("No exchanges available. Call init_binance() first."));
        }

        // Start event forwarding for Binance (if not already started)
        if let Some(exchange) = exchanges.first() {
            if exchange.name() == ExchangeName::Binance {
                // This will spawn forwarding tasks if they don't exist
                let _ = self.start_event_forwarding(ExchangeName::Binance).await;
            }
        }

        for exchange in exchanges.iter() {
            exchange.subscribe_kline(symbols.clone(), interval).await?;
        }

        log::info!("Subscribed to klines: {:?} {:?}", symbols, interval);
        Ok(())
    }

    /// Get K-line data from exchange (with cache support - P6-05)
    pub async fn get_klines(
        &self,
        symbol: &str,
        interval: &str,
        limit: usize,
    ) -> Result<Vec<Kline>> {
        let start_time = Instant::now();

        // Try to get from in-memory cache first
        let exchange_name = "binance";
        if let Some(cached) = get_klines(exchange_name, symbol, interval).await {
            log::debug!("Klines cache HIT: {} {} ({} items)", symbol, interval, cached.len());
            let elapsed = start_time.elapsed().as_millis();
            log::info!("get_klines (cached): {}ms", elapsed);

            // Return cached data if it has enough items
            if cached.len() >= limit {
                return Ok(cached.into_iter().take(limit).collect());
            }
        }

        log::debug!("Klines cache MISS: {} {}", symbol, interval);

        // Cache miss - fetch from exchange
        let exchange = self.get_exchange(ExchangeName::Binance)
            .await
            .ok_or_else(|| anyhow!("Exchange not found"))?;

        let interval_enum = Interval::parse(interval)
            .ok_or_else(|| anyhow!("Invalid interval: {}", interval))?;

        let klines = exchange.get_klines(symbol, interval_enum, limit).await?;

        // Cache the results
        insert_klines(exchange_name, symbol, interval, klines.clone()).await;

        // Also cache to database for persistence
        if let Err(e) = self.save_klines(&klines).await {
            log::warn!("Failed to cache klines to database: {}", e);
        }

        let elapsed = start_time.elapsed().as_millis();
        log::info!("get_klines (exchange): {}ms", elapsed);

        Ok(klines)
    }

    /// Save K-lines to database
    pub async fn save_klines(&self, klines: &[Kline]) -> Result<()> {
        for kline in klines {
            sqlx::query(
                r#"
                INSERT OR REPLACE INTO klines
                (exchange_name, symbol, timeframe, timestamp, open, high, low, close, volume)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#
            )
            .bind("binance")
            .bind(&kline.symbol)
            .bind(&kline.timeframe)
            .bind(kline.timestamp)
            .bind(kline.open)
            .bind(kline.high)
            .bind(kline.low)
            .bind(kline.close)
            .bind(kline.volume)
            .execute(&self.db.pool)
            .await?;
        }

        log::debug!("Saved {} klines to database", klines.len());
        Ok(())
    }

    /// Get cached klines from database
    pub async fn get_cached_klines(
        &self,
        symbol: &str,
        timeframe: &str,
        limit: usize,
    ) -> Result<Vec<Kline>> {
        let rows = sqlx::query(
            r#"
            SELECT symbol, timeframe, timestamp, open, high, low, close, volume
            FROM klines
            WHERE symbol = ? AND timeframe = ?
            ORDER BY timestamp DESC
            LIMIT ?
            "#
        )
        .bind(symbol)
        .bind(timeframe)
        .bind(limit as i64)
        .fetch_all(&self.db.pool)
        .await?;

        let klines = rows.into_iter().map(|row| {
            Kline {
                symbol: row.get("symbol"),
                timeframe: row.get("timeframe"),
                timestamp: row.get("timestamp"),
                open: row.get("open"),
                high: row.get("high"),
                low: row.get("low"),
                close: row.get("close"),
                volume: row.get("volume"),
                quote_volume: None,
            }
        }).collect();

        Ok(klines)
    }

    /// Start event forwarding from exchange to event bus
    pub async fn start_event_forwarding(&self, exchange_name: ExchangeName) -> Result<()> {
        let exchange = self.get_exchange(exchange_name)
            .await
            .ok_or_else(|| anyhow!("Exchange not found"))?;

        let event_bus_ticker = self.event_bus.clone();
        let event_bus_kline = self.event_bus.clone();
        let mut ticker_rx = exchange.ticker_stream();
        let mut kline_rx = exchange.kline_stream();

        // Spawn ticker forwarding task
        let ticker_handle = tokio::spawn(async move {
            while let Ok(ticker) = ticker_rx.recv().await {
                event_bus_ticker.publish_ticker(ticker);
            }
            log::info!("Ticker forwarding task ended");
        });

        // Spawn kline forwarding task
        let kline_handle = tokio::spawn(async move {
            while let Ok(kline) = kline_rx.recv().await {
                event_bus_kline.publish_kline(kline);
            }
            log::info!("Kline forwarding task ended");
        });

        // Store handles for cleanup
        let mut handles = self.ws_handles.write().await;
        handles.push(ticker_handle);
        handles.push(kline_handle);

        log::info!("Event forwarding started for {:?}", exchange_name);
        Ok(())
    }

    /// Stop all event forwarding tasks
    pub async fn stop_event_forwarding(&self) -> Result<()> {
        let mut handles = self.ws_handles.write().await;

        for handle in handles.drain(..) {
            handle.abort();
        }

        log::info!("Event forwarding stopped");
        Ok(())
    }

    /// Shutdown the market service
    pub async fn shutdown(&self) -> Result<()> {
        // Stop event forwarding
        self.stop_event_forwarding().await?;

        // Disconnect all exchanges
        let exchanges = self.exchanges.read().await;
        for exchange in exchanges.iter() {
            let _ = exchange.disconnect().await;
        }

        log::info!("MarketService shutdown complete");
        Ok(())
    }

    /// Get the converter for a specific exchange
    pub fn get_converter(&self, exchange_name: ExchangeName) -> Box<dyn MarketDataConverter> {
        ConverterFactory::create(exchange_name)
    }

    /// Normalize symbol to internal format using the exchange's converter
    pub fn normalize_symbol(&self, symbol: &str, exchange_name: ExchangeName) -> String {
        let converter = self.get_converter(exchange_name);
        converter.normalize_symbol(symbol)
    }

    /// Denormalize symbol from internal format to exchange format
    pub fn denormalize_symbol(&self, symbol: &str, exchange_name: ExchangeName) -> String {
        let converter = self.get_converter(exchange_name);
        converter.denormalize_symbol(symbol)
    }

    /// Convert raw ticker data using the appropriate converter
    pub fn convert_ticker(
        &self,
        raw: &serde_json::Value,
        exchange_name: ExchangeName,
    ) -> Result<Ticker, crate::core::trade::converter::ConversionError> {
        let converter = self.get_converter(exchange_name);
        converter.convert_ticker(raw)
    }

    /// Convert raw kline data using the appropriate converter
    pub fn convert_kline(
        &self,
        raw: &serde_json::Value,
        interval: Interval,
        exchange_name: ExchangeName,
    ) -> Result<Kline, crate::core::trade::converter::ConversionError> {
        let converter = self.get_converter(exchange_name);
        converter.convert_kline(raw, interval)
    }

    /// Convert raw order data using the appropriate converter
    pub fn convert_order(
        &self,
        raw: &serde_json::Value,
        exchange_name: ExchangeName,
    ) -> Result<Order, crate::core::trade::converter::ConversionError> {
        let converter = self.get_converter(exchange_name);
        converter.convert_order(raw)
    }

    /// Convert raw balance data using the appropriate converter
    pub fn convert_balance(
        &self,
        raw: &serde_json::Value,
        exchange_name: ExchangeName,
    ) -> Result<Vec<Balance>, crate::core::trade::converter::ConversionError> {
        let converter = self.get_converter(exchange_name);
        converter.convert_balance(raw)
    }

    /// Convert raw position data using the appropriate converter
    pub fn convert_position(
        &self,
        raw: &serde_json::Value,
        exchange_name: ExchangeName,
    ) -> Result<Vec<Position>, crate::core::trade::converter::ConversionError> {
        let converter = self.get_converter(exchange_name);
        converter.convert_position(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_parsing() {
        // Test Interval parsing
        assert_eq!(Interval::from_str("1m"), Some(Interval::OneMinute));
        assert_eq!(Interval::from_str("1h"), Some(Interval::OneHour));
        assert_eq!(Interval::from_str("1d"), Some(Interval::OneDay));
        assert_eq!(Interval::from_str("invalid"), None);
    }
}
