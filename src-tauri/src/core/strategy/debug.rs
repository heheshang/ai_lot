//! Strategy Debug Module
//!
//! Provides debugging capabilities for strategy scripts including console logging,
//! variable monitoring, and performance metrics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::Instant;

/// Log level for strategy debug output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
}

impl LogLevel {
    /// Check if this level should be displayed given the minimum level
    pub fn should_display(self, min_level: LogLevel) -> bool {
        (self as i32) >= (min_level as i32)
    }

    /// Get the level name as a string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DEBUG" => Ok(Self::Debug),
            "INFO" => Ok(Self::Info),
            "WARN" | "WARNING" => Ok(Self::Warn),
            "ERROR" => Ok(Self::Error),
            _ => Err(format!("Invalid log level: {}", s)),
        }
    }
}

/// A debug log entry from strategy execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugLog {
    /// Log level
    pub level: LogLevel,
    /// Log message
    pub message: String,
    /// Timestamp (Unix milliseconds)
    pub timestamp: i64,
    /// Optional source line number
    pub line: Option<u32>,
    /// Optional function name
    pub function: Option<String>,
    /// Strategy instance ID
    pub instance_id: Option<String>,
}

impl DebugLog {
    /// Create a new debug log entry
    pub fn new(level: LogLevel, message: String) -> Self {
        Self {
            level,
            message,
            timestamp: chrono::Utc::now().timestamp_millis(),
            line: None,
            function: None,
            instance_id: None,
        }
    }

    /// Set the instance ID
    pub fn with_instance_id(mut self, instance_id: String) -> Self {
        self.instance_id = Some(instance_id);
        self
    }

    /// Set the source location
    pub fn with_location(mut self, line: u32, function: String) -> Self {
        self.line = Some(line);
        self.function = Some(function);
        self
    }
}

/// A variable snapshot for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugVariable {
    /// Variable name
    pub name: String,
    /// Variable value (JSON serialized)
    pub value: serde_json::Value,
    /// Variable type
    pub var_type: String,
    /// Timestamp
    pub timestamp: i64,
}

/// Performance metrics for strategy execution
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PerformanceMetrics {
    /// Execution times for each function call (function name -> list of durations in ms)
    pub execution_times: HashMap<String, Vec<f64>>,
    /// Call counts for each function
    pub call_counts: HashMap<String, usize>,
    /// Total execution time
    pub total_execution_time_ms: f64,
    /// Number of errors
    pub error_count: usize,
    /// Number of warnings
    pub warning_count: usize,
}

impl PerformanceMetrics {
    /// Record a function execution
    pub fn record_execution(&mut self, function: String, duration_ms: f64) {
        *self.call_counts.entry(function.clone()).or_insert(0) += 1;
        self.execution_times
            .entry(function)
            .or_insert_with(Vec::new)
            .push(duration_ms);
        self.total_execution_time_ms += duration_ms;
    }

    /// Get average execution time for a function
    pub fn avg_execution_time(&self, function: &str) -> Option<f64> {
        let times = self.execution_times.get(function)?;
        if times.is_empty() {
            return Some(0.0);
        }
        Some(times.iter().sum::<f64>() / times.len() as f64)
    }

    /// Get max execution time for a function
    pub fn max_execution_time(&self, function: &str) -> Option<f64> {
        self.execution_times.get(function)?.iter().cloned().reduce(f64::max)
    }

    /// Get total execution time for a function
    pub fn total_execution_time(&self, function: &str) -> Option<f64> {
        Some(self.execution_times.get(function)?.iter().sum::<f64>())
    }

    /// Get statistics for all functions
    pub fn get_function_stats(&self) -> Vec<FunctionStats> {
        let mut stats = Vec::new();

        for (function, times) in &self.execution_times {
            let call_count = *self.call_counts.get(function).unwrap_or(&0);
            let total_time: f64 = times.iter().sum();
            let avg_time = total_time / times.len() as f64;
            let max_time = times.iter().cloned().reduce(f64::max).unwrap_or(0.0);
            let min_time = times.iter().cloned().reduce(f64::min).unwrap_or(0.0);

            stats.push(FunctionStats {
                function: function.clone(),
                call_count,
                total_time_ms: total_time,
                avg_time_ms: avg_time,
                max_time_ms: max_time,
                min_time_ms: min_time,
            });
        }

        // Sort by total execution time (descending)
        stats.sort_by(|a, b| b.total_time_ms.partial_cmp(&a.total_time_ms).unwrap());

        stats
    }
}

/// Statistics for a single function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionStats {
    /// Function name
    pub function: String,
    /// Number of times called
    pub call_count: usize,
    /// Total execution time (ms)
    pub total_time_ms: f64,
    /// Average execution time (ms)
    pub avg_time_ms: f64,
    /// Maximum execution time (ms)
    pub max_time_ms: f64,
    /// Minimum execution time (ms)
    pub min_time_ms: f64,
}

/// Debug context for strategy execution
#[derive(Debug, Clone)]
pub struct DebugContext {
    /// Collected log entries
    logs: Arc<RwLock<Vec<DebugLog>>>,
    /// Monitored variables
    variables: Arc<RwLock<HashMap<String, DebugVariable>>>,
    /// Performance metrics
    metrics: Arc<RwLock<PerformanceMetrics>>,
    /// Minimum log level to display
    min_log_level: Arc<RwLock<LogLevel>>,
    /// Enable/disable logging
    enabled: Arc<RwLock<bool>>,
}

impl DebugContext {
    /// Create a new debug context
    pub fn new() -> Self {
        Self {
            logs: Arc::new(RwLock::new(Vec::new())),
            variables: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(PerformanceMetrics::default())),
            min_log_level: Arc::new(RwLock::new(LogLevel::Info)),
            enabled: Arc::new(RwLock::new(true)),
        }
    }

    /// Log a message at the specified level
    pub fn log(&self, level: LogLevel, message: String) {
        if !*self.enabled.read().unwrap() {
            return;
        }

        let min_level = *self.min_log_level.read().unwrap();
        if !level.should_display(min_level) {
            return;
        }

        let entry = DebugLog::new(level, message);
        self.logs.write().unwrap().push(entry);

        // Update metrics
        let mut metrics = self.metrics.write().unwrap();
        match level {
            LogLevel::Error => metrics.error_count += 1,
            LogLevel::Warn => metrics.warning_count += 1,
            _ => {}
        }
    }

    /// Log a debug message
    pub fn debug(&self, message: String) {
        self.log(LogLevel::Debug, message);
    }

    /// Log an info message
    pub fn info(&self, message: String) {
        self.log(LogLevel::Info, message);
    }

    /// Log a warning message
    pub fn warn(&self, message: String) {
        self.log(LogLevel::Warn, message);
    }

    /// Log an error message
    pub fn error(&self, message: String) {
        self.log(LogLevel::Error, message);
    }

    /// Set a variable for monitoring
    pub fn set_variable(&self, name: String, value: serde_json::Value) {
        let var_type = match value {
            serde_json::Value::Null => "null".to_string(),
            serde_json::Value::Bool(_) => "boolean".to_string(),
            serde_json::Value::Number(_) => "number".to_string(),
            serde_json::Value::String(_) => "string".to_string(),
            serde_json::Value::Array(_) => "array".to_string(),
            serde_json::Value::Object(_) => "object".to_string(),
        };

        let variable = DebugVariable {
            name: name.clone(),
            value,
            var_type,
            timestamp: chrono::Utc::now().timestamp_millis(),
        };

        self.variables.write().unwrap().insert(name, variable);
    }

    /// Get all log entries
    pub fn get_logs(&self) -> Vec<DebugLog> {
        self.logs.read().unwrap().clone()
    }

    /// Get logs since a specific timestamp
    pub fn get_logs_since(&self, timestamp: i64) -> Vec<DebugLog> {
        self.logs
            .read()
            .unwrap()
            .iter()
            .filter(|log| log.timestamp >= timestamp)
            .cloned()
            .collect()
    }

    /// Get logs at or above a specific level
    pub fn get_logs_by_level(&self, min_level: LogLevel) -> Vec<DebugLog> {
        self.logs
            .read()
            .unwrap()
            .iter()
            .filter(|log| log.level.should_display(min_level))
            .cloned()
            .collect()
    }

    /// Clear all log entries
    pub fn clear_logs(&self) {
        self.logs.write().unwrap().clear();
    }

    /// Get all monitored variables
    pub fn get_variables(&self) -> HashMap<String, DebugVariable> {
        self.variables.read().unwrap().clone()
    }

    /// Get a specific variable
    pub fn get_variable(&self, name: &str) -> Option<DebugVariable> {
        self.variables.read().unwrap().get(name).cloned()
    }

    /// Get performance metrics
    pub fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.read().unwrap().clone()
    }

    /// Set minimum log level
    pub fn set_min_log_level(&self, level: LogLevel) {
        *self.min_log_level.write().unwrap() = level;
    }

    /// Get minimum log level
    pub fn get_min_log_level(&self) -> LogLevel {
        *self.min_log_level.read().unwrap()
    }

    /// Enable or disable logging
    pub fn set_enabled(&self, enabled: bool) {
        *self.enabled.write().unwrap() = enabled;
    }

    /// Check if logging is enabled
    pub fn is_enabled(&self) -> bool {
        *self.enabled.read().unwrap()
    }

    /// Start a performance timer
    pub fn start_timer(&self) -> PerformanceTimer {
        PerformanceTimer::new(self.metrics.clone())
    }
}

impl Default for DebugContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance timer for measuring execution time
pub struct PerformanceTimer {
    metrics: Arc<RwLock<PerformanceMetrics>>,
    function: Option<String>,
    start: Instant,
}

impl PerformanceTimer {
    /// Create a new timer
    fn new(metrics: Arc<RwLock<PerformanceMetrics>>) -> Self {
        Self {
            metrics,
            function: None,
            start: Instant::now(),
        }
    }

    /// Set the function name to measure
    pub fn for_function(mut self, function: String) -> Self {
        self.function = Some(function);
        self
    }

    /// Complete the timer and record the duration
    pub fn finish(self) {
        let duration_ms = self.start.elapsed().as_secs_f64() * 1000.0;

        if let Some(function) = self.function {
            self.metrics
                .write()
                .unwrap()
                .record_execution(function, duration_ms);
        }
    }
}

/// Get global debug context for a strategy instance
pub fn get_debug_context(_instance_id: &str) -> DebugContext {
    // TODO: Implement per-instance context caching
    DebugContext::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_display() {
        assert!(LogLevel::Info.should_display(LogLevel::Debug));
        assert!(LogLevel::Info.should_display(LogLevel::Info));
        assert!(!LogLevel::Debug.should_display(LogLevel::Info));
        assert!(!LogLevel::Debug.should_display(LogLevel::Error));
    }

    #[test]
    fn test_debug_context() {
        let ctx = DebugContext::new();

        ctx.debug("Test debug message".to_string());
        ctx.info("Test info message".to_string());
        ctx.warn("Test warning message".to_string());
        ctx.error("Test error message".to_string());

        let logs = ctx.get_logs();
        assert_eq!(logs.len(), 4);

        // Check error count
        let metrics = ctx.get_metrics();
        assert_eq!(metrics.error_count, 1);
        assert_eq!(metrics.warning_count, 1);
    }

    #[test]
    fn test_log_filtering() {
        let ctx = DebugContext::new();
        ctx.set_min_log_level(LogLevel::Warn);

        ctx.debug("Debug".to_string());
        ctx.info("Info".to_string());
        ctx.warn("Warning".to_string());
        ctx.error("Error".to_string());

        let logs = ctx.get_logs();
        // Debug and Info should be filtered out
        assert_eq!(logs.len(), 4); // All stored, but filter when retrieving

        let warn_logs = ctx.get_logs_by_level(LogLevel::Warn);
        assert_eq!(warn_logs.len(), 2); // Warn + Error
    }

    #[test]
    fn test_variables() {
        let ctx = DebugContext::new();

        ctx.set_variable("test_num".to_string(), serde_json::json!(42));
        ctx.set_variable("test_str".to_string(), serde_json::json!("hello"));

        let var = ctx.get_variable("test_num");
        assert!(var.is_some());
        assert_eq!(var.unwrap().var_type, "number");
    }

    #[test]
    fn test_performance_metrics() {
        let ctx = DebugContext::new();

        // Simulate some function calls
        for _ in 0..10 {
            let mut metrics = ctx.metrics.write().unwrap();
            metrics.record_execution("onBar".to_string(), 5.0);
        }

        let metrics = ctx.get_metrics();
        let stats = metrics.get_function_stats();

        assert!(!stats.is_empty());
        let on_bar_stats = stats.iter().find(|s| s.function == "onBar").unwrap();
        assert_eq!(on_bar_stats.call_count, 10);
        assert_eq!(on_bar_stats.total_time_ms, 50.0);
        assert_eq!(on_bar_stats.avg_time_ms, 5.0);
    }
}
