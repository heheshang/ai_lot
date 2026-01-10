use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExchangeName {
    Binance,
    OKX,
    Bybit,
}

impl ExchangeName {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Binance => "binance",
            Self::OKX => "okx",
            Self::Bybit => "bybit",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "binance" => Some(Self::Binance),
            "okx" => Some(Self::OKX),
            "bybit" => Some(Self::Bybit),
            _ => None,
        }
    }
}

impl fmt::Display for ExchangeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<String> for ExchangeName {
    fn from(s: String) -> Self {
        Self::parse(s.as_str()).unwrap_or(Self::Binance)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ExchangeConfig {
    pub id: String,
    pub user_id: String,
    pub exchange_name: String,
    pub display_name: String,
    pub api_key_encrypted: String,
    pub api_secret_encrypted: String,
    pub passphrase_encrypted: Option<String>,
    pub is_testnet: bool,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl ExchangeConfig {
    /// 检查配置是否活跃
    pub fn is_active(&self) -> bool {
        self.status == "active"
    }

    /// 获取解密后的 API 密钥对
    ///
    /// 返回 (api_key, api_secret)
    pub fn get_decrypted_keys(&self) -> Result<(String, String), String> {
        use crate::infrastructure::crypto::CryptoService;

        let api_key = CryptoService::decrypt_api_key(&self.api_key_encrypted)
            .map_err(|e| format!("Failed to decrypt API key: {}", e))?;
        let api_secret = CryptoService::decrypt_api_key(&self.api_secret_encrypted)
            .map_err(|e| format!("Failed to decrypt API secret: {}", e))?;

        Ok((api_key, api_secret))
    }

    /// 获取解密后的 passphrase（如果存在）
    pub fn get_decrypted_passphrase(&self) -> Result<Option<String>, String> {
        use crate::infrastructure::crypto::CryptoService;

        if let Some(encrypted) = &self.passphrase_encrypted {
            let decrypted = CryptoService::decrypt_api_key(encrypted)
                .map_err(|e| format!("Failed to decrypt passphrase: {}", e))?;
            Ok(Some(decrypted))
        } else {
            Ok(None)
        }
    }

    /// 创建新的交易所配置（加密敏感信息）
    ///
    /// # 参数
    /// - `id`: 配置 ID
    /// - `user_id`: 用户 ID
    /// - `exchange_name`: 交易所名称
    /// - `display_name`: 显示名称
    /// - `api_key`: API Key（将被加密）
    /// - `api_secret`: API Secret（将被加密）
    /// - `passphrase`: Passphrase（可选，将被加密）
    /// - `is_testnet`: 是否为测试网
    ///
    /// # 返回
    /// 返回加密后的 ExchangeConfig
    pub fn create_encrypted(
        id: String,
        user_id: String,
        exchange_name: String,
        display_name: String,
        api_key: &str,
        api_secret: &str,
        passphrase: Option<&str>,
        is_testnet: bool,
    ) -> Result<Self, String> {
        use crate::infrastructure::crypto::CryptoService;
        use chrono::Utc;

        let api_key_encrypted = CryptoService::encrypt_api_key(api_key)
            .map_err(|e| format!("Failed to encrypt API key: {}", e))?;
        let api_secret_encrypted = CryptoService::encrypt_api_key(api_secret)
            .map_err(|e| format!("Failed to encrypt API secret: {}", e))?;
        let passphrase_encrypted = passphrase
            .map(|p| CryptoService::encrypt_api_key(p)
                .map_err(|e| format!("Failed to encrypt passphrase: {}", e)))
            .transpose()?;

        let now = Utc::now().timestamp_millis();

        Ok(Self {
            id,
            user_id,
            exchange_name,
            display_name,
            api_key_encrypted,
            api_secret_encrypted,
            passphrase_encrypted,
            is_testnet,
            status: "active".to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    /// 更新 API 密钥（加密）
    pub fn update_api_keys(
        &mut self,
        api_key: &str,
        api_secret: &str,
        passphrase: Option<&str>,
    ) -> Result<(), String> {
        use crate::infrastructure::crypto::CryptoService;

        self.api_key_encrypted = CryptoService::encrypt_api_key(api_key)
            .map_err(|e| format!("Failed to encrypt API key: {}", e))?;
        self.api_secret_encrypted = CryptoService::encrypt_api_key(api_secret)
            .map_err(|e| format!("Failed to encrypt API secret: {}", e))?;
        self.passphrase_encrypted = passphrase
            .map(|p| CryptoService::encrypt_api_key(p)
                .map_err(|e| format!("Failed to encrypt passphrase: {}", e)))
            .transpose()?;

        self.updated_at = chrono::Utc::now().timestamp_millis();
        Ok(())
    }
}
