use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;

use crate::repository::{UserRepository, StrategyRepository};
use crate::infrastructure::audit::AuditLogger;
use crate::core::{EventBus, StrategyEngine};

pub struct Database {
    pub pool: SqlitePool,
    event_bus: Arc<EventBus>,
    strategy_engine: Arc<StrategyEngine>,
}

impl Database {
    /// 创建数据库连接
    pub async fn new(db_path: PathBuf) -> Result<Self> {
        // 确保目录存在 (使用同步方式)
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                anyhow::anyhow!("Failed to create database directory {}: {}", parent.display(), e)
            })?;
            log::info!("Database directory ready: {}", parent.display());
        }

        // 使用 SqliteConnectOptions 来配置连接
        let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path.display()))?
            .create_if_missing(true)
            .foreign_keys(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
            .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);

        log::info!("Connecting to database: {}", db_path.display());

        // 创建连接池
        let pool = SqlitePool::connect_with(options).await
            .map_err(|e| anyhow::anyhow!("Failed to connect to database at {}: {}", db_path.display(), e))?;

        log::info!("Database connected successfully: {}", db_path.display());

        // 创建 EventBus
        let event_bus = Arc::new(EventBus::new());
        log::info!("EventBus initialized");

        // 创建 StrategyEngine
        let strategy_engine = Arc::new(StrategyEngine::new(event_bus.clone()));
        log::info!("StrategyEngine initialized");

        Ok(Self {
            pool,
            event_bus,
            strategy_engine,
        })
    }

    /// 运行数据库迁移
    pub async fn run_migrations(&self) -> Result<()> {
        log::info!("Running database migrations...");
        sqlx::migrate!("./migrations").run(&self.pool).await?;
        log::info!("Migrations completed successfully");
        Ok(())
    }

    /// 从 AppHandle 获取数据库实例
    pub fn from_app(handle: &AppHandle) -> Result<&Self> {
        Ok(handle.state::<Self>().inner())
    }

    /// 获取 User Repository
    pub fn user_repo(&self) -> UserRepository {
        UserRepository::new(self.pool.clone())
    }

    /// 获取 Strategy Repository
    pub fn strategy_repo(&self) -> StrategyRepository {
        StrategyRepository::new(self.pool.clone())
    }

    /// 获取审计日志记录器
    pub fn audit_logger(&self) -> AuditLogger {
        AuditLogger::new(self.pool.clone())
    }

    /// 获取 EventBus
    pub fn get_event_bus(&self) -> Arc<EventBus> {
        self.event_bus.clone()
    }

    /// 获取 StrategyEngine
    pub fn get_strategy_engine(&self) -> Arc<StrategyEngine> {
        self.strategy_engine.clone()
    }
}

// 全局类型别名
pub type DbPool = SqlitePool;
