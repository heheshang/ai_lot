use crate::core::strategy::{StrategyEngine, StrategyConfig};
use crate::infrastructure::database::Database;
use std::sync::Arc;
use tauri::State;

/// 启动策略实例
#[tauri::command]
pub async fn strategy_engine_start(
    db: State<'_, Database>,
    config: StrategyConfig,
) -> Result<String, String> {
    log::info!("Starting strategy engine instance: {}", config.name);

    // 获取或创建 EventBus
    let event_bus = db.get_event_bus();

    // 获取或创建 StrategyEngine
    let engine = db.get_strategy_engine();

    // 启动策略实例
    engine
        .start_instance(config)
        .await
        .map_err(|e| format!("Failed to start strategy: {}", e))
}

/// 停止策略实例
#[tauri::command]
pub async fn strategy_engine_stop(
    db: State<'_, Database>,
    id: String,
) -> Result<(), String> {
    log::info!("Stopping strategy engine instance: {}", id);

    let engine = db.get_strategy_engine();
    engine
        .stop_instance(&id)
        .await
        .map_err(|e| format!("Failed to stop strategy: {}", e))
}

/// 列出所有策略实例
#[tauri::command]
pub async fn strategy_engine_list(
    db: State<'_, Database>,
) -> Result<Vec<crate::core::strategy::InstanceInfo>, String> {
    let engine = db.get_strategy_engine();
    Ok(engine.list_instances().await)
}

/// 获取单个策略实例信息
#[tauri::command]
pub async fn strategy_engine_get(
    db: State<'_, Database>,
    id: String,
) -> Result<Option<crate::core::strategy::InstanceInfo>, String> {
    let engine = db.get_strategy_engine();
    Ok(engine.get_instance(&id).await)
}
