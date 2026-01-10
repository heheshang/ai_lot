//! Backup commands module
//!
//! This module provides Tauri commands for database backup and restore operations.

use crate::core::response::{ApiResponse, ApiError};
use crate::services::backup_service::{BackupInfo, BackupService};

/// Create a database backup
#[tauri::command]
pub async fn backup_create(
    db_path: String,
    backup_dir: String,
    retention_days: Option<u64>,
) -> Result<ApiResponse<BackupInfo>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Backup create requested", request_id);

    let retention = retention_days.unwrap_or(30);
    let service = BackupService::new(
        std::path::PathBuf::from(db_path),
        std::path::PathBuf::from(backup_dir),
        retention,
    );

    let backup_path = match service.create_backup().await {
        Ok(path) => path,
        Err(e) => {
            log::error!("[{}] Failed to create backup: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed(format!("创建备份失败: {}", e))).with_request_id(request_id));
        }
    };

    // Get backup metadata
    let metadata = match std::fs::metadata(&backup_path) {
        Ok(m) => m,
        Err(e) => {
            log::error!("[{}] Failed to get backup metadata: {}", request_id, e);
            return Ok(ApiResponse::error(ApiError::operation_failed("获取备份元数据失败")).with_request_id(request_id));
        }
    };

    Ok(ApiResponse::success(BackupInfo {
        path: backup_path.to_string_lossy().to_string(),
        size: metadata.len(),
        created_at: chrono::Utc::now().timestamp(),
        compressed: true,
    }).with_request_id(request_id))
}

/// Restore a database from backup
#[tauri::command]
pub async fn backup_restore(
    db_path: String,
    backup_dir: String,
    backup_path: String,
    retention_days: Option<u64>,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Backup restore requested for: {}", request_id, backup_path);

    let retention = retention_days.unwrap_or(30);
    let service = BackupService::new(
        std::path::PathBuf::from(db_path),
        std::path::PathBuf::from(backup_dir),
        retention,
    );

    match service.restore_backup(std::path::Path::new(&backup_path)).await {
        Ok(()) => Ok(ApiResponse::success_empty().with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to restore backup: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed(format!("恢复备份失败: {}", e))).with_request_id(request_id))
        }
    }
}

/// List all available backups
#[tauri::command]
pub async fn backup_list(
    backup_dir: String,
) -> Result<ApiResponse<Vec<BackupInfo>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Backup list requested", request_id);

    // Create a dummy service for listing
    let service = BackupService::new(
        std::path::PathBuf::from("/tmp/dummy.db"),
        std::path::PathBuf::from(backup_dir),
        30,
    );

    match service.list_backups().await {
        Ok(backups) => Ok(ApiResponse::success(backups).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to list backups: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("列出备份失败")).with_request_id(request_id))
        }
    }
}

/// Delete a specific backup
#[tauri::command]
pub async fn backup_delete(
    backup_dir: String,
    backup_path: String,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Backup delete requested for: {}", request_id, backup_path);

    let service = BackupService::new(
        std::path::PathBuf::from("/tmp/dummy.db"),
        std::path::PathBuf::from(backup_dir),
        30,
    );

    match service.delete_backup(std::path::Path::new(&backup_path)).await {
        Ok(()) => Ok(ApiResponse::success_empty().with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to delete backup: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("删除备份失败")).with_request_id(request_id))
        }
    }
}

/// Manual cleanup of old backups
#[tauri::command]
pub async fn backup_cleanup(
    db_path: String,
    backup_dir: String,
    retention_days: Option<u64>,
) -> Result<ApiResponse<Vec<String>>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Backup cleanup requested", request_id);

    let retention = retention_days.unwrap_or(30);
    let service = BackupService::new(
        std::path::PathBuf::from(db_path),
        std::path::PathBuf::from(backup_dir),
        retention,
    );

    match service.cleanup_old_backups().await {
        Ok(removed) => Ok(ApiResponse::success(
            removed.iter().map(|p| p.to_string_lossy().to_string()).collect()
        ).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to cleanup backups: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("清理备份失败")).with_request_id(request_id))
        }
    }
}

/// Verify database integrity
#[tauri::command]
pub async fn backup_verify_integrity(
    db_path: String,
) -> Result<ApiResponse<bool>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Database integrity verification requested", request_id);

    let service = BackupService::new(
        std::path::PathBuf::from(&db_path),
        std::path::PathBuf::from("/tmp"),
        30,
    );

    match service.verify_database(std::path::Path::new(&db_path)).await {
        Ok(()) => Ok(ApiResponse::success(true).with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to verify database: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed(format!("验证数据库失败: {}", e))).with_request_id(request_id))
        }
    }
}

/// Start automatic backup
#[tauri::command]
pub async fn backup_start_auto(
    db_path: String,
    backup_dir: String,
    interval_hours: u64,
    retention_days: Option<u64>,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Start auto backup requested with interval: {} hours", request_id, interval_hours);

    let retention = retention_days.unwrap_or(30);
    let service = BackupService::new(
        std::path::PathBuf::from(db_path),
        std::path::PathBuf::from(backup_dir),
        retention,
    );

    match service.start_auto_backup(interval_hours).await {
        Ok(()) => Ok(ApiResponse::success_empty().with_request_id(request_id)),
        Err(e) => {
            log::error!("[{}] Failed to start auto backup: {}", request_id, e);
            Ok(ApiResponse::error(ApiError::operation_failed("启动自动备份失败")).with_request_id(request_id))
        }
    }
}

/// Stop automatic backup
#[tauri::command]
pub async fn backup_stop_auto(
    db_path: String,
    backup_dir: String,
    retention_days: Option<u64>,
) -> Result<ApiResponse<()>, String> {
    let request_id = uuid::Uuid::new_v4().to_string();
    log::info!("[{}] Stop auto backup requested", request_id);

    let retention = retention_days.unwrap_or(30);
    let service = BackupService::new(
        std::path::PathBuf::from(db_path),
        std::path::PathBuf::from(backup_dir),
        retention,
    );

    service.stop_auto_backup().await;
    Ok(ApiResponse::success_empty().with_request_id(request_id))
}
