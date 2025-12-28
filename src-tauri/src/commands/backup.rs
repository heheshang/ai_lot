//! Backup commands module
//!
//! This module provides Tauri commands for database backup and restore operations.

use crate::services::backup_service::{BackupInfo, BackupService};

/// Create a database backup
#[tauri::command]
pub async fn backup_create(
    db_path: String,
    backup_dir: String,
    retention_days: Option<u64>,
) -> Result<BackupInfo, String> {
    log::info!("Backup create requested");

    let retention = retention_days.unwrap_or(30);
    let service = BackupService::new(
        std::path::PathBuf::from(db_path),
        std::path::PathBuf::from(backup_dir),
        retention,
    );

    let backup_path = service.create_backup().await.map_err(|e| e.to_string())?;

    // Get backup metadata
    let metadata = std::fs::metadata(&backup_path).map_err(|e| e.to_string())?;

    Ok(BackupInfo {
        path: backup_path.to_string_lossy().to_string(),
        size: metadata.len(),
        created_at: chrono::Utc::now().timestamp(),
        compressed: true,
    })
}

/// Restore a database from backup
#[tauri::command]
pub async fn backup_restore(
    db_path: String,
    backup_dir: String,
    backup_path: String,
    retention_days: Option<u64>,
) -> Result<(), String> {
    log::info!("Backup restore requested for: {}", backup_path);

    let retention = retention_days.unwrap_or(30);
    let service = BackupService::new(
        std::path::PathBuf::from(db_path),
        std::path::PathBuf::from(backup_dir),
        retention,
    );

    service
        .restore_backup(std::path::Path::new(&backup_path))
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// List all available backups
#[tauri::command]
pub async fn backup_list(
    backup_dir: String,
) -> Result<Vec<BackupInfo>, String> {
    log::info!("Backup list requested");

    // Create a dummy service for listing
    let service = BackupService::new(
        std::path::PathBuf::from("/tmp/dummy.db"),
        std::path::PathBuf::from(backup_dir),
        30,
    );

    service
        .list_backups()
        .await
        .map_err(|e| e.to_string())
}

/// Delete a specific backup
#[tauri::command]
pub async fn backup_delete(
    backup_dir: String,
    backup_path: String,
) -> Result<(), String> {
    log::info!("Backup delete requested for: {}", backup_path);

    let service = BackupService::new(
        std::path::PathBuf::from("/tmp/dummy.db"),
        std::path::PathBuf::from(backup_dir),
        30,
    );

    service
        .delete_backup(std::path::Path::new(&backup_path))
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Manual cleanup of old backups
#[tauri::command]
pub async fn backup_cleanup(
    db_path: String,
    backup_dir: String,
    retention_days: Option<u64>,
) -> Result<Vec<String>, String> {
    log::info!("Backup cleanup requested");

    let retention = retention_days.unwrap_or(30);
    let service = BackupService::new(
        std::path::PathBuf::from(db_path),
        std::path::PathBuf::from(backup_dir),
        retention,
    );

    let removed = service
        .cleanup_old_backups()
        .await
        .map_err(|e| e.to_string())?;

    Ok(removed
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect())
}

/// Verify database integrity
#[tauri::command]
pub async fn backup_verify_integrity(
    db_path: String,
) -> Result<bool, String> {
    log::info!("Database integrity verification requested");

    let service = BackupService::new(
        std::path::PathBuf::from(&db_path),
        std::path::PathBuf::from("/tmp"),
        30,
    );

    service
        .verify_database(std::path::Path::new(&db_path))
        .await
        .map_err(|e| e.to_string())?;

    Ok(true)
}

/// Start automatic backup
#[tauri::command]
pub async fn backup_start_auto(
    db_path: String,
    backup_dir: String,
    interval_hours: u64,
    retention_days: Option<u64>,
) -> Result<(), String> {
    log::info!("Start auto backup requested with interval: {} hours", interval_hours);

    let retention = retention_days.unwrap_or(30);
    let service = BackupService::new(
        std::path::PathBuf::from(db_path),
        std::path::PathBuf::from(backup_dir),
        retention,
    );

    service
        .start_auto_backup(interval_hours)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Stop automatic backup
#[tauri::command]
pub async fn backup_stop_auto(
    db_path: String,
    backup_dir: String,
    retention_days: Option<u64>,
) -> Result<(), String> {
    log::info!("Stop auto backup requested");

    let retention = retention_days.unwrap_or(30);
    let service = BackupService::new(
        std::path::PathBuf::from(db_path),
        std::path::PathBuf::from(backup_dir),
        retention,
    );

    service.stop_auto_backup().await;
    Ok(())
}
