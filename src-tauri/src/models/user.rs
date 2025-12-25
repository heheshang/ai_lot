use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub role_id: String,
    pub status: String,
    pub salt: String,
    pub failed_attempts: i32,
    pub locked_until: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub permissions: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserWithRole {
    pub id: String,
    pub username: String,
    pub display_name: Option<String>,
    pub role_id: String,
    pub role_name: String,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl User {
    /// 创建新用户
    pub fn new(
        id: String,
        username: String,
        password_hash: String,
        salt: String,
        role_id: String,
    ) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id,
            username,
            password_hash,
            display_name: None,
            role_id,
            status: "active".to_string(),
            salt,
            failed_attempts: 0,
            locked_until: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// 检查用户是否被锁定
    pub fn is_locked(&self) -> bool {
        if self.status != "active" {
            return true;
        }
        if let Some(locked_until) = self.locked_until {
            let now = chrono::Utc::now().timestamp();
            return locked_until > now;
        }
        false
    }
}
