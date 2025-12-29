//! Backtest commands
//!
//! Tauri commands for backtesting functionality

use crate::services::BacktestService;
use crate::types::backtest::*;
use tauri::State;
use std::sync::Arc;

/// Create a new backtest job
#[tauri::command]
pub async fn backtest_create_job(
    backtest_service: State<'_, Arc<BacktestService>>,
    config: BacktestConfig,
) -> Result<String, String> {
    backtest_service
        .create_job(config)
        .await
        .map_err(|e| e.to_string())
}

/// Get a backtest job by ID
#[tauri::command]
pub async fn backtest_get_job(
    backtest_service: State<'_, Arc<BacktestService>>,
    job_id: String,
) -> Result<Option<BacktestJob>, String> {
    Ok(backtest_service.get_job(&job_id).await)
}

/// List all backtest jobs
#[tauri::command]
pub async fn backtest_list_jobs(
    backtest_service: State<'_, Arc<BacktestService>>,
) -> Result<Vec<BacktestJob>, String> {
    Ok(backtest_service.list_jobs().await)
}

/// Run a backtest job
#[tauri::command]
pub async fn backtest_run_job(
    backtest_service: State<'_, Arc<BacktestService>>,
    job_id: String,
) -> Result<BacktestResult, String> {
    backtest_service
        .run_job(&job_id)
        .await
        .map_err(|e| e.to_string())
}

/// Create and run a backtest in one command (convenience function)
#[tauri::command]
pub async fn backtest_run(
    backtest_service: State<'_, Arc<BacktestService>>,
    config: BacktestConfig,
) -> Result<BacktestResult, String> {
    // Create job
    let job_id = backtest_service
        .create_job(config.clone())
        .await
        .map_err(|e| e.to_string())?;

    // Run job
    let result = backtest_service
        .run_job(&job_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(result)
}

/// Delete a backtest job
#[tauri::command]
pub async fn backtest_delete_job(
    backtest_service: State<'_, Arc<BacktestService>>,
    job_id: String,
) -> Result<(), String> {
    // TODO: Implement delete in BacktestService
    Ok(())
}

/// Get backtest result by job ID
#[tauri::command]
pub async fn backtest_get_result(
    backtest_service: State<'_, Arc<BacktestService>>,
    job_id: String,
) -> Result<Option<BacktestResult>, String> {
    let job = backtest_service.get_job(&job_id).await;
    Ok(job.and_then(|j| j.result))
}
