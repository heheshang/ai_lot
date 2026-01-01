//! JWT (JSON Web Token) 认证模块
//!
//! 提供安全的 JWT token 生成、验证和刷新功能

use crate::core::response::ApiError;
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::env;

/// JWT Claims 结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// 用户 ID
    pub sub: String,
    /// 用户名
    pub username: String,
    /// 角色
    pub role: String,
    /// 签发时间
    pub iat: i64,
    /// 过期时间
    pub exp: i64,
    /// 签发者
    pub iss: String,
    /// JWT ID (用于令牌撤销)
    pub jti: String,
}

/// JWT 配置
pub struct JwtConfig {
    /// 访问令牌有效期（小时）
    pub access_token_duration_hours: i64,
    /// 刷新令牌有效期（天）
    pub refresh_token_duration_days: i64,
    /// 签发者
    pub issuer: String,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            access_token_duration_hours: 24,
            refresh_token_duration_days: 7,
            issuer: "ai-lot".to_string(),
        }
    }
}

/// JWT 管理器
pub struct JwtManager {
    config: JwtConfig,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtManager {
    /// 创建新的 JWT 管理器
    ///
    /// # 错误
    /// - 如果环境变量 JWT_SECRET 未设置，将使用默认密钥（不推荐生产环境）
    pub fn new(config: JwtConfig) -> Result<Self, ApiError> {
        // 从环境变量获取密钥，或使用默认密钥
        let secret = env::var("JWT_SECRET").unwrap_or_else(|_| {
            log::warn!("JWT_SECRET not set, using default key (NOT recommended for production)");
            "ai-lot-secret-key-change-in-production-2024".to_string()
        });

        // 验证密钥长度（至少 32 字节）
        if secret.len() < 32 {
            return Err(ApiError::internal_error(
                "JWT_SECRET must be at least 32 characters"
            ));
        }

        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        let decoding_key = DecodingKey::from_secret(secret.as_ref());

        Ok(Self {
            config,
            encoding_key,
            decoding_key,
        })
    }

    /// 使用默认配置创建 JWT 管理器
    pub fn with_default_config() -> Result<Self, ApiError> {
        Self::new(JwtConfig::default())
    }

    /// 生成访问令牌
    ///
    /// # 参数
    /// - `user_id`: 用户 ID
    /// - `username`: 用户名
    /// - `role`: 用户角色
    pub fn generate_access_token(
        &self,
        user_id: &str,
        username: &str,
        role: &str,
    ) -> Result<String, ApiError> {
        let now = Utc::now();
        let expiration = now
            .checked_add_signed(Duration::hours(self.config.access_token_duration_hours))
            .ok_or_else(|| ApiError::internal_error("Failed to calculate expiration time"))?;

        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            role: role.to_string(),
            iat: now.timestamp(),
            exp: expiration.timestamp(),
            iss: self.config.issuer.clone(),
            jti: uuid::Uuid::new_v4().to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &self.encoding_key,
        )
        .map_err(|e| ApiError::internal_error(format!("Failed to encode token: {}", e)))
    }

    /// 生成刷新令牌
    ///
    /// # 参数
    /// - `user_id`: 用户 ID
    pub fn generate_refresh_token(&self, user_id: &str) -> Result<String, ApiError> {
        let now = Utc::now();
        let expiration = now
            .checked_add_signed(Duration::days(self.config.refresh_token_duration_days))
            .ok_or_else(|| ApiError::internal_error("Failed to calculate expiration time"))?;

        let claims = Claims {
            sub: user_id.to_string(),
            username: "".to_string(), // 刷新令牌不需要用户名
            role: "".to_string(),      // 刷新令牌不需要角色
            iat: now.timestamp(),
            exp: expiration.timestamp(),
            iss: self.config.issuer.clone(),
            jti: uuid::Uuid::new_v4().to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &self.encoding_key,
        )
        .map_err(|e| ApiError::internal_error(format!("Failed to encode refresh token: {}", e)))
    }

    /// 验证令牌
    ///
    /// # 返回
    /// 返回解析后的 Claims
    pub fn verify_token(&self, token: &str) -> Result<Claims, ApiError> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::default()
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                ApiError::unauthorized("Token has expired")
            }
            jsonwebtoken::errors::ErrorKind::InvalidSignature => {
                ApiError::unauthorized("Invalid token signature")
            }
            _ => ApiError::unauthorized(format!("Invalid token: {}", e)),
        })?;

        // 验证签发者
        if token_data.claims.iss != self.config.issuer {
            return Err(ApiError::unauthorized("Invalid token issuer"));
        }

        Ok(token_data.claims)
    }

    /// 刷新访问令牌
    ///
    /// # 参数
    /// - `refresh_token`: 刷新令牌
    /// - `username`: 用户名
    /// - `role`: 用户角色
    ///
    /// # 返回
    /// 新的访问令牌
    pub fn refresh_access_token(
        &self,
        refresh_token: &str,
        username: &str,
        role: &str,
    ) -> Result<String, ApiError> {
        // 验证刷新令牌
        let claims = self.verify_token(refresh_token)?;

        // 生成新的访问令牌
        self.generate_access_token(&claims.sub, username, role)
    }

    /// 从令牌中提取用户 ID
    ///
    /// # 注意
    /// 此方法不验证令牌，只解析。用于快速获取用户 ID
    pub fn extract_user_id_unsafe(&self, token: &str) -> Result<String, ApiError> {
        let claims = self.verify_token(token)?;
        Ok(claims.sub)
    }
}

// ============== 便捷函数 ==============

lazy_static::lazy_static! {
    /// 全局 JWT 管理器实例
    static ref JWT_MANAGER: JwtManager = {
        JwtManager::with_default_config()
            .expect("Failed to initialize JWT manager")
    };
}

/// 生成访问令牌（便捷函数）
pub fn generate_access_token(
    user_id: &str,
    username: &str,
    role: &str,
) -> Result<String, ApiError> {
    JWT_MANAGER.generate_access_token(user_id, username, role)
}

/// 生成刷新令牌（便捷函数）
pub fn generate_refresh_token(user_id: &str) -> Result<String, ApiError> {
    JWT_MANAGER.generate_refresh_token(user_id)
}

/// 验证令牌（便捷函数）
pub fn verify_token(token: &str) -> Result<Claims, ApiError> {
    JWT_MANAGER.verify_token(token)
}

/// 刷新访问令牌（便捷函数）
pub fn refresh_access_token(
    refresh_token: &str,
    username: &str,
    role: &str,
) -> Result<String, ApiError> {
    JWT_MANAGER.refresh_access_token(refresh_token, username, role)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_verify_access_token() {
        let manager = JwtManager::with_default_config().unwrap();

        let token = manager
            .generate_access_token("user123", "testuser", "管理员")
            .unwrap();

        let claims = manager.verify_token(&token).unwrap();

        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.role, "管理员");
        assert_eq!(claims.iss, "ai-lot");
    }

    #[test]
    fn test_expired_token() {
        let manager = JwtManager {
            config: JwtConfig {
                access_token_duration_hours: -1, // 已过期
                ..Default::default()
            },
            encoding_key: JWT_MANAGER.encoding_key.clone(),
            decoding_key: JWT_MANAGER.decoding_key.clone(),
        };

        let token = manager
            .generate_access_token("user123", "testuser", "管理员")
            .unwrap();

        let result = manager.verify_token(&token);

        assert!(result.is_err());
        match result {
            Err(e) => assert_eq!(e.code, "UNAUTHORIZED"),
            Ok(_) => panic!("Expected error for expired token"),
        }
    }

    #[test]
    fn test_invalid_token() {
        let manager = JwtManager::with_default_config().unwrap();

        let result = manager.verify_token("invalid_token");

        assert!(result.is_err());
    }

    #[test]
    fn test_refresh_token() {
        let manager = JwtManager::with_default_config().unwrap();

        let refresh_token = manager.generate_refresh_token("user123").unwrap();

        let new_token = manager
            .refresh_access_token(&refresh_token, "testuser", "管理员")
            .unwrap();

        let claims = manager.verify_token(&new_token).unwrap();

        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.role, "管理员");
    }

    #[test]
    fn test_convenience_functions() {
        let token = generate_access_token("user456", "convenience", "交易员").unwrap();
        let claims = verify_token(&token).unwrap();

        assert_eq!(claims.sub, "user456");
        assert_eq!(claims.username, "convenience");
        assert_eq!(claims.role, "交易员");
    }
}
