//! OKX Market Data Converter
//!
//! Converts OKX exchange data to the unified AI-LOT format.

use crate::core::trade::types::*;
use crate::core::trade::exchange::ExchangeName;
use crate::core::trade::converter::{MarketDataConverter, ConversionError, SymbolFormat, helpers};
use serde_json::Value;

/// OKX data converter
pub struct OkxConverter;

impl Default for OkxConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl OkxConverter {
    pub fn new() -> Self {
        Self
    }
}

impl MarketDataConverter for OkxConverter {
    fn exchange_name(&self) -> ExchangeName {
        ExchangeName::OKX
    }

    fn symbol_format(&self) -> SymbolFormat {
        SymbolFormat::Dash
    }

    fn convert_ticker(&self, raw: &Value) -> Result<Ticker, ConversionError> {
        let data = helpers::get_field(raw, "data")?;

        Ok(Ticker {
            symbol: helpers::parse_str(data.get("instId").unwrap_or(&Value::Null), "instId")?,
            price: helpers::parse_f64(data.get("last").unwrap_or(&Value::Null), "last")?,
            price_change: helpers::parse_f64(data.get("last").unwrap_or(&Value::Null), "last")?
                - helpers::parse_f64(data.get("open24h").unwrap_or(&Value::Null), "open24h")?,
            price_change_percent: helpers::parse_f64(data.get("changePercent").unwrap_or(&Value::Null), "changePercent")
                .or(Ok::<f64, ConversionError>(0.0))?,
            high_24h: helpers::parse_f64(data.get("high24h").unwrap_or(&Value::Null), "high24h")?,
            low_24h: helpers::parse_f64(data.get("low24h").unwrap_or(&Value::Null), "low24h")?,
            volume_24h: helpers::parse_f64(data.get("vol24h").unwrap_or(&Value::Null), "vol24h")
                .or(Ok::<f64, ConversionError>(0.0))?,
            timestamp: helpers::normalize_timestamp(data.get("ts").unwrap_or(&Value::Null), "ts")?,
        })
    }

    fn convert_kline(&self, raw: &Value, interval: Interval) -> Result<Kline, ConversionError> {
        let data = helpers::get_field(raw, "data")?;

        // OKX returns array of candles
        if let Some(candles) = data.as_array() {
            if let Some(candle) = candles.first() {
                return Ok(Kline {
                    symbol: helpers::parse_str(candle.get("instId").unwrap_or(&Value::Null), "instId")?,
                    timeframe: interval.as_str().to_string(),
                    timestamp: helpers::normalize_timestamp(candle.get("ts").unwrap_or(&Value::Null), "ts")?,
                    open: helpers::parse_f64(candle.get("o").unwrap_or(&Value::Null), "o")?,
                    high: helpers::parse_f64(candle.get("h").unwrap_or(&Value::Null), "h")?,
                    low: helpers::parse_f64(candle.get("l").unwrap_or(&Value::Null), "l")?,
                    close: helpers::parse_f64(candle.get("c").unwrap_or(&Value::Null), "c")?,
                    volume: helpers::parse_f64(candle.get("vol").unwrap_or(&Value::Null), "vol")
                        .or(Ok::<f64, ConversionError>(0.0))?,
                    quote_volume: candle.get("volCcy").and_then(|v| v.as_str().and_then(|s| s.parse().ok())),
                });
            }
        }

        // Single candle format (from WebSocket)
        Ok(Kline {
            symbol: helpers::parse_str(data.get("instId").unwrap_or(&Value::Null), "instId")?,
            timeframe: interval.as_str().to_string(),
            timestamp: helpers::normalize_timestamp(data.get("ts").unwrap_or(&Value::Null), "ts")?,
            open: helpers::parse_f64(data.get("o").unwrap_or(&Value::Null), "o")?,
            high: helpers::parse_f64(data.get("h").unwrap_or(&Value::Null), "h")?,
            low: helpers::parse_f64(data.get("l").unwrap_or(&Value::Null), "l")?,
            close: helpers::parse_f64(data.get("c").unwrap_or(&Value::Null), "c")?,
            volume: helpers::parse_f64(data.get("vol").unwrap_or(&Value::Null), "vol")
                .or(Ok::<f64, ConversionError>(0.0))?,
            quote_volume: data.get("volCcy").and_then(|v| v.as_str().and_then(|s| s.parse().ok())),
        })
    }

    fn convert_order(&self, raw: &Value) -> Result<Order, ConversionError> {
        let data = helpers::get_field(raw, "data")?;

        let side_str = helpers::parse_str(data.get("side").unwrap_or(&Value::Null), "side")?;
        let side = match side_str.as_str() {
            "buy" => OrderSide::Buy,
            "sell" => OrderSide::Sell,
            _ => return Err(ConversionError::InvalidValue {
                field: "side".to_string(),
                value: side_str,
            }),
        };

        let type_str = helpers::parse_str(data.get("tdMode").unwrap_or(&Value::Null), "tdMode")?;
        let order_type = match type_str.as_str() {
            "market" => OrderType::Market,
            "limit" => OrderType::Limit,
            "stop_loss" => OrderType::StopLoss,
            "stop_limit" => OrderType::StopLimit,
            _ => OrderType::Limit,
        };

        let state_str = helpers::parse_str(data.get("state").unwrap_or(&Value::Null), "state")?;
        let status = match state_str.as_str() {
            "live" | "partially_filled" => OrderState::Open,
            "filled" => OrderState::Filled,
            "canceled" => OrderState::Canceled,
            _ => OrderState::Pending,
        };

        Ok(Order {
            id: helpers::parse_str(data.get("ordId").unwrap_or(&Value::Null), "ordId")?,
            exchange_order_id: Some(helpers::parse_str(data.get("ordId").unwrap_or(&Value::Null), "ordId")?),
            client_order_id: data.get("clOrdId").and_then(|v| v.as_str()).map(String::from),
            symbol: self.normalize_symbol(
                helpers::parse_str(data.get("instId").unwrap_or(&Value::Null), "instId")?.as_str()
            ),
            side,
            order_type,
            price: data.get("px").and_then(|v| v.as_str().and_then(|s| s.parse().ok())),
            quantity: helpers::parse_f64(data.get("fillSz").unwrap_or(&Value::Null), "fillSz")
                .or_else(|_| helpers::parse_f64(data.get("sz").unwrap_or(&Value::Null), "sz"))?,
            filled_quantity: helpers::parse_f64(data.get("fillSz").unwrap_or(&Value::Null), "fillSz")
                .or(Ok::<f64, ConversionError>(0.0))?,
            avg_price: data.get("avgPx").and_then(|v| v.as_str().and_then(|s| s.parse().ok())),
            status,
            commission: helpers::parse_f64(data.get("fee").unwrap_or(&Value::Null), "fee")
                .or(Ok::<f64, ConversionError>(0.0))?,
            created_at: helpers::normalize_timestamp(data.get("cTime").unwrap_or(&Value::Null), "cTime")?,
            filled_at: data.get("uTime").and_then(|v| helpers::normalize_timestamp(v, "uTime").ok()),
        })
    }

    fn convert_balance(&self, raw: &Value) -> Result<Vec<Balance>, ConversionError> {
        let data = helpers::get_field(raw, "data")?;
        let details = helpers::get_field(data, "details")?;

        let mut result = Vec::new();

        if let Some(balances_array) = details.as_array() {
            for balance_obj in balances_array {
                let ccy = helpers::parse_str(balance_obj.get("ccy").unwrap_or(&Value::Null), "ccy")?;

                let avail_bal = helpers::parse_f64(balance_obj.get("availBal").unwrap_or(&Value::Null), "availBal")?;
                let frozen_bal = helpers::parse_f64(balance_obj.get("frozenBal").unwrap_or(&Value::Null), "frozenBal")?;

                result.push(Balance {
                    asset: ccy,
                    free: avail_bal,
                    locked: frozen_bal,
                    total: avail_bal + frozen_bal,
                });
            }
        }

        Ok(result)
    }

    fn convert_position(&self, raw: &Value) -> Result<Vec<Position>, ConversionError> {
        let data = helpers::get_field(raw, "data")?;

        let mut result = Vec::new();

        if let Some(positions_array) = data.as_array() {
            for pos_obj in positions_array {
                let inst_id = helpers::parse_str(pos_obj.get("instId").unwrap_or(&Value::Null), "instId")?;

                let pos = helpers::parse_f64(pos_obj.get("pos").unwrap_or(&Value::Null), "pos")?;

                let side = if pos >= 0.0 {
                    "long".to_string()
                } else {
                    "short".to_string()
                };

                result.push(Position {
                    id: helpers::parse_str(pos_obj.get("posId").unwrap_or(&Value::Null), "posId")?,
                    symbol: self.normalize_symbol(inst_id.as_str()),
                    side,
                    quantity: pos.abs(),
                    entry_price: helpers::parse_f64(pos_obj.get("avgPx").unwrap_or(&Value::Null), "avgPx")?,
                    current_price: pos_obj.get("last").and_then(|v| v.as_str().and_then(|s| s.parse().ok())),
                    unrealized_pnl: helpers::parse_f64(pos_obj.get("upl").unwrap_or(&Value::Null), "upl")
                        .or(Ok::<f64, ConversionError>(0.0))?,
                    realized_pnl: helpers::parse_f64(pos_obj.get("realizedPnl").unwrap_or(&Value::Null), "realizedPnl")
                        .or(Ok::<f64, ConversionError>(0.0))?,
                    opened_at: helpers::normalize_timestamp(pos_obj.get("uTime").unwrap_or(&Value::Null), "uTime")?,
                });
            }
        }

        Ok(result)
    }

    fn normalize_symbol(&self, symbol: &str) -> String {
        // OKX uses BTC-USDT format, convert to BTCUSDT
        helpers::normalize_symbol_raw(symbol)
    }

    fn denormalize_symbol(&self, symbol: &str) -> String {
        // Convert to OKX format: BTCUSDT -> BTC-USDT
        // Find common currency pairs and insert dash
        let common_quotes = ["USDT", "USD", "EUR", "BTC", "ETH", "BNB"];

        for quote in &common_quotes {
            if let Some(base) = symbol.strip_suffix(quote) {
                return format!("{}-{}", base, quote);
            }
        }

        // Default: insert dash after 3 or 4 characters
        if symbol.len() <= 6 {
            format!("{}-{}", &symbol[..3], &symbol[3..])
        } else {
            format!("{}-{}", &symbol[..4], &symbol[4..])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_convert_ticker() {
        let converter = OkxConverter::new();

        let raw = json!({
            "code": "0",
            "msg": "",
            "data": [{
                "instType": "SPOT",
                "instId": "BTC-USDT",
                "last": "50000.50",
                "lastSz": "0.001",
                "askPx": "50001.00",
                "bidPx": "50000.00",
                "open24h": "49000.00",
                "high24h": "51000.00",
                "low24h": "48500.00",
                "changePercent": "2.05",
                "vol24h": "10000.5",
                "volCcy24h": "50000000",
                "ts": "1234567890000"
            }]
        });

        let ticker = converter.convert_ticker(&raw).unwrap();
        assert_eq!(ticker.symbol, "BTC-USDT");
        assert_eq!(ticker.price, 50000.50);
    }

    #[test]
    fn test_normalize_symbol() {
        let converter = OkxConverter::new();

        assert_eq!(converter.normalize_symbol("BTC-USDT"), "BTCUSDT");
        assert_eq!(converter.denormalize_symbol("BTCUSDT"), "BTC-USDT");
        assert_eq!(converter.denormalize_symbol("ETHUSDT"), "ETH-USDT");
    }
}
