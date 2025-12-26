/// Integration tests for Binance REST API
///
/// These tests verify that the BinanceExchange implementation correctly
/// communicates with the Binance API.
///
/// Run tests with: cargo test --package ai-lot-lib --test test_binance_api

use ai_lot_lib::core::trade::exchange::{Exchange, ExchangeName, BinanceExchange};
use ai_lot_lib::core::trade::types::Interval;

#[tokio::test]
async fn test_binance_exchange_creation() {
    let exchange = BinanceExchange::new(None, None);

    // Verify exchange name
    assert_eq!(exchange.name(), ExchangeName::Binance);

    // Verify initial state
    assert!(!exchange.is_connected());

    // Verify connect works
    let result = exchange.connect().await;
    assert!(result.is_ok());
    assert!(exchange.is_connected());

    // Verify disconnect works
    let result = exchange.disconnect().await;
    assert!(result.is_ok());
    assert!(!exchange.is_connected());
}

#[tokio::test]
async fn test_get_ticker_btcusdt() {
    let exchange = BinanceExchange::new(None, None);

    // Test getting ticker for BTCUSDT
    let ticker_result = exchange.get_ticker("BTCUSDT").await;

    assert!(ticker_result.is_ok(), "Failed to get ticker: {:?}", ticker_result.err());

    let ticker = ticker_result.unwrap();

    // Verify ticker structure
    assert_eq!(ticker.symbol, "BTCUSDT");
    assert!(ticker.price > 0.0, "Price should be positive: {}", ticker.price);
    assert!(ticker.timestamp > 0, "Timestamp should be positive: {}", ticker.timestamp);

    // Verify 24h data is populated (using 24hr endpoint)
    assert!(ticker.high_24h > 0.0, "24h high should be positive");
    assert!(ticker.low_24h > 0.0, "24h low should be positive");
    assert!(ticker.volume_24h > 0.0, "24h volume should be positive");

    println!("BTCUSDT Ticker: ${} (24h change: {:+.2}%)",
        ticker.price, ticker.price_change_percent);
}

#[tokio::test]
async fn test_get_ticker_ethusdt() {
    let exchange = BinanceExchange::new(None, None);

    let ticker_result = exchange.get_ticker("ETHUSDT").await;

    assert!(ticker_result.is_ok(), "Failed to get ETHUSDT ticker");

    let ticker = ticker_result.unwrap();

    assert_eq!(ticker.symbol, "ETHUSDT");
    assert!(ticker.price > 0.0);

    println!("ETHUSDT Ticker: ${} (24h change: {:+.2}%)",
        ticker.price, ticker.price_change_percent);
}

#[tokio::test]
async fn test_get_klines_btcusdt_1h() {
    let exchange = BinanceExchange::new(None, None);

    // Test getting 1-hour klines for BTCUSDT
    let klines_result = exchange.get_klines("BTCUSDT", Interval::OneHour, 10).await;

    assert!(klines_result.is_ok(), "Failed to get klines: {:?}", klines_result.err());

    let klines = klines_result.unwrap();

    // Verify we got the expected number of klines
    assert_eq!(klines.len(), 10, "Expected 10 klines, got {}", klines.len());

    // Verify first kline structure
    let first_kline = &klines[0];
    assert_eq!(first_kline.symbol, "BTCUSDT");
    assert_eq!(first_kline.timeframe, "1h");
    assert!(first_kline.timestamp > 0);
    assert!(first_kline.open > 0.0);
    assert!(first_kline.high > 0.0);
    assert!(first_kline.low > 0.0);
    assert!(first_kline.close > 0.0);
    assert!(first_kline.volume > 0.0);

    // Verify OHLC relationship
    assert!(first_kline.high >= first_kline.open, "High should be >= open");
    assert!(first_kline.high >= first_kline.close, "High should be >= close");
    assert!(first_kline.low <= first_kline.open, "Low should be <= open");
    assert!(first_kline.low <= first_kline.close, "Low should be <= close");

    println!("BTCUSDT 1h Klines:");
    for (i, k) in klines.iter().take(3).enumerate() {
        println!("  [{}] O:${:.2} H:${:.2} L:${:.2} C:${:.2} Vol:{:.2}",
            i, k.open, k.high, k.low, k.close, k.volume);
    }
}

#[tokio::test]
async fn test_get_klines_multiple_intervals() {
    let exchange = BinanceExchange::new(None, None);

    // Test different intervals
    let intervals = [
        (Interval::OneMinute, "1m"),
        (Interval::FiveMinutes, "5m"),
        (Interval::FifteenMinutes, "15m"),
        (Interval::OneHour, "1h"),
    ];

    for (interval, expected_timeframe) in intervals {
        let klines_result = exchange.get_klines("BTCUSDT", interval, 5).await;

        assert!(klines_result.is_ok(),
            "Failed to get klines for interval {:?}", interval);

        let klines = klines_result.unwrap();
        assert_eq!(klines.len(), 5);
        assert_eq!(klines[0].timeframe, expected_timeframe);

        println!("{} interval: got {} klines", expected_timeframe, klines.len());
    }
}

#[tokio::test]
async fn test_broadcast_streams() {
    let exchange = BinanceExchange::new(None, None);

    // Verify ticker stream can be subscribed
    let mut ticker_stream = exchange.ticker_stream();
    let mut kline_stream = exchange.kline_stream();

    // Streams should be empty initially (no data published yet)
    // Try_recv should return Err(Empty) since no data has been sent
    let ticker_result = ticker_stream.try_recv();
    assert!(ticker_result.is_err(), "Ticker stream should be empty initially");

    let kline_result = kline_stream.try_recv();
    assert!(kline_result.is_err(), "Kline stream should be empty initially");
}

#[tokio::test]
async fn test_invalid_symbol() {
    let exchange = BinanceExchange::new(None, None);

    // Test with invalid symbol
    let result = exchange.get_ticker("INVALIDSYMBOL").await;

    // Binance API may return error for invalid symbols
    // The implementation should handle this gracefully
    // Either Ok with empty data or Err is acceptable
    match result {
        Ok(ticker) => {
            println!("Invalid symbol returned ticker: {:?}", ticker);
        }
        Err(e) => {
            println!("Invalid symbol correctly returned error: {}", e);
        }
    }
}

#[tokio::test]
async fn test_case_insensitive_symbol() {
    let exchange = BinanceExchange::new(None, None);

    // Test that symbols are case-insensitive (should be uppercased internally)
    let ticker_lower = exchange.get_ticker("btcusdt").await;
    let ticker_upper = exchange.get_ticker("BTCUSDT").await;
    let ticker_mixed = exchange.get_ticker("BtcUsDt").await;

    assert!(ticker_lower.is_ok(), "Lowercase symbol failed");
    assert!(ticker_upper.is_ok(), "Uppercase symbol failed");
    assert!(ticker_mixed.is_ok(), "Mixed case symbol failed");

    // All should return the same symbol (uppercase)
    assert_eq!(ticker_lower.unwrap().symbol, "BTCUSDT");
    assert_eq!(ticker_upper.unwrap().symbol, "BTCUSDT");
    assert_eq!(ticker_mixed.unwrap().symbol, "BTCUSDT");
}
