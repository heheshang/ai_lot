//! 用户相关命令
//!
//! 包含用户登录、登出、信息获取等功能

use crate::core::auth::{generate_access_token, generate_refresh_token};
use crate::core::response::{ApiResponse, ApiError};
use crate::infrastructure::{CryptoService, Database, AuditEvent};
use crate::models::UserWithRole;
use serde::{Deserialize, Serialize};
use tauri::State;

/// 登录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 登录响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: UserWithRole,
    /// 访问令牌 (24小时有效)
    pub access_token: String,
    /// 刷新令牌 (7天有效)
    pub refresh_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间（秒）
    pub expires_in: i64,
}

/// 刷新令牌请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// 刷新令牌响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenResponse {
    /// 新的访问令牌
    pub access_token: String,
    /// 过期时间（秒）
    pub expires_in: i64,
}

/// 用户登录
///
/// 验证用户凭据并返回访问令牌和刷新令牌
///
/// # 参数
/// - `db`: 数据库状态
/// - `request`: 登录请求，包含用户名和密码
///
/// # 返回
/// 成功时返回用户信息和 JWT 令牌，失败时返回错误信息
#[tauri::command]
pub async fn login(
    db: State<'_, Database>,
    request: LoginRequest,
) -> Result<ApiResponse<LoginResponse>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Login attempt for user: {}", request_id, request.username);

    // 获取用户仓库
    let user_repo = db.user_repo();

    // 查找用户（包含密码哈希）
    let user = match user_repo
        .find_by_username_with_hash(&request.username)
        .await
    {
        Ok(Some(u)) => u,
        Ok(None) => {
            log::warn!("[{}] User not found: {}", request_id, request.username);
            return Ok(ApiResponse::error(ApiError::unauthorized("用户名或密码错误")));
        }
        Err(e) => {
            log::error!("[{}] Database error: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("查询用户失败: {}", e))));
        }
    };

    // 检查用户是否被锁定
    if user.is_locked() {
        log::warn!("[{}] User locked: {}", request_id, request.username);
        return Ok(ApiResponse::error(ApiError::unauthorized("用户已被锁定")));
    }

    // 验证密码
    match CryptoService::verify_password(&request.password, &user.password_hash) {
        Ok(false) => {
            log::warn!("[{}] Invalid password for user: {}", request_id, request.username);
            return Ok(ApiResponse::error(ApiError::unauthorized("用户名或密码错误")));
        }
        Err(e) => {
            log::error!("[{}] Password verification error: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::internal_error("密码验证失败")));
        }
        Ok(true) => {}
    }

    // 检查用户状态
    if user.status != "active" {
        log::warn!("[{}] User status not active: {}", request_id, user.status);
        return Ok(ApiResponse::error(ApiError::forbidden(format!("用户状态异常: {}", user.status))));
    }

    // 获取用户信息（包含角色）
    let user_with_role = match user_repo
        .find_by_id_with_role(&user.id)
        .await
    {
        Ok(Some(u)) => u,
        Ok(None) => {
            log::error!("[{}] User info not found after login: {}", request_id, user.id);
            return Ok(ApiResponse::error(ApiError::internal_error("获取用户信息失败")));
        }
        Err(e) => {
            log::error!("[{}] Database error getting user info: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("获取用户信息失败: {}", e))));
        }
    };

    // 生成 JWT 访问令牌 (24小时有效)
    let access_token = match generate_access_token(
        &user_with_role.id,
        &user_with_role.username,
        &user_with_role.role_name,
    ) {
        Ok(token) => token,
        Err(e) => {
            log::error!("[{}] Failed to generate access token: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::internal_error("生成访问令牌失败")));
        }
    };

    // 生成 JWT 刷新令牌 (7天有效)
    let refresh_token = match generate_refresh_token(&user_with_role.id) {
        Ok(token) => token,
        Err(e) => {
            log::error!("[{}] Failed to generate refresh token: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::internal_error("生成刷新令牌失败")));
        }
    };

    log::info!(
        "[{}] User {} logged in successfully",
        request_id,
        request.username
    );

    // 记录审计日志
    let audit_service = db.audit_logger();
    if let Err(e) = audit_service
        .log(AuditEvent::UserLogin {
            user_id: user_with_role.id.clone(),
            username: user_with_role.username.clone(),
        })
        .await
    {
        log::warn!("[{}] Failed to log audit event: {}", request_id, e);
    }

    Ok(ApiResponse::success(LoginResponse {
        user: user_with_role,
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: 24 * 60 * 60, // 24小时（秒）
    })
    .with_request_id(request_id))
}

/// 刷新访问令牌
///
/// 使用刷新令牌获取新的访问令牌
///
/// # 参数
/// - `request`: 包含刷新令牌的请求
///
/// # 返回
/// 成功时返回新的访问令牌，失败时返回错误信息
#[tauri::command]
pub async fn refresh_access_token(
    request: RefreshTokenRequest,
) -> Result<ApiResponse<RefreshTokenResponse>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Refresh token request", request_id);

    // 验证刷新令牌
    let claims = match crate::core::auth::verify_token(&request.refresh_token) {
        Ok(claims) => claims,
        Err(e) => {
            log::warn!("[{}] Invalid refresh token: {}", request_id, e.message);
            return Ok(ApiResponse::error(ApiError::unauthorized("刷新令牌无效或已过期")));
        }
    };

    // 获取用户信息以获取最新的用户名和角色
    // 注意：这里我们需要访问数据库来获取最新的用户信息
    // 但为了避免循环依赖，我们暂时使用 claims 中的信息
    // 在实际应用中，应该从数据库获取最新的用户信息

    // 生成新的访问令牌
    let new_access_token = match generate_access_token(
        &claims.sub,
        &claims.username, // 从 claims 中获取用户名（可能为空）
        &claims.role,     // 从 claims 中获取角色（可能为空）
    ) {
        Ok(token) => token,
        Err(e) => {
            log::error!("[{}] Failed to generate new access token: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::internal_error("生成访问令牌失败")));
        }
    };

    log::info!("[{}] Access token refreshed for user: {}", request_id, claims.sub);

    Ok(ApiResponse::success(RefreshTokenResponse {
        access_token: new_access_token,
        expires_in: 24 * 60 * 60, // 24小时（秒）
    })
    .with_request_id(request_id))
}

/// 获取当前登录用户
///
/// 根据用户 ID 获取用户信息（包含角色）
///
/// # 参数
/// - `db`: 数据库状态
/// - `user_id`: 用户 ID
///
/// # 返回
/// 成功时返回用户信息，失败时返回错误信息
#[tauri::command]
pub async fn get_current_user(
    db: State<'_, Database>,
    user_id: String,
) -> Result<ApiResponse<UserWithRole>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Get current user for id: {}", request_id, user_id);

    let user_repo = db.user_repo();

    match user_repo.find_by_id_with_role(&user_id).await {
        Ok(Some(user)) => {
            log::debug!("[{}] User found: {}", request_id, user.username);
            Ok(ApiResponse::success(user).with_request_id(request_id))
        }
        Ok(None) => {
            log::warn!("[{}] User not found: {}", request_id, user_id);
            Ok(ApiResponse::error(ApiError::not_found("用户")))
        }
        Err(e) => {
            log::error!("[{}] Database error: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::database_error(format!("获取用户信息失败: {}", e))))
        }
    }
}

/// 验证令牌
///
/// 验证 JWT 令牌的有效性并返回用户信息
///
/// # 参数
/// - `token`: JWT 令牌
///
/// # 返回
/// 成功时返回用户 ID、用户名和角色，失败时返回错误信息
#[tauri::command]
pub async fn verify_token(token: String) -> Result<ApiResponse<TokenInfo>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Verify token request", request_id);

    match crate::core::auth::verify_token(&token) {
        Ok(claims) => {
            log::debug!("[{}] Token valid for user: {}", request_id, claims.sub);
            Ok(ApiResponse::success(TokenInfo {
                user_id: claims.sub,
                username: claims.username,
                role: claims.role,
                expires_at: claims.exp,
            })
            .with_request_id(request_id))
        }
        Err(e) => {
            log::warn!("[{}] Invalid token: {}", request_id, e.message);
            Ok(ApiResponse::error(e))
        }
    }
}

/// 令牌信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub user_id: String,
    pub username: String,
    pub role: String,
    pub expires_at: i64,
}

/// 退出登录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoutRequest {
    pub user_id: String,
    pub username: String,
}

/// 用户退出登录
///
/// 记录退出登录审计日志
///
/// # 参数
/// - `db`: 数据库状态
/// - `request`: 退出请求，包含用户ID和用户名
///
/// # 返回
/// 成功时返回确认信息，失败时返回错误信息
#[tauri::command]
pub async fn logout(
    db: State<'_, Database>,
    request: LogoutRequest,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!(
        "[{}] Logout request for user: {} ({})",
        request_id,
        request.username,
        request.user_id
    );

    // 记录审计日志
    let audit_service = db.audit_logger();
    if let Err(e) = audit_service
        .log(AuditEvent::UserLogout {
            user_id: request.user_id.clone(),
            username: request.username.clone(),
        })
        .await
    {
        log::warn!("[{}] Failed to log audit event: {}", request_id, e);
    }

    log::info!(
        "[{}] User {} logged out successfully",
        request_id,
        request.username
    );

    Ok(ApiResponse::success(()).with_request_id(request_id))
}