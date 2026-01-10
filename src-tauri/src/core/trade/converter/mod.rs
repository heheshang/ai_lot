//! Market Data Converter Module
//!
//! Provides a unified interface for converting market data from different exchanges
//! into a standardized format, abstracting away the differences between exchange APIs.

pub mod binance;
pub mod okx;
pub mod bybit;

use serde_json::Value;
use crate::core::trade::types::*;
use crate::core::trade::exchange::ExchangeName;

pub use binance::BinanceConverter;
pub use okx::OkxConverter;
pub use bybit::BybitConverter;

/// Unified Market Data Converter trait
///
/// This trait defines how to convert exchange-specific raw data into the unified
/// internal format used by AI-LOT. Each exchange implements its own converter.
pub trait MarketDataConverter: Send + Sync {
    /// Get the exchange name this converter handles
    fn exchange_name(&self) -> ExchangeName;

    /// Convert ticker from exchange format to unified format
    fn convert_ticker(&self, raw: &Value) -> Result<Ticker, ConversionError>;

    /// Convert kline from exchange format to unified format
    fn convert_kline(&self, raw: &Value, interval: Interval) -> Result<Kline, ConversionError>;

    /// Convert order from exchange format to unified format
    fn convert_order(&self, raw: &Value) -> Result<Order, ConversionError>;

    /// Convert balance from exchange format to unified format
    fn convert_balance(&self, raw: &Value) -> Result<Vec<Balance>, ConversionError>;

    /// Convert position from exchange format to unified format
    fn convert_position(&self, raw: &Value) -> Result<Vec<Position>, ConversionError>;

    /// Normalize symbol format (e.g., BTC/USDT -> BTCUSDT)
    fn normalize_symbol(&self, symbol: &str) -> String;

    /// Denormalize symbol format (e.g., BTCUSDT -> BTC/USDT)
    fn denormalize_symbol(&self, symbol: &str) -> String;

    /// Get the symbol format used by this exchange
    fn symbol_format(&self) -> SymbolFormat;
}

/// Symbol format used by exchanges
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolFormat {
    /// No separator (BTCUSDT)
    NoSeparator,
    /// Slash separator (BTC/USDT)
    Slash,
    /// Dash separator (BTC-USDT)
    Dash,
    /// Underscore separator (BTC_USDT)
    Underscore,
}

/// Conversion error type
#[derive(Debug, Clone, thiserror::Error)]
pub enum ConversionError {
    #[error("Missing required field: {field}")]
    MissingField { field: String },

    #[error("Invalid field value: {field} - {value}")]
    InvalidValue { field: String, value: String },

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Unsupported data format")]
    UnsupportedFormat,

    #[error("JSON error: {0}")]
    JsonError(String),
}

impl From<serde_json::Error> for ConversionError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonError(err.to_string())
    }
}

/// Factory for creating converters
pub struct ConverterFactory;

impl ConverterFactory {
    /// Create a converter for the specified exchange
    pub fn create(exchange_name: ExchangeName) -> Box<dyn MarketDataConverter> {
        match exchange_name {
            ExchangeName::Binance => Box::new(BinanceConverter::new()),
            ExchangeName::OKX => Box::new(OkxConverter::new()),
            ExchangeName::Bybit => Box::new(BybitConverter::new()),
        }
    }

    /// Create converter from string name
    pub fn from_name(name: &str) -> Result<Box<dyn MarketDataConverter>, String> {
        let exchange = ExchangeName::parse(name)
            .ok_or_else(|| format!("Unknown exchange: {}", name))?;

        Ok(Self::create(exchange))
    }
}

/// Helper functions for common conversions
pub mod helpers {
    use super::*;

    /// Parse f64 from JSON value with error handling
    pub fn parse_f64(value: &Value, field: &str) -> Result<f64, ConversionError> {
        value.as_str()
            .and_then(|s| s.parse().ok())
            .or_else(|| value.as_f64())
            .ok_or_else(|| ConversionError::InvalidValue {
                field: field.to_string(),
                value: value.to_string(),
            })
    }

    /// Parse i64 from JSON value with error handling
    pub fn parse_i64(value: &Value, field: &str) -> Result<i64, ConversionError> {
        value.as_str()
            .and_then(|s| s.parse().ok())
            .or_else(|| value.as_i64())
            .ok_or_else(|| ConversionError::InvalidValue {
                field: field.to_string(),
                value: value.to_string(),
            })
    }

    /// Parse string from JSON value with error handling
    pub fn parse_str(value: &Value, field: &str) -> Result<String, ConversionError> {
        value.as_str()
            .map(String::from)
            .ok_or_else(|| ConversionError::InvalidValue {
                field: field.to_string(),
                value: value.to_string(),
            })
    }

    /// Get required field from JSON object
    pub fn get_field<'a>(value: &'a Value, field: &str) -> Result<&'a Value, ConversionError> {
        value.get(field)
            .ok_or_else(|| ConversionError::MissingField {
                field: field.to_string(),
            })
    }

    /// Convert timestamp to milliseconds (some exchanges use seconds)
    pub fn normalize_timestamp(value: &Value, field: &str) -> Result<i64, ConversionError> {
        let ts = parse_i64(value, field)?;

        // If timestamp is in seconds (less than year 2000 in milliseconds), convert to ms
        if ts < 946_684_800_000 {
            Ok(ts * 1000)
        } else {
            Ok(ts)
        }
    }

    /// Normalize symbol by removing separators
    pub fn normalize_symbol_raw(symbol: &str) -> String {
        symbol.replace("/", "")
            .replace("-", "")
            .replace("_", "")
            .to_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converter_factory() {
        let binance_converter = ConverterFactory::create(ExchangeName::Binance);
        assert_eq!(binance_converter.exchange_name(), ExchangeName::Binance);

        let okx_converter = ConverterFactory::create(ExchangeName::OKX);
        assert_eq!(okx_converter.exchange_name(), ExchangeName::OKX);

        let bybit_converter = ConverterFactory::create(ExchangeName::Bybit);
        assert_eq!(bybit_converter.exchange_name(), ExchangeName::Bybit);
    }

    #[test]
    fn test_converter_factory_from_name() {
        let converter = ConverterFactory::from_name("binance").unwrap();
        assert_eq!(converter.exchange_name(), ExchangeName::Binance);

        let converter = ConverterFactory::from_name("okx").unwrap();
        assert_eq!(converter.exchange_name(), ExchangeName::OKX);

        let result = ConverterFactory::from_name("unknown");
        assert!(result.is_err());
    }

    #[test]
    fn test_helpers_normalize_symbol() {
        assert_eq!(helpers::normalize_symbol_raw("BTC/USDT"), "BTCUSDT");
        assert_eq!(helpers::normalize_symbol_raw("BTC-USDT"), "BTCUSDT");
        assert_eq!(helpers::normalize_symbol_raw("BTC_USDT"), "BTCUSDT");
        assert_eq!(helpers::normalize_symbol_raw("btcusdt"), "BTCUSDT");
    }
}
