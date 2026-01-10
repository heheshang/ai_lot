use crate::core::response::{ApiResponse, ApiError};
use crate::infrastructure::config::AppConfig;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

/// 配置更新器 (部分更新)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfigUpdater {
    pub app: Option<AppSettingsUpdater>,
    pub database: Option<DatabaseConfigUpdater>,
    pub risk: Option<RiskConfigUpdater>,
    pub notifications: Option<NotificationConfigUpdater>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettingsUpdater {
    pub language: Option<String>,
    pub theme: Option<String>,
    pub auto_save_interval: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfigUpdater {
    pub path: Option<String>,
    pub backup_interval_hours: Option<u32>,
    pub backup_retention_days: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskConfigUpdater {
    pub enabled: Option<bool>,
    pub default_action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfigUpdater {
    pub dingtalk_webhook: Option<String>,
    pub smtp_server: Option<String>,
    pub smtp_port: Option<u16>,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
    pub notification_emails: Option<String>,
}

/// 获取配置
#[tauri::command]
pub async fn config_get(handle: AppHandle) -> Result<ApiResponse<AppConfig>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let config_manager = match get_config_manager(&handle) {
        Ok(m) => m,
        Err(e) => {
            log::error!("[{}] Failed to get config manager: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("获取配置管理器失败")).with_request_id(request_id));
        }
    };

    match config_manager.load() {
        Ok(config) => Ok(ApiResponse::success(config).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to load config: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed(format!("加载配置失败: {}", e))).with_request_id(request_id))
        }
    }
}

/// 更新配置
#[tauri::command]
pub async fn config_update(
    handle: AppHandle,
    updater: serde_json::Value,
) -> Result<ApiResponse<AppConfig>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let config_manager = match get_config_manager(&handle) {
        Ok(m) => m,
        Err(e) => {
            log::error!("[{}] Failed to get config manager: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("获取配置管理器失败")).with_request_id(request_id));
        }
    };

    match config_manager.update(|cfg| apply_config_update(cfg, updater)) {
        Ok(config) => Ok(ApiResponse::success(config).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to update config: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed(format!("更新配置失败: {}", e))).with_request_id(request_id))
        }
    }
}

/// 重置配置为默认值
#[tauri::command]
pub async fn config_reset(handle: AppHandle) -> Result<ApiResponse<AppConfig>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let config_manager = match get_config_manager(&handle) {
        Ok(m) => m,
        Err(e) => {
            log::error!("[{}] Failed to get config manager: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("获取配置管理器失败")).with_request_id(request_id));
        }
    };

    let default_config = AppConfig::default();
    if let Err(e) = config_manager.save(&default_config) {
        log::error!("[{}] Failed to save default config: {}", request_id, e);
        return Ok(ApiResponse::error(ApiError::operation_failed("保存默认配置失败")).with_request_id(request_id));
    }

    Ok(ApiResponse::success(default_config).with_request_id(request_id))
}

/// 获取配置管理器
fn get_config_manager(handle: &AppHandle) -> Result<crate::infrastructure::ConfigManager, String> {
    let data_dir = handle.path().app_data_dir().map_err(|e| e.to_string())?;
    Ok(crate::infrastructure::ConfigManager::new(data_dir))
}

/// 应用配置更新
fn apply_config_update(config: &mut AppConfig, updater: serde_json::Value) {
    if let Some(obj) = updater.as_object() {
        // 更新 app 配置
        if let Some(app) = obj.get("app").and_then(|v| v.as_object()) {
            if let Some(language) = app.get("language").and_then(|v| v.as_str()) {
                config.app.language = language.to_string();
            }
            if let Some(theme) = app.get("theme").and_then(|v| v.as_str()) {
                config.app.theme = theme.to_string();
            }
            if let Some(interval) = app.get("auto_save_interval").and_then(|v| v.as_u64()) {
                config.app.auto_save_interval = interval;
            }
        }

        // 更新 database 配置
        if let Some(db) = obj.get("database").and_then(|v| v.as_object()) {
            if let Some(path) = db.get("path").and_then(|v| v.as_str()) {
                config.database.path = path.to_string();
            }
            if let Some(interval) = db.get("backup_interval_hours").and_then(|v| v.as_u64()) {
                config.database.backup_interval_hours = interval as u32;
            }
            if let Some(retention) = db.get("backup_retention_days").and_then(|v| v.as_u64()) {
                config.database.backup_retention_days = retention as u32;
            }
        }

        // 更新 risk 配置
        if let Some(risk) = obj.get("risk").and_then(|v| v.as_object()) {
            if let Some(enabled) = risk.get("enabled").and_then(|v| v.as_bool()) {
                config.risk.enabled = enabled;
            }
            if let Some(action) = risk.get("default_action").and_then(|v| v.as_str()) {
                config.risk.default_action = action.to_string();
            }
        }

        // 更新 notifications 配置
        if let Some(notif) = obj.get("notifications").and_then(|v| v.as_object()) {
            if let Some(webhook) = notif.get("dingtalk_webhook").and_then(|v| v.as_str()) {
                if !webhook.is_empty() {
                    config.notifications.dingtalk_webhook = Some(webhook.to_string());
                } else {
                    config.notifications.dingtalk_webhook = None;
                }
            }
            if let Some(server) = notif.get("smtp_server").and_then(|v| v.as_str()) {
                if !server.is_empty() {
                    config.notifications.smtp_server = Some(server.to_string());
                } else {
                    config.notifications.smtp_server = None;
                }
            }
            if let Some(port) = notif.get("smtp_port").and_then(|v| v.as_u64()) {
                config.notifications.smtp_port = Some(port as u16);
            }
            if let Some(username) = notif.get("smtp_username").and_then(|v| v.as_str()) {
                if !username.is_empty() {
                    config.notifications.smtp_username = Some(username.to_string());
                } else {
                    config.notifications.smtp_username = None;
                }
            }
            if let Some(password) = notif.get("smtp_password").and_then(|v| v.as_str()) {
                if !password.is_empty() {
                    config.notifications.smtp_password = Some(password.to_string());
                } else {
                    config.notifications.smtp_password = None;
                }
            }
            if let Some(emails) = notif.get("notification_emails").and_then(|v| v.as_str()) {
                if !emails.is_empty() {
                    config.notifications.notification_emails = Some(emails.to_string());
                } else {
                    config.notifications.notification_emails = None;
                }
            }
        }
    }
}
