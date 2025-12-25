use anyhow::Result;
use chrono::Utc;
use serde_json::Value;
use sqlx::{Pool, Sqlite};
use uuid::Uuid;

/// 审计日志记录器
pub struct AuditLogger {
    pool: Pool<Sqlite>,
}

impl AuditLogger {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// 记录操作日志
    pub async fn log(&self, entry: AuditLogEntry) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO audit_logs (
                id, user_id, username, operation_type, resource_type, resource_id,
                before_data, after_data, result, error_message, ip_address, user_agent, timestamp
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&entry.id)
        .bind(&entry.user_id)
        .bind(&entry.username)
        .bind(&entry.operation_type)
        .bind(&entry.resource_type)
        .bind(&entry.resource_id)
        .bind(&entry.before_data)
        .bind(&entry.after_data)
        .bind(&entry.result)
        .bind(&entry.error_message)
        .bind(&entry.ip_address)
        .bind(&entry.user_agent)
        .bind(entry.timestamp)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// 记录成功操作
    pub async fn log_success(
        &self,
        user_id: &str,
        username: &str,
        operation_type: &str,
        resource_type: &str,
        resource_id: Option<&str>,
        before_data: Option<Value>,
        after_data: Option<Value>,
    ) -> Result<()> {
        let entry = AuditLogEntry {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            username: username.to_string(),
            operation_type: operation_type.to_string(),
            resource_type: resource_type.to_string(),
            resource_id: resource_id.map(|s| s.to_string()),
            before_data: before_data.map(|v| v.to_string()),
            after_data: after_data.map(|v| v.to_string()),
            result: "success".to_string(),
            error_message: None,
            ip_address: None,
            user_agent: None,
            timestamp: Utc::now().timestamp(),
        };
        self.log(entry).await
    }

    /// 记录失败操作
    pub async fn log_failure(
        &self,
        user_id: &str,
        username: &str,
        operation_type: &str,
        resource_type: &str,
        error_message: &str,
    ) -> Result<()> {
        let entry = AuditLogEntry {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            username: username.to_string(),
            operation_type: operation_type.to_string(),
            resource_type: resource_type.to_string(),
            resource_id: None,
            before_data: None,
            after_data: None,
            result: "failure".to_string(),
            error_message: Some(error_message.to_string()),
            ip_address: None,
            user_agent: None,
            timestamp: Utc::now().timestamp(),
        };
        self.log(entry).await
    }

    /// 记录带完整信息的失败操作
    pub async fn log_failure_full(
        &self,
        user_id: &str,
        username: &str,
        operation_type: &str,
        resource_type: &str,
        resource_id: Option<&str>,
        error_message: &str,
        before_data: Option<Value>,
    ) -> Result<()> {
        let entry = AuditLogEntry {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            username: username.to_string(),
            operation_type: operation_type.to_string(),
            resource_type: resource_type.to_string(),
            resource_id: resource_id.map(|s| s.to_string()),
            before_data: before_data.map(|v| v.to_string()),
            after_data: None,
            result: "failure".to_string(),
            error_message: Some(error_message.to_string()),
            ip_address: None,
            user_agent: None,
            timestamp: Utc::now().timestamp(),
        };
        self.log(entry).await
    }
}

/// 审计日志条目
#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub id: String,
    pub user_id: String,
    pub username: String,
    pub operation_type: String,
    pub resource_type: String,
    pub resource_id: Option<String>,
    pub before_data: Option<String>,
    pub after_data: Option<String>,
    pub result: String,
    pub error_message: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub timestamp: i64,
}

/// 操作类型常量
impl AuditLogger {
    // 登录操作
    pub const OP_LOGIN: &'static str = "login";
    pub const OP_LOGOUT: &'static str = "logout";

    // 用户操作
    pub const OP_USER_CREATE: &'static str = "user.create";
    pub const OP_USER_UPDATE: &'static str = "user.update";
    pub const OP_USER_DELETE: &'static str = "user.delete";
    pub const OP_USER_READ: &'static str = "user.read";

    // 策略操作
    pub const OP_STRATEGY_CREATE: &'static str = "strategy.create";
    pub const OP_STRATEGY_UPDATE: &'static str = "strategy.update";
    pub const OP_STRATEGY_DELETE: &'static str = "strategy.delete";
    pub const OP_STRATEGY_READ: &'static str = "strategy.read";

    // 回测操作
    pub const OP_BACKTEST_EXECUTE: &'static str = "backtest.execute";

    // 交易操作
    pub const OP_TRADE_EXECUTE: &'static str = "trade.execute";
    pub const OP_ORDER_CREATE: &'static str = "order.create";
    pub const OP_ORDER_CANCEL: &'static str = "order.cancel";

    // 配置操作
    pub const OP_CONFIG_UPDATE: &'static str = "config.update";
}

/// 资源类型常量
impl AuditLogger {
    pub const RESOURCE_USER: &'static str = "user";
    pub const RESOURCE_STRATEGY: &'static str = "strategy";
    pub const RESOURCE_BACKTEST: &'static str = "backtest";
    pub const RESOURCE_ORDER: &'static str = "order";
    pub const RESOURCE_POSITION: &'static str = "position";
    pub const RESOURCE_EXCHANGE: &'static str = "exchange";
    pub const RESOURCE_CONFIG: &'static str = "config";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_constants() {
        assert_eq!(AuditLogger::OP_LOGIN, "login");
        assert_eq!(AuditLogger::OP_USER_CREATE, "user.create");
        assert_eq!(AuditLogger::OP_STRATEGY_UPDATE, "strategy.update");
    }

    #[test]
    fn test_resource_constants() {
        assert_eq!(AuditLogger::RESOURCE_USER, "user");
        assert_eq!(AuditLogger::RESOURCE_STRATEGY, "strategy");
        assert_eq!(AuditLogger::RESOURCE_ORDER, "order");
    }
}
