//! Strategy Debug Commands
//!
//! Tauri commands for accessing strategy debug information including logs,
//! variables, and performance metrics.

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
) -> Result<Vec<DebugLog>, String> {
    let ctx = get_global_debug();

    let mut logs = if let Some(level_str) = min_level {
        let level = level_str.parse::<LogLevel>().map_err(|e| e)?;
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

    Ok(logs)
}

/// Get performance metrics for a strategy instance
#[tauri::command]
pub async fn get_strategy_metrics(
    _instance_id: String,
) -> Result<PerformanceMetrics, String> {
    let ctx = get_global_debug();
    Ok(ctx.get_metrics())
}

/// Get monitored variables for a strategy instance
#[tauri::command]
pub async fn get_strategy_variables(
    _instance_id: String,
) -> Result<std::collections::HashMap<String, crate::core::strategy::debug::DebugVariable>, String> {
    let ctx = get_global_debug();
    Ok(ctx.get_variables())
}

/// Clear debug logs for a strategy instance
#[tauri::command]
pub async fn clear_strategy_logs(_instance_id: String) -> Result<(), String> {
    let ctx = get_global_debug();
    ctx.clear_logs();
    Ok(())
}

/// Set the minimum log level for a strategy instance
#[tauri::command]
pub async fn set_strategy_log_level(
    _instance_id: String,
    level: String,
) -> Result<(), String> {
    let log_level = level.parse::<LogLevel>().map_err(|e| e)?;
    let ctx = get_global_debug();
    ctx.set_min_log_level(log_level);
    Ok(())
}

/// Get the current log level for a strategy instance
#[tauri::command]
pub async fn get_strategy_log_level(_instance_id: String) -> Result<String, String> {
    let ctx = get_global_debug();
    Ok(ctx.get_min_log_level().to_string())
}

/// Test command to generate sample debug logs
#[tauri::command]
pub async fn generate_test_logs(_instance_id: String) -> Result<(), String> {
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

    Ok(())
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
