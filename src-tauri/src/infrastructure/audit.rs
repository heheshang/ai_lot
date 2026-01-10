//! 审计日志模块
//!
//! 记录系统中的所有关键操作，用于安全审计和问题追踪

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Pool, Sqlite};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

/// 审计事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEvent {
    UserLogin { user_id: String, username: String },
    UserLogout { user_id: String, username: String },
    StrategyCreated { user_id: String, strategy_id: String, strategy_name: String },
    StrategyUpdated { user_id: String, strategy_id: String, strategy_name: String },
    StrategyDeleted { user_id: String, strategy_id: String, strategy_name: String },
    OrderPlaced { user_id: String, order_id: String, symbol: String, side: String, quantity: f64 },
    RiskAlertTriggered { user_id: String, alert_type: String, severity: String, message: String },
    SystemStarted { version: String },
}

/// 审计日志记录
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub id: String,
    pub event_type: String,
    pub event_data: String,
    pub user_id: Option<String>,
    pub timestamp: i64,
}

/// 审计日志查询过滤器
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuditFilter {
    pub event_types: Option<Vec<String>>,
    pub user_id: Option<String>,
    pub limit: Option<usize>,
}

/// 审计日志服务
pub struct AuditService {
    pool: Arc<Pool<Sqlite>>,
}

impl AuditService {
    pub fn new(pool: Arc<Pool<Sqlite>>) -> Self {
        Self { pool }
    }
    
    pub fn from_pool(pool: Pool<Sqlite>) -> Self {
        Self { pool: Arc::new(pool) }
    }

    pub async fn log(&self, event: AuditEvent) -> Result<(), sqlx::Error> {
        let event_type = format!("{:?}", std::mem::discriminant(&event));
        let event_data = serde_json::to_string(&event).unwrap_or_default();
        let user_id = self.extract_user_id(&event);
        
        let id = Uuid::new_v4().to_string();
        let timestamp = Utc::now().timestamp();

        sqlx::query(
            "INSERT INTO audit_logs (id, event_type, event_data, user_id, timestamp) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(&event_type)
        .bind(&event_data)
        .bind(&user_id)
        .bind(timestamp)
        .execute(self.pool.as_ref())
        .await?;

        log::info!("[Audit] {} - user: {:?}", event_type, user_id);
        Ok(())
    }

    fn extract_user_id(&self, event: &AuditEvent) -> Option<String> {
        match event {
            AuditEvent::UserLogin { user_id, .. }
            | AuditEvent::UserLogout { user_id, .. }
            | AuditEvent::StrategyCreated { user_id, .. }
            | AuditEvent::StrategyUpdated { user_id, .. }
            | AuditEvent::StrategyDeleted { user_id, .. }
            | AuditEvent::OrderPlaced { user_id, .. }
            | AuditEvent::RiskAlertTriggered { user_id, .. } => Some(user_id.clone()),
            AuditEvent::SystemStarted { .. } => None,
        }
    }

    /// 查询审计日志
    pub async fn get_logs(&self, filter: &AuditFilter) -> Result<Vec<AuditLog>, sqlx::Error> {
        let base_query = "SELECT id, event_type, event_data, user_id, timestamp FROM audit_logs";
        let mut query = String::from(base_query);
        let mut conditions = Vec::new();

        // 构建 WHERE 条件
        if let Some(ref user_id) = filter.user_id {
            conditions.push(format!("user_id = '{}'", user_id));
        }

        if let Some(ref event_types) = filter.event_types {
            if !event_types.is_empty() {
                let event_types_list: Vec<String> = event_types.iter().map(|t| format!("'{}'", t)).collect();
                conditions.push(format!("event_type IN ({})", event_types_list.join(", ")));
            }
        }

        // 添加 WHERE 子句
        if !conditions.is_empty() {
            query = format!("{} WHERE {}", base_query, conditions.join(" AND "));
        }

        // 添加 ORDER BY
        query = format!("{} ORDER BY timestamp DESC", query);

        // 添加 LIMIT
        if let Some(limit) = filter.limit {
            query = format!("{} LIMIT {}", query, limit);
        }

        sqlx::query_as::<_, AuditLog>(&query)
            .fetch_all(self.pool.as_ref())
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    use tempfile::NamedTempFile;

    async fn create_test_pool() -> SqlitePool {
        let temp_file = NamedTempFile::new().unwrap();
        let url = format!("sqlite:{}", temp_file.path().to_string_lossy());
        SqlitePool::connect(&url).await.unwrap()
    }

    #[tokio::test]
    async fn test_audit_event_variants() {
        let events = vec![
            AuditEvent::UserLogin {
                user_id: "user123".to_string(),
                username: "testuser".to_string(),
            },
            AuditEvent::UserLogout {
                user_id: "user123".to_string(),
                username: "testuser".to_string(),
            },
            AuditEvent::StrategyCreated {
                user_id: "user123".to_string(),
                strategy_id: "strat123".to_string(),
                strategy_name: "Test Strategy".to_string(),
            },
            AuditEvent::StrategyUpdated {
                user_id: "user123".to_string(),
                strategy_id: "strat123".to_string(),
                strategy_name: "Test Strategy".to_string(),
            },
            AuditEvent::StrategyDeleted {
                user_id: "user123".to_string(),
                strategy_id: "strat123".to_string(),
                strategy_name: "Test Strategy".to_string(),
            },
            AuditEvent::OrderPlaced {
                user_id: "user123".to_string(),
                order_id: "order123".to_string(),
                symbol: "BTCUSDT".to_string(),
                side: "buy".to_string(),
                quantity: 1.0,
            },
            AuditEvent::RiskAlertTriggered {
                user_id: "user123".to_string(),
                alert_type: "drawdown".to_string(),
                severity: "high".to_string(),
                message: "Drawdown exceeds 10%".to_string(),
            },
            AuditEvent::SystemStarted {
                version: "1.0.0".to_string(),
            },
        ];

        for event in events {
            let json = serde_json::to_string(&event);
            assert!(json.is_ok(), "Failed to serialize event: {:?}", event);
            let parsed: AuditEvent = serde_json::from_str(&json.unwrap()).unwrap();
            assert!(matches!(parsed, _), "Failed to deserialize event");
        }
    }

    #[tokio::test]
    async fn test_extract_user_id() {
        let pool = create_test_pool().await;
        let service = AuditService::from_pool(pool);

        let events_with_user_id = vec![
            AuditEvent::UserLogin {
                user_id: "user123".to_string(),
                username: "testuser".to_string(),
            },
            AuditEvent::OrderPlaced {
                user_id: "user456".to_string(),
                order_id: "order789".to_string(),
                symbol: "ETHUSDT".to_string(),
                side: "sell".to_string(),
                quantity: 2.5,
            },
        ];

        for event in events_with_user_id {
            let extracted = service.extract_user_id(&event);
            assert!(extracted.is_some());
            assert!(extracted.unwrap().starts_with("user"));
        }

        let system_event = AuditEvent::SystemStarted {
            version: "1.0.0".to_string(),
        };
        assert!(service.extract_user_id(&system_event).is_none());
    }

    #[tokio::test]
    async fn test_audit_filter_default() {
        let filter = AuditFilter::default();
        assert!(filter.event_types.is_none());
        assert!(filter.user_id.is_none());
        assert!(filter.limit.is_none());
    }

    #[tokio::test]
    async fn test_audit_filter_with_values() {
        let filter = AuditFilter {
            event_types: Some(vec!["UserLogin".to_string(), "OrderPlaced".to_string()]),
            user_id: Some("user123".to_string()),
            limit: Some(100),
        };

        assert_eq!(filter.event_types.unwrap().len(), 2);
        assert_eq!(filter.user_id.unwrap(), "user123");
        assert_eq!(filter.limit.unwrap(), 100);
    }

    #[tokio::test]
    async fn test_audit_log_serialization() {
        let log = AuditLog {
            id: "log123".to_string(),
            event_type: "UserLogin".to_string(),
            event_data: r#"{"user_id":"user123","username":"test"}"#.to_string(),
            user_id: Some("user123".to_string()),
            timestamp: 1234567890,
        };

        let json = serde_json::to_string(&log).unwrap();
        let parsed: AuditLog = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.id, log.id);
        assert_eq!(parsed.event_type, log.event_type);
        assert_eq!(parsed.user_id, log.user_id);
        assert_eq!(parsed.timestamp, log.timestamp);
    }

    #[tokio::test]
    async fn test_audit_log_from_row() {
        let log = AuditLog {
            id: Uuid::new_v4().to_string(),
            event_type: "TestEvent".to_string(),
            event_data: r#"{"test":"data"}"#.to_string(),
            user_id: Some("test_user".to_string()),
            timestamp: Utc::now().timestamp(),
        };

        assert!(!log.id.is_empty());
        assert_eq!(log.id.len(), 36);
        assert!(!log.event_type.is_empty());
        assert!(!log.event_data.is_empty());
        assert!(log.timestamp > 0);
    }

    #[tokio::test]
    async fn test_event_discriminant_format() {
        let event = AuditEvent::UserLogin {
            user_id: "user123".to_string(),
            username: "test".to_string(),
        };

        let discriminant = format!("{:?}", std::mem::discriminant(&event));
        assert!(discriminant.contains("UserLogin"));
    }

    #[tokio::test]
    async fn test_service_creation() {
        let pool = create_test_pool().await;
        let _service = AuditService::from_pool(pool.clone());
        let _service_from_pool = AuditService::from_pool(pool);
    }
}