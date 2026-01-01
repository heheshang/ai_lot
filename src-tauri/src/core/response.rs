//! 统一API响应格式
//!
//! 提供标准的API响应结构，确保前后端通信的一致性

use serde::{Deserialize, Serialize};
use std::fmt;

/// 统一API响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// 是否成功
    pub success: bool,
    /// 响应数据
    pub data: Option<T>,
    /// 错误信息
    pub error: Option<ErrorDetail>,
    /// 请求追踪ID（用于日志关联）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// 时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            request_id: None,
            timestamp: Some(chrono::Utc::now().timestamp_millis()),
        }
    }

    /// 创建成功响应（无数据）
    pub fn success_empty() -> Self {
        Self {
            success: true,
            data: None,
            error: None,
            request_id: None,
            timestamp: Some(chrono::Utc::now().timestamp_millis()),
        }
    }

    /// 创建错误响应
    pub fn error(error: impl Into<ApiError>) -> Self {
        let api_error = error.into();
        Self {
            success: false,
            data: None,
            error: Some(ErrorDetail {
                code: api_error.code.clone(),
                message: api_error.message.clone(),
                details: api_error.details,
            }),
            request_id: None,
            timestamp: Some(chrono::Utc::now().timestamp_millis()),
        }
    }

    /// 设置请求追踪ID
    pub fn with_request_id(mut self, request_id: String) -> Self {
        self.request_id = Some(request_id);
        self
    }

    /// 转换为Result
    pub fn into_result(self) -> Result<T, ApiError> {
        if self.success {
            self.data.ok_or_else(|| ApiError::internal_error("Missing data in success response"))
        } else {
            let error_detail = self.error.unwrap_or_else(|| ErrorDetail {
                code: "UNKNOWN_ERROR".to_string(),
                message: "Unknown error".to_string(),
                details: None,
            });
            Err(ApiError {
                code: error_detail.code,
                message: error_detail.message,
                details: error_detail.details,
            })
        }
    }
}

/// 错误详情
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorDetail {
    /// 错误代码
    pub code: String,
    /// 错误消息（用户友好的）
    pub message: String,
    /// 详细信息（可选，用于调试）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

/// API错误类型
#[derive(Debug, Clone)]
pub struct ApiError {
    /// 错误代码
    pub code: String,
    /// 错误消息
    pub message: String,
    /// 详细信息
    pub details: Option<String>,
}

impl ApiError {
    // ============== 认证错误 ==============
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            code: "UNAUTHORIZED".to_string(),
            message: message.into(),
            details: None,
        }
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self {
            code: "FORBIDDEN".to_string(),
            message: message.into(),
            details: None,
        }
    }

    pub fn invalid_credentials() -> Self {
        Self {
            code: "INVALID_CREDENTIALS".to_string(),
            message: "用户名或密码错误".to_string(),
            details: None,
        }
    }

    pub fn token_expired() -> Self {
        Self {
            code: "TOKEN_EXPIRED".to_string(),
            message: "登录已过期，请重新登录".to_string(),
            details: None,
        }
    }

    // ============== 验证错误 ==============
    pub fn validation_failed(field: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            code: "VALIDATION_FAILED".to_string(),
            message: format!("{}: {}", field.into(), reason.into()),
            details: None,
        }
    }

    pub fn invalid_parameter(param: impl Into<String>) -> Self {
        Self {
            code: "INVALID_PARAMETER".to_string(),
            message: format!("参数无效: {}", param.into()),
            details: None,
        }
    }

    pub fn missing_parameter(param: impl Into<String>) -> Self {
        Self {
            code: "MISSING_PARAMETER".to_string(),
            message: format!("缺少必需参数: {}", param.into()),
            details: None,
        }
    }

    // ============== 资源错误 ==============
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self {
            code: "NOT_FOUND".to_string(),
            message: format!("{}不存在", resource.into()),
            details: None,
        }
    }

    pub fn already_exists(resource: impl Into<String>) -> Self {
        Self {
            code: "ALREADY_EXISTS".to_string(),
            message: format!("{}已存在", resource.into()),
            details: None,
        }
    }

    // ============== 业务错误 ==============
    pub fn business_error(message: impl Into<String>) -> Self {
        Self {
            code: "BUSINESS_ERROR".to_string(),
            message: message.into(),
            details: None,
        }
    }

    pub fn operation_failed(message: impl Into<String>) -> Self {
        Self {
            code: "OPERATION_FAILED".to_string(),
            message: message.into(),
            details: None,
        }
    }

    // ============== 系统错误 ==============
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self {
            code: "INTERNAL_ERROR".to_string(),
            message: "系统错误，请稍后重试".to_string(),
            details: Some(message.into()),
        }
    }

    pub fn database_error(message: impl Into<String>) -> Self {
        Self {
            code: "DATABASE_ERROR".to_string(),
            message: "数据库操作失败".to_string(),
            details: Some(message.into()),
        }
    }

    pub fn network_error(message: impl Into<String>) -> Self {
        Self {
            code: "NETWORK_ERROR".to_string(),
            message: "网络请求失败".to_string(),
            details: Some(message.into()),
        }
    }

    /// 从anyhow::Error转换
    pub fn from_anyhow(err: anyhow::Error) -> Self {
        Self::internal_error(err.to_string())
    }

    /// 设置详细信息
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

/// 转换 anyhow::Error 为 ApiError
pub trait IntoApiError {
    fn into_api_error(self) -> ApiError;
}

impl<E: Into<anyhow::Error>> IntoApiError for E {
    fn into_api_error(self) -> ApiError {
        ApiError::from_anyhow(self.into())
    }
}

/// 从Result转换为ApiResponse
pub trait ToApiResponse<T> {
    fn to_response(self) -> ApiResponse<T>;
}

impl<T, E: IntoApiError> ToApiResponse<T> for Result<T, E> {
    fn to_response(self) -> ApiResponse<T> {
        match self {
            Ok(data) => ApiResponse::success(data),
            Err(err) => ApiResponse::error(err.into_api_error()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success_response() {
        let resp = ApiResponse::success(42);
        assert!(resp.success);
        assert_eq!(resp.data, Some(42));
        assert!(resp.error.is_none());
    }

    #[test]
    fn test_error_response() {
        let resp: ApiResponse<()> = ApiResponse::error(ApiError::not_found("用户"));
        assert!(!resp.success);
        assert_eq!(resp.error.unwrap().code, "NOT_FOUND");
    }

    #[test]
    fn test_validation_error() {
        let err = ApiError::validation_failed("email", "格式不正确");
        assert_eq!(err.code, "VALIDATION_FAILED");
        assert!(err.message.contains("email"));
    }
}
