use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// 策略实例模型（对应数据库表 strategy_instances）
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StrategyInstance {
    pub id: String,
    pub strategy_id: String,
    pub user_id: String,
    pub name: String,
    pub parameters: String,      // JSON string
    pub exchange_id: String,
    pub symbol: String,
    pub timeframe: String,
    pub mode: String,            // 'paper' or 'live'
    pub status: String,          // 'running', 'stopped', 'error', 'paused'
    pub error_message: Option<String>,
    pub start_time: Option<i64>,
    pub stop_time: Option<i64>,
    pub total_trades: i64,
    pub total_pnl: f64,
    pub max_drawdown: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 策略实例列表项
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StrategyInstanceListItem {
    pub id: String,
    pub name: String,
    pub strategy_id: String,
    pub symbol: String,
    pub timeframe: String,
    pub mode: String,
    pub status: String,
    pub total_trades: i64,
    pub total_pnl: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 创建策略实例请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateInstanceRequest {
    pub strategy_id: String,
    pub user_id: String,
    pub name: String,
    pub parameters: serde_json::Value,
    pub exchange_id: String,
    pub symbol: String,
    pub timeframe: String,
    pub mode: String,
}

/// 更新策略实例请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInstanceRequest {
    pub id: String,
    pub status: Option<String>,
    pub error_message: Option<String>,
    pub total_trades: Option<i64>,
    pub total_pnl: Option<f64>,
    pub max_drawdown: Option<f64>,
}

impl StrategyInstance {
    /// 创建新实例（用于插入）
    pub fn new(req: CreateInstanceRequest) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();
        let parameters = serde_json::to_string(&req.parameters).unwrap_or_else(|_| "{}".to_string());

        Self {
            id,
            strategy_id: req.strategy_id,
            user_id: req.user_id,
            name: req.name,
            parameters,
            exchange_id: req.exchange_id,
            symbol: req.symbol,
            timeframe: req.timeframe,
            mode: req.mode,
            status: "stopped".to_string(),
            error_message: None,
            start_time: None,
            stop_time: None,
            total_trades: 0,
            total_pnl: 0.0,
            max_drawdown: 0.0,
            created_at: now,
            updated_at: now,
        }
    }

    /// 标记为运行中
    pub fn mark_running(&mut self) {
        self.status = "running".to_string();
        self.start_time = Some(chrono::Utc::now().timestamp());
        self.stop_time = None;
        self.error_message = None;
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// 标记为已停止
    pub fn mark_stopped(&mut self) {
        self.status = "stopped".to_string();
        self.stop_time = Some(chrono::Utc::now().timestamp());
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// 标记为错误状态
    pub fn mark_error(&mut self, error: String) {
        self.status = "error".to_string();
        self.error_message = Some(error);
        self.stop_time = Some(chrono::Utc::now().timestamp());
        self.updated_at = chrono::Utc::now().timestamp();
    }

    /// 更新统计数据
    pub fn update_stats(&mut self, trades: i64, pnl: f64, drawdown: f64) {
        self.total_trades = trades;
        self.total_pnl = pnl;
        self.max_drawdown = drawdown;
        self.updated_at = chrono::Utc::now().timestamp();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_instance() {
        let req = CreateInstanceRequest {
            strategy_id: "strategy_1".to_string(),
            user_id: "user_1".to_string(),
            name: "Test Instance".to_string(),
            parameters: serde_json::json!({"param1": "value1"}),
            exchange_id: "exchange_1".to_string(),
            symbol: "BTCUSDT".to_string(),
            timeframe: "1h".to_string(),
            mode: "paper".to_string(),
        };

        let instance = StrategyInstance::new(req);

        assert_eq!(instance.status, "stopped");
        assert_eq!(instance.total_trades, 0);
        assert_eq!(instance.total_pnl, 0.0);
        assert!(instance.start_time.is_none());
        assert!(instance.stop_time.is_none());
    }

    #[test]
    fn test_mark_running() {
        let req = CreateInstanceRequest {
            strategy_id: "strategy_1".to_string(),
            user_id: "user_1".to_string(),
            name: "Test Instance".to_string(),
            parameters: serde_json::json!({}),
            exchange_id: "exchange_1".to_string(),
            symbol: "BTCUSDT".to_string(),
            timeframe: "1h".to_string(),
            mode: "paper".to_string(),
        };

        let mut instance = StrategyInstance::new(req);
        instance.mark_running();

        assert_eq!(instance.status, "running");
        assert!(instance.start_time.is_some());
        assert!(instance.stop_time.is_none());
        assert!(instance.error_message.is_none());
    }

    #[test]
    fn test_mark_stopped() {
        let req = CreateInstanceRequest {
            strategy_id: "strategy_1".to_string(),
            user_id: "user_1".to_string(),
            name: "Test Instance".to_string(),
            parameters: serde_json::json!({}),
            exchange_id: "exchange_1".to_string(),
            symbol: "BTCUSDT".to_string(),
            timeframe: "1h".to_string(),
            mode: "paper".to_string(),
        };

        let mut instance = StrategyInstance::new(req);
        instance.mark_running();
        instance.mark_stopped();

        assert_eq!(instance.status, "stopped");
        assert!(instance.start_time.is_some());
        assert!(instance.stop_time.is_some());
    }

    #[test]
    fn test_mark_error() {
        let req = CreateInstanceRequest {
            strategy_id: "strategy_1".to_string(),
            user_id: "user_1".to_string(),
            name: "Test Instance".to_string(),
            parameters: serde_json::json!({}),
            exchange_id: "exchange_1".to_string(),
            symbol: "BTCUSDT".to_string(),
            timeframe: "1h".to_string(),
            mode: "paper".to_string(),
        };

        let mut instance = StrategyInstance::new(req);
        instance.mark_error("Test error".to_string());

        assert_eq!(instance.status, "error");
        assert_eq!(instance.error_message, Some("Test error".to_string()));
    }

    #[test]
    fn test_update_stats() {
        let req = CreateInstanceRequest {
            strategy_id: "strategy_1".to_string(),
            user_id: "user_1".to_string(),
            name: "Test Instance".to_string(),
            parameters: serde_json::json!({}),
            exchange_id: "exchange_1".to_string(),
            symbol: "BTCUSDT".to_string(),
            timeframe: "1h".to_string(),
            mode: "paper".to_string(),
        };

        let mut instance = StrategyInstance::new(req);
        instance.update_stats(10, 500.0, 50.0);

        assert_eq!(instance.total_trades, 10);
        assert_eq!(instance.total_pnl, 500.0);
        assert_eq!(instance.max_drawdown, 50.0);
    }
}
