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