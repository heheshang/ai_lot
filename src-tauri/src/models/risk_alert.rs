//! Risk alert model for recording risk management events
//!
//! This module provides the RiskAlert model which represents
//! a risk alert record in the database.

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::str::FromStr;

/// Risk alert model (corresponding to database table risk_alerts)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct RiskAlert {
    pub id: String,
    pub rule_id: String,
    pub user_id: String,
    pub severity: String,
    pub title: String,
    pub message: String,
    pub strategy_instance_id: Option<String>,
    pub symbol: Option<String>,
    pub current_value: f64,
    pub threshold_value: f64,
    pub status: String,
    pub handled_by: Option<String>,
    pub handled_at: Option<i64>,
    pub created_at: i64,
}

/// Risk alert list item (for UI display)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RiskAlertListItem {
    pub id: String,
    pub severity: String,
    pub title: String,
    pub strategy_instance_id: Option<String>,
    pub symbol: Option<String>,
    pub status: String,
    pub created_at: i64,
}

/// Create risk alert request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAlertRequest {
    pub rule_id: String,
    pub user_id: String,
    pub severity: String,
    pub title: String,
    pub message: String,
    pub strategy_instance_id: Option<String>,
    pub symbol: Option<String>,
    pub current_value: f64,
    pub threshold_value: f64,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

impl AlertSeverity {
    /// Convert severity to string
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertSeverity::Low => "low",
            AlertSeverity::Medium => "medium",
            AlertSeverity::High => "high",
            AlertSeverity::Critical => "critical",
        }
    }

}

impl FromStr for AlertSeverity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" => Ok(AlertSeverity::Low),
            "medium" => Ok(AlertSeverity::Medium),
            "high" => Ok(AlertSeverity::High),
            "critical" => Ok(AlertSeverity::Critical),
            _ => Err(format!("Invalid severity: {}", s)),
        }
    }
}

/// Alert status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AlertStatus {
    Active,
    Handled,
    Ignored,
}

impl AlertStatus {
    /// Convert status to string
    pub fn as_str(&self) -> &'static str {
        match self {
            AlertStatus::Active => "active",
            AlertStatus::Handled => "handled",
            AlertStatus::Ignored => "ignored",
        }
    }

}

impl FromStr for AlertStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(AlertStatus::Active),
            "handled" => Ok(AlertStatus::Handled),
            "ignored" => Ok(AlertStatus::Ignored),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}

impl RiskAlert {
    /// Create a new alert (for insertion)
    pub fn new(req: CreateAlertRequest) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        Self {
            id,
            rule_id: req.rule_id,
            user_id: req.user_id,
            severity: req.severity,
            title: req.title,
            message: req.message,
            strategy_instance_id: req.strategy_instance_id,
            symbol: req.symbol,
            current_value: req.current_value,
            threshold_value: req.threshold_value,
            status: AlertStatus::Active.as_str().to_string(),
            handled_by: None,
            handled_at: None,
            created_at: now,
        }
    }

    /// Mark alert as handled
    pub fn mark_handled(&mut self, handled_by: String) {
        self.status = AlertStatus::Handled.as_str().to_string();
        self.handled_by = Some(handled_by);
        self.handled_at = Some(chrono::Utc::now().timestamp());
    }

    /// Mark alert as ignored
    pub fn mark_ignored(&mut self) {
        self.status = AlertStatus::Ignored.as_str().to_string();
    }

    /// Check if alert is active
    pub fn is_active(&self) -> bool {
        self.status == AlertStatus::Active.as_str()
    }

    /// Check if alert is handled
    pub fn is_handled(&self) -> bool {
        self.status == AlertStatus::Handled.as_str()
    }

    /// Check if alert is critical
    pub fn is_critical(&self) -> bool {
        self.severity == AlertSeverity::Critical.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_alert() {
        let req = CreateAlertRequest {
            rule_id: "rule_1".to_string(),
            user_id: "user_1".to_string(),
            severity: AlertSeverity::High.as_str().to_string(),
            title: "High Drawdown".to_string(),
            message: "Drawdown exceeded 15%".to_string(),
            strategy_instance_id: Some("instance_1".to_string()),
            symbol: Some("BTCUSDT".to_string()),
            current_value: 15.5,
            threshold_value: 10.0,
        };

        let alert = RiskAlert::new(req);

        assert_eq!(alert.status, "active");
        assert!(alert.handled_by.is_none());
        assert!(alert.handled_at.is_none());
        assert!(alert.is_active());
        assert!(!alert.is_handled());
    }

    #[test]
    fn test_mark_handled() {
        let req = CreateAlertRequest {
            rule_id: "rule_1".to_string(),
            user_id: "user_1".to_string(),
            severity: AlertSeverity::Medium.as_str().to_string(),
            title: "Test Alert".to_string(),
            message: "Test message".to_string(),
            strategy_instance_id: None,
            symbol: None,
            current_value: 5.0,
            threshold_value: 3.0,
        };

        let mut alert = RiskAlert::new(req);
        alert.mark_handled("admin".to_string());

        assert_eq!(alert.status, "handled");
        assert_eq!(alert.handled_by, Some("admin".to_string()));
        assert!(alert.handled_at.is_some());
        assert!(alert.is_handled());
        assert!(!alert.is_active());
    }

    #[test]
    fn test_mark_ignored() {
        let req = CreateAlertRequest {
            rule_id: "rule_1".to_string(),
            user_id: "user_1".to_string(),
            severity: AlertSeverity::Low.as_str().to_string(),
            title: "Test Alert".to_string(),
            message: "Test message".to_string(),
            strategy_instance_id: None,
            symbol: None,
            current_value: 1.0,
            threshold_value: 2.0,
        };

        let mut alert = RiskAlert::new(req);
        alert.mark_ignored();

        assert_eq!(alert.status, "ignored");
        assert!(!alert.is_active());
        assert!(!alert.is_handled());
    }

    #[test]
    fn test_severity_enum() {
        assert_eq!(AlertSeverity::Low.as_str(), "low");
        assert_eq!(AlertSeverity::Medium.as_str(), "medium");
        assert_eq!(AlertSeverity::High.as_str(), "high");
        assert_eq!(AlertSeverity::Critical.as_str(), "critical");

        assert_eq!(AlertSeverity::from_str("low"), Ok(AlertSeverity::Low));
        assert_eq!(AlertSeverity::from_str("HIGH"), Ok(AlertSeverity::High));
        assert!(AlertSeverity::from_str("invalid").is_err());
    }

    #[test]
    fn test_status_enum() {
        assert_eq!(AlertStatus::Active.as_str(), "active");
        assert_eq!(AlertStatus::Handled.as_str(), "handled");
        assert_eq!(AlertStatus::Ignored.as_str(), "ignored");

        assert_eq!(AlertStatus::from_str("active"), Ok(AlertStatus::Active));
        assert_eq!(AlertStatus::from_str("HANDLED"), Ok(AlertStatus::Handled));
        assert!(AlertStatus::from_str("invalid").is_err());
    }

    #[test]
    fn test_is_critical() {
        let req = CreateAlertRequest {
            rule_id: "rule_1".to_string(),
            user_id: "user_1".to_string(),
            severity: AlertSeverity::Critical.as_str().to_string(),
            title: "Critical Alert".to_string(),
            message: "Critical message".to_string(),
            strategy_instance_id: None,
            symbol: None,
            current_value: 100.0,
            threshold_value: 50.0,
        };

        let alert = RiskAlert::new(req);
        assert!(alert.is_critical());
    }

    #[test]
    fn test_alert_with_optional_fields() {
        let req = CreateAlertRequest {
            rule_id: "rule_1".to_string(),
            user_id: "user_1".to_string(),
            severity: AlertSeverity::High.as_str().to_string(),
            title: "Test Alert".to_string(),
            message: "Test message".to_string(),
            strategy_instance_id: Some("instance_1".to_string()),
            symbol: Some("ETHUSDT".to_string()),
            current_value: 10.0,
            threshold_value: 5.0,
        };

        let alert = RiskAlert::new(req);
        assert_eq!(alert.strategy_instance_id, Some("instance_1".to_string()));
        assert_eq!(alert.symbol, Some("ETHUSDT".to_string()));
    }

    #[test]
    fn test_alert_without_optional_fields() {
        let req = CreateAlertRequest {
            rule_id: "rule_1".to_string(),
            user_id: "user_1".to_string(),
            severity: AlertSeverity::Low.as_str().to_string(),
            title: "Test Alert".to_string(),
            message: "Test message".to_string(),
            strategy_instance_id: None,
            symbol: None,
            current_value: 1.0,
            threshold_value: 2.0,
        };

        let alert = RiskAlert::new(req);
        assert!(alert.strategy_instance_id.is_none());
        assert!(alert.symbol.is_none());
    }
}
