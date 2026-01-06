use super::Repository;
use crate::models::{Strategy, StrategyDto, StrategyListItem};
use async_trait::async_trait;
use anyhow::Result;
use sqlx::{Pool, Sqlite};

pub struct StrategyRepository {
    pool: Pool<Sqlite>,
}

impl StrategyRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// 查找用户的所有策略（列表视图）
    pub async fn find_by_user(&self, user_id: &str) -> Result<Vec<StrategyListItem>> {
        let strategies = sqlx::query_as::<_, StrategyListItem>(
            r#"
            SELECT id, name, category, tags, version, status, created_at, updated_at
            FROM strategies
            WHERE user_id = ?
            ORDER BY updated_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(strategies)
    }

    /// 通过 ID 查找策略（返回 DTO）
    pub async fn find_by_id_dto(&self, id: &str) -> Result<Option<StrategyDto>> {
        let strategy = sqlx::query_as::<_, Strategy>(
            "SELECT * FROM strategies WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match strategy {
            Some(s) => {
                let dto = s.to_dto()?;
                Ok(Some(dto))
            }
            None => Ok(None)
        }
    }

    /// 保存策略（插入或更新）
    pub async fn save(&self, dto: &StrategyDto) -> Result<String> {
        let entity = dto.to_entity()?;
        let now = chrono::Utc::now().timestamp();

        // 检查是否存在
        let existing = sqlx::query_as::<_, Strategy>(
            "SELECT * FROM strategies WHERE id = ?"
        )
        .bind(&dto.id)
        .fetch_optional(&self.pool)
        .await?;

        if existing.is_some() {
            // 更新
            sqlx::query(
                r#"
                UPDATE strategies
                SET name = ?, description = ?, code = ?, language = ?, parameters = ?,
                    category = ?, tags = ?, status = ?, updated_at = ?
                WHERE id = ?
                "#
            )
            .bind(&entity.name)
            .bind(&entity.description)
            .bind(&entity.code)
            .bind(&entity.language)
            .bind(&entity.parameters)
            .bind(&entity.category)
            .bind(&entity.tags)
            .bind(&entity.status)
            .bind(now)
            .bind(&entity.id)
            .execute(&self.pool)
            .await?;
        } else {
            // 插入
            sqlx::query(
                r#"
                INSERT INTO strategies (id, user_id, name, description, code, language,
                                       parameters, category, tags, version, parent_id,
                                       status, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#
            )
            .bind(&entity.id)
            .bind(&entity.user_id)
            .bind(&entity.name)
            .bind(&entity.description)
            .bind(&entity.code)
            .bind(&entity.language)
            .bind(&entity.parameters)
            .bind(&entity.category)
            .bind(&entity.tags)
            .bind(entity.version)
            .bind(&entity.parent_id)
            .bind(&entity.status)
            .bind(entity.created_at)
            .bind(now)
            .execute(&self.pool)
            .await?;
        }

        Ok(dto.id.clone())
    }

    /// 删除策略
    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM strategies WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// 检查策略名称是否存在（同一用户下）
    pub async fn name_exists(&self, user_id: &str, name: &str, exclude_id: Option<&str>) -> Result<bool> {
        let query = if let Some(exclude) = exclude_id {
            sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM strategies WHERE user_id = ? AND name = ? AND id != ?"
            )
            .bind(user_id)
            .bind(name)
            .bind(exclude)
        } else {
            sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM strategies WHERE user_id = ? AND name = ?"
            )
            .bind(user_id)
            .bind(name)
        };

        let count = query.fetch_one(&self.pool).await?;
        Ok(count.0 > 0)
    }
}

#[async_trait]
impl Repository<Strategy, String> for StrategyRepository {
    async fn find_by_id(&self, id: String) -> Result<Option<Strategy>> {
        let strategy = sqlx::query_as::<_, Strategy>("SELECT * FROM strategies WHERE id = ?")
            .bind(&id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(strategy)
    }

    async fn find_all(&self) -> Result<Vec<Strategy>> {
        let strategies = sqlx::query_as::<_, Strategy>("SELECT * FROM strategies ORDER BY updated_at DESC")
            .fetch_all(&self.pool)
            .await?;
        Ok(strategies)
    }

    async fn insert(&self, entity: &Strategy) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO strategies (id, user_id, name, description, code, language,
                                   parameters, category, tags, version, parent_id,
                                   status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&entity.id)
        .bind(&entity.user_id)
        .bind(&entity.name)
        .bind(&entity.description)
        .bind(&entity.code)
        .bind(&entity.language)
        .bind(&entity.parameters)
        .bind(&entity.category)
        .bind(&entity.tags)
        .bind(entity.version)
        .bind(&entity.parent_id)
        .bind(&entity.status)
        .bind(entity.created_at)
        .bind(entity.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, entity: &Strategy) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE strategies
            SET name = ?, description = ?, code = ?, language = ?, parameters = ?,
                category = ?, tags = ?, status = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&entity.name)
        .bind(&entity.description)
        .bind(&entity.code)
        .bind(&entity.language)
        .bind(&entity.parameters)
        .bind(&entity.category)
        .bind(&entity.tags)
        .bind(&entity.status)
        .bind(entity.updated_at)
        .bind(&entity.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: String) -> Result<()> {
        sqlx::query("DELETE FROM strategies WHERE id = ?")
            .bind(&id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
