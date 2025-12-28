//! Exchange configuration repository
//!
//! This module provides database operations for managing exchange configurations.

use crate::models::exchange::ExchangeConfig;
use anyhow::Result;
use sqlx::{SqlitePool, Row};

/// Exchange configuration repository
pub struct ExchangeRepository {
    pool: SqlitePool,
}

impl ExchangeRepository {
    /// Create a new exchange repository
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Create a new exchange configuration
    pub async fn create(&self, config: &ExchangeConfig) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO exchange_configs (
                id, user_id, exchange_name, display_name,
                api_key_encrypted, api_secret_encrypted, passphrase_encrypted,
                is_testnet, status, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&config.id)
        .bind(&config.user_id)
        .bind(&config.exchange_name)
        .bind(&config.display_name)
        .bind(&config.api_key_encrypted)
        .bind(&config.api_secret_encrypted)
        .bind(&config.passphrase_encrypted)
        .bind(config.is_testnet)
        .bind(&config.status)
        .bind(config.created_at)
        .bind(config.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get exchange configuration by ID
    pub async fn find_by_id(&self, id: &str) -> Result<Option<ExchangeConfig>> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, exchange_name, display_name,
                   api_key_encrypted, api_secret_encrypted, passphrase_encrypted,
                   is_testnet, status, created_at, updated_at
            FROM exchange_configs
            WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(ExchangeConfig {
                id: r.get("id"),
                user_id: r.get("user_id"),
                exchange_name: r.get("exchange_name"),
                display_name: r.get("display_name"),
                api_key_encrypted: r.get("api_key_encrypted"),
                api_secret_encrypted: r.get("api_secret_encrypted"),
                passphrase_encrypted: r.get("passphrase_encrypted"),
                is_testnet: r.get("is_testnet"),
                status: r.get("status"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            })),
            None => Ok(None),
        }
    }

    /// Get all exchange configurations for a user
    pub async fn find_by_user(&self, user_id: &str) -> Result<Vec<ExchangeConfig>> {
        let rows = sqlx::query(
            r#"
            SELECT id, user_id, exchange_name, display_name,
                   api_key_encrypted, api_secret_encrypted, passphrase_encrypted,
                   is_testnet, status, created_at, updated_at
            FROM exchange_configs
            WHERE user_id = ?
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        let configs = rows
            .iter()
            .map(|r| ExchangeConfig {
                id: r.get("id"),
                user_id: r.get("user_id"),
                exchange_name: r.get("exchange_name"),
                display_name: r.get("display_name"),
                api_key_encrypted: r.get("api_key_encrypted"),
                api_secret_encrypted: r.get("api_secret_encrypted"),
                passphrase_encrypted: r.get("passphrase_encrypted"),
                is_testnet: r.get("is_testnet"),
                status: r.get("status"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            })
            .collect();

        Ok(configs)
    }

    /// Get active exchange configuration for a user by exchange name
    pub async fn find_active_by_name(&self, user_id: &str, exchange_name: &str) -> Result<Option<ExchangeConfig>> {
        let row = sqlx::query(
            r#"
            SELECT id, user_id, exchange_name, display_name,
                   api_key_encrypted, api_secret_encrypted, passphrase_encrypted,
                   is_testnet, status, created_at, updated_at
            FROM exchange_configs
            WHERE user_id = ? AND exchange_name = ? AND status = 'active'
            ORDER BY updated_at DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .bind(exchange_name)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(r) => Ok(Some(ExchangeConfig {
                id: r.get("id"),
                user_id: r.get("user_id"),
                exchange_name: r.get("exchange_name"),
                display_name: r.get("display_name"),
                api_key_encrypted: r.get("api_key_encrypted"),
                api_secret_encrypted: r.get("api_secret_encrypted"),
                passphrase_encrypted: r.get("passphrase_encrypted"),
                is_testnet: r.get("is_testnet"),
                status: r.get("status"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
            })),
            None => Ok(None),
        }
    }

    /// Update exchange configuration
    pub async fn update(&self, config: &ExchangeConfig) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE exchange_configs
            SET display_name = ?,
                api_key_encrypted = ?,
                api_secret_encrypted = ?,
                passphrase_encrypted = ?,
                is_testnet = ?,
                status = ?,
                updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&config.display_name)
        .bind(&config.api_key_encrypted)
        .bind(&config.api_secret_encrypted)
        .bind(&config.passphrase_encrypted)
        .bind(config.is_testnet)
        .bind(&config.status)
        .bind(config.updated_at)
        .bind(&config.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Delete exchange configuration
    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM exchange_configs WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Update exchange status
    pub async fn update_status(&self, id: &str, status: &str) -> Result<()> {
        let updated_at = chrono::Utc::now().timestamp_millis();
        sqlx::query(
            r#"
            UPDATE exchange_configs
            SET status = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(status)
        .bind(updated_at)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
