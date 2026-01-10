//! Exchange configuration commands for Tauri
//!
//! This module provides Tauri command handlers for managing exchange configurations.

use crate::core::response::{ApiResponse, ApiError};
use crate::models::exchange::{ExchangeConfig, ExchangeName};
use crate::repository::ExchangeRepository;
use serde::{Deserialize, Serialize};
use tauri::State;
use uuid::Uuid;

/// Request to add or update an exchange configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveExchangeRequest {
    pub exchange_name: String,
    pub display_name: String,
    pub api_key: String,
    pub api_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_testnet: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>, // If provided, update existing config
}

/// Response with exchange config (without sensitive data)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeConfigResponse {
    pub id: String,
    pub exchange_name: String,
    pub display_name: String,
    pub is_testnet: bool,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
    /// Masked API key (first 4 chars + ****)
    pub api_key_masked: String,
}

/// Response with full exchange config (including decrypted keys for editing)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExchangeConfigDetailResponse {
    pub id: String,
    pub exchange_name: String,
    pub display_name: String,
    pub api_key: String,
    pub api_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,
    pub is_testnet: bool,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl From<ExchangeConfig> for ExchangeConfigResponse {
    fn from(config: ExchangeConfig) -> Self {
        let api_key_masked = if config.api_key_encrypted.len() > 8 {
            format!("{}****", &config.api_key_encrypted[..4])
        } else {
            "****".to_string()
        };

        Self {
            id: config.id,
            exchange_name: config.exchange_name,
            display_name: config.display_name,
            is_testnet: config.is_testnet,
            status: config.status,
            created_at: config.created_at,
            updated_at: config.updated_at,
            api_key_masked,
        }
    }
}

/// Add a new exchange configuration
#[tauri::command]
pub async fn exchange_add(
    db: State<'_, crate::infrastructure::Database>,
    user_id: String,
    request: SaveExchangeRequest,
) -> Result<ApiResponse<ExchangeConfigResponse>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!(
        "[{}] exchange_add called: user_id={}, exchange={}",
        request_id, user_id, request.exchange_name
    );

    // Validate exchange name
    let exchange_name = match ExchangeName::parse(&request.exchange_name) {
        Some(name) => name,
        None => {
            return Ok(ApiResponse::error(ApiError::invalid_parameter("exchange_name")).with_request_id(request_id));
        }
    };

    // Validate required passphrase for OKX
    if exchange_name == ExchangeName::OKX && request.passphrase.is_none() {
        return Ok(ApiResponse::error(ApiError::validation_failed("passphrase", "OKX必须提供passphrase")).with_request_id(request_id));
    }

    // Create encrypted config
    let config = match ExchangeConfig::create_encrypted(
        Uuid::new_v4().to_string(),
        user_id,
        request.exchange_name,
        request.display_name,
        &request.api_key,
        &request.api_secret,
        request.passphrase.as_deref(),
        request.is_testnet.unwrap_or(false),
    ) {
        Ok(config) => config,
        Err(e) => {
            log::error!("[{}] Failed to create exchange config: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("创建交易所配置失败")).with_request_id(request_id));
        }
    };

    // Save to database
    let repo = ExchangeRepository::new(db.pool.clone());
    if let Err(e) = repo.create(&config).await {
        log::error!("[{}] Failed to save exchange config: {}", request_id, e);
        return Ok(ApiResponse::error(ApiError::database_error(format!("保存失败: {}", e))).with_request_id(request_id));
    }

    log::info!("[{}] Exchange config created successfully", request_id);
    Ok(ApiResponse::success(config.into()).with_request_id(request_id))
}

/// Update an existing exchange configuration
#[tauri::command]
pub async fn exchange_update(
    db: State<'_, crate::infrastructure::Database>,
    config_id: String,
    request: SaveExchangeRequest,
) -> Result<ApiResponse<ExchangeConfigResponse>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!(
        "[{}] exchange_update called: config_id={}, exchange={}",
        request_id, config_id, request.exchange_name
    );

    // Get existing config
    let repo = ExchangeRepository::new(db.pool.clone());
    let mut config = match repo.find_by_id(&config_id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return Ok(ApiResponse::error(ApiError::not_found("交易所配置")).with_request_id(request_id));
        }
        Err(e) => {
            log::error!("[{}] Failed to find exchange config: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::database_error(format!("查询失败: {}", e))).with_request_id(request_id));
        }
    };

    // Validate passphrase for OKX
    if config.exchange_name == "okx" && request.passphrase.is_none() {
        return Ok(ApiResponse::error(ApiError::validation_failed("passphrase", "OKX必须提供passphrase")).with_request_id(request_id));
    }

    // Update config fields
    config.display_name = request.display_name;
    config.is_testnet = request.is_testnet.unwrap_or(false);

    // Update encrypted keys
    if let Err(e) = config.update_api_keys(
        &request.api_key,
        &request.api_secret,
        request.passphrase.as_deref(),
    ) {
        log::error!("[{}] Failed to encrypt keys: {}", request_id, e);
        return Ok(ApiResponse::error(ApiError::operation_failed("加密密钥失败")).with_request_id(request_id));
    }

    // Save to database
    if let Err(e) = repo.update(&config).await {
        log::error!("[{}] Failed to update exchange config: {}", request_id, e);
        return Ok(ApiResponse::error(ApiError::database_error(format!("更新失败: {}", e))).with_request_id(request_id));
    }

    log::info!("[{}] Exchange config updated successfully", request_id);
    Ok(ApiResponse::success(config.into()).with_request_id(request_id))
}

/// Get all exchange configurations for a user
#[tauri::command]
pub async fn exchange_list(
    db: State<'_, crate::infrastructure::Database>,
    user_id: String,
) -> Result<ApiResponse<Vec<ExchangeConfigResponse>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] exchange_list called: user_id={}", request_id, user_id);

    let repo = ExchangeRepository::new(db.pool.clone());
    let configs = match repo.find_by_user(&user_id).await {
        Ok(configs) => configs,
        Err(e) => {
            log::error!("[{}] Failed to get exchange configs: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("查询交易所配置失败")).with_request_id(request_id));
        }
    };

    Ok(ApiResponse::success(configs.into_iter().map(|c| c.into()).collect()).with_request_id(request_id))
}

/// Get exchange configuration by ID
#[tauri::command]
pub async fn exchange_get(
    db: State<'_, crate::infrastructure::Database>,
    config_id: String,
) -> Result<ApiResponse<ExchangeConfigResponse>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] exchange_get called: config_id={}", request_id, config_id);

    let repo = ExchangeRepository::new(db.pool.clone());
    let config = match repo.find_by_id(&config_id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return Ok(ApiResponse::error(ApiError::not_found("交易所配置")).with_request_id(request_id));
        }
        Err(e) => {
            log::error!("[{}] Failed to get exchange config: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("查询交易所配置失败")).with_request_id(request_id));
        }
    };

    Ok(ApiResponse::success(config.into()).with_request_id(request_id))
}

/// Get exchange configuration with decrypted keys (for editing)
#[tauri::command]
pub async fn exchange_get_detail(
    db: State<'_, crate::infrastructure::Database>,
    config_id: String,
) -> Result<ApiResponse<ExchangeConfigDetailResponse>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] exchange_get_detail called: config_id={}", request_id, config_id);

    let repo = ExchangeRepository::new(db.pool.clone());
    let config = match repo.find_by_id(&config_id).await {
        Ok(Some(c)) => c,
        Ok(None) => {
            return Ok(ApiResponse::error(ApiError::not_found("交易所配置")).with_request_id(request_id));
        }
        Err(e) => {
            log::error!("[{}] Failed to get exchange config: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("查询交易所配置失败")).with_request_id(request_id));
        }
    };

    // Decrypt the keys
    let (api_key, api_secret) = match config.get_decrypted_keys() {
        Ok(keys) => keys,
        Err(e) => {
            log::error!("[{}] Failed to decrypt keys: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("解密密钥失败")).with_request_id(request_id));
        }
    };
    let passphrase = match config.get_decrypted_passphrase() {
        Ok(p) => p,
        Err(e) => {
            log::error!("[{}] Failed to decrypt passphrase: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("解密passphrase失败")).with_request_id(request_id));
        }
    };

    Ok(ApiResponse::success(ExchangeConfigDetailResponse {
        id: config.id,
        exchange_name: config.exchange_name,
        display_name: config.display_name,
        api_key,
        api_secret,
        passphrase,
        is_testnet: config.is_testnet,
        status: config.status,
        created_at: config.created_at,
        updated_at: config.updated_at,
    }).with_request_id(request_id))
}

/// Delete an exchange configuration
#[tauri::command]
pub async fn exchange_delete(
    db: State<'_, crate::infrastructure::Database>,
    config_id: String,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] exchange_delete called: config_id={}", request_id, config_id);

    let repo = ExchangeRepository::new(db.pool.clone());
    match repo.delete(&config_id).await {
        Ok(()) => {
            log::info!("[{}] Exchange config deleted successfully", request_id);
            Ok(ApiResponse::success_empty().with_request_id(request_id))
        }
        Err(e) => {
            log::error!("[{}] Failed to delete exchange config: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("删除交易所配置失败")).with_request_id(request_id))
        }
    }
}

/// Update exchange configuration status
#[tauri::command]
pub async fn exchange_update_status(
    db: State<'_, crate::infrastructure::Database>,
    config_id: String,
    status: String,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!(
        "[{}] exchange_update_status called: config_id={}, status={}",
        request_id, config_id, status
    );

    // Validate status
    if !matches!(status.as_str(), "active" | "inactive" | "disabled") {
        return Ok(ApiResponse::error(ApiError::invalid_parameter("status")).with_request_id(request_id));
    }

    let repo = ExchangeRepository::new(db.pool.clone());
    match repo.update_status(&config_id, &status).await {
        Ok(()) => {
            log::info!("[{}] Exchange status updated successfully", request_id);
            Ok(ApiResponse::success_empty().with_request_id(request_id))
        }
        Err(e) => {
            log::error!("[{}] Failed to update exchange status: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("更新状态失败")).with_request_id(request_id))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_name_parse() {
        assert!(ExchangeName::parse("binance").is_some());
        assert!(ExchangeName::parse("okx").is_some());
        assert!(ExchangeName::parse("bybit").is_some());
        assert!(ExchangeName::parse("invalid").is_none());
    }

    #[test]
    fn test_exchange_config_response_masking() {
        // This would require creating a full ExchangeConfig with encrypted data
        // For now, just verify the struct compiles
        let response = ExchangeConfigResponse {
            id: "test".to_string(),
            exchange_name: "binance".to_string(),
            display_name: "Binance".to_string(),
            is_testnet: false,
            status: "active".to_string(),
            created_at: 0,
            updated_at: 0,
            api_key_masked: "abcd****".to_string(),
        };
        assert_eq!(response.api_key_masked, "abcd****");
    }
}
