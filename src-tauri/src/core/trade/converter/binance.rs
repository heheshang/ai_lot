//! Binance Market Data Converter
//!
//! Converts Binance exchange data to the unified AI-LOT format.

use crate::core::trade::types::*;
use crate::core::trade::exchange::ExchangeName;
use crate::core::trade::converter::{MarketDataConverter, ConversionError, SymbolFormat, helpers};
use serde_json::Value;

/// Binance data converter
pub struct BinanceConverter;

impl BinanceConverter {
    pub fn new() -> Self {
        Self
    }
}

impl MarketDataConverter for BinanceConverter {
    fn exchange_name(&self) -> ExchangeName {
        ExchangeName::Binance
    }

    fn symbol_format(&self) -> SymbolFormat {
        SymbolFormat::NoSeparator
    }

    fn convert_ticker(&self, raw: &Value) -> Result<Ticker, ConversionError> {
        Ok(Ticker {
            symbol: helpers::parse_str(raw.get("s").unwrap_or(&Value::Null), "s")?,
            price: helpers::parse_f64(raw.get("c").unwrap_or(&Value::Null), "c")?,
            price_change: helpers::parse_f64(raw.get("p").unwrap_or(&Value::Null), "p")?,
            price_change_percent: helpers::parse_f64(raw.get("P").unwrap_or(&Value::Null), "P")?,
            high_24h: helpers::parse_f64(raw.get("h").unwrap_or(&Value::Null), "h")?,
            low_24h: helpers::parse_f64(raw.get("l").unwrap_or(&Value::Null), "l")?,
            volume_24h: helpers::parse_f64(raw.get("v").unwrap_or(&Value::Null), "v")?,
            timestamp: helpers::parse_i64(raw.get("E").unwrap_or(&Value::Null), "E")?,
        })
    }

    fn convert_kline(&self, raw: &Value, interval: Interval) -> Result<Kline, ConversionError> {
        let k = helpers::get_field(raw, "k")?;

        Ok(Kline {
            symbol: helpers::parse_str(raw.get("s").unwrap_or(&Value::Null), "s")?,
            timeframe: interval.as_str().to_string(),
            timestamp: helpers::normalize_timestamp(k.get("t").unwrap_or(&Value::Null), "t")?,
            open: helpers::parse_f64(k.get("o").unwrap_or(&Value::Null), "o")?,
            high: helpers::parse_f64(k.get("h").unwrap_or(&Value::Null), "h")?,
            low: helpers::parse_f64(k.get("l").unwrap_or(&Value::Null), "l")?,
            close: helpers::parse_f64(k.get("c").unwrap_or(&Value::Null), "c")?,
            volume: helpers::parse_f64(k.get("v").unwrap_or(&Value::Null), "v")?,
            quote_volume: k.get("q").and_then(|v| v.as_str().and_then(|s| s.parse().ok())),
        })
    }

    fn convert_order(&self, raw: &Value) -> Result<Order, ConversionError> {
        let side_str = helpers::parse_str(raw.get("side").unwrap_or(&Value::Null), "side")?;
        let side = match side_str.as_str() {
            "BUY" => OrderSide::Buy,
            "SELL" => OrderSide::Sell,
            _ => return Err(ConversionError::InvalidValue {
                field: "side".to_string(),
                value: side_str,
            }),
        };

        let type_str = helpers::parse_str(raw.get("type").unwrap_or(&Value::Null), "type")?;
        let order_type = match type_str.as_str() {
            "MARKET" => OrderType::Market,
            "LIMIT" => OrderType::Limit,
            "STOP_LOSS" => OrderType::StopLoss,
            "STOP_LOSS_LIMIT" => OrderType::StopLimit,
            _ => OrderType::Limit, // Default to LIMIT
        };

        let status_str = helpers::parse_str(raw.get("status").unwrap_or(&Value::Null), "status")?;
        let status = match status_str.as_str() {
            "NEW" => OrderState::Pending,
            "PARTIALLY_FILLED" => OrderState::PartiallyFilled,
            "FILLED" => OrderState::Filled,
            "CANCELED" => OrderState::Canceled,
            "REJECTED" => OrderState::Rejected,
            "EXPIRED" => OrderState::Canceled,
            _ => OrderState::Pending,
        };

        Ok(Order {
            id: helpers::parse_str(raw.get("orderId").unwrap_or(&Value::Null), "orderId")?,
            exchange_order_id: raw.get("clientOrderId").and_then(|v| v.as_str()).map(String::from),
            client_order_id: raw.get("clientOrderId").and_then(|v| v.as_str()).map(String::from),
            symbol: helpers::parse_str(raw.get("symbol").unwrap_or(&Value::Null), "symbol")?,
            side,
            order_type,
            price: raw.get("price").and_then(|v| v.as_str().and_then(|s| s.parse().ok())),
            quantity: helpers::parse_f64(raw.get("executedQty").unwrap_or(&Value::Null), "executedQty")
                .unwrap_or_else(|_| helpers::parse_f64(raw.get("origQty").unwrap_or(&Value::Null), "origQty").unwrap_or(0.0)),
            filled_quantity: helpers::parse_f64(raw.get("executedQty").unwrap_or(&Value::Null), "executedQty").unwrap_or(0.0),
            avg_price: raw.get("avgPrice").and_then(|v| v.as_str().and_then(|s| s.parse().ok())),
            status,
            commission: helpers::parse_f64(raw.get("commission").unwrap_or(&Value::Null), "commission").unwrap_or(0.0),
            created_at: helpers::normalize_timestamp(raw.get("time").unwrap_or(&Value::Null), "time")?,
            filled_at: raw.get("updateTime").and_then(|v| helpers::normalize_timestamp(v, "updateTime").ok()),
        })
    }

    fn convert_balance(&self, raw: &Value) -> Result<Vec<Balance>, ConversionError> {
        let balances = helpers::get_field(raw, "balances")?;

        let mut result = Vec::new();

        if let Some(balances_array) = balances.as_array() {
            for balance_obj in balances_array {
                let asset = helpers::parse_str(balance_obj.get("asset").unwrap_or(&Value::Null), "asset")?;
                let free = helpers::parse_f64(balance_obj.get("free").unwrap_or(&Value::Null), "free")?;
                let locked = helpers::parse_f64(balance_obj.get("locked").unwrap_or(&Value::Null), "locked")?;

                result.push(Balance {
                    asset,
                    free,
                    locked,
                    total: free + locked,
                });
            }
        }

        Ok(result)
    }

    fn convert_position(&self, raw: &Value) -> Result<Vec<Position>, ConversionError> {
        let positions = helpers::get_field(raw, "positions")?;

        let mut result = Vec::new();

        if let Some(positions_array) = positions.as_array() {
            for pos_obj in positions_array {
                let symbol = helpers::parse_str(pos_obj.get("symbol").unwrap_or(&Value::Null), "symbol")?;

                let position_amt = helpers::parse_f64(pos_obj.get("positionAmt").unwrap_or(&Value::Null), "positionAmt")?;

                let side = if position_amt >= 0.0 {
                    "long".to_string()
                } else {
                    "short".to_string()
                };

                result.push(Position {
                    id: helpers::parse_str(pos_obj.get("positionId").unwrap_or(&Value::Null), "positionId")?,
                    symbol,
                    side,
                    quantity: position_amt.abs(),
                    entry_price: helpers::parse_f64(pos_obj.get("entryPrice").unwrap_or(&Value::Null), "entryPrice")?,
                    current_price: pos_obj.get("breakEvenPrice").and_then(|v| v.as_str().and_then(|s| s.parse().ok())),
                    unrealized_pnl: helpers::parse_f64(pos_obj.get("unRealizedProfit").unwrap_or(&Value::Null), "unRealizedProfit").unwrap_or(0.0),
                    realized_pnl: 0.0,
                    opened_at: helpers::normalize_timestamp(pos_obj.get("updateTime").unwrap_or(&Value::Null), "updateTime")?,
                });
            }
        }

        Ok(result)
    }

    fn normalize_symbol(&self, symbol: &str) -> String {
        // Binance uses no separator, just uppercase
        helpers::normalize_symbol_raw(symbol)
    }

    fn denormalize_symbol(&self, symbol: &str) -> String {
        // Binance uses no separator
        self.normalize_symbol(symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_convert_ticker() {
        let converter = BinanceConverter::new();

        let raw = json!({
            "e": "24hrTicker",
            "E": 1234567890000,
            "s": "BTCUSDT",
            "c": "50000.50",
            "p": "1000.00",
            "P": "2.05",
            "h": "51000.00",
            "l": "49000.00",
            "v": "10000.5"
        });

        let ticker = converter.convert_ticker(&raw).unwrap();
        assert_eq!(ticker.symbol, "BTCUSDT");
        assert_eq!(ticker.price, 50000.50);
        assert_eq!(ticker.price_change, 1000.00);
        assert_eq!(ticker.price_change_percent, 2.05);
    }

    #[test]
    fn test_convert_kline() {
        let converter = BinanceConverter::new();

        let raw = json!({
            "e": "kline",
            "E": 1234567890000,
            "s": "BTCUSDT",
            "k": {
                "t": 1234567890000,
                "s": "1h",
                "o": "50000.00",
                "h": "50500.00",
                "l": "49800.00",
                "c": "50200.00",
                "v": "1000.5",
                "q": "50000000.00"
            }
        });

        let kline = converter.convert_kline(&raw, Interval::OneHour).unwrap();
        assert_eq!(kline.symbol, "BTCUSDT");
        assert_eq!(kline.open, 50000.00);
        assert_eq!(kline.high, 50500.00);
        assert_eq!(kline.low, 49800.00);
        assert_eq!(kline.close, 50200.00);
    }

    #[test]
    fn test_normalize_symbol() {
        let converter = BinanceConverter::new();

        assert_eq!(converter.normalize_symbol("BTCUSDT"), "BTCUSDT");
        assert_eq!(converter.normalize_symbol("btcusdt"), "BTCUSDT");
        assert_eq!(converter.denormalize_symbol("BTCUSDT"), "BTCUSDT");
    }
}
