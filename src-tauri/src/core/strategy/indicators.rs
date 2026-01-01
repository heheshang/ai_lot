//! Technical Indicators Library
//!
//! This module provides common technical analysis indicators for trading strategies.
//!
//! # Example
//!
//! ```rust
//! use crate::core::strategy::indicators::IndicatorCalculator;
//! use crate::core::trade::types::Kline;
//!
//! let klines = vec![/* ... */];
//! let calculator = IndicatorCalculator::new(klines);
//!
//! // Calculate SMA
//! let sma20 = calculator.sma(20);
//!
//! // Calculate RSI
//! let rsi14 = calculator.rsi(14);
//!
//! // Calculate MACD
//! let macd = calculator.macd(12, 26, 9);
//! ```

use crate::core::trade::types::Kline;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Technical indicator calculator
#[derive(Debug, Clone)]
pub struct IndicatorCalculator {
    /// Historical kline data
    klines: Vec<Kline>,
}

impl IndicatorCalculator {
    /// Create a new indicator calculator with historical kline data
    pub fn new(klines: Vec<Kline>) -> Self {
        Self { klines }
    }

    /// Get the number of klines
    pub fn len(&self) -> usize {
        self.klines.len()
    }

    /// Check if there are any klines
    pub fn is_empty(&self) -> bool {
        self.klines.is_empty()
    }

    /// Get close prices from klines
    fn closes(&self) -> Vec<f64> {
        self.klines.iter().map(|k| k.close).collect()
    }

    /// ==================== Trend Indicators ====================

    /// Simple Moving Average (SMA)
    ///
    /// # Arguments
    ///
    /// * `period` - The number of periods to average
    ///
    /// # Returns
    ///
    /// A vector of optional SMA values. None is returned for periods where
    /// there isn't enough historical data.
    ///
    /// # Example
    ///
    /// ```rust
    /// let sma20 = calculator.sma(20);
    /// ```
    pub fn sma(&self, period: usize) -> Vec<Option<f64>> {
        if period == 0 || self.klines.len() < period {
            return vec![None; self.klines.len()];
        }

        let closes = self.closes();
        let mut result = vec![None; period - 1];
        let mut sum: f64 = closes.iter().take(period).sum();

        result.push(Some(sum / period as f64));

        for i in period..closes.len() {
            sum = sum - closes[i - period] + closes[i];
            result.push(Some(sum / period as f64));
        }

        result
    }

    /// Exponential Moving Average (EMA)
    ///
    /// # Arguments
    ///
    /// * `period` - The number of periods for the EMA
    ///
    /// # Returns
    ///
    /// A vector of optional EMA values
    ///
    /// # Example
    ///
    /// ```rust
    /// let ema12 = calculator.ema(12);
    /// ```
    pub fn ema(&self, period: usize) -> Vec<Option<f64>> {
        if period == 0 || self.klines.len() < period {
            return vec![None; self.klines.len()];
        }

        let closes = self.closes();
        let multiplier = 2.0 / (period as f64 + 1.0);
        let mut result = vec![None; period - 1];

        // Start with SMA
        let mut ema: f64 = closes.iter().take(period).sum::<f64>() / period as f64;
        result.push(Some(ema));

        // Calculate EMA for remaining values
        for i in period..closes.len() {
            ema = (closes[i] - ema) * multiplier + ema;
            result.push(Some(ema));
        }

        result
    }

    /// Weighted Moving Average (WMA)
    ///
    /// Gives more weight to recent prices
    pub fn wma(&self, period: usize) -> Vec<Option<f64>> {
        if period == 0 || self.klines.len() < period {
            return vec![None; self.klines.len()];
        }

        let closes = self.closes();
        let mut result = vec![None; period - 1];

        for i in (period - 1)..closes.len() {
            let mut sum = 0.0;
            let mut weight_sum = 0.0;

            for j in 0..period {
                let weight = (period - j) as f64;
                sum += closes[i - j] * weight;
                weight_sum += weight;
            }

            result.push(Some(sum / weight_sum));
        }

        result
    }

    /// Volume Weighted Average Price (VWAP)
    ///
    /// Calculates the cumulative typical price * volume divided by cumulative volume
    pub fn vwap(&self, period: usize) -> Vec<Option<f64>> {
        if period == 0 || self.klines.len() < period {
            return vec![None; self.klines.len()];
        }

        let mut result = vec![None; period - 1];
        let mut tp_volume_sum = 0.0;
        let mut volume_sum = 0.0;
        let mut volumes: VecDeque<f64> = VecDeque::with_capacity(period);
        let mut tp_volumes: VecDeque<f64> = VecDeque::with_capacity(period);

        for (i, kline) in self.klines.iter().enumerate() {
            let typical_price = (kline.high + kline.low + kline.close) / 3.0;
            let tp_volume = typical_price * kline.volume;

            tp_volume_sum += tp_volume;
            volume_sum += kline.volume;

            tp_volumes.push_back(tp_volume);
            volumes.push_back(kline.volume);

            if i >= period {
                tp_volume_sum -= tp_volumes.pop_front().unwrap();
                volume_sum -= volumes.pop_front().unwrap();
            }

            if i >= period - 1 {
                result.push(Some(tp_volume_sum / volume_sum));
            }
        }

        result
    }

    /// ==================== Momentum Indicators ====================

    /// Relative Strength Index (RSI)
    ///
    /// # Arguments
    ///
    /// * `period` - The number of periods for RSI calculation (typically 14)
    ///
    /// # Returns
    ///
    /// A vector of optional RSI values (0-100)
    ///
    /// # Example
    ///
    /// ```rust
    /// let rsi14 = calculator.rsi(14);
    /// ```
    pub fn rsi(&self, period: usize) -> Vec<Option<f64>> {
        if period == 0 || self.klines.len() <= period {
            return vec![None; self.klines.len()];
        }

        let closes = self.closes();
        let mut result = vec![None; period];

        // Calculate initial average gain/loss
        let mut gains: Vec<f64> = Vec::new();
        let mut losses: Vec<f64> = Vec::new();

        for i in 1..=period {
            let change = closes[i] - closes[i - 1];
            gains.push(change.max(0.0));
            losses.push((-change).max(0.0));
        }

        let mut avg_gain: f64 = gains.iter().sum::<f64>() / period as f64;
        let mut avg_loss: f64 = losses.iter().sum::<f64>() / period as f64;

        // Calculate first RSI
        let rs = if avg_loss == 0.0 {
            100.0
        } else {
            avg_gain / avg_loss
        };
        let rsi = 100.0 - (100.0 / (1.0 + rs));
        result.push(Some(rsi));

        // Calculate subsequent RSI values using Wilder's smoothing
        for i in (period + 1)..closes.len() {
            let change = closes[i] - closes[i - 1];
            let gain = change.max(0.0);
            let loss = (-change).max(0.0);

            avg_gain = (avg_gain * (period - 1) as f64 + gain) / period as f64;
            avg_loss = (avg_loss * (period - 1) as f64 + loss) / period as f64;

            let rs = if avg_loss == 0.0 {
                100.0
            } else {
                avg_gain / avg_loss
            };
            let rsi = 100.0 - (100.0 / (1.0 + rs));
            result.push(Some(rsi));
        }

        result
    }

    /// MACD (Moving Average Convergence Divergence)
    ///
    /// # Arguments
    ///
    /// * `fast` - Fast EMA period (typically 12)
    /// * `slow` - Slow EMA period (typically 26)
    /// * `signal` - Signal line EMA period (typically 9)
    ///
    /// # Returns
    ///
    /// MACD result containing MACD line, signal line, and histogram
    ///
    /// # Example
    ///
    /// ```rust
    /// let macd = calculator.macd(12, 26, 9);
    /// ```
    pub fn macd(&self, fast: usize, slow: usize, signal: usize) -> MacdResult {
        let ema_fast = self.ema(fast);
        let ema_slow = self.ema(slow);

        // Calculate MACD line
        let mut macd_line = Vec::with_capacity(self.klines.len());
        for i in 0..self.klines.len() {
            if let (Some(fast_val), Some(slow_val)) = (ema_fast[i], ema_slow[i]) {
                macd_line.push(Some(fast_val - slow_val));
            } else {
                macd_line.push(None);
            }
        }

        // Calculate signal line (EMA of MACD)
        let macd_values: Vec<f64> = macd_line
            .iter()
            .filter_map(|v| *v)
            .collect();

        let mut signal_line_values = Vec::new();
        if !macd_values.is_empty() && macd_values.len() >= signal {
            let multiplier = 2.0 / (signal as f64 + 1.0);
            let mut ema = macd_values.iter().take(signal).sum::<f64>() / signal as f64;

            for _ in 0..signal {
                signal_line_values.push(ema);
            }

            for i in signal..macd_values.len() {
                ema = (macd_values[i] - ema) * multiplier + ema;
                signal_line_values.push(ema);
            }
        }

        // Align signal line with original data
        let mut signal_line = vec![None; self.klines.len()];
        let signal_start = signal - 1;

        for (i, val) in signal_line_values.iter().enumerate() {
            let offset = slow + signal_start + i - 1;
            if offset < signal_line.len() {
                signal_line[offset] = Some(*val);
            }
        }

        // Calculate histogram
        let mut histogram = Vec::with_capacity(self.klines.len());
        for i in 0..self.klines.len() {
            if let (Some(macd_val), Some(signal_val)) = (macd_line[i], signal_line[i]) {
                histogram.push(Some(macd_val - signal_val));
            } else {
                histogram.push(None);
            }
        }

        MacdResult {
            macd: macd_line,
            signal: signal_line,
            histogram,
        }
    }

    /// ==================== Volatility Indicators ====================

    /// Bollinger Bands
    ///
    /// # Arguments
    ///
    /// * `period` - Period for SMA and standard deviation (typically 20)
    /// * `std_dev` - Number of standard deviations (typically 2)
    ///
    /// # Returns
    ///
    /// Bollinger Bands result containing upper, middle, and lower bands
    ///
    /// # Example
    ///
    /// ```rust
    /// let bb = calculator.bollinger_bands(20, 2.0);
    /// ```
    pub fn bollinger_bands(&self, period: usize, std_dev: f64) -> BollingerBandsResult {
        let sma = self.sma(period);
        let closes = self.closes();

        let mut upper_band = vec![None; self.klines.len()];
        let mut lower_band = vec![None; self.klines.len()];

        for i in (period - 1)..closes.len() {
            // Calculate standard deviation
            let slice = &closes[i - period + 1..=i];
            let mean: f64 = sma[i].unwrap();
            let variance: f64 = slice.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / period as f64;
            let std = variance.sqrt();

            upper_band[i] = Some(mean + std_dev * std);
            lower_band[i] = Some(mean - std_dev * std);
        }

        BollingerBandsResult {
            upper: upper_band,
            middle: sma,
            lower: lower_band,
        }
    }

    /// Average True Range (ATR)
    ///
    /// Measures market volatility
    ///
    /// # Arguments
    ///
    /// * `period` - ATR period (typically 14)
    ///
    /// # Returns
    ///
    /// A vector of optional ATR values
    ///
    /// # Example
    ///
    /// ```rust
    /// let atr14 = calculator.atr(14);
    /// ```
    pub fn atr(&self, period: usize) -> Vec<Option<f64>> {
        if period == 0 || self.klines.len() <= period {
            return vec![None; self.klines.len()];
        }

        let mut true_ranges: Vec<f64> = Vec::with_capacity(self.klines.len());

        // Calculate true range for each bar
        for i in 0..self.klines.len() {
            if i == 0 {
                true_ranges.push(self.klines[i].high - self.klines[i].low);
            } else {
                let high_low = self.klines[i].high - self.klines[i].low;
                let high_close = (self.klines[i].high - self.klines[i - 1].close).abs();
                let low_close = (self.klines[i].low - self.klines[i - 1].close).abs();

                true_ranges.push(high_low.max(high_close).max(low_close));
            }
        }

        let mut result = vec![None; period];

        // Calculate initial ATR using SMA
        let atr_sum: f64 = true_ranges.iter().take(period).sum();
        let mut atr = atr_sum / period as f64;
        result.push(Some(atr));

        // Calculate subsequent ATR values using Wilder's smoothing
        for i in period..true_ranges.len() {
            atr = (atr * (period - 1) as f64 + true_ranges[i]) / period as f64;
            result.push(Some(atr));
        }

        result
    }

    /// Keltner Channels
    ///
    /// Similar to Bollinger Bands but uses ATR instead of standard deviation
    pub fn keltner_channels(&self, period: usize, multiplier: f64) -> KeltnerChannelsResult {
        let ema = self.ema(period);
        let atr = self.atr(period);

        let mut upper_band = vec![None; self.klines.len()];
        let mut lower_band = vec![None; self.klines.len()];

        for i in 0..self.klines.len() {
            if let (Some(ema_val), Some(atr_val)) = (ema[i], atr[i]) {
                upper_band[i] = Some(ema_val + multiplier * atr_val);
                lower_band[i] = Some(ema_val - multiplier * atr_val);
            }
        }

        KeltnerChannelsResult {
            upper: upper_band,
            middle: ema,
            lower: lower_band,
        }
    }

    /// ==================== Volume Indicators ====================

    /// On-Balance Volume (OBV)
    ///
    /// Measures buying and selling pressure
    pub fn obv(&self) -> Vec<Option<f64>> {
        if self.klines.is_empty() {
            return vec![];
        }

        let mut result = Vec::with_capacity(self.klines.len());
        let mut obv = 0.0;

        for i in 0..self.klines.len() {
            if i == 0 {
                obv = self.klines[i].volume;
            } else {
                if self.klines[i].close > self.klines[i - 1].close {
                    obv += self.klines[i].volume;
                } else if self.klines[i].close < self.klines[i - 1].close {
                    obv -= self.klines[i].volume;
                }
                // If close is equal, OBV doesn't change
            }
            result.push(Some(obv));
        }

        result
    }

    /// Volume Moving Average
    pub fn volume_ma(&self, period: usize) -> Vec<Option<f64>> {
        if period == 0 || self.klines.len() < period {
            return vec![None; self.klines.len()];
        }

        let volumes: Vec<f64> = self.klines.iter().map(|k| k.volume).collect();
        let mut result = vec![None; period - 1];
        let mut sum: f64 = volumes.iter().take(period).sum();

        result.push(Some(sum / period as f64));

        for i in period..volumes.len() {
            sum = sum - volumes[i - period] + volumes[i];
            result.push(Some(sum / period as f64));
        }

        result
    }

    /// ==================== Helper Methods ====================

    /// Get the latest value from an indicator vector
    pub fn latest(&self, values: &[Option<f64>]) -> Option<f64> {
        values.iter().rev().find_map(|v| *v)
    }

    /// Get indicator value at a specific index
    pub fn at(&self, values: &[Option<f64>], index: usize) -> Option<f64> {
        values.get(index).and_then(|v| *v)
    }
}

/// MACD result containing MACD line, signal line, and histogram
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacdResult {
    /// MACD line (fast EMA - slow EMA)
    pub macd: Vec<Option<f64>>,
    /// Signal line (EMA of MACD)
    pub signal: Vec<Option<f64>>,
    /// Histogram (MACD - Signal)
    pub histogram: Vec<Option<f64>>,
}

/// Bollinger Bands result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BollingerBandsResult {
    /// Upper band
    pub upper: Vec<Option<f64>>,
    /// Middle band (SMA)
    pub middle: Vec<Option<f64>>,
    /// Lower band
    pub lower: Vec<Option<f64>>,
}

/// Keltner Channels result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeltnerChannelsResult {
    /// Upper channel
    pub upper: Vec<Option<f64>>,
    /// Middle line (EMA)
    pub middle: Vec<Option<f64>>,
    /// Lower channel
    pub lower: Vec<Option<f64>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_klines() -> Vec<Kline> {
        vec![
            Kline {
                symbol: "BTCUSDT".to_string(),
                timeframe: "1h".to_string(),
                timestamp: 1000,
                open: 100.0,
                high: 105.0,
                low: 98.0,
                close: 103.0,
                volume: 1000.0,
                quote_volume: None,
            },
            Kline {
                symbol: "BTCUSDT".to_string(),
                timeframe: "1h".to_string(),
                timestamp: 2000,
                open: 103.0,
                high: 108.0,
                low: 102.0,
                close: 106.0,
                volume: 1200.0,
                quote_volume: None,
            },
            Kline {
                symbol: "BTCUSDT".to_string(),
                timeframe: "1h".to_string(),
                timestamp: 3000,
                open: 106.0,
                high: 110.0,
                low: 105.0,
                close: 109.0,
                volume: 900.0,
                quote_volume: None,
            },
            Kline {
                symbol: "BTCUSDT".to_string(),
                timeframe: "1h".to_string(),
                timestamp: 4000,
                open: 109.0,
                high: 112.0,
                low: 108.0,
                close: 110.0,
                volume: 1100.0,
                quote_volume: None,
            },
            Kline {
                symbol: "BTCUSDT".to_string(),
                timeframe: "1h".to_string(),
                timestamp: 5000,
                open: 110.0,
                high: 115.0,
                low: 109.0,
                close: 114.0,
                volume: 1300.0,
                quote_volume: None,
            },
        ]
    }

    #[test]
    fn test_sma() {
        let klines = create_test_klines();
        let calculator = IndicatorCalculator::new(klines);

        let sma3 = calculator.sma(3);

        // First 2 values should be None
        assert!(sma3[0].is_none());
        assert!(sma3[1].is_none());

        // Third value: (103 + 106 + 109) / 3 = 106
        assert!((sma3[2].unwrap() - 106.0).abs() < 0.01);

        // Fourth value: (106 + 109 + 110) / 3 = 108.33
        assert!((sma3[3].unwrap() - 108.33).abs() < 0.01);

        // Fifth value: (109 + 110 + 114) / 3 = 111
        assert!((sma3[4].unwrap() - 111.0).abs() < 0.01);
    }

    #[test]
    fn test_ema() {
        let klines = create_test_klines();
        let calculator = IndicatorCalculator::new(klines);

        let ema3 = calculator.ema(3);

        // First 2 values should be None
        assert!(ema3[0].is_none());
        assert!(ema3[1].is_none());

        // Third value starts with SMA then applies EMA formula
        assert!(ema3[2].is_some());
        assert!(ema3[3].is_some());
        assert!(ema3[4].is_some());
    }

    #[test]
    fn test_rsi() {
        // Create test data with alternating price movements
        let klines = (0..20)
            .map(|i| Kline {
                symbol: "BTCUSDT".to_string(),
                timeframe: "1h".to_string(),
                timestamp: (i * 1000) as i64,
                open: 100.0 + i as f64,
                high: 105.0 + i as f64,
                low: 98.0 + i as f64,
                close: 100.0 + i as f64 + if i % 2 == 0 { 2.0 } else { -1.0 },
                volume: 1000.0,
                quote_volume: None,
            })
            .collect();

        let calculator = IndicatorCalculator::new(klines);
        let rsi14 = calculator.rsi(14);

        // First 14 values should be None
        assert!(rsi14[0].is_none());
        assert!(rsi14[13].is_none());

        // RSI should be between 0 and 100
        if let Some(val) = rsi14[14] {
            assert!(val >= 0.0 && val <= 100.0);
        }
    }

    #[test]
    fn test_atr() {
        let klines = create_test_klines();
        let calculator = IndicatorCalculator::new(klines);

        let atr2 = calculator.atr(2);

        // First 2 values should be None
        assert!(atr2[0].is_none());
        assert!(atr2[1].is_none());

        // ATR should be positive
        if let Some(val) = atr2[2] {
            assert!(val > 0.0);
        }
    }

    #[test]
    fn test_bollinger_bands() {
        let klines = create_test_klines();
        let calculator = IndicatorCalculator::new(klines);

        let bb = calculator.bollinger_bands(3, 2.0);

        // Upper band should be >= middle band >= lower band
        for i in 0..bb.upper.len() {
            if let (Some(upper), Some(middle), Some(lower)) =
                (bb.upper[i], bb.middle[i], bb.lower[i])
            {
                assert!(upper >= middle);
                assert!(middle >= lower);
            }
        }
    }

    #[test]
    fn test_obv() {
        let klines = create_test_klines();
        let calculator = IndicatorCalculator::new(klines);

        let obv = calculator.obv();

        // All values should be Some
        assert!(obv.iter().all(|v| v.is_some()));

        // OBV should increase when price increases
        assert!(obv[1].unwrap() > obv[0].unwrap());
    }

    #[test]
    fn test_macd() {
        let klines = create_test_klines();
        let klines_len = klines.len();
        let calculator = IndicatorCalculator::new(klines);

        let macd = calculator.macd(2, 3, 2);

        // MACD result should have correct length
        assert_eq!(macd.macd.len(), klines_len);
        assert_eq!(macd.signal.len(), klines_len);
        assert_eq!(macd.histogram.len(), klines_len);
    }
}
