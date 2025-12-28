//! Risk alert repository for database operations
//!
//! This module provides the RiskAlertRepository which handles
//! all database operations for risk alert records.

use super::Repository;
use crate::models::{RiskAlert, RiskAlertListItem, CreateAlertRequest};
use async_trait::async_trait;
use anyhow::Result;
use sqlx::Pool;
use sqlx::Sqlite;

pub struct RiskAlertRepository {
    pool: Pool<Sqlite>,
}

impl RiskAlertRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// Create a new risk alert
    pub async fn create(&self, req: CreateAlertRequest) -> Result<RiskAlert> {
        let alert = RiskAlert::new(req.clone());

        sqlx::query(
            r#"
            INSERT INTO risk_alerts
            (id, rule_id, user_id, severity, title, message, strategy_instance_id,
             symbol, current_value, threshold_value, status, handled_by, handled_at, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&alert.id)
        .bind(&alert.rule_id)
        .bind(&alert.user_id)
        .bind(&alert.severity)
        .bind(&alert.title)
        .bind(&alert.message)
        .bind(&alert.strategy_instance_id)
        .bind(&alert.symbol)
        .bind(alert.current_value)
        .bind(alert.threshold_value)
        .bind(&alert.status)
        .bind(&alert.handled_by)
        .bind(alert.handled_at)
        .bind(alert.created_at)
        .execute(&self.pool)
        .await?;

        Ok(alert)
    }

    /// Find alerts by user ID (list view)
    pub async fn find_by_user(&self, user_id: &str) -> Result<Vec<RiskAlertListItem>> {
        let alerts = sqlx::query_as::<_, RiskAlertListItem>(
            r#"
            SELECT id, severity, title, strategy_instance_id, symbol, status, created_at
            FROM risk_alerts
            WHERE user_id = ?
            ORDER BY created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(alerts)
    }

    /// Find alerts by strategy instance ID
    pub async fn find_by_instance(&self, instance_id: &str) -> Result<Vec<RiskAlert>> {
        let alerts = sqlx::query_as::<_, RiskAlert>(
            r#"
            SELECT * FROM risk_alerts
            WHERE strategy_instance_id = ?
            ORDER BY created_at DESC
            "#
        )
        .bind(instance_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(alerts)
    }

    /// Find unresolved (active) alerts
    pub async fn find_unresolved(&self) -> Result<Vec<RiskAlert>> {
        let alerts = sqlx::query_as::<_, RiskAlert>(
            r#"
            SELECT * FROM risk_alerts
            WHERE status = 'active'
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(alerts)
    }

    /// Find unresolved alerts by user
    pub async fn find_unresolved_by_user(&self, user_id: &str) -> Result<Vec<RiskAlert>> {
        let alerts = sqlx::query_as::<_, RiskAlert>(
            r#"
            SELECT * FROM risk_alerts
            WHERE user_id = ? AND status = 'active'
            ORDER BY created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(alerts)
    }

    /// Find critical alerts
    pub async fn find_critical(&self) -> Result<Vec<RiskAlert>> {
        let alerts = sqlx::query_as::<_, RiskAlert>(
            r#"
            SELECT * FROM risk_alerts
            WHERE severity = 'critical' AND status = 'active'
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(alerts)
    }

    /// Mark alert as handled
    pub async fn mark_handled(&self, id: &str, handled_by: &str) -> Result<()> {
        let now = chrono::Utc::now().timestamp();

        sqlx::query(
            r#"
            UPDATE risk_alerts
            SET status = 'handled', handled_by = ?, handled_at = ?
            WHERE id = ?
            "#
        )
        .bind(handled_by)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Mark alert as ignored
    pub async fn mark_ignored(&self, id: &str) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE risk_alerts
            SET status = 'ignored'
            WHERE id = ?
            "#
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Delete old alerts (older than specified days)
    pub async fn delete_old(&self, days: i64) -> Result<u64> {
        let cutoff_time = chrono::Utc::now().timestamp() - (days * 24 * 60 * 60);

        let result = sqlx::query(
            "DELETE FROM risk_alerts WHERE created_at < ?"
        )
        .bind(cutoff_time)
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Count active alerts by user
    pub async fn count_active_by_user(&self, user_id: &str) -> Result<i64> {
        let count = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM risk_alerts WHERE user_id = ? AND status = 'active'"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(count.0)
    }

    /// Count critical alerts by user
    pub async fn count_critical_by_user(&self, user_id: &str) -> Result<i64> {
        let count = sqlx::query_as::<_, (i64,)>(
            "SELECT COUNT(*) FROM risk_alerts WHERE user_id = ? AND severity = 'critical' AND status = 'active'"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(count.0)
    }
}

#[async_trait]
impl Repository<RiskAlert, String> for RiskAlertRepository {
    async fn find_by_id(&self, id: String) -> Result<Option<RiskAlert>> {
        let alert = sqlx::query_as::<_, RiskAlert>(
            "SELECT * FROM risk_alerts WHERE id = ?"
        )
        .bind(&id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(alert)
    }

    async fn find_all(&self) -> Result<Vec<RiskAlert>> {
        let alerts = sqlx::query_as::<_, RiskAlert>(
            "SELECT * FROM risk_alerts ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(alerts)
    }

    async fn insert(&self, entity: &RiskAlert) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO risk_alerts
            (id, rule_id, user_id, severity, title, message, strategy_instance_id,
             symbol, current_value, threshold_value, status, handled_by, handled_at, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&entity.id)
        .bind(&entity.rule_id)
        .bind(&entity.user_id)
        .bind(&entity.severity)
        .bind(&entity.title)
        .bind(&entity.message)
        .bind(&entity.strategy_instance_id)
        .bind(&entity.symbol)
        .bind(entity.current_value)
        .bind(entity.threshold_value)
        .bind(&entity.status)
        .bind(&entity.handled_by)
        .bind(entity.handled_at)
        .bind(entity.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, entity: &RiskAlert) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE risk_alerts
            SET status = ?, handled_by = ?, handled_at = ?
            WHERE id = ?
            "#
        )
        .bind(&entity.status)
        .bind(&entity.handled_by)
        .bind(entity.handled_at)
        .bind(&entity.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: String) -> Result<()> {
        sqlx::query("DELETE FROM risk_alerts WHERE id = ?")
            .bind(&id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests would require a test database
    // These are placeholder test structures

    #[test]
    fn test_repository_creation() {
        // Test would require a database connection
        // This is a compilation test only
        assert!(true);
    }
}
