use anyhow::Result;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use tauri::{AppHandle, Manager};

use crate::repository::{UserRepository, StrategyRepository, StrategyInstanceRepository};
use crate::infrastructure::audit::AuditLogger;
use crate::core::EventBus;
use crate::core::strategy::StrategyEngine;
use crate::core::trade::exchange::binance::BinanceExchange;
use crate::core::trade::exchange::Exchange;
use crate::services::TradeService;
use tokio::sync::RwLock;

pub struct Database {
    pub pool: SqlitePool,
    event_bus: Arc<EventBus>,
    strategy_engine: Arc<crate::core::strategy::StrategyEngine>,
    exchange: Arc<dyn Exchange>,
    trade_service: Arc<RwLock<Option<Arc<TradeService>>>>,
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

        // 创建 Exchange（使用 Binance，暂不配置 API 密钥）
        let exchange: Arc<dyn Exchange> = Arc::new(BinanceExchange::new(None, None));
        log::info!("Exchange initialized (Binance)");

        // 创建 StrategyInstanceRepository
        let instance_repo = Arc::new(StrategyInstanceRepository::new(pool.clone()));
        log::info!("StrategyInstanceRepository initialized");

        // 创建 StrategyEngine
        let strategy_engine = Arc::new(StrategyEngine::new(
            event_bus.clone(),
            exchange.clone(),
            instance_repo,
        ));
        log::info!("StrategyEngine initialized");

        // TradeService will be initialized lazily when needed
        let trade_service = Arc::new(RwLock::new(None));

        Ok(Self {
            pool,
            event_bus,
            strategy_engine,
            exchange,
            trade_service,
        })
    }

    /// 使用现有连接池创建 Database 实例
    pub async fn new_with_pool(pool: SqlitePool) -> Result<Self> {
        log::info!("Creating Database with existing pool");

        // 创建 EventBus
        let event_bus = Arc::new(EventBus::new());
        log::info!("EventBus initialized");

        // 创建 Exchange（使用 Binance，暂不配置 API 密钥）
        let exchange: Arc<dyn Exchange> = Arc::new(BinanceExchange::new(None, None));
        log::info!("Exchange initialized (Binance)");

        // 创建 StrategyInstanceRepository
        let instance_repo = Arc::new(StrategyInstanceRepository::new(pool.clone()));
        log::info!("StrategyInstanceRepository initialized");

        // 创建 StrategyEngine
        let strategy_engine = Arc::new(StrategyEngine::new(
            event_bus.clone(),
            exchange.clone(),
            instance_repo,
        ));
        log::info!("StrategyEngine initialized");

        // TradeService will be initialized lazily when needed
        let trade_service = Arc::new(RwLock::new(None));

        Ok(Self {
            pool,
            event_bus,
            strategy_engine,
            exchange,
            trade_service,
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

    /// 获取 StrategyInstance Repository
    pub fn strategy_instance_repo(&self) -> StrategyInstanceRepository {
        StrategyInstanceRepository::new(self.pool.clone())
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
    pub fn get_strategy_engine(&self) -> Arc<crate::core::strategy::StrategyEngine> {
        self.strategy_engine.clone()
    }

    /// 获取 Exchange
    pub fn get_exchange(&self) -> Arc<dyn Exchange> {
        self.exchange.clone()
    }

    /// 获取或初始化 TradeService (async)
    pub async fn get_trade_service(&self) -> Arc<TradeService> {
        let mut service_guard = self.trade_service.write().await;
        if let Some(service) = &*service_guard {
            service.clone()
        } else {
            let db = crate::infrastructure::Database {
                pool: self.pool.clone(),
                event_bus: self.event_bus.clone(),
                strategy_engine: self.strategy_engine.clone(),
                exchange: self.exchange.clone(),
                trade_service: Arc::new(RwLock::new(None)),
            };
            let service = Arc::new(TradeService::new(self.exchange.clone(), db));
            *service_guard = Some(service.clone());
            service
        }
    }

    /// Create a Database instance suitable for TradeService
    /// This creates a new Database wrapper with the same pool but cloned references
    pub fn as_trade_db(&self) -> crate::infrastructure::Database {
        crate::infrastructure::Database {
            pool: self.pool.clone(),
            event_bus: self.event_bus.clone(),
            strategy_engine: self.strategy_engine.clone(),
            exchange: self.exchange.clone(),
            trade_service: Arc::new(RwLock::new(None)),
        }
    }
}

// 全局类型别名
pub type DbPool = SqlitePool;
