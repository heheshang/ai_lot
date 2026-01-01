//! Backtest types
//!
//! Type definitions for backtesting functionality

use serde::{Deserialize, Serialize};
use chrono::Utc;

/// Backtest configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestConfig {
    /// Strategy ID to backtest
    pub strategy_id: String,
    /// Trading symbol (e.g., BTCUSDT)
    pub symbol: String,
    /// Timeframe (e.g., 1h, 4h, 1d)
    pub timeframe: String,
    /// Start timestamp
    pub start_time: i64,
    /// End timestamp
    pub end_time: i64,
    /// Initial capital
    pub initial_capital: f64,
    /// Fee rate (percentage, e.g., 0.1 for 0.1%)
    #[serde(rename = "feeRate")]
    pub fee_rate: f64,
    /// Slippage (percentage)
    pub slippage: f64,
    /// Maximum number of open positions
    #[serde(rename = "maxPositions")]
    pub max_positions: usize,
    /// Maximum position ratio (percentage of capital)
    #[serde(rename = "maxPositionRatio")]
    pub max_position_ratio: f64,
    /// Stop loss ratio (percentage)
    #[serde(rename = "stopLossRatio")]
    pub stop_loss_ratio: f64,
}

/// Backtest result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestResult {
    /// Backtest ID
    pub id: String,
    /// Strategy ID
    #[serde(rename = "strategyId")]
    pub strategy_id: String,
    /// Symbol
    pub symbol: String,
    /// Timeframe
    pub timeframe: String,
    /// Start time
    #[serde(rename = "startTime")]
    pub start_time: i64,
    /// End time
    #[serde(rename = "endTime")]
    pub end_time: i64,

    // ============== Capital Metrics ==============
    /// Initial capital
    #[serde(rename = "initialCapital")]
    pub initial_capital: f64,
    /// Final capital
    #[serde(rename = "finalCapital")]
    pub final_capital: f64,
    /// Total profit/loss
    pub profit: f64,
    /// Total return (percentage)
    #[serde(rename = "totalReturn")]
    pub total_return: f64,
    /// Peak capital
    #[serde(rename = "peakCapital")]
    pub peak_capital: f64,
    /// Trough capital
    #[serde(rename = "troughCapital")]
    pub trough_capital: f64,

    // ============== Risk Metrics ==============
    /// Maximum drawdown (percentage)
    #[serde(rename = "maxDrawdown")]
    pub max_drawdown: f64,
    /// Average drawdown (percentage)
    #[serde(rename = "avgDrawdown")]
    pub avg_drawdown: f64,
    /// Maximum drawdown duration (in bars)
    #[serde(rename = "maxDrawdownDuration")]
    pub max_drawdown_duration: i64,
    /// Sharpe ratio
    #[serde(rename = "sharpeRatio")]
    pub sharpe_ratio: f64,
    /// Sortino ratio
    #[serde(rename = "sortinoRatio")]
    pub sortino_ratio: Option<f64>,
    /// Calmar ratio
    #[serde(rename = "calmarRatio")]
    pub calmar_ratio: Option<f64>,

    // ============== Trade Statistics ==============
    /// Total number of trades
    #[serde(rename = "totalTrades")]
    pub total_trades: usize,
    /// Number of winning trades
    #[serde(rename = "winningTrades")]
    pub winning_trades: usize,
    /// Number of losing trades
    #[serde(rename = "losingTrades")]
    pub losing_trades: usize,
    /// Win rate (percentage)
    #[serde(rename = "winRate")]
    pub win_rate: f64,
    /// Average win
    #[serde(rename = "avgWin")]
    pub avg_win: f64,
    /// Average loss
    #[serde(rename = "avgLoss")]
    pub avg_loss: f64,
    /// Profit factor
    #[serde(rename = "profitFactor")]
    pub profit_factor: f64,
    /// Expected value per trade
    #[serde(rename = "expectedValue")]
    pub expected_value: f64,
    /// Maximum consecutive wins
    #[serde(rename = "maxConsecutiveWins")]
    pub max_consecutive_wins: usize,
    /// Maximum consecutive losses
    #[serde(rename = "maxConsecutiveLosses")]
    pub max_consecutive_losses: usize,
    /// Maximum single win
    #[serde(rename = "maxSingleWin")]
    pub max_single_win: f64,
    /// Maximum single loss
    #[serde(rename = "maxSingleLoss")]
    pub max_single_loss: f64,
    /// Average capital utilization
    #[serde(rename = "avgCapitalUtilization")]
    pub avg_capital_utilization: f64,

    // ============== Detailed Data ==============
    /// List of trades
    pub trades: Vec<TradeDetail>,
    /// Equity curve
    #[serde(rename = "equityCurve")]
    pub equity_curve: Vec<EquityPoint>,
    /// Drawdown curve
    #[serde(rename = "drawdownCurve")]
    pub drawdown_curve: Vec<DrawdownPoint>,
    /// Monthly returns
    #[serde(rename = "monthlyReturns")]
    pub monthly_returns: Vec<MonthlyReturn>,
}

/// Trade detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeDetail {
    /// Trade ID
    pub id: usize,
    /// Entry time
    #[serde(rename = "entryTime")]
    pub entry_time: i64,
    /// Exit time
    #[serde(rename = "exitTime")]
    pub exit_time: Option<i64>,
    /// Side (buy/sell)
    pub side: String,
    /// Entry price
    #[serde(rename = "entryPrice")]
    pub entry_price: f64,
    /// Exit price
    #[serde(rename = "exitPrice")]
    pub exit_price: Option<f64>,
    /// Quantity
    pub quantity: f64,
    /// Position value
    pub value: f64,
    /// Fee
    pub fee: f64,
    /// Profit/Loss
    #[serde(rename = "pnl")]
    pub pnl: Option<f64>,
    /// Balance after trade
    pub balance: f64,
    /// Exit reason
    #[serde(rename = "exitReason")]
    pub exit_reason: Option<String>,
}

/// Equity curve point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityPoint {
    /// Timestamp
    pub time: i64,
    /// Equity value
    pub equity: f64,
}

/// Drawdown point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawdownPoint {
    /// Timestamp
    pub time: i64,
    /// Drawdown percentage (negative value)
    pub drawdown: f64,
}

/// Monthly return
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyReturn {
    /// Year
    pub year: i32,
    /// Month (1-12)
    pub month: u32,
    /// Return percentage
    #[serde(rename = "returnValue")]
    pub return_value: f64,
}

/// Backtest status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BacktestStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

/// Backtest job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktestJob {
    /// Job ID
    pub id: String,
    /// Configuration
    pub config: BacktestConfig,
    /// Status
    pub status: BacktestStatus,
    /// Progress (0-100)
    pub progress: u8,
    /// Result (available when status is Completed)
    pub result: Option<BacktestResult>,
    /// Error message (available when status is Failed)
    pub error: Option<String>,
    /// Created at
    #[serde(rename = "createdAt")]
    pub created_at: i64,
    /// Updated at
    #[serde(rename = "updatedAt")]
    pub updated_at: i64,
}

impl BacktestJob {
    /// Create a new backtest job
    pub fn new(config: BacktestConfig) -> Self {
        let id = format!("bt_{}", uuid::Uuid::new_v4().simple());
        let now = Utc::now().timestamp_millis();

        Self {
            id,
            config,
            status: BacktestStatus::Pending,
            progress: 0,
            result: None,
            error: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Update the job status
    pub fn update_status(&mut self, status: BacktestStatus) {
        self.status = status;
        self.updated_at = Utc::now().timestamp_millis();
    }

    /// Update the progress
    pub fn update_progress(&mut self, progress: u8) {
        self.progress = progress.min(100);
        self.updated_at = Utc::now().timestamp_millis();
    }

    /// Set the result
    pub fn set_result(&mut self, result: BacktestResult) {
        self.result = Some(result);
        self.status = BacktestStatus::Completed;
        self.progress = 100;
        self.updated_at = Utc::now().timestamp_millis();
    }

    /// Set error
    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
        self.status = BacktestStatus::Failed;
        self.updated_at = Utc::now().timestamp_millis();
    }
}
