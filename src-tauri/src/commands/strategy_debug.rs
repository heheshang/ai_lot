//! Strategy Debug Commands
//!
//! Tauri commands for accessing strategy debug information including logs,
//! variables, and performance metrics.

use crate::core::response::{ApiResponse, ApiError};
use crate::core::strategy::{DebugContext, LogLevel};
use crate::core::strategy::debug::{DebugLog, PerformanceMetrics};

/// Get or create the global debug context
fn get_global_debug() -> DebugContext {
    // TODO: Implement proper per-instance context management
    DebugContext::new()
}

/// Get debug logs for a strategy instance
///
/// # Arguments
///
/// * `instance_id` - The strategy instance ID
/// * `min_level` - Optional minimum log level (debug, info, warn, error)
/// * `since` - Optional timestamp to get logs since
/// * `limit` - Optional maximum number of logs to return
#[tauri::command]
pub async fn get_strategy_logs(
    _instance_id: String,
    min_level: Option<String>,
    since: Option<i64>,
    limit: Option<usize>,
) -> Result<ApiResponse<Vec<DebugLog>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let ctx = get_global_debug();

    let mut logs = if let Some(level_str) = min_level {
        let level = match level_str.parse::<LogLevel>() {
            Ok(level) => level,
            Err(e) => return Ok(ApiResponse::error(ApiError::invalid_parameter(format!("Invalid log level: {}", e))).with_request_id(request_id)),
        };
        ctx.get_logs_by_level(level)
    } else if let Some(timestamp) = since {
        ctx.get_logs_since(timestamp)
    } else {
        ctx.get_logs()
    };

    // Apply limit
    if let Some(limit) = limit {
        if logs.len() > limit {
            logs = logs.into_iter().rev().take(limit).collect();
            logs.reverse();
        }
    }

    Ok(ApiResponse::success(logs).with_request_id(request_id))
}

/// Get performance metrics for a strategy instance
#[tauri::command]
pub async fn get_strategy_metrics(
    _instance_id: String,
) -> Result<ApiResponse<PerformanceMetrics>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let ctx = get_global_debug();
    Ok(ApiResponse::success(ctx.get_metrics()).with_request_id(request_id))
}

/// Get monitored variables for a strategy instance
#[tauri::command]
pub async fn get_strategy_variables(
    _instance_id: String,
) -> Result<ApiResponse<std::collections::HashMap<String, crate::core::strategy::debug::DebugVariable>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let ctx = get_global_debug();
    Ok(ApiResponse::success(ctx.get_variables()).with_request_id(request_id))
}

/// Clear debug logs for a strategy instance
#[tauri::command]
pub async fn clear_strategy_logs(_instance_id: String) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let ctx = get_global_debug();
    ctx.clear_logs();
    Ok(ApiResponse::success_empty().with_request_id(request_id))
}

/// Set the minimum log level for a strategy instance
#[tauri::command]
pub async fn set_strategy_log_level(
    _instance_id: String,
    level: String,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let log_level = match level.parse::<LogLevel>() {
        Ok(level) => level,
        Err(e) => return Ok(ApiResponse::error(ApiError::invalid_parameter(format!("Invalid log level: {}", e))).with_request_id(request_id)),
    };
    let ctx = get_global_debug();
    ctx.set_min_log_level(log_level);
    Ok(ApiResponse::success_empty().with_request_id(request_id))
}

/// Get the current log level for a strategy instance
#[tauri::command]
pub async fn get_strategy_log_level(_instance_id: String) -> Result<ApiResponse<String>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let ctx = get_global_debug();
    Ok(ApiResponse::success(ctx.get_min_log_level().to_string()).with_request_id(request_id))
}

/// Test command to generate sample debug logs
#[tauri::command]
pub async fn generate_test_logs(_instance_id: String) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let ctx = get_global_debug();

    ctx.debug("This is a debug message".to_string());
    ctx.info("Strategy initialized successfully".to_string());
    ctx.info("Processing kline data...".to_string());
    ctx.warn("High volatility detected".to_string());
    ctx.error("Failed to execute order".to_string());

    // Set some test variables
    ctx.set_variable("price".to_string(), serde_json::json!(50000.0));
    ctx.set_variable("quantity".to_string(), serde_json::json!(0.1));
    ctx.set_variable("signal".to_string(), serde_json::json!("buy"));

    Ok(ApiResponse::success_empty().with_request_id(request_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_parsing() {
        assert_eq!("debug".parse::<LogLevel>().unwrap(), LogLevel::Debug);
        assert_eq!("info".parse::<LogLevel>().unwrap(), LogLevel::Info);
        assert_eq!("warn".parse::<LogLevel>().unwrap(), LogLevel::Warn);
        assert_eq!("error".parse::<LogLevel>().unwrap(), LogLevel::Error);
    }

    #[test]
    fn test_invalid_log_level() {
        assert!("invalid".parse::<LogLevel>().is_err());
    }
}
