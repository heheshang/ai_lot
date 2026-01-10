//! Backtest Service
//!
//! Service for running strategy backtests with historical data

use crate::core::trade::types::Kline;
use crate::core::strategy::{ScriptExecutor, IndicatorCalculator};
use crate::core::Signal;
use crate::types::backtest::*;
use crate::infrastructure::Database;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Backtest service
pub struct BacktestService {
    db: Database,
    jobs: Arc<RwLock<HashMap<String, BacktestJob>>>,
}

impl BacktestService {
    /// Create a new backtest service
    pub fn new(db: Database) -> Self {
        Self {
            db,
            jobs: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new backtest job
    pub async fn create_job(&self, config: BacktestConfig) -> Result<String> {
        let job = BacktestJob::new(config.clone());
        let job_id = job.id.clone();

        self.jobs.write().await.insert(job_id.clone(), job);

        log::info!("Created backtest job: {}", job_id);
        Ok(job_id)
    }

    /// Get a backtest job by ID
    pub async fn get_job(&self, job_id: &str) -> Option<BacktestJob> {
        self.jobs.read().await.get(job_id).cloned()
    }

    /// List all backtest jobs
    pub async fn list_jobs(&self) -> Vec<BacktestJob> {
        self.jobs.read().await.values().cloned().collect()
    }

    /// Run a backtest job
    pub async fn run_job(&self, job_id: &str) -> Result<BacktestResult> {
        let mut job = self.get_job(job_id).await
            .ok_or_else(|| anyhow!("Job not found: {}", job_id))?;

        if job.status == BacktestStatus::Running {
            return Err(anyhow!("Job is already running"));
        }

        job.update_status(BacktestStatus::Running);
        self.jobs.write().await.insert(job_id.to_string(), job.clone());

        // Load historical data
        let klines = self.load_historical_data(
            &job.config.symbol,
            &job.config.timeframe,
            job.config.start_time,
            job.config.end_time,
        ).await?;

        log::info!("Loaded {} klines for backtest", klines.len());

        // Run backtest
        let result = self.run_backtest(&job.config, klines)?;

        // Update job with result
        let mut jobs = self.jobs.write().await;
        if let Some(job) = jobs.get_mut(job_id) {
            job.set_result(result.clone());
        }

        Ok(result)
    }

    /// Load historical kline data from database or API
    async fn load_historical_data(
        &self,
        symbol: &str,
        timeframe: &str,
        start_time: i64,
        end_time: i64,
    ) -> Result<Vec<Kline>> {
        // Try to load from cache first
        let cached = self.load_cached_klines(symbol, timeframe, start_time, end_time).await?;
        if !cached.is_empty() {
            return Ok(cached);
        }

        // If no cached data, fetch from exchange
        // For now, return empty vec - should be implemented with exchange API
        Ok(Vec::new())
    }

    /// Load cached klines from database
    async fn load_cached_klines(
        &self,
        symbol: &str,
        timeframe: &str,
        start_time: i64,
        end_time: i64,
    ) -> Result<Vec<Kline>> {
        let query = r#"
            SELECT symbol, timeframe, timestamp, open, high, low, close, volume
            FROM klines
            WHERE symbol = ? AND timeframe = ?
            AND timestamp >= ? AND timestamp <= ?
            ORDER BY timestamp ASC
        "#;

        let rows = sqlx::query_as::<_, (String, String, i64, f64, f64, f64, f64, f64)>(query)
            .bind(symbol)
            .bind(timeframe)
            .bind(start_time)
            .bind(end_time)
            .fetch_all(&self.db.pool)
            .await
            .map_err(|e| anyhow!("Failed to load klines: {}", e))?;

        let klines = rows.into_iter().map(|(symbol, timeframe, timestamp, open, high, low, close, volume)| {
            Kline {
                symbol,
                timeframe,
                timestamp,
                open,
                high,
                low,
                close,
                volume,
                quote_volume: None,
            }
        }).collect();

        Ok(klines)
    }

    /// Run the actual backtest
    fn run_backtest(&self, config: &BacktestConfig, klines: Vec<Kline>) -> Result<BacktestResult> {
        if klines.is_empty() {
            return Err(anyhow!("No historical data available"));
        }

        let executor = ScriptExecutor::new()?;

        // Get strategy code from database
        let code = self.get_strategy_code(&config.strategy_id)?;

        // Initialize backtest state
        let mut state = BacktestState::new(config.initial_capital, config.fee_rate, config.slippage);

        // Process each kline
        for (i, kline) in klines.iter().enumerate() {
            // Update progress
            if i % 100 == 0 {
                log::debug!("Processing kline {}/{}", i, klines.len());
            }

            // Calculate indicators with all history up to this point
            let history = &klines[..=i];
            let _calculator = IndicatorCalculator::new(history.to_vec());

            // Execute strategy
            let signal = self.execute_strategy(&executor, &code, config, kline, history)?;

            // Process signal
            if let Some(signal) = signal {
                self.process_signal(&mut state, config, kline, &signal)?;
            }

            // Update equity curve
            state.update_equity(kline.timestamp);

            // Check stop loss / take profit
            self.check_exits(&mut state, kline, config);
        }

        // Close all remaining positions
        self.close_all_positions(&mut state, klines.last().unwrap())?;

        // Calculate final result
        let result = self.calculate_result(config, &state, &klines)?;
        Ok(result)
    }

    /// Get strategy code from database
    fn get_strategy_code(&self, _strategy_id: &str) -> Result<String> {
        // For now, return mock code
        // TODO: Load from database
        Ok(r#"
function onInit(context) {
    context.storage.set('initialized', 'true');
}

function onBar(context, kline) {
    const sma20 = context.indicators.sma(20);
    const sma50 = context.indicators.sma(50);

    const lastSMA20 = context.indicators.latest(sma20);
    const lastSMA50 = context.indicators.latest(sma50);

    if (!lastSMA20 || !lastSMA50) return null;

    // Golden cross
    if (lastSMA20 > lastSMA50) {
        return {
            symbol: kline.symbol,
            action: 'buy',
            quantity: 0.1,
            price: kline.close
        };
    }

    // Death cross
    if (lastSMA20 < lastSMA50) {
        return {
            symbol: kline.symbol,
            action: 'sell',
            quantity: 0.1,
            price: kline.close
        };
    }

    return null;
}

function onStop(context) {}
        "#.to_string())
    }

    /// Execute strategy on a single kline
    fn execute_strategy(
        &self,
        executor: &ScriptExecutor,
        code: &str,
        config: &BacktestConfig,
        kline: &Kline,
        history: &[Kline],
    ) -> Result<Option<Signal>> {
        // Convert config to JSON value
        let params = serde_json::json!({
            "symbol": config.symbol,
            "timeframe": config.timeframe,
            "initial_capital": config.initial_capital,
            "fee_rate": config.fee_rate,
            "max_positions": config.max_positions,
        });

        executor.on_bar(code, kline, &params, history)
    }

    /// Process a trading signal
    fn process_signal(
        &self,
        state: &mut BacktestState,
        config: &BacktestConfig,
        kline: &Kline,
        signal: &Signal,
    ) -> Result<()> {
        let price = signal.price.unwrap_or(kline.close);
        let quantity = signal.quantity;

        // Apply slippage
        let adjusted_price = if signal.action == "buy" {
            price * (1.0 + config.slippage / 100.0)
        } else {
            price * (1.0 - config.slippage / 100.0)
        };

        match signal.action.as_str() {
            "buy" => {
                // Check if we can open a position
                if state.positions.len() >= config.max_positions {
                    log::debug!("Max positions reached, skipping buy signal");
                    return Ok(());
                }

                // Calculate position size
                let max_value = state.balance * (config.max_position_ratio / 100.0);
                let position_value = (adjusted_price * quantity).min(max_value);
                let actual_quantity = position_value / adjusted_price;

                // Calculate fee
                let fee = position_value * (config.fee_rate / 100.0);

                if state.balance >= position_value + fee {
                    state.balance -= position_value + fee;
                    state.total_fees += fee;

                    let position = BacktestPosition {
                        entry_price: adjusted_price,
                        quantity: actual_quantity,
                        side: "buy".to_string(),
                        stop_loss: if config.stop_loss_ratio > 0.0 {
                            Some(adjusted_price * (1.0 - config.stop_loss_ratio / 100.0))
                        } else {
                            None
                        },
                    };

                    state.positions.push(position);
                    log::debug!("Opened BUY position: {} @ {}", actual_quantity, adjusted_price);
                }
            }
            "sell" => {
                // Close existing buy positions
                let total_quantity: f64 = state.positions.iter()
                    .filter(|p| p.side == "buy")
                    .map(|p| p.quantity)
                    .sum();

                let close_quantity = quantity.min(total_quantity);

                if close_quantity > 0.0 {
                    let mut remaining = close_quantity;
                    let mut closed_pnl = 0.0;

                    for position in &mut state.positions {
                        if position.side == "buy" && remaining > 0.0 {
                            let close_qty = position.quantity.min(remaining);

                            let pnl = (adjusted_price - position.entry_price) * close_qty;
                            let fee = (adjusted_price * close_qty) * (config.fee_rate / 100.0);

                            closed_pnl += pnl - fee;
                            state.balance += (adjusted_price * close_qty) - fee;
                            state.total_fees += fee;

                            remaining -= close_qty;
                            position.quantity -= close_qty;

                            log::debug!("Closed position: {} @ {}, PnL: {}", close_qty, adjusted_price, pnl);
                        }
                    }

                    // Remove closed positions
                    state.positions.retain(|p| p.quantity > 0.001);

                    state.trades.push(TradeDetail {
                        id: state.trades.len() + 1,
                        entry_time: kline.timestamp,
                        exit_time: Some(kline.timestamp),
                        side: "sell".to_string(),
                        entry_price: 0.0,
                        exit_price: Some(adjusted_price),
                        quantity: close_quantity,
                        value: adjusted_price * close_quantity,
                        fee: (adjusted_price * close_quantity) * (config.fee_rate / 100.0),
                        pnl: Some(closed_pnl),
                        balance: state.balance,
                        exit_reason: Some("signal".to_string()),
                    });
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Check for stop loss / take profit exits
    fn check_exits(&self, state: &mut BacktestState, kline: &Kline, _config: &BacktestConfig) {
        for position in &mut state.positions {
            if position.side != "buy" {
                continue;
            }

            // Check stop loss
            if let Some(stop_loss) = position.stop_loss {
                if kline.low <= stop_loss {
                    log::debug!("Stop loss triggered at {}", kline.low);
                    // Position will be closed in next iteration
                }
            }
        }
    }

    /// Close all positions at the end of backtest
    fn close_all_positions(&self, state: &mut BacktestState, _last_kline: &Kline) -> Result<()> {
        for position in &state.positions {
            let value = position.entry_price * position.quantity;
            state.balance += value;
        }

        state.positions.clear();
        Ok(())
    }

    /// Calculate final backtest result
    fn calculate_result(&self, config: &BacktestConfig, state: &BacktestState, _klines: &[Kline]) -> Result<BacktestResult> {
        let final_capital = state.balance;
        let profit = final_capital - config.initial_capital;
        let total_return = (profit / config.initial_capital) * 100.0;

        // Calculate drawdown
        let (max_drawdown, avg_drawdown) = self.calculate_drawdown(&state.equity_curve);

        // Calculate trade statistics
        let winning_trades = state.trades.iter().filter(|t| t.pnl.unwrap_or(0.0) > 0.0).count();
        let losing_trades = state.trades.iter().filter(|t| t.pnl.unwrap_or(0.0) < 0.0).count();
        let total_trades = state.trades.len();

        let total_wins: f64 = state.trades.iter().filter_map(|t| t.pnl).filter(|&p| p > 0.0).sum();
        let total_losses: f64 = state.trades.iter().filter_map(|t| t.pnl).filter(|&p| p < 0.0).map(|p| p.abs()).sum();

        let avg_win = if winning_trades > 0 { total_wins / winning_trades as f64 } else { 0.0 };
        let avg_loss = if losing_trades > 0 { total_losses / losing_trades as f64 } else { 0.0 };
        let profit_factor = if total_losses > 0.0 { total_wins / total_losses } else { 0.0 };

        let win_rate = if total_trades > 0 {
            (winning_trades as f64 / total_trades as f64) * 100.0
        } else {
            0.0
        };

        // Calculate consecutive wins/losses
        let (max_consecutive_wins, max_consecutive_losses) = self.calculate_consecutive(&state.trades);

        // Calculate Sharpe ratio (simplified)
        let sharpe_ratio = if total_trades > 1 {
            let returns: Vec<f64> = state.trades.iter().filter_map(|t| t.pnl).collect();
            let avg_return = returns.iter().sum::<f64>() / returns.len() as f64;
            let variance = returns.iter().map(|r| (r - avg_return).powi(2)).sum::<f64>() / returns.len() as f64;
            let std_dev = variance.sqrt();

            if std_dev > 0.0 {
                (avg_return / std_dev) * (255.0_f64).sqrt() // Annualized
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Create equity and drawdown curves
        let equity_curve: Vec<EquityPoint> = state.equity_curve.iter().map(|(t, e)| {
            EquityPoint {
                time: *t,
                equity: *e,
            }
        }).collect();

        let drawdown_curve: Vec<DrawdownPoint> = state.drawdown_curve.iter().map(|(t, d)| {
            DrawdownPoint {
                time: *t,
                drawdown: *d,
            }
        }).collect();

        Ok(BacktestResult {
            id: format!("bt_{}", Uuid::new_v4().simple()),
            strategy_id: config.strategy_id.clone(),
            symbol: config.symbol.clone(),
            timeframe: config.timeframe.clone(),
            start_time: config.start_time,
            end_time: config.end_time,

            initial_capital: config.initial_capital,
            final_capital,
            profit,
            total_return,
            peak_capital: state.peak_equity,
            trough_capital: state.trough_equity,

            max_drawdown,
            avg_drawdown,
            max_drawdown_duration: 0,
            sharpe_ratio,
            sortino_ratio: None,
            calmar_ratio: None,

            total_trades,
            winning_trades,
            losing_trades,
            win_rate,
            avg_win,
            avg_loss,
            profit_factor,
            expected_value: if total_trades > 0 { profit / total_trades as f64 } else { 0.0 },
            max_consecutive_wins,
            max_consecutive_losses,
            max_single_win: state.trades.iter().filter_map(|t| t.pnl).filter(|&p| p > 0.0).fold(0.0_f64, f64::max),
            max_single_loss: state.trades.iter().filter_map(|t| t.pnl).filter(|&p| p < 0.0).fold(0.0_f64, f64::min),
            avg_capital_utilization: 50.0, // Placeholder

            trades: state.trades.clone(),
            equity_curve,
            drawdown_curve,
            monthly_returns: Vec::new(), // TODO: Implement
        })
    }

    fn calculate_drawdown(&self, equity_curve: &Vec<(i64, f64)>) -> (f64, f64) {
        if equity_curve.is_empty() {
            return (0.0, 0.0);
        }

        let mut peak = equity_curve[0].1;
        let mut max_dd: f64 = 0.0;
        let mut total_dd = 0.0;
        let mut dd_count = 0;

        for (_, equity) in equity_curve {
            if *equity > peak {
                peak = *equity;
            }

            let dd = if peak > 0.0 {
                ((peak - equity) / peak) * 100.0
            } else {
                0.0
            };

            if dd > 0.0 {
                total_dd += dd;
                dd_count += 1;
            }

            max_dd = max_dd.max(dd);
        }

        let avg_dd = if dd_count > 0 { total_dd / dd_count as f64 } else { 0.0 };

        (max_dd, avg_dd)
    }

    fn calculate_consecutive(&self, trades: &[TradeDetail]) -> (usize, usize) {
        let mut max_wins = 0;
        let mut max_losses = 0;
        let mut current_wins = 0;
        let mut current_losses = 0;

        for trade in trades {
            match trade.pnl {
                Some(pnl) if pnl > 0.0 => {
                    current_wins += 1;
                    current_losses = 0;
                    max_wins = max_wins.max(current_wins);
                }
                Some(pnl) if pnl < 0.0 => {
                    current_losses += 1;
                    current_wins = 0;
                    max_losses = max_losses.max(current_losses);
                }
                _ => {}
            }
        }

        (max_wins, max_losses)
    }
}

/// Internal backtest state
struct BacktestState {
    balance: f64,
    peak_equity: f64,
    trough_equity: f64,
    positions: Vec<BacktestPosition>,
    trades: Vec<TradeDetail>,
    equity_curve: Vec<(i64, f64)>,
    drawdown_curve: Vec<(i64, f64)>,
    total_fees: f64,
}

impl BacktestState {
    fn new(initial_capital: f64, _fee_rate: f64, _slippage: f64) -> Self {
        Self {
            balance: initial_capital,
            peak_equity: initial_capital,
            trough_equity: initial_capital,
            positions: Vec::new(),
            trades: Vec::new(),
            equity_curve: Vec::new(),
            drawdown_curve: Vec::new(),
            total_fees: 0.0,
        }
    }

    fn update_equity(&mut self, timestamp: i64) {
        let equity = self.balance + self.positions.iter()
            .map(|p| p.entry_price * p.quantity)
            .sum::<f64>();

        self.peak_equity = self.peak_equity.max(equity);
        self.trough_equity = self.trough_equity.min(equity);

        self.equity_curve.push((timestamp, equity));

        let drawdown = if self.peak_equity > 0.0 {
            ((self.peak_equity - equity) / self.peak_equity) * 100.0
        } else {
            0.0
        };

        self.drawdown_curve.push((timestamp, -drawdown));
    }
}

/// Backtest position
struct BacktestPosition {
    entry_price: f64,
    quantity: f64,
    side: String,
    stop_loss: Option<f64>,
}
