//! Risk Rule Repository
//!
//! Database operations for risk rule management.

use anyhow::Result;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use std::collections::HashMap;

/// Risk rule configuration from database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RiskRule {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub rule_type: String,
    pub enabled: bool,
    pub action: String,
    #[serde(rename = "notifyMethods")]
    pub notify_methods: String,  // JSON array
    #[serde(rename = "paramsJson")]
    pub params_json: String,     // JSON object
    pub created_at: i64,
    pub updated_at: i64,
}

/// Risk rule parameters (parsed from JSON)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskRuleParams {
    pub enabled: bool,
    pub action: String,
    #[serde(rename = "notifyMethods")]
    pub notify_methods: Vec<String>,
    pub params: HashMap<String, f64>,
}

impl RiskRule {
    /// Parse notify methods from JSON
    pub fn get_notify_methods(&self) -> Result<Vec<String>> {
        serde_json::from_str(&self.notify_methods)
            .map_err(|e| anyhow!("Failed to parse notify_methods: {}", e))
    }

    /// Parse parameters from JSON
    pub fn get_params(&self) -> Result<HashMap<String, f64>> {
        serde_json::from_str(&self.params_json)
            .map_err(|e| anyhow!("Failed to parse params_json: {}", e))
    }

    /// Convert to full parameter structure
    pub fn to_params(&self) -> Result<RiskRuleParams> {
        Ok(RiskRuleParams {
            enabled: self.enabled,
            action: self.action.clone(),
            notify_methods: self.get_notify_methods()?,
            params: self.get_params()?,
        })
    }
}

/// Risk Rule Repository
pub struct RiskRuleRepository {
    pool: SqlitePool,
}

impl RiskRuleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Get all risk rules
    pub async fn find_all(&self) -> Result<Vec<RiskRule>> {
        let rules = sqlx::query_as::<_, RiskRule>(
            "SELECT * FROM risk_rules ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rules)
    }

    /// Find rule by name
    pub async fn find_by_name(&self, name: &str) -> Result<Option<RiskRule>> {
        let rule = sqlx::query_as::<_, RiskRule>(
            "SELECT * FROM risk_rules WHERE name = ?"
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;

        Ok(rule)
    }

    /// Update rule configuration
    pub async fn update(
        &self,
        name: &str,
        enabled: bool,
        action: &str,
        notify_methods: &[String],
        params: &HashMap<String, f64>,
    ) -> Result<()> {
        let notify_json = serde_json::to_string(notify_methods)?;
        let params_json = serde_json::to_string(params)?;

        sqlx::query(
            "UPDATE risk_rules
             SET enabled = ?, action = ?, notify_methods = ?, params_json = ?,
                 updated_at = strftime('%s', 'now')
             WHERE name = ?"
        )
        .bind(enabled as i32)
        .bind(action)
        .bind(&notify_json)
        .bind(&params_json)
        .bind(name)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Create new rule
    pub async fn create(
        &self,
        name: &str,
        display_name: &str,
        description: &str,
        rule_type: &str,
        enabled: bool,
        action: &str,
        notify_methods: &[String],
        params: &HashMap<String, f64>,
    ) -> Result<i64> {
        let notify_json = serde_json::to_string(notify_methods)?;
        let params_json = serde_json::to_string(params)?;

        let result = sqlx::query(
            "INSERT INTO risk_rules (name, display_name, description, rule_type, enabled, action, notify_methods, params_json)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(name)
        .bind(display_name)
        .bind(description)
        .bind(rule_type)
        .bind(enabled as i32)
        .bind(action)
        .bind(&notify_json)
        .bind(&params_json)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Delete rule
    pub async fn delete(&self, name: &str) -> Result<()> {
        sqlx::query("DELETE FROM risk_rules WHERE name = ?")
            .bind(name)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_notify_methods() {
        let rule = RiskRule {
            id: 1,
            name: "test".to_string(),
            display_name: "Test".to_string(),
            description: "Test rule".to_string(),
            rule_type: "test".to_string(),
            enabled: true,
            action: "warning".to_string(),
            notify_methods: r#"["dingtalk", "email"]"#.to_string(),
            params_json: "{}".to_string(),
            created_at: 0,
            updated_at: 0,
        };

        let methods = rule.get_notify_methods().unwrap();
        assert_eq!(methods, vec!["dingtalk", "email"]);
    }
}
