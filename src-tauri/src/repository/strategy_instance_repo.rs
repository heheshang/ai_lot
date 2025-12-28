use super::Repository;
use crate::models::{StrategyInstance, StrategyInstanceListItem, CreateInstanceRequest};
use async_trait::async_trait;
use anyhow::Result;
use sqlx::{Pool, Sqlite};

pub struct StrategyInstanceRepository {
    pool: Pool<Sqlite>,
}

impl StrategyInstanceRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }

    /// 查找用户的所有策略实例（列表视图）
    pub async fn find_by_user(&self, user_id: &str) -> Result<Vec<StrategyInstanceListItem>> {
        let instances = sqlx::query_as::<_, StrategyInstanceListItem>(
            r#"
            SELECT id, name, strategy_id, symbol, timeframe, mode, status,
                   total_trades, total_pnl, created_at, updated_at
            FROM strategy_instances
            WHERE user_id = ?
            ORDER BY updated_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(instances)
    }

    /// 查找指定状态的策略实例
    pub async fn find_by_status(&self, status: &str) -> Result<Vec<StrategyInstance>> {
        let instances = sqlx::query_as::<_, StrategyInstance>(
            "SELECT * FROM strategy_instances WHERE status = ?"
        )
        .bind(status)
        .fetch_all(&self.pool)
        .await?;
        Ok(instances)
    }

    /// 查找用户运行中的实例
    pub async fn find_running_by_user(&self, user_id: &str) -> Result<Vec<StrategyInstance>> {
        let instances = sqlx::query_as::<_, StrategyInstance>(
            r#"
            SELECT * FROM strategy_instances
            WHERE user_id = ? AND status = 'running'
            ORDER BY created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(instances)
    }

    /// 创建新实例
    pub async fn create(&self, req: CreateInstanceRequest) -> Result<StrategyInstance> {
        let instance = StrategyInstance::new(req);

        sqlx::query(
            r#"
            INSERT INTO strategy_instances
            (id, strategy_id, user_id, name, parameters, exchange_id, symbol, timeframe,
             mode, status, error_message, start_time, stop_time, total_trades, total_pnl,
             max_drawdown, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&instance.id)
        .bind(&instance.strategy_id)
        .bind(&instance.user_id)
        .bind(&instance.name)
        .bind(&instance.parameters)
        .bind(&instance.exchange_id)
        .bind(&instance.symbol)
        .bind(&instance.timeframe)
        .bind(&instance.mode)
        .bind(&instance.status)
        .bind(&instance.error_message)
        .bind(instance.start_time)
        .bind(instance.stop_time)
        .bind(instance.total_trades)
        .bind(instance.total_pnl)
        .bind(instance.max_drawdown)
        .bind(instance.created_at)
        .bind(instance.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(instance)
    }

    /// 更新实例状态
    pub async fn update_status(&self, id: &str, status: &str, error_message: Option<&str>) -> Result<()> {
        let now = chrono::Utc::now().timestamp();

        if status == "running" {
            // 启动时更新 start_time
            sqlx::query(
                r#"
                UPDATE strategy_instances
                SET status = ?, start_time = ?, stop_time = NULL, error_message = NULL, updated_at = ?
                WHERE id = ?
                "#
            )
            .bind(status)
            .bind(now)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;
        } else if status == "stopped" {
            // 停止时更新 stop_time
            sqlx::query(
                r#"
                UPDATE strategy_instances
                SET status = ?, stop_time = ?, error_message = NULL, updated_at = ?
                WHERE id = ?
                "#
            )
            .bind(status)
            .bind(now)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;
        } else if status == "error" {
            // 错误状态
            sqlx::query(
                r#"
                UPDATE strategy_instances
                SET status = ?, error_message = ?, stop_time = ?, updated_at = ?
                WHERE id = ?
                "#
            )
            .bind(status)
            .bind(error_message)
            .bind(now)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;
        } else {
            // 其他状态
            sqlx::query(
                "UPDATE strategy_instances SET status = ?, updated_at = ? WHERE id = ?"
            )
            .bind(status)
            .bind(now)
            .bind(id)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    /// 更新实例统计信息
    pub async fn update_stats(
        &self,
        id: &str,
        trades: i64,
        pnl: f64,
        drawdown: f64,
    ) -> Result<()> {
        let now = chrono::Utc::now().timestamp();

        sqlx::query(
            r#"
            UPDATE strategy_instances
            SET total_trades = ?, total_pnl = ?, max_drawdown = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(trades)
        .bind(pnl)
        .bind(drawdown)
        .bind(now)
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// 删除实例
    pub async fn delete(&self, id: &str) -> Result<()> {
        sqlx::query("DELETE FROM strategy_instances WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// 检查实例名称是否存在（同一用户下）
    pub async fn name_exists(&self, user_id: &str, name: &str, exclude_id: Option<&str>) -> Result<bool> {
        let query = if let Some(exclude) = exclude_id {
            sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM strategy_instances WHERE user_id = ? AND name = ? AND id != ?"
            )
            .bind(user_id)
            .bind(name)
            .bind(exclude)
        } else {
            sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) FROM strategy_instances WHERE user_id = ? AND name = ?"
            )
            .bind(user_id)
            .bind(name)
        };

        let count = query.fetch_one(&self.pool).await?;
        Ok(count.0 > 0)
    }
}

#[async_trait]
impl Repository<StrategyInstance, String> for StrategyInstanceRepository {
    async fn find_by_id(&self, id: String) -> Result<Option<StrategyInstance>> {
        let instance = sqlx::query_as::<_, StrategyInstance>(
            "SELECT * FROM strategy_instances WHERE id = ?"
        )
        .bind(&id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(instance)
    }

    async fn find_all(&self) -> Result<Vec<StrategyInstance>> {
        let instances = sqlx::query_as::<_, StrategyInstance>(
            "SELECT * FROM strategy_instances ORDER BY updated_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(instances)
    }

    async fn insert(&self, entity: &StrategyInstance) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO strategy_instances
            (id, strategy_id, user_id, name, parameters, exchange_id, symbol, timeframe,
             mode, status, error_message, start_time, stop_time, total_trades, total_pnl,
             max_drawdown, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&entity.id)
        .bind(&entity.strategy_id)
        .bind(&entity.user_id)
        .bind(&entity.name)
        .bind(&entity.parameters)
        .bind(&entity.exchange_id)
        .bind(&entity.symbol)
        .bind(&entity.timeframe)
        .bind(&entity.mode)
        .bind(&entity.status)
        .bind(&entity.error_message)
        .bind(entity.start_time)
        .bind(entity.stop_time)
        .bind(entity.total_trades)
        .bind(entity.total_pnl)
        .bind(entity.max_drawdown)
        .bind(entity.created_at)
        .bind(entity.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn update(&self, entity: &StrategyInstance) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE strategy_instances
            SET name = ?, parameters = ?, status = ?, error_message = ?,
                start_time = ?, stop_time = ?, total_trades = ?, total_pnl = ?,
                max_drawdown = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&entity.name)
        .bind(&entity.parameters)
        .bind(&entity.status)
        .bind(&entity.error_message)
        .bind(entity.start_time)
        .bind(entity.stop_time)
        .bind(entity.total_trades)
        .bind(entity.total_pnl)
        .bind(entity.max_drawdown)
        .bind(entity.updated_at)
        .bind(&entity.id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: String) -> Result<()> {
        sqlx::query("DELETE FROM strategy_instances WHERE id = ?")
            .bind(&id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: 添加集成测试（需要测试数据库）
}
