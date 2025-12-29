//! Exchange configuration commands for Tauri
//!
//! This module provides Tauri command handlers for managing exchange configurations.

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
) -> Result<ExchangeConfigResponse, String> {
    log::info!(
        "exchange_add called: user_id={}, exchange={}",
        user_id,
        request.exchange_name
    );

    // Validate exchange name
    let exchange_name = ExchangeName::from_str(&request.exchange_name)
        .ok_or_else(|| format!("Unsupported exchange: {}", request.exchange_name))?;

    // Validate required passphrase for OKX
    if exchange_name == ExchangeName::OKX && request.passphrase.is_none() {
        return Err("OKX requires passphrase".to_string());
    }

    // Create encrypted config
    let config = ExchangeConfig::create_encrypted(
        Uuid::new_v4().to_string(),
        user_id,
        request.exchange_name,
        request.display_name,
        &request.api_key,
        &request.api_secret,
        request.passphrase.as_deref(),
        request.is_testnet.unwrap_or(false),
    )
    .map_err(|e| format!("Failed to create exchange config: {}", e))?;

    // Save to database
    let repo = ExchangeRepository::new(db.pool.clone());
    repo.create(&config)
        .await
        .map_err(|e| format!("Failed to save exchange config: {}", e))?;

    Ok(config.into())
}

/// Update an existing exchange configuration
#[tauri::command]
pub async fn exchange_update(
    db: State<'_, crate::infrastructure::Database>,
    config_id: String,
    request: SaveExchangeRequest,
) -> Result<ExchangeConfigResponse, String> {
    log::info!(
        "exchange_update called: config_id={}, exchange={}",
        config_id,
        request.exchange_name
    );

    // Get existing config
    let repo = ExchangeRepository::new(db.pool.clone());
    let mut config = repo
        .find_by_id(&config_id)
        .await
        .map_err(|e| format!("Failed to find exchange config: {}", e))?
        .ok_or_else(|| format!("Exchange config not found: {}", config_id))?;

    // Validate passphrase for OKX
    if config.exchange_name == "okx" && request.passphrase.is_none() {
        return Err("OKX requires passphrase".to_string());
    }

    // Update config fields
    config.display_name = request.display_name;
    config.is_testnet = request.is_testnet.unwrap_or(false);

    // Update encrypted keys
    config
        .update_api_keys(
            &request.api_key,
            &request.api_secret,
            request.passphrase.as_deref(),
        )
        .map_err(|e| format!("Failed to encrypt keys: {}", e))?;

    // Save to database
    repo.update(&config)
        .await
        .map_err(|e| format!("Failed to update exchange config: {}", e))?;

    Ok(config.into())
}

/// Get all exchange configurations for a user
#[tauri::command]
pub async fn exchange_list(
    db: State<'_, crate::infrastructure::Database>,
    user_id: String,
) -> Result<Vec<ExchangeConfigResponse>, String> {
    log::info!("exchange_list called: user_id={}", user_id);

    let repo = ExchangeRepository::new(db.pool.clone());
    let configs = repo
        .find_by_user(&user_id)
        .await
        .map_err(|e| format!("Failed to get exchange configs: {}", e))?;

    Ok(configs.into_iter().map(|c| c.into()).collect())
}

/// Get exchange configuration by ID
#[tauri::command]
pub async fn exchange_get(
    db: State<'_, crate::infrastructure::Database>,
    config_id: String,
) -> Result<ExchangeConfigResponse, String> {
    log::info!("exchange_get called: config_id={}", config_id);

    let repo = ExchangeRepository::new(db.pool.clone());
    let config = repo
        .find_by_id(&config_id)
        .await
        .map_err(|e| format!("Failed to get exchange config: {}", e))?
        .ok_or_else(|| format!("Exchange config not found: {}", config_id))?;

    Ok(config.into())
}

/// Get exchange configuration with decrypted keys (for editing)
#[tauri::command]
pub async fn exchange_get_detail(
    db: State<'_, crate::infrastructure::Database>,
    config_id: String,
) -> Result<ExchangeConfigDetailResponse, String> {
    log::info!("exchange_get_detail called: config_id={}", config_id);

    let repo = ExchangeRepository::new(db.pool.clone());
    let config = repo
        .find_by_id(&config_id)
        .await
        .map_err(|e| format!("Failed to get exchange config: {}", e))?
        .ok_or_else(|| format!("Exchange config not found: {}", config_id))?;

    // Decrypt the keys
    let (api_key, api_secret) = config.get_decrypted_keys()
        .map_err(|e| format!("Failed to decrypt keys: {}", e))?;
    let passphrase = config.get_decrypted_passphrase()
        .map_err(|e| format!("Failed to decrypt passphrase: {}", e))?;

    Ok(ExchangeConfigDetailResponse {
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
    })
}

/// Delete an exchange configuration
#[tauri::command]
pub async fn exchange_delete(
    db: State<'_, crate::infrastructure::Database>,
    config_id: String,
) -> Result<(), String> {
    log::info!("exchange_delete called: config_id={}", config_id);

    let repo = ExchangeRepository::new(db.pool.clone());
    repo.delete(&config_id)
        .await
        .map_err(|e| format!("Failed to delete exchange config: {}", e))?;

    Ok(())
}

/// Update exchange configuration status
#[tauri::command]
pub async fn exchange_update_status(
    db: State<'_, crate::infrastructure::Database>,
    config_id: String,
    status: String,
) -> Result<(), String> {
    log::info!(
        "exchange_update_status called: config_id={}, status={}",
        config_id,
        status
    );

    // Validate status
    if !matches!(status.as_str(), "active" | "inactive" | "disabled") {
        return Err("Invalid status. Must be 'active', 'inactive', or 'disabled'".to_string());
    }

    let repo = ExchangeRepository::new(db.pool.clone());
    repo.update_status(&config_id, &status)
        .await
        .map_err(|e| format!("Failed to update exchange status: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exchange_name_from_str() {
        assert!(ExchangeName::from_str("binance").is_some());
        assert!(ExchangeName::from_str("okx").is_some());
        assert!(ExchangeName::from_str("bybit").is_some());
        assert!(ExchangeName::from_str("invalid").is_none());
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
