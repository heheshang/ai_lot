//! Strategy Instance Commands for Tauri
//!
//! This module provides Tauri command handlers for strategy instance management.

use crate::core::response::{ApiResponse, ApiError};
use crate::infrastructure::Database;
use crate::models::{CreateInstanceRequest, StrategyInstance, StrategyInstanceListItem};
use crate::repository::{Repository, StrategyInstanceRepository};
use tauri::State;

/// 获取用户的所有策略实例列表
#[tauri::command]
pub async fn instance_list(
    db: State<'_, Database>,
    user_id: String,
) -> Result<ApiResponse<Vec<StrategyInstanceListItem>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] instance_list called: user_id={}", request_id, user_id);

    let repo = db.strategy_instance_repo();
    match repo.find_by_user(&user_id).await {
        Ok(instances) => Ok(ApiResponse::success(instances).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to list instances: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("查询实例列表失败")).with_request_id(request_id))
        }
    }
}

/// 获取单个策略实例详情
#[tauri::command]
pub async fn instance_get(
    db: State<'_, Database>,
    id: String,
) -> Result<ApiResponse<Option<StrategyInstance>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] instance_get called: id={}", request_id, id);

    let repo = db.strategy_instance_repo();
    match <StrategyInstanceRepository as Repository<StrategyInstance, String>>::find_by_id(&repo, id).await {
        Ok(instance) => Ok(ApiResponse::success(instance).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to get instance: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("查询实例失败")).with_request_id(request_id))
        }
    }
}

/// 创建新的策略实例
#[tauri::command]
pub async fn instance_create(
    db: State<'_, Database>,
    request: CreateInstanceRequest,
) -> Result<ApiResponse<StrategyInstance>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] instance_create called: name={}, strategy={}", request_id, request.name, request.strategy_id);

    let repo = db.strategy_instance_repo();

    // 检查名称是否已存在
    let name_exists = match repo.name_exists(&request.user_id, &request.name, None).await {
        Ok(exists) => exists,
        Err(e) => {
            log::error!("[{}] Failed to check name: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("检查名称失败")).with_request_id(request_id));
        }
    };

    if name_exists {
        return Ok(ApiResponse::error(ApiError::already_exists("实例名称")).with_request_id(request_id));
    }

    // 创建实例
    match repo.create(request).await {
        Ok(instance) => Ok(ApiResponse::success(instance).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to create instance: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("创建实例失败")).with_request_id(request_id))
        }
    }
}

/// 更新策略实例状态
#[tauri::command]
pub async fn instance_update_status(
    db: State<'_, Database>,
    id: String,
    status: String,
    error_message: Option<String>,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] instance_update_status called: id={}, status={}", request_id, id, status);

    let repo = db.strategy_instance_repo();
    match repo.update_status(&id, &status, error_message.as_deref()).await {
        Ok(()) => Ok(ApiResponse::success_empty().with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to update status: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("更新状态失败")).with_request_id(request_id))
        }
    }
}

/// 更新策略实例统计信息
#[tauri::command]
pub async fn instance_update_stats(
    db: State<'_, Database>,
    id: String,
    trades: i64,
    pnl: f64,
    drawdown: f64,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!(
        "[{}] instance_update_stats called: id={}, trades={}, pnl={}",
        request_id, id, trades, pnl
    );

    let repo = db.strategy_instance_repo();
    match repo.update_stats(&id, trades, pnl, drawdown).await {
        Ok(()) => Ok(ApiResponse::success_empty().with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to update stats: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("更新统计失败")).with_request_id(request_id))
        }
    }
}

/// 删除策略实例
#[tauri::command]
pub async fn instance_delete(
    db: State<'_, Database>,
    id: String,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] instance_delete called: id={}", request_id, id);

    let repo = db.strategy_instance_repo();
    match repo.delete(&id).await {
        Ok(()) => Ok(ApiResponse::success_empty().with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to delete instance: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("删除实例失败")).with_request_id(request_id))
        }
    }
}

/// 获取用户运行中的实例
#[tauri::command]
pub async fn instance_list_running(
    db: State<'_, Database>,
    user_id: String,
) -> Result<ApiResponse<Vec<StrategyInstance>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] instance_list_running called: user_id={}", request_id, user_id);

    let repo = db.strategy_instance_repo();
    match repo.find_running_by_user(&user_id).await {
        Ok(instances) => Ok(ApiResponse::success(instances).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to list running instances: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("查询运行中实例失败")).with_request_id(request_id))
        }
    }
}

/// 获取内存中所有运行中的策略实例
#[tauri::command]
pub async fn strategy_instance_list_all(
    db: State<'_, Database>,
) -> Result<ApiResponse<Vec<crate::core::strategy::engine::InstanceInfo>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] strategy_instance_list_all called", request_id);

    let engine = db.get_strategy_engine();
    let instances = engine.list_instances().await;
    Ok(ApiResponse::success(instances).with_request_id(request_id))
}

/// 获取内存中指定的运行中策略实例
#[tauri::command]
pub async fn strategy_instance_get(
    db: State<'_, Database>,
    id: String,
) -> Result<ApiResponse<Option<crate::core::strategy::engine::InstanceInfo>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] strategy_instance_get called: id={}", request_id, id);

    let engine = db.get_strategy_engine();
    let instance = engine.get_instance(&id).await;
    Ok(ApiResponse::success(instance).with_request_id(request_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: 添加集成测试
}
