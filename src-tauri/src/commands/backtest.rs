//! Backtest commands
//!
//! Tauri commands for backtesting functionality

use crate::core::response::{ApiResponse, ApiError};
use crate::services::BacktestService;
use crate::types::backtest::*;
use tauri::State;
use std::sync::Arc;

/// Create a new backtest job
#[tauri::command]
pub async fn backtest_create_job(
    backtest_service: State<'_, Arc<BacktestService>>,
    config: BacktestConfig,
) -> Result<ApiResponse<String>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    match backtest_service.create_job(config).await {
        Ok(job_id) => Ok(ApiResponse::success(job_id).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to create backtest job: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed(format!("创建回测任务失败: {}", e))).with_request_id(request_id))
        }
    }
}

/// Get a backtest job by ID
#[tauri::command]
pub async fn backtest_get_job(
    backtest_service: State<'_, Arc<BacktestService>>,
    job_id: String,
) -> Result<ApiResponse<Option<BacktestJob>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    Ok(ApiResponse::success(backtest_service.get_job(&job_id).await).with_request_id(request_id))
}

/// List all backtest jobs
#[tauri::command]
pub async fn backtest_list_jobs(
    backtest_service: State<'_, Arc<BacktestService>>,
) -> Result<ApiResponse<Vec<BacktestJob>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    Ok(ApiResponse::success(backtest_service.list_jobs().await).with_request_id(request_id))
}

/// Run a backtest job
#[tauri::command]
pub async fn backtest_run_job(
    backtest_service: State<'_, Arc<BacktestService>>,
    job_id: String,
) -> Result<ApiResponse<BacktestResult>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    match backtest_service.run_job(&job_id).await {
        Ok(result) => Ok(ApiResponse::success(result).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to run backtest job: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed(format!("运行回测任务失败: {}", e))).with_request_id(request_id))
        }
    }
}

/// Create and run a backtest in one command (convenience function)
#[tauri::command]
pub async fn backtest_run(
    backtest_service: State<'_, Arc<BacktestService>>,
    config: BacktestConfig,
) -> Result<ApiResponse<BacktestResult>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();

    // Create job
    let job_id = match backtest_service.create_job(config.clone()).await {
        Ok(id) => id,
        Err(e) => {
            log::error!("[{}] Failed to create backtest job: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed(format!("创建回测任务失败: {}", e))).with_request_id(request_id));
        }
    };

    // Run job
    match backtest_service.run_job(&job_id).await {
        Ok(result) => Ok(ApiResponse::success(result).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to run backtest job: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed(format!("运行回测任务失败: {}", e))).with_request_id(request_id))
        }
    }
}

/// Delete a backtest job
#[tauri::command]
pub async fn backtest_delete_job(
    _backtest_service: State<'_, Arc<BacktestService>>,
    _job_id: String,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    // TODO: Implement delete in BacktestService
    log::info!("[{}] Backtest job delete requested (not yet implemented)", request_id);
    Ok(ApiResponse::success_empty().with_request_id(request_id))
}

/// Get backtest result by job ID
#[tauri::command]
pub async fn backtest_get_result(
    backtest_service: State<'_, Arc<BacktestService>>,
    job_id: String,
) -> Result<ApiResponse<Option<BacktestResult>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    let job = backtest_service.get_job(&job_id).await;
    Ok(ApiResponse::success(job.and_then(|j| j.result)).with_request_id(request_id))
}
