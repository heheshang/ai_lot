use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

    pub fn from_str(s: &str) -> Option<Self> {
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
        Self::from_str(s.as_str()).unwrap_or(Self::Binance)
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
}
