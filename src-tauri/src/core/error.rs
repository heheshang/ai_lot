//! Unified error handling for AI-LOT trading platform
//!
//! This module provides a centralized error type that can be used across
//! the entire application for consistent error handling and reporting.

use thiserror::Error;

/// Unified application error type
#[derive(Error, Debug)]
pub enum AppError {
    /// Database-related errors
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Exchange API errors
    #[error("Exchange error: {0}")]
    Exchange(String),

    /// Strategy execution errors
    #[error("Strategy error: {0}")]
    Strategy(String),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Authorization/permission errors
    #[error("Permission denied: {0}")]
    Permission(String),

    /// Input validation errors
    #[error("Validation error: {0}")]
    Validation(String),

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Network-related errors
    #[error("Network error: {0}")]
    Network(String),

    /// Risk limit errors
    #[error("Risk limit exceeded: {0}")]
    RiskLimit(String),

    /// Generic application errors
    #[error("Application error: {0}")]
    Generic(String),
}

impl AppError {
    /// Get error code for frontend handling
    pub fn code(&self) -> &'static str {
        match self {
            AppError::Database(_) => "DATABASE_ERROR",
            AppError::Exchange(_) => "EXCHANGE_ERROR",
            AppError::Strategy(_) => "STRATEGY_ERROR",
            AppError::Auth(_) => "AUTH_ERROR",
            AppError::Permission(_) => "PERMISSION_ERROR",
            AppError::Validation(_) => "VALIDATION_ERROR",
            AppError::Io(_) => "IO_ERROR",
            AppError::Serialization(_) => "SERIALIZATION_ERROR",
            AppError::Network(_) => "NETWORK_ERROR",
            AppError::RiskLimit(_) => "RISK_LIMIT_ERROR",
            AppError::Generic(_) => "GENERIC_ERROR",
        }
    }

    /// Get user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            AppError::Database(err) => {
                log::error!("Database error: {:?}", err);
                "数据库操作失败，请稍后重试".to_string()
            }
            AppError::Exchange(msg) => {
                log::error!("Exchange error: {}", msg);
                format!("交易所错误: {}", msg)
            }
            AppError::Strategy(msg) => {
                log::error!("Strategy error: {}", msg);
                format!("策略执行错误: {}", msg)
            }
            AppError::Auth(msg) => {
                log::error!("Auth error: {}", msg);
                format!("认证失败: {}", msg)
            }
            AppError::Permission(msg) => {
                log::error!("Permission error: {}", msg);
                format!("权限不足: {}", msg)
            }
            AppError::Validation(msg) => {
                log::warn!("Validation error: {}", msg);
                format!("输入验证失败: {}", msg)
            }
            AppError::Io(err) => {
                log::error!("I/O error: {:?}", err);
                "文件操作失败".to_string()
            }
            AppError::Serialization(err) => {
                log::error!("Serialization error: {:?}", err);
                "数据序列化失败".to_string()
            }
            AppError::Network(msg) => {
                log::error!("Network error: {}", msg);
                format!("网络错误: {}", msg)
            }
            AppError::RiskLimit(msg) => {
                log::warn!("Risk limit error: {}", msg);
                format!("风险限制: {}", msg)
            }
            AppError::Generic(msg) => {
                log::error!("Generic error: {}", msg);
                msg.clone()
            }
        }
    }

    /// Create a validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        AppError::Validation(msg.into())
    }

    /// Create an exchange error
    pub fn exchange(msg: impl Into<String>) -> Self {
        AppError::Exchange(msg.into())
    }

    /// Create a strategy error
    pub fn strategy(msg: impl Into<String>) -> Self {
        AppError::Strategy(msg.into())
    }

    /// Create an auth error
    pub fn auth(msg: impl Into<String>) -> Self {
        AppError::Auth(msg.into())
    }

    /// Create a permission error
    pub fn permission(msg: impl Into<String>) -> Self {
        AppError::Permission(msg.into())
    }

    /// Create a network error
    pub fn network(msg: impl Into<String>) -> Self {
        AppError::Network(msg.into())
    }

    /// Create a risk limit error
    pub fn risk_limit(msg: impl Into<String>) -> Self {
        AppError::RiskLimit(msg.into())
    }

    /// Create a generic error
    pub fn generic(msg: impl Into<String>) -> Self {
        AppError::Generic(msg.into())
    }
}

/// Convert AppError to String for Tauri commands
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        // Format: [ERROR_CODE] User-friendly message
        format!("[{}] {}", error.code(), error.user_message())
    }
}

/// Type alias for Result with AppError
pub type AppResult<T> = Result<T, AppError>;

/// Macro for unified Tauri command error handling
///
/// This macro wraps command handlers with consistent error logging and conversion.
/// It automatically converts AppError to String and logs errors with context.
///
/// # Example
///
/// ```rust
/// #[tauri::command]
/// pub async fn my_command(user_id: String) -> Result<MyData, String> {
///     command_handler!(async {
///         // Your command logic here
///         // Return Ok(data) or Err(AppError)
///         let data = fetch_data(&user_id).await?;
///         Ok(data)
///     }, "my_command", user_id)
/// }
/// ```
#[macro_export]
macro_rules! command_handler {
    // Async block with context logging
    (
        $async_block:block,
        $command_name:expr
    ) => {{
        let command_name = $command_name;
        log::info!("Command '{}' called", command_name);

        let result = async move $async_block.await;

        match &result {
            Ok(_) => {
                log::info!("Command '{}' completed successfully", command_name);
            }
            Err(err) => {
                log::error!("Command '{}' failed: {:?}", command_name, err);
            }
        }

        result.map_err(|e: $crate::core::error::AppError| {
            <String as From<$crate::core::error::AppError>>::from(e)
        })
    }};

    // Async block with context logging and parameters
    (
        $async_block:block,
        $command_name:expr,
        $($param:expr),*
    ) => {{
        let command_name = $command_name;
        let params = std::format![$(std::format!("{}={:?}"), ,)* $($param),*];
        log::info!("Command '{}' called with: {}", command_name, params);

        let result = async move $async_block.await;

        match &result {
            Ok(_) => {
                log::info!("Command '{}' completed successfully", command_name);
            }
            Err(err) => {
                log::error!("Command '{}' failed: {:?}", command_name, err);
            }
        }

        result.map_err(|e: $crate::core::error::AppError| {
            <String as From<$crate::core::error::AppError>>::from(e)
        })
    }};
}

/// Convert anyhow::Error to AppError
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Generic(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        let err = AppError::validation("Invalid input");
        assert_eq!(err.code(), "VALIDATION_ERROR");

        let err = AppError::exchange("Connection failed");
        assert_eq!(err.code(), "EXCHANGE_ERROR");

        let err: AppError = sqlx::Error::RowNotFound.into();
        assert_eq!(err.code(), "DATABASE_ERROR");
    }

    #[test]
    fn test_user_messages() {
        let err = AppError::validation("Invalid email format");
        let msg = err.user_message();
        assert!(msg.contains("输入验证失败"));

        let err = AppError::auth("Invalid credentials");
        let msg = err.user_message();
        assert!(msg.contains("认证失败"));
    }

    #[test]
    fn test_error_to_string_conversion() {
        let err = AppError::validation("Invalid input");
        let str_err: String = err.into();
        assert!(str_err.contains("[VALIDATION_ERROR]"));
        assert!(str_err.contains("输入验证失败"));
    }

    #[test]
    fn test_error_constructors() {
        let err = AppError::validation("test");
        assert!(matches!(err, AppError::Validation(_)));

        let err = AppError::exchange("test");
        assert!(matches!(err, AppError::Exchange(_)));

        let err = AppError::auth("test");
        assert!(matches!(err, AppError::Auth(_)));

        let err = AppError::permission("test");
        assert!(matches!(err, AppError::Permission(_)));
    }

    #[test]
    fn test_from_std_error() {
        // Test io::Error conversion
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let app_err: AppError = io_err.into();
        assert!(matches!(app_err, AppError::Io(_)));

        // Test serde_json::Error conversion (using a mock scenario)
        // In real tests, you'd create an actual serde_json error
    }

    #[test]
    fn test_from_anyhow() {
        let anyhow_err = anyhow::anyhow!("Something went wrong");
        let app_err: AppError = anyhow_err.into();
        assert!(matches!(app_err, AppError::Generic(_)));
        assert!(app_err.to_string().contains("Something went wrong"));
    }

    #[test]
    fn test_appresult_type_alias() {
        // Test that AppResult works correctly
        fn success_function() -> AppResult<i32> {
            Ok(42)
        }

        fn error_function() -> AppResult<i32> {
            Err(AppError::validation("test error"))
        }

        assert!(success_function().is_ok());
        assert!(error_function().is_err());
    }
}
