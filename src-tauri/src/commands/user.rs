use crate::infrastructure::{CryptoService, Database};
use crate::models::UserWithRole;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: UserWithRole,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResult<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
    }
}

/// 用户登录
#[tauri::command]
pub async fn login(
    db: State<'_, Database>,
    request: LoginRequest,
) -> Result<LoginResponse, String> {
    log::info!("Login attempt for user: {}", request.username);

    // 获取用户仓库
    let user_repo = db.user_repo();

    // 查找用户（包含密码哈希）
    let user = user_repo
        .find_by_username_with_hash(&request.username)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "用户名或密码错误".to_string())?;

    // 检查用户是否被锁定
    if user.is_locked() {
        return Err("用户已被锁定".to_string());
    }

    // 验证密码
    let is_valid = CryptoService::verify_password(&request.password, &user.password_hash)
        .map_err(|e| e.to_string())?;

    if !is_valid {
        return Err("用户名或密码错误".to_string());
    }

    // 检查用户状态
    if user.status != "active" {
        return Err(format!("用户状态异常: {}", user.status));
    }

    // 获取用户信息（包含角色）
    let user_with_role = user_repo
        .find_by_id_with_role(&user.id)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "获取用户信息失败".to_string())?;

    // 生成简单 token (生产环境应使用 JWT)
    let token = format!("{}:{}", user.id, chrono::Utc::now().timestamp());

    log::info!("User {} logged in successfully", request.username);

    Ok(LoginResponse {
        user: user_with_role,
        token,
    })
}

/// 获取当前登录用户
#[tauri::command]
pub async fn get_current_user(
    db: State<'_, Database>,
    user_id: String,
) -> Result<Option<UserWithRole>, String> {
    log::info!("Get current user for id: {}", user_id);

    let user_repo = db.user_repo();
    user_repo
        .find_by_id_with_role(&user_id)
        .await
        .map_err(|e| e.to_string())
}
