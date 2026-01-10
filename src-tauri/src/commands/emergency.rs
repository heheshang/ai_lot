//! Emergency commands for Tauri
//!
//! This module provides Tauri command handlers for emergency operations.

use crate::core::response::{ApiResponse, ApiError};
use crate::infrastructure::Database;
use crate::services::{EmergencyService, EmergencyReport};
use serde::{Deserialize, Serialize};
use tauri::State;

/// Emergency stop report for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmergencyStopReport {
    pub strategies_stopped: usize,
    pub orders_canceled: usize,
    pub positions_closed: usize,
    pub alert_sent: bool,
    pub errors: Vec<String>,
    pub timestamp: i64,
}

impl From<EmergencyReport> for EmergencyStopReport {
    fn from(report: EmergencyReport) -> Self {
        Self {
            strategies_stopped: report.strategies_stopped,
            orders_canceled: report.orders_canceled,
            positions_closed: report.positions_closed,
            alert_sent: report.alert_sent,
            errors: report.errors,
            timestamp: chrono::Utc::now().timestamp(),
        }
    }
}

/// Execute emergency stop - halts all trading activities
///
/// This command will:
/// 1. Stop all running strategy instances
/// 2. Cancel all active orders
/// 3. Close all positions with market orders
/// 4. Send emergency alert notification
///
/// Each step continues even if previous steps fail, ensuring maximum cleanup.
#[tauri::command]
pub async fn emergency_stop(
    db: State<'_, Database>,
    user_id: String,
) -> Result<ApiResponse<EmergencyStopReport>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::error!("[{}] EMERGENCY STOP triggered by user: {}", request_id, user_id);

    // Get services from database state
    let strategy_engine = db.get_strategy_engine();
    let exchange = db.get_exchange();

    let trade_service = std::sync::Arc::new(crate::services::TradeService::new(
        exchange,
        db.pool.clone(),
    ));

    // Create emergency service
    let emergency_service = EmergencyService::new(trade_service, strategy_engine);

    // Execute emergency stop
    match emergency_service.emergency_stop_all(&user_id).await {
        Ok(report) => {
            log::error!("[{}] EMERGENCY STOP completed successfully: {:?}", request_id, report);
            Ok(ApiResponse::success(report.into()).with_request_id(request_id))
        }
        Err(e) => {
            log::error!("[{}] EMERGENCY STOP failed: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed(format!("紧急停止失败: {}", e))).with_request_id(request_id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emergency_stop_report_conversion() {
        let mut report = EmergencyReport::default();
        report.strategies_stopped = 3;
        report.orders_canceled = 10;
        report.positions_closed = 5;
        report.alert_sent = true;
        report.errors.push("Test error".to_string());

        let frontend_report: EmergencyStopReport = report.clone().into();

        assert_eq!(frontend_report.strategies_stopped, 3);
        assert_eq!(frontend_report.orders_canceled, 10);
        assert_eq!(frontend_report.positions_closed, 5);
        assert_eq!(frontend_report.alert_sent, true);
        assert_eq!(frontend_report.errors.len(), 1);
        assert!(frontend_report.timestamp > 0);
    }

    #[test]
    fn test_emergency_stop_report_empty() {
        let report = EmergencyReport::default();
        let frontend_report: EmergencyStopReport = report.into();

        assert_eq!(frontend_report.strategies_stopped, 0);
        assert_eq!(frontend_report.orders_canceled, 0);
        assert_eq!(frontend_report.positions_closed, 0);
        assert_eq!(frontend_report.alert_sent, false);
        assert!(frontend_report.errors.is_empty());
    }
}
