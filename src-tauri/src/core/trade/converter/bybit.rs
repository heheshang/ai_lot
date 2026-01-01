//! Bybit Market Data Converter
//!
//! Converts Bybit exchange data to the unified AI-LOT format.

use crate::core::trade::types::*;
use crate::core::trade::exchange::ExchangeName;
use crate::core::trade::converter::{MarketDataConverter, ConversionError, SymbolFormat, helpers};
use serde_json::Value;

/// Bybit data converter
pub struct BybitConverter;

impl BybitConverter {
    pub fn new() -> Self {
        Self
    }
}

impl MarketDataConverter for BybitConverter {
    fn exchange_name(&self) -> ExchangeName {
        ExchangeName::Bybit
    }

    fn symbol_format(&self) -> SymbolFormat {
        SymbolFormat::NoSeparator
    }

    fn convert_ticker(&self, raw: &Value) -> Result<Ticker, ConversionError> {
        let result = helpers::get_field(raw, "result")?;
        let data = helpers::get_field(result, "d")?;

        // Bybit returns array of tickers in "d" field
        let ticker_data = if let Some(arr) = data.as_array() {
            arr.first().unwrap_or(data)
        } else {
            data
        };

        Ok(Ticker {
            symbol: helpers::parse_str(
                ticker_data.get("i").unwrap_or(&Value::Null), "i"
            )?,
            price: helpers::parse_f64(
                ticker_data.get("c").unwrap_or(&Value::Null), "c"
            )?,
            price_change: helpers::parse_f64(
                ticker_data.get("p1").unwrap_or(&Value::Null), "p1"
            ).unwrap_or(0.0),
            price_change_percent: helpers::parse_f64(
                ticker_data.get("p1").unwrap_or(&Value::Null), "p1"
            ).unwrap_or(0.0),
            high_24h: helpers::parse_f64(
                ticker_data.get("h").unwrap_or(&Value::Null), "h"
            )?,
            low_24h: helpers::parse_f64(
                ticker_data.get("l").unwrap_or(&Value::Null), "l"
            )?,
            volume_24h: helpers::parse_f64(
                ticker_data.get("v").unwrap_or(&Value::Null), "v"
            )?,
            timestamp: helpers::normalize_timestamp(
                ticker_data.get("t").unwrap_or(&Value::Null), "t"
            )?,
        })
    }

    fn convert_kline(&self, raw: &Value, interval: Interval) -> Result<Kline, ConversionError> {
        let result = helpers::get_field(raw, "result")?;

        // Bybit v5 API format
        if let Some(arr) = result.as_array() {
            if let Some(candle_arr) = arr.first().and_then(|v| v.as_array()) {
                // Format: [startTime, open, high, low, close, volume, turnover]
                return Ok(Kline {
                    symbol: self.normalize_symbol(
                        helpers::parse_str(
                            raw.get("symbol").unwrap_or(&Value::Null), "symbol"
                        )?.as_str()
                    ),
                    timeframe: interval.as_str().to_string(),
                    timestamp: candle_arr.get(0).and_then(|v| v.as_i64())
                        .map(|ts| if ts < 946_684_800_000 { ts * 1000 } else { ts })
                        .unwrap_or(0),
                    open: candle_arr.get(1).and_then(|v| v.as_str().and_then(|s| s.parse().ok()))
                        .or_else(|| candle_arr.get(1).and_then(|v| v.as_f64()))
                        .unwrap_or(0.0),
                    high: candle_arr.get(2).and_then(|v| v.as_str().and_then(|s| s.parse().ok()))
                        .or_else(|| candle_arr.get(2).and_then(|v| v.as_f64()))
                        .unwrap_or(0.0),
                    low: candle_arr.get(3).and_then(|v| v.as_str().and_then(|s| s.parse().ok()))
                        .or_else(|| candle_arr.get(3).and_then(|v| v.as_f64()))
                        .unwrap_or(0.0),
                    close: candle_arr.get(4).and_then(|v| v.as_str().and_then(|s| s.parse().ok()))
                        .or_else(|| candle_arr.get(4).and_then(|v| v.as_f64()))
                        .unwrap_or(0.0),
                    volume: candle_arr.get(5).and_then(|v| v.as_str().and_then(|s| s.parse().ok()))
                        .or_else(|| candle_arr.get(5).and_then(|v| v.as_f64()))
                        .unwrap_or(0.0),
                    quote_volume: candle_arr.get(6).and_then(|v| v.as_str().and_then(|s| s.parse().ok()))
                        .or_else(|| candle_arr.get(6).and_then(|v| v.as_f64())),
                });
            }
        }

        // Single candle format (from WebSocket)
        let _start = helpers::get_field(result, "start");
        Ok(Kline {
            symbol: self.normalize_symbol(
                helpers::parse_str(raw.get("symbol").unwrap_or(&Value::Null), "symbol")?.as_str()
            ),
            timeframe: interval.as_str().to_string(),
            timestamp: helpers::normalize_timestamp(
                result.get("t").unwrap_or(&Value::Null), "t"
            )?,
            open: helpers::parse_f64(result.get("o").unwrap_or(&Value::Null), "o")?,
            high: helpers::parse_f64(result.get("h").unwrap_or(&Value::Null), "h")?,
            low: helpers::parse_f64(result.get("l").unwrap_or(&Value::Null), "l")?,
            close: helpers::parse_f64(result.get("c").unwrap_or(&Value::Null), "c")?,
            volume: helpers::parse_f64(result.get("v").unwrap_or(&Value::Null), "v")?,
            quote_volume: result.get("turnover").and_then(|v| v.as_str().and_then(|s| s.parse().ok())),
        })
    }

    fn convert_order(&self, raw: &Value) -> Result<Order, ConversionError> {
        let result = helpers::get_field(raw, "result")?;

        let side_str = helpers::parse_str(
            result.get("side").unwrap_or(&Value::Null), "side"
        )?;
        let side = match side_str.as_str() {
            "Buy" => OrderSide::Buy,
            "Sell" => OrderSide::Sell,
            _ => return Err(ConversionError::InvalidValue {
                field: "side".to_string(),
                value: side_str,
            }),
        };

        let type_str = helpers::parse_str(
            result.get("orderType").unwrap_or(&Value::Null), "orderType"
        );
        let order_type = match type_str.as_ref().map(|s| s.as_str()).ok() {
            Some("Market") => OrderType::Market,
            Some("Limit") => OrderType::Limit,
            Some("Stop") => OrderType::StopLoss,
            Some("StopLimit") => OrderType::StopLimit,
            _ => OrderType::Limit,
        };

        let status_str = helpers::parse_str(
            result.get("orderStatus").unwrap_or(&Value::Null), "orderStatus"
        )?;
        let status = match status_str.as_str() {
            "New" => OrderState::Pending,
            "PartiallyFilled" => OrderState::PartiallyFilled,
            "Filled" => OrderState::Filled,
            "Cancelled" => OrderState::Canceled,
            "Rejected" => OrderState::Rejected,
            _ => OrderState::Pending,
        };

        Ok(Order {
            id: helpers::parse_str(
                result.get("orderId").unwrap_or(&Value::Null), "orderId"
            )?,
            exchange_order_id: Some(helpers::parse_str(
                result.get("orderId").unwrap_or(&Value::Null), "orderId"
            )?),
            client_order_id: result.get("orderLinkId").and_then(|v| v.as_str()).map(String::from),
            symbol: self.normalize_symbol(
                helpers::parse_str(
                    result.get("symbol").unwrap_or(&Value::Null), "symbol"
                )?.as_str()
            ),
            side,
            order_type,
            price: result.get("price").and_then(|v| v.as_str().and_then(|s| s.parse().ok())),
            quantity: helpers::parse_f64(
                result.get("qty").unwrap_or(&Value::Null), "qty"
            )?,
            filled_quantity: helpers::parse_f64(
                result.get("cumExecQty").unwrap_or(&Value::Null), "cumExecQty"
            ).or_else(|_| Ok::<f64, ConversionError>(0.0))?,
            avg_price: result.get("avgPrice").and_then(|v| v.as_str().and_then(|s| s.parse().ok())),
            status,
            commission: helpers::parse_f64(
                result.get("execFee").unwrap_or(&Value::Null), "execFee"
            ).or_else(|_| Ok::<f64, ConversionError>(0.0))?,
            created_at: helpers::normalize_timestamp(
                result.get("createdTime").unwrap_or(&Value::Null), "createdTime"
            )?,
            filled_at: result.get("updatedTime").and_then(|v| helpers::normalize_timestamp(v, "updatedTime").ok()),
        })
    }

    fn convert_balance(&self, raw: &Value) -> Result<Vec<Balance>, ConversionError> {
        let result = helpers::get_field(raw, "result")?;
        let list = helpers::get_field(result, "list")?;

        let mut balances = Vec::new();

        if let Some(list_arr) = list.as_array() {
            for item in list_arr {
                if let Some(coin_arr) = item.get("coin").and_then(|v| v.as_array()) {
                    for coin_obj in coin_arr {
                        let asset = helpers::parse_str(
                            coin_obj.get("coin").unwrap_or(&Value::Null), "coin"
                        )?;

                        let wallet_balance = helpers::parse_f64(
                            coin_obj.get("walletBalance").unwrap_or(&Value::Null), "walletBalance"
                        );
                        let transfer_balance = helpers::parse_f64(
                            coin_obj.get("transferBalance").unwrap_or(&Value::Null), "transferBalance"
                        );

                        let (total, locked, free) = match (wallet_balance, transfer_balance) {
                            (Ok(w), Ok(t)) => (w, w - t, t),
                            _ => continue,
                        };

                        if total > 0.0 {
                            balances.push(Balance {
                                asset,
                                free,
                                locked,
                                total,
                            });
                        }
                    }
                }
            }
        }

        Ok(balances)
    }

    fn convert_position(&self, raw: &Value) -> Result<Vec<Position>, ConversionError> {
        let result = helpers::get_field(raw, "result")?;
        let list = helpers::get_field(result, "list")?;

        let mut positions = Vec::new();

        if let Some(list_arr) = list.as_array() {
            for item in list_arr {
                if let Some(pos_arr) = item.get("position").and_then(|v| v.as_array()) {
                    for pos_obj in pos_arr {
                        let size = helpers::parse_f64(
                            pos_obj.get("size").unwrap_or(&Value::Null), "size"
                        )?;

                        // Skip positions with zero size
                        if size == 0.0 {
                            continue;
                        }

                        let side_str = helpers::parse_str(
                            pos_obj.get("side").unwrap_or(&Value::Null), "side"
                        )?;

                        positions.push(Position {
                            id: helpers::parse_str(
                                pos_obj.get("positionId").unwrap_or(&Value::Null), "positionId"
                            )?,
                            symbol: self.normalize_symbol(
                                helpers::parse_str(
                                    pos_obj.get("symbol").unwrap_or(&Value::Null), "symbol"
                                )?.as_str()
                            ),
                            side: side_str.to_lowercase(),
                            quantity: size.abs(),
                            entry_price: helpers::parse_f64(
                                pos_obj.get("avgPrice").unwrap_or(&Value::Null), "avgPrice"
                            )?,
                            current_price: pos_obj.get("markPrice").and_then(|v| v.as_str().and_then(|s| s.parse().ok())),
                            unrealized_pnl: helpers::parse_f64(
                                pos_obj.get("unrealisedPnl").unwrap_or(&Value::Null), "unrealisedPnl"
                            ).unwrap_or(0.0),
                            realized_pnl: helpers::parse_f64(
                                pos_obj.get("realisedPnl").unwrap_or(&Value::Null), "realisedPnl"
                            ).unwrap_or(0.0),
                            opened_at: helpers::normalize_timestamp(
                                pos_obj.get("createdTime").unwrap_or(&Value::Null), "createdTime"
                            )?,
                        });
                    }
                }
            }
        }

        Ok(positions)
    }

    fn normalize_symbol(&self, symbol: &str) -> String {
        // Bybit uses no separator (BTCUSDT)
        helpers::normalize_symbol_raw(symbol)
    }

    fn denormalize_symbol(&self, symbol: &str) -> String {
        // Bybit uses no separator
        self.normalize_symbol(symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_convert_ticker() {
        let converter = BybitConverter::new();

        let raw = json!({
            "retCode": 0,
            "retMsg": "",
            "result": {
                "d": [{
                    "i": "BTCUSDT",
                    "c": "50000.50",
                    "h": "51000.00",
                    "l": "49000.00",
                    "v": "10000.5",
                    "p1": "2.05",
                    "t": "1234567890000"
                }]
            }
        });

        let ticker = converter.convert_ticker(&raw).unwrap();
        assert_eq!(ticker.symbol, "BTCUSDT");
        assert_eq!(ticker.price, 50000.50);
    }

    #[test]
    fn test_normalize_symbol() {
        let converter = BybitConverter::new();

        assert_eq!(converter.normalize_symbol("BTCUSDT"), "BTCUSDT");
        assert_eq!(converter.normalize_symbol("btcusdt"), "BTCUSDT");
        assert_eq!(converter.denormalize_symbol("BTCUSDT"), "BTCUSDT");
    }
}
