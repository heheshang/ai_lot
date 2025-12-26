// 模块声明
pub mod commands;
pub mod core;
pub mod infrastructure;
pub mod models;
pub mod repository;
pub mod services;

// 重新导出常用类型
pub use infrastructure::database::Database;
pub use services::MarketService;

// 导入 Tauri Manager trait 以使用状态管理
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    env_logger::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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
            tauri::async_runtime::block_on(async {
                let db = Database::new(db_path)
                    .await
                    .expect("Failed to create database");

                db.run_migrations()
                    .await
                    .expect("Failed to run migrations");

                // 注册到 Tauri 状态
                app.manage(db);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::user::login,
            commands::user::get_current_user,
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
            commands::strategy_engine::strategy_engine_list,
            commands::strategy_engine::strategy_engine_get,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
