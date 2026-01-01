// 模块声明
pub mod commands;
pub mod core;
pub mod infrastructure;
pub mod models;
pub mod repository;
pub mod services;
pub mod types;

// Test helpers - public but only meant for testing
pub mod test_helpers;

// 重新导出常用类型
pub use infrastructure::database::Database;
pub use services::{MarketService, TradeService};

// 导入 Tauri Manager trait 以使用状态管理
use tauri::{Manager, Emitter};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            // 获取数据目录
            let data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get data dir");
            let db_path = data_dir.join("ai-lot.db");

            log::info!("Data directory: {}", data_dir.display());
            log::info!("Database path: {}", db_path.display());

            // 创建数据库连接并运行迁移
            let (db, market_service, backtest_service) = tauri::async_runtime::block_on(async {
                use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
                use std::str::FromStr;

                // 确保目录存在
                if let Some(parent) = db_path.parent() {
                    std::fs::create_dir_all(parent)
                        .expect("Failed to create database directory");
                }

                // 配置数据库连接
                let options = SqliteConnectOptions::from_str(&format!("sqlite:{}", db_path.display()))
                    .expect("Invalid database path")
                    .create_if_missing(true)
                    .foreign_keys(true)
                    .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
                    .synchronous(sqlx::sqlite::SqliteSynchronous::Normal);

                // 创建连接池
                let pool = SqlitePool::connect_with(options)
                    .await
                    .expect("Failed to connect to database");

                log::info!("Database pool created: {}", db_path.display());

                // 创建 Database 实例
                let db = Database::new_with_pool(pool.clone())
                    .await
                    .expect("Failed to create Database");

                // 运行迁移
                db.run_migrations()
                    .await
                    .expect("Failed to run migrations");

                // 创建 MarketService (使用内部的 Database 结构)
                let market_service = std::sync::Arc::new({
                    use crate::infrastructure::Database;
                    let ms_db = Database::new_with_pool(pool.clone()).await.expect("Failed to create MarketService Database");
                    services::MarketService::new(ms_db)
                });

                // 初始化 Binance 交易所
                if let Err(e) = market_service.init_binance(None, None).await {
                    log::warn!("Failed to initialize Binance exchange: {}", e);
                } else {
                    log::info!("Binance exchange initialized successfully");
                }

                // 创建 BacktestService
                let backtest_service = std::sync::Arc::new({
                    use crate::infrastructure::Database;
                    let bt_db = Database::new_with_pool(pool).await.expect("Failed to create BacktestService Database");
                    services::BacktestService::new(bt_db)
                });

                (db, market_service, backtest_service)
            });

            // 注册 Database 到 Tauri 状态
            app.manage(db);

            // 注册 MarketService 到 Tauri 状态
            app.manage(market_service.clone());

            // 注册 BacktestService 到 Tauri 状态
            app.manage(backtest_service.clone());

            // 启动市场事件转发器 - 将 EventBus 事件转发到前端
            let app_handle = app.handle().clone();
            let event_bus = market_service.event_bus();
            let mut market_event_rx = event_bus.subscribe_market();

            tauri::async_runtime::spawn(async move {
                log::info!("Market event forwarder started");

                use crate::core::event::MarketEvent;
                use serde_json::json;

                while let Ok(event) = market_event_rx.recv().await {
                    let event_name = match &event {
                        MarketEvent::Ticker(_) => "market:ticker",
                        MarketEvent::Kline(_) => "market:kline",
                    };

                    let payload = match event {
                        MarketEvent::Ticker(ticker) => json!(ticker),
                        MarketEvent::Kline(kline) => json!(kline),
                    };

                    // 发送事件到前端
                    if let Err(e) = app_handle.emit(event_name, payload) {
                        log::error!("Failed to emit market event: {}", e);
                    }
                }

                log::info!("Market event forwarder stopped");
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::user::login,
            commands::user::logout,
            commands::user::get_current_user,
            commands::user::refresh_access_token,
            commands::user::verify_token,
            commands::audit::get_audit_logs,
            commands::audit::audit_export_csv,
            commands::market::market_subscribe_ticker,
            commands::market::market_get_klines,
            commands::market::market_get_symbols,
            commands::market::market_get_status,
            commands::market::market_unsubscribe_ticker,
            commands::strategy::strategy_list,
            commands::strategy::strategy_get,
            commands::strategy::strategy_save,
            commands::strategy::strategy_delete,
            commands::strategy_test::strategy_test_execute,
            commands::strategy_test::strategy_validate_code,
            commands::strategy_engine::strategy_engine_start,
            commands::strategy_engine::strategy_engine_stop,
            commands::strategy_engine::strategy_engine_pause,
            commands::strategy_engine::strategy_engine_resume,
            commands::strategy_engine::strategy_engine_list,
            commands::strategy_engine::strategy_engine_get,
            // Strategy Instance commands
            commands::strategy_instance::instance_list,
            commands::strategy_instance::instance_get,
            commands::strategy_instance::instance_create,
            commands::strategy_instance::instance_update_status,
            commands::strategy_instance::instance_update_stats,
            commands::strategy_instance::instance_delete,
            commands::strategy_instance::instance_list_running,
            commands::strategy_instance::strategy_instance_list_all,
            commands::strategy_instance::strategy_instance_get,
            // Trade commands
            commands::trade::trade_place_order,
            commands::trade::trade_cancel_order,
            commands::trade::trade_get_order,
            commands::trade::trade_get_orders,
            commands::trade::trade_get_open_orders,
            commands::trade::trade_sync_order_status,
            commands::trade::trade_get_positions,
            commands::trade::trade_get_balance,
            commands::trade::trade_cancel_all_orders,
            commands::trade::trade_close_position,
            // Risk commands
            commands::risk::get_risk_overview,
            commands::risk::get_active_alerts,
            commands::risk::handle_alert,
            commands::risk::ignore_alert,
            commands::risk::get_risk_rules,
            commands::risk::update_risk_rule,
            commands::risk::get_alert_history,
            commands::risk::get_alert_detail,
            commands::risk::add_alert_note,
            commands::risk::delete_alert,
            // Emergency commands
            commands::emergency::emergency_stop,
            // Config commands
            commands::config::config_get,
            commands::config::config_update,
            commands::config::config_reset,
            // Backup commands
            commands::backup::backup_create,
            commands::backup::backup_restore,
            commands::backup::backup_list,
            commands::backup::backup_delete,
            commands::backup::backup_cleanup,
            commands::backup::backup_verify_integrity,
            commands::backup::backup_start_auto,
            commands::backup::backup_stop_auto,
            // Exchange commands
            commands::exchange::exchange_add,
            commands::exchange::exchange_update,
            commands::exchange::exchange_list,
            commands::exchange::exchange_get,
            commands::exchange::exchange_get_detail,
            commands::exchange::exchange_delete,
            commands::exchange::exchange_update_status,
            // Strategy Debug commands
            commands::strategy_debug::get_strategy_logs,
            commands::strategy_debug::get_strategy_metrics,
            commands::strategy_debug::get_strategy_variables,
            commands::strategy_debug::clear_strategy_logs,
            commands::strategy_debug::set_strategy_log_level,
            commands::strategy_debug::get_strategy_log_level,
            commands::strategy_debug::generate_test_logs,
            // Backtest commands
            commands::backtest::backtest_create_job,
            commands::backtest::backtest_get_job,
            commands::backtest::backtest_list_jobs,
            commands::backtest::backtest_run_job,
            commands::backtest::backtest_run,
            commands::backtest::backtest_delete_job,
            commands::backtest::backtest_get_result,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
