//! Data Quality Monitoring Service
//!
//! Monitors the quality of market data including connection status,
//! latency, message frequency, and data integrity.

use crate::core::trade::types::Kline;
use crate::infrastructure::Database;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use chrono::Utc;

/// Data quality status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataQualityStatus {
    Good,
    Degraded,
    Poor,
    Disconnected,
}

/// Connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Reconnecting,
}

/// Data quality metrics for a symbol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityMetrics {
    /// Symbol
    pub symbol: String,
    /// Connection status
    #[serde(rename = "connectionStatus")]
    pub connection_status: ConnectionStatus,
    /// Last message timestamp
    #[serde(rename = "lastMessageTime")]
    pub last_message_time: i64,
    /// Average latency in milliseconds
    #[serde(rename = "avgLatencyMs")]
    pub avg_latency_ms: f64,
    /// Maximum latency in milliseconds
    #[serde(rename = "maxLatencyMs")]
    pub max_latency_ms: f64,
    /// Message rate (messages per second)
    #[serde(rename = "messageRate")]
    pub message_rate: f64,
    /// Number of detected data gaps
    #[serde(rename = "gapCount")]
    pub gap_count: usize,
    /// Number of stale data points
    #[serde(rename = "staleDataCount")]
    pub stale_data_count: usize,
    /// Number of duplicate data points
    #[serde(rename = "duplicateCount")]
    pub duplicate_count: usize,
    /// Total messages received
    #[serde(rename = "totalMessages")]
    pub total_messages: u64,
    /// Total errors encountered
    #[serde(rename = "errorCount")]
    pub error_count: u64,
    /// Overall quality score (0-100)
    #[serde(rename = "qualityScore")]
    pub quality_score: f64,
    /// Quality status
    pub status: DataQualityStatus,
}

/// Data quality configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQualityConfig {
    /// Maximum acceptable latency (ms)
    #[serde(rename = "maxLatencyMs")]
    pub max_latency_ms: f64,
    /// Minimum message rate (messages/sec)
    #[serde(rename = "minMessageRate")]
    pub min_message_rate: f64,
    /// Maximum gap duration (seconds) before considering it a gap
    #[serde(rename = "maxGapSeconds")]
    pub max_gap_seconds: u64,
    /// Stale data threshold (seconds)
    #[serde(rename = "staleThresholdSeconds")]
    pub stale_threshold_seconds: u64,
    /// Window size for calculating statistics
    #[serde(rename = "statsWindowSize")]
    pub stats_window_size: usize,
}

impl Default for DataQualityConfig {
    fn default() -> Self {
        Self {
            max_latency_ms: 1000.0,
            min_message_rate: 0.1,
            max_gap_seconds: 60,
            stale_threshold_seconds: 300,
            stats_window_size: 1000,
        }
    }
}

/// Data quality monitoring service
pub struct  DataQualityMonitor {
    db: Database,
    config: Arc<RwLock<DataQualityConfig>>,
    // Track metrics per symbol
    metrics: Arc<RwLock<HashMap<String, SymbolMetrics>>>,
}

/// Internal metrics tracking for a symbol
struct SymbolMetrics {
    connection_status: ConnectionStatus,
    last_message_time: Option<i64>,
    latencies: VecDeque<f64>,
    message_timestamps: VecDeque<i64>,
    last_kline: Option<Kline>,
    gap_count: usize,
    stale_data_count: usize,
    duplicate_count: usize,
    total_messages: u64,
    error_count: u64,
    last_update: Instant,
}

impl SymbolMetrics {
    fn new() -> Self {
        Self {
            connection_status: ConnectionStatus::Disconnected,
            last_message_time: None,
            latencies: VecDeque::with_capacity(1000),
            message_timestamps: VecDeque::with_capacity(1000),
            last_kline: None,
            gap_count: 0,
            stale_data_count: 0,
            duplicate_count: 0,
            total_messages: 0,
            error_count: 0,
            last_update: Instant::now(),
        }
    }
}

impl DataQualityMonitor {
    /// Create a new data quality monitor
    pub fn new(db: Database, config: DataQualityConfig) -> Self {
        Self {
            db,
            config: Arc::new(RwLock::new(config)),
            metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create with default configuration
    pub fn with_defaults(db: Database) -> Self {
        Self::new(db, DataQualityConfig::default())
    }

    /// Record a received message
    pub async fn record_message(&self, symbol: String, latency_ms: f64) {
        let mut metrics = self.metrics.write().await;
        let symbol_metrics = metrics.entry(symbol.clone()).or_insert_with(SymbolMetrics::new);

        let now = Utc::now().timestamp_millis();

        symbol_metrics.connection_status = ConnectionStatus::Connected;
        symbol_metrics.last_message_time = Some(now);
        symbol_metrics.latencies.push_back(latency_ms);
        symbol_metrics.message_timestamps.push_back(now);
        symbol_metrics.total_messages += 1;
        symbol_metrics.last_update = Instant::now();

        // Keep window size limited
        if symbol_metrics.latencies.len() > 1000 {
            symbol_metrics.latencies.pop_front();
        }
        if symbol_metrics.message_timestamps.len() > 1000 {
            symbol_metrics.message_timestamps.pop_front();
        }
    }

    /// Record a received kline
    pub async fn record_kline(&self, kline: &Kline) {
        let mut metrics = self.metrics.write().await;
        let symbol_metrics = metrics.entry(kline.symbol.clone()).or_insert_with(SymbolMetrics::new);

        let config = self.config.read().await;

        // Check for gaps
        if let Some(ref last_kline) = symbol_metrics.last_kline {
            let expected_interval = self.parse_timeframe(&kline.timeframe);
            if let Some(expected) = expected_interval {
                let actual_gap = (kline.timestamp - last_kline.timestamp) / 1000;

                if actual_gap > (expected as i64 * 2) {
                    symbol_metrics.gap_count += 1;
                    log::warn!(
                        "Data gap detected for {}: expected {}s, got {}s",
                        kline.symbol, expected, actual_gap
                    );
                }

                // Check for duplicates (same timestamp)
                if kline.timestamp == last_kline.timestamp {
                    symbol_metrics.duplicate_count += 1;
                }
            }
        }

        // Check for stale data
        if let Some(last_time) = symbol_metrics.last_message_time {
            let age = (Utc::now().timestamp_millis() - last_time) / 1000;
            if age > config.stale_threshold_seconds as i64 {
                symbol_metrics.stale_data_count += 1;
            }
        }

        symbol_metrics.last_kline = Some(kline.clone());
        symbol_metrics.last_update = Instant::now();
    }

    /// Record an error
    pub async fn record_error(&self, symbol: String) {
        let mut metrics = self.metrics.write().await;
        if let Some(symbol_metrics) = metrics.get_mut(&symbol) {
            symbol_metrics.error_count += 1;
        }
    }

    /// Mark symbol as disconnected
    pub async fn mark_disconnected(&self, symbol: String) {
        let mut metrics = self.metrics.write().await;
        if let Some(symbol_metrics) = metrics.get_mut(&symbol) {
            symbol_metrics.connection_status = ConnectionStatus::Disconnected;
        }
    }

    /// Mark symbol as reconnecting
    pub async fn mark_reconnecting(&self, symbol: String) {
        let mut metrics = self.metrics.write().await;
        if let Some(symbol_metrics) = metrics.get_mut(&symbol) {
            symbol_metrics.connection_status = ConnectionStatus::Reconnecting;
        }
    }

    /// Get quality metrics for a symbol
    pub async fn get_metrics(&self, symbol: &str) -> Option<DataQualityMetrics> {
        let metrics = self.metrics.read().await;
        let symbol_metrics = metrics.get(symbol)?;

        let avg_latency = if !symbol_metrics.latencies.is_empty() {
            symbol_metrics.latencies.iter().sum::<f64>() / symbol_metrics.latencies.len() as f64
        } else {
            0.0
        };

        let max_latency = symbol_metrics.latencies.iter().cloned().fold(0.0_f64, f64::max);

        // Calculate message rate
        let message_rate = if symbol_metrics.message_timestamps.len() > 1 {
            let time_span = (symbol_metrics.message_timestamps.back().unwrap()
                - symbol_metrics.message_timestamps.front().unwrap()) as f64 / 1000.0;
            if time_span > 0.0 {
                symbol_metrics.message_timestamps.len() as f64 / time_span
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Calculate quality score
        let quality_score = self.calculate_quality_score(
            avg_latency,
            max_latency,
            message_rate,
            symbol_metrics.gap_count,
            symbol_metrics.stale_data_count,
            symbol_metrics.error_count,
        ).await;

        // Determine status
        let status = if quality_score >= 80.0 {
            DataQualityStatus::Good
        } else if quality_score >= 50.0 {
            DataQualityStatus::Degraded
        } else if quality_score >= 20.0 {
            DataQualityStatus::Poor
        } else {
            DataQualityStatus::Disconnected
        };

        Some(DataQualityMetrics {
            symbol: symbol.to_string(),
            connection_status: symbol_metrics.connection_status,
            last_message_time: symbol_metrics.last_message_time.unwrap_or(0),
            avg_latency_ms: avg_latency,
            max_latency_ms: max_latency,
            message_rate,
            gap_count: symbol_metrics.gap_count,
            stale_data_count: symbol_metrics.stale_data_count,
            duplicate_count: symbol_metrics.duplicate_count,
            total_messages: symbol_metrics.total_messages,
            error_count: symbol_metrics.error_count,
            quality_score,
            status,
        })
    }

    /// Get all metrics
    pub async fn get_all_metrics(&self) -> Vec<DataQualityMetrics> {
        let metrics = self.metrics.read().await;
        let mut result = Vec::new();

        for symbol in metrics.keys() {
            if let Some(m) = self.get_metrics(symbol).await {
                result.push(m);
            }
        }

        result
    }

    /// Get symbols with poor data quality
    pub async fn get_poor_quality_symbols(&self) -> Vec<String> {
        let all_metrics = self.get_all_metrics().await;
        all_metrics
            .into_iter()
            .filter(|m| m.status == DataQualityStatus::Poor || m.status == DataQualityStatus::Disconnected)
            .map(|m| m.symbol)
            .collect()
    }

    /// Reset metrics for a symbol
    pub async fn reset_metrics(&self, symbol: &str) {
        let mut metrics = self.metrics.write().await;
        metrics.insert(symbol.to_string(), SymbolMetrics::new());
    }

    /// Update configuration
    pub async fn update_config(&self, config: DataQualityConfig) {
        let mut cfg = self.config.write().await;
        *cfg = config;
    }

    /// Calculate quality score (0-100)
    async fn calculate_quality_score(
        &self,
        avg_latency: f64,
        max_latency: f64,
        message_rate: f64,
        gap_count: usize,
        stale_count: usize,
        error_count: u64,
    ) -> f64 {
        let config = self.config.read().await;

        let mut score = 100.0;

        // Latency score (0-25 points)
        if avg_latency > config.max_latency_ms {
            score -= 25.0 * (avg_latency / config.max_latency_ms).min(2.0);
        }

        // Message rate score (0-25 points)
        if message_rate < config.min_message_rate {
            score -= 25.0 * ((config.min_message_rate - message_rate) / config.min_message_rate).min(1.0);
        }

        // Gap penalty (0-25 points)
        let gap_penalty = (gap_count as f64 * 5.0).min(25.0);
        score -= gap_penalty;

        // Error penalty (0-25 points)
        let error_penalty = (error_count as f64 * 2.0).min(25.0);
        score -= error_penalty;

        score.max(0.0)
    }

    /// Parse timeframe string to seconds
    fn parse_timeframe(&self, timeframe: &str) -> Option<u64> {
        match timeframe {
            "1m" => Some(60),
            "5m" => Some(300),
            "15m" => Some(900),
            "30m" => Some(1800),
            "1h" => Some(3600),
            "4h" => Some(14400),
            "1d" => Some(86400),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_quality_score_calculation() {
        let monitor = DataQualityMonitor::with_defaults(
            // Would need a real Database instance
        );

        // Perfect score
        let score = monitor.calculate_quality_score(100.0, 200.0, 10.0, 0, 0, 0).await;
        assert_eq!(score, 100.0);

        // Poor latency
        let score = monitor.calculate_quality_score(5000.0, 5000.0, 10.0, 0, 0, 0).await;
        assert!(score < 100.0);

        // Gaps
        let score = monitor.calculate_quality_score(100.0, 200.0, 10.0, 5, 0, 0).await;
        assert!(score < 100.0);
    }

    #[test]
    fn test_timeframe_parsing() {
        let monitor = DataQualityMonitor::with_defaults(
            // Would need a real Database instance
        );

        assert_eq!(monitor.parse_timeframe("1m"), Some(60));
        assert_eq!(monitor.parse_timeframe("1h"), Some(3600));
        assert_eq!(monitor.parse_timeframe("1d"), Some(86400));
        assert_eq!(monitor.parse_timeframe("invalid"), None);
    }
}
