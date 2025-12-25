use super::Repository;
use crate::models::{User, UserWithRole};
use async_trait::async_trait;
use anyhow::Result;
use sqlx::{Pool, Sqlite};

pub struct UserRepository {
    pool: Pool<Sqlite>,
}

impl UserRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// 通过用户名查找用户（用于验证密码，包含哈希）
    pub async fn find_by_username_with_hash(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    /// 通过用户名查找用户
    pub async fn find_by_username(&self, username: &str) -> Result<Option<UserWithRole>> {
        let user = sqlx::query_as::<_, UserWithRole>(
            r#"
            SELECT u.id, u.username, u.display_name, u.role_id, r.name as role_name,
                   u.status, u.created_at, u.updated_at
            FROM users u
            LEFT JOIN roles r ON u.role_id = r.id
            WHERE u.username = ?
            "#
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    /// 通过 ID 查找用户（带角色信息）
    pub async fn find_by_id_with_role(&self, id: &str) -> Result<Option<UserWithRole>> {
        let user = sqlx::query_as::<_, UserWithRole>(
            r#"
            SELECT u.id, u.username, u.display_name, u.role_id, r.name as role_name,
                   u.status, u.created_at, u.updated_at
            FROM users u
            LEFT JOIN roles r ON u.role_id = r.id
            WHERE u.id = ?
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    /// 检查用户名是否存在
    pub async fn username_exists(&self, username: &str) -> Result<bool> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(&self.pool)
            .await?;
        Ok(count.0 > 0)
    }
}

#[async_trait]
impl Repository<User, String> for UserRepository {
    async fn find_by_id(&self, id: String) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(&id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn find_all(&self) -> Result<Vec<User>> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await?;
        Ok(users)
    }

    async fn insert(&self, entity: &User) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO users (id, username, password_hash, display_name, role_id, status,
                              salt, failed_attempts, locked_until, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&entity.id)
        .bind(&entity.username)
        .bind(&entity.password_hash)
        .bind(&entity.display_name)
        .bind(&entity.role_id)
        .bind(&entity.status)
        .bind(&entity.salt)
        .bind(entity.failed_attempts)
        .bind(entity.locked_until)
        .bind(entity.created_at)
        .bind(entity.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, entity: &User) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE users
            SET username = ?, display_name = ?, role_id = ?, status = ?,
                failed_attempts = ?, locked_until = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&entity.username)
        .bind(&entity.display_name)
        .bind(&entity.role_id)
        .bind(&entity.status)
        .bind(entity.failed_attempts)
        .bind(entity.locked_until)
        .bind(entity.updated_at)
        .bind(&entity.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: String) -> Result<()> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(&id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
