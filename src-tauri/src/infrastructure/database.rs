use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::path::PathBuf;
use std::str::FromStr;
use tauri::{AppHandle, Manager};

use crate::repository::UserRepository;
use crate::infrastructure::audit::AuditLogger;

pub struct Database {
    pub pool: SqlitePool,
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

        Ok(Self { pool })
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

    /// 获取审计日志记录器
    pub fn audit_logger(&self) -> AuditLogger {
        AuditLogger::new(self.pool.clone())
    }
}

// 全局类型别名
pub type DbPool = SqlitePool;
