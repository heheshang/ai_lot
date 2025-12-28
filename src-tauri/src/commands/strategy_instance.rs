//! Strategy Instance Commands for Tauri
//!
//! This module provides Tauri command handlers for strategy instance management.

use crate::infrastructure::Database;
use crate::models::{CreateInstanceRequest, StrategyInstance, StrategyInstanceListItem};
use crate::repository::{Repository, StrategyInstanceRepository};
use tauri::State;

/// 获取用户的所有策略实例列表
#[tauri::command]
pub async fn instance_list(
    db: State<'_, Database>,
    user_id: String,
) -> Result<Vec<StrategyInstanceListItem>, String> {
    log::info!("instance_list called: user_id={}", user_id);

    let repo = db.strategy_instance_repo();
    repo.find_by_user(&user_id)
        .await
        .map_err(|e| format!("Failed to list instances: {}", e))
}

/// 获取单个策略实例详情
#[tauri::command]
pub async fn instance_get(
    db: State<'_, Database>,
    id: String,
) -> Result<Option<StrategyInstance>, String> {
    log::info!("instance_get called: id={}", id);

    let repo = db.strategy_instance_repo();
    <StrategyInstanceRepository as Repository<StrategyInstance, String>>::find_by_id(&repo, id)
        .await
        .map_err(|e| format!("Failed to get instance: {}", e))
}

/// 创建新的策略实例
#[tauri::command]
pub async fn instance_create(
    db: State<'_, Database>,
    request: CreateInstanceRequest,
) -> Result<StrategyInstance, String> {
    log::info!("instance_create called: name={}, strategy={}", request.name, request.strategy_id);

    let repo = db.strategy_instance_repo();

    // 检查名称是否已存在
    let name_exists = repo
        .name_exists(&request.user_id, &request.name, None)
        .await
        .map_err(|e| format!("Failed to check name: {}", e))?;

    if name_exists {
        return Err(format!("Instance name '{}' already exists", request.name));
    }

    // 创建实例
    repo.create(request)
        .await
        .map_err(|e| format!("Failed to create instance: {}", e))
}

/// 更新策略实例状态
#[tauri::command]
pub async fn instance_update_status(
    db: State<'_, Database>,
    id: String,
    status: String,
    error_message: Option<String>,
) -> Result<(), String> {
    log::info!("instance_update_status called: id={}, status={}", id, status);

    let repo = db.strategy_instance_repo();
    repo.update_status(&id, &status, error_message.as_deref())
        .await
        .map_err(|e| format!("Failed to update status: {}", e))
}

/// 更新策略实例统计信息
#[tauri::command]
pub async fn instance_update_stats(
    db: State<'_, Database>,
    id: String,
    trades: i64,
    pnl: f64,
    drawdown: f64,
) -> Result<(), String> {
    log::info!(
        "instance_update_stats called: id={}, trades={}, pnl={}",
        id,
        trades,
        pnl
    );

    let repo = db.strategy_instance_repo();
    repo.update_stats(&id, trades, pnl, drawdown)
        .await
        .map_err(|e| format!("Failed to update stats: {}", e))
}

/// 删除策略实例
#[tauri::command]
pub async fn instance_delete(
    db: State<'_, Database>,
    id: String,
) -> Result<(), String> {
    log::info!("instance_delete called: id={}", id);

    let repo = db.strategy_instance_repo();
    repo.delete(&id)
        .await
        .map_err(|e| format!("Failed to delete instance: {}", e))
}

/// 获取用户运行中的实例
#[tauri::command]
pub async fn instance_list_running(
    db: State<'_, Database>,
    user_id: String,
) -> Result<Vec<StrategyInstance>, String> {
    log::info!("instance_list_running called: user_id={}", user_id);

    let repo = db.strategy_instance_repo();
    repo.find_running_by_user(&user_id)
        .await
        .map_err(|e| format!("Failed to list running instances: {}", e))
}

/// 获取内存中所有运行中的策略实例
#[tauri::command]
pub async fn strategy_instance_list_all(
    db: State<'_, Database>,
) -> Result<Vec<crate::core::strategy::engine::InstanceInfo>, String> {
    log::info!("strategy_instance_list_all called");

    let engine = db.get_strategy_engine();
    let instances = engine.list_instances().await;
    Ok(instances)
}

/// 获取内存中指定的运行中策略实例
#[tauri::command]
pub async fn strategy_instance_get(
    db: State<'_, Database>,
    id: String,
) -> Result<Option<crate::core::strategy::engine::InstanceInfo>, String> {
    log::info!("strategy_instance_get called: id={}", id);

    let engine = db.get_strategy_engine();
    let instance = engine.get_instance(&id).await;
    Ok(instance)
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: 添加集成测试
}
