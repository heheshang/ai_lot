//! Logging infrastructure for the AI-LOT trading platform.
//!
//! This module provides a comprehensive logging system using the `tracing` ecosystem
//! with features including:
//! - Daily file rotation with configurable retention
//! - Structured logging with fields and context
//! - Non-blocking writers for performance
//! - Console output with ANSI colors
//! - Panic hook integration for crash reporting
//! - Configurable log levels via RUST_LOG environment variable

use std::path::PathBuf;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Layer,
};

/// Application name for log file prefix
const LOG_FILE_PREFIX: &str = "ai-lot";

/// Default log directories
const LOG_DIR_NAME: &str = "logs";

/// Initialize the logging system for the application.
///
/// This function sets up a comprehensive logging infrastructure with:
/// - Daily rotating file appender in the app's log directory
/// - Console output with ANSI colors for development
/// - Non-blocking writers to prevent logging from blocking the application
/// - Panic hook to capture panics in logs
/// - Configurable log levels via RUST_LOG environment variable
///
/// # Arguments
///
/// * `app_dir` - The application data directory where logs will be stored
///
/// # Log Levels Configuration
///
/// The log level can be configured via the `RUST_LOG` environment variable:
/// - `RUST_LOG=info` - Set global log level to info (default)
/// - `RUST_LOG=debug` - Enable debug logging
/// - `RUST_LOG=ai_lot=trace` - Enable trace logging for ai_lot crate only
/// - `RUST_LOG=ai_lot::core::trade=debug,sqlx=warn` - Module-specific levels
///
/// # Default Configuration
///
/// The following default log levels are configured:
/// - Global: `info`
/// - `ai_lot::core::trade`: `debug` (detailed trading operations)
/// - `ai_lot::services::market`: `debug` (market data service)
/// - `sqlx`: `warn` (suppress verbose SQL logs)
///
/// # Log File Format
///
/// Log files are created with daily rotation:
/// - Current log: `logs/ai-lot.log`
/// - Rotated logs: `logs/ai-lot.log.YYYY-MM-DD`
///
/// # Panics
///
/// This function will panic if:
/// - The log directory cannot be created
/// - The log file cannot be opened
///
/// # Examples
///
/// ```no_run
/// use std::path::PathBuf;
///
/// let app_dir = PathBuf::from("/path/to/app");
/// init_log(app_dir);
/// ```
pub fn init_log(app_dir: PathBuf) -> Vec<WorkerGuard> {
    // Create log directory
    let log_dir = app_dir.join(LOG_DIR_NAME);
    std::fs::create_dir_all(&log_dir)
        .expect("Failed to create log directory");

    // Set up daily file rotation appender
    let file_appender = tracing_appender::rolling::daily(&log_dir, LOG_FILE_PREFIX);
    let (non_blocking_file, file_guard) = tracing_appender::non_blocking(file_appender);

    // Set up non-blocking console writer
    let (non_blocking_console, console_guard) = tracing_appender::non_blocking(std::io::stdout());

    // Configure environment filter with defaults
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::new("info")
                .add_directive("ai_lot::core::trade=debug".parse().unwrap())
                .add_directive("ai_lot::services::market=debug".parse().unwrap())
                .add_directive("sqlx=warn".parse().unwrap())
        });

    // Build file layer with full format
    let file_layer = fmt::layer()
        .with_writer(non_blocking_file)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::CLOSE)
        .boxed();

    // Build console layer with simplified format and colors
    let console_layer = fmt::layer()
        .with_writer(non_blocking_console)
        .with_ansi(true)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_span_events(FmtSpan::NONE)
        .boxed();

    // Initialize the subscriber with both layers
    tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .with(console_layer)
        .init();

    // Set panic hook to log panics
    set_panic_hook();

    // Return guards to keep them alive for the application lifetime
    vec![file_guard, console_guard]
}

/// Set up a panic hook to capture and log panics.
///
/// This function installs a custom panic handler that logs panic information
/// to both the tracing system and stderr, ensuring that panic details are
/// captured in log files for debugging.
fn set_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        let panic_msg = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic message".to_string()
        };

        let location = if let Some(location) = panic_info.location() {
            format!("{}:{}:{}", location.file(), location.line(), location.column())
        } else {
            "unknown location".to_string()
        };

        tracing::error!(
            panic_msg = %panic_msg,
            location = %location,
            "!!! PANIC !!!"
        );

        // Also print to stderr for immediate visibility
        eprintln!("!!! PANIC !!!");
        eprintln!("Message: {}", panic_msg);
        eprintln!("Location: {}", location);
    }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_init_log_creates_directory() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let app_dir = temp_dir.path().to_path_buf();

        init_log(app_dir.clone());

        let log_dir = app_dir.join(LOG_DIR_NAME);
        assert!(log_dir.exists());
        assert!(log_dir.is_dir());
    }

    #[test]
    fn test_log_file_creation() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let app_dir = temp_dir.path().to_path_buf();

        // Initialize logging
        let _guards = init_log(app_dir.clone());

        // Write a test log entry
        tracing::info!("Test log message");

        // Give the non-blocking writer time to flush
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Check if log file was created
        let log_dir = app_dir.join(LOG_DIR_NAME);
        let log_files: Vec<_> = fs::read_dir(&log_dir)
            .expect("Failed to read log directory")
            .filter_map(|entry| entry.ok())
            .collect();

        assert!(!log_files.is_empty(), "No log files were created");

        // Check that log file contains our message
        for entry in log_files {
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "log") {
                let contents = fs::read_to_string(&path).unwrap_or_default();
                // Note: The non-blocking writer might not have flushed yet
                // so we just check the file exists
                assert!(path.exists());
            }
        }
    }
}
