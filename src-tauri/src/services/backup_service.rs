//! Backup service module
//!
//! This module provides automated database backup with compression,
//! integrity verification, and restore capabilities.

use anyhow::{anyhow, bail, Result};
use chrono::Utc;
use flate2::write::GzEncoder;
use flate2::Compression;
use log::{error, info, warn};
use sqlx::Row;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::sync::RwLock;

/// Backup service for managing database backups
pub struct BackupService {
    db_path: PathBuf,
    backup_dir: PathBuf,
    retention_days: u64,
    _auto_backup_handle: RwLock<Option<tokio::task::JoinHandle<()>>>,
}

impl BackupService {
    /// Create a new backup service
    pub fn new(db_path: PathBuf, backup_dir: PathBuf, retention_days: u64) -> Self {
        Self {
            db_path,
            backup_dir,
            retention_days,
            _auto_backup_handle: RwLock::new(None),
        }
    }

    /// Create a backup of the database
    pub async fn create_backup(&self) -> Result<PathBuf> {
        info!("Starting database backup");

        // Ensure backup directory exists
        fs::create_dir_all(&self.backup_dir).map_err(|e| {
            anyhow!("Failed to create backup directory {}: {}", self.backup_dir.display(), e)
        })?;

        // Generate timestamp for backup filename
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("ai-lot-backup-{}.db", timestamp);
        let backup_path = self.backup_dir.join(&backup_name);

        info!("Copying database to {}", backup_path.display());

        // Copy database file
        fs::copy(&self.db_path, &backup_path).map_err(|e| {
            anyhow!(
                "Failed to copy database from {} to {}: {}",
                self.db_path.display(),
                backup_path.display(),
                e
            )
        })?;

        info!("Database copied successfully");

        // Compress the backup
        self.compress_backup(&backup_path).await?;

        info!("Backup created successfully at {}", backup_path.display());

        Ok(backup_path)
    }

    /// Compress a backup file using gzip
    async fn compress_backup(&self, path: &Path) -> Result<()> {
        info!("Compressing backup: {}", path.display());

        // Open source file
        let source = File::open(path).map_err(|e| {
            anyhow!("Failed to open backup file for compression {}: {}", path.display(), e)
        })?;

        // Create compressed file path
        let compressed_path = path.with_extension("db.gz");

        // Create gzip encoder
        let encoder = GzEncoder::new(
            File::create(&compressed_path).map_err(|e| {
                anyhow!(
                    "Failed to create compressed file {}: {}",
                    compressed_path.display(),
                    e
                )
            })?,
            Compression::default(),
        );

        // Compress using flate2 (this is synchronous, so we need to wrap it)
        tokio::task::spawn_blocking(move || {
            let mut encoder = encoder;
            let source = source;

            // Copy data to encoder
            let mut reader = std::io::BufReader::new(source);
            std::io::copy(&mut reader, &mut encoder)?;

            encoder.finish()?;
            Ok::<(), anyhow::Error>(())
        })
        .await
        .map_err(|e| anyhow!("Compression task failed: {}", e))??;

        // Get file sizes for comparison
        let original_size = fs::metadata(path)?.len();
        let compressed_size = fs::metadata(&compressed_path)?.len();
        let compression_ratio = if original_size > 0 {
            (compressed_size as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };

        info!(
            "Backup compressed: {} -> {} ({:.1}% of original)",
            format_size(original_size),
            format_size(compressed_size),
            compression_ratio
        );

        // Remove uncompressed file
        fs::remove_file(path).map_err(|e| {
            anyhow!(
                "Failed to remove uncompressed backup file {}: {}",
                path.display(),
                e
            )
        })?;

        Ok(())
    }

    /// Restore a database from a backup
    pub async fn restore_backup(&self, backup_path: &Path) -> Result<()> {
        info!("Starting database restore from {}", backup_path.display());

        // Check if backup exists
        if !backup_path.exists() {
            bail!("Backup file not found: {}", backup_path.display());
        }

        // Determine if backup is compressed
        let is_compressed = backup_path.extension().is_some_and(|ext| ext == "gz");
        let uncompressed_path = if is_compressed {
            // Decompress to temporary file
            let temp_path = self.backup_dir.join(format!(
                "temp_restore_{}.db",
                Utc::now().timestamp_millis()
            ));
            self.decompress_backup(backup_path, &temp_path).await?;
            temp_path
        } else {
            backup_path.to_path_buf()
        };

        // Create backup of current database
        let current_backup = self.backup_dir.join(format!(
            "current_db_backup_{}.db",
            Utc::now().timestamp_millis()
        ));
        fs::copy(&self.db_path, &current_backup).map_err(|e| {
            anyhow!(
                "Failed to backup current database to {}: {}",
                current_backup.display(),
                e
            )
        })?;

        info!("Current database backed up to {}", current_backup.display());

        // Restore operation with rollback on failure
        let restore_result = try_restore(
            &uncompressed_path,
            &self.db_path,
            &current_backup,
        )
        .await;

        // Clean up temp file if it was created
        if is_compressed {
            let _ = fs::remove_file(&uncompressed_path);
        }

        match restore_result {
            Ok(_) => {
                info!("Database restored successfully from {}", backup_path.display());
                Ok(())
            }
            Err(e) => {
                error!("Restore failed: {}", e);

                // Attempt rollback
                if let Err(rollback_err) = try_rollback(&current_backup, &self.db_path).await {
                    error!("Rollback failed: {}", rollback_err);
                    bail!("Restore failed and rollback also failed: {}. Database may be in inconsistent state.", e);
                }

                warn!("Database rolled back to previous state");
                Err(e)
            }
        }
    }

    /// Decompress a gzip backup file
    async fn decompress_backup(&self, compressed_path: &Path, output_path: &Path) -> Result<()> {
        info!(
            "Decompressing backup: {} -> {}",
            compressed_path.display(),
            output_path.display()
        );

        tokio::task::spawn_blocking({
            let compressed_path = compressed_path.to_path_buf();
            let output_path = output_path.to_path_buf();
            move || {
                // Open compressed file
                let compressed_file = File::open(&compressed_path).map_err(|e| {
                    anyhow!(
                        "Failed to open compressed backup {}: {}",
                        compressed_path.display(),
                        e
                    )
                })?;

                // Create decoder
                let decoder = flate2::read::GzDecoder::new(compressed_file);

                // Create output file
                let mut output_file = File::create(&output_path).map_err(|e| {
                    anyhow!(
                        "Failed to create output file {}: {}",
                        output_path.display(),
                        e
                    )
                })?;

                // Copy decompressed data
                let mut reader = std::io::BufReader::new(decoder);
                std::io::copy(&mut reader, &mut output_file)?;

                info!("Backup decompressed successfully");
                Ok::<(), anyhow::Error>(())
            }
        })
        .await
        .map_err(|e| anyhow!("Decompression task failed: {}", e))??;

        Ok(())
    }

    /// Verify database integrity using PRAGMA integrity_check
    pub async fn verify_database(&self, db_path: &Path) -> Result<()> {
        info!("Verifying database integrity: {}", db_path.display());

        // Use sqlx to run integrity check
        let options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(db_path)
            .read_only(true);

        let pool = sqlx::sqlite::SqlitePool::connect_with(options).await?;

        let result: String = sqlx::query_scalar("PRAGMA integrity_check")
            .fetch_one(&pool)
            .await?;

        pool.close().await;

        if result != "ok" {
            bail!("Database integrity check failed: {}", result);
        }

        info!("Database integrity verified: ok");
        Ok(())
    }

    /// Clean up old backups based on retention policy
    pub async fn cleanup_old_backups(&self) -> Result<Vec<PathBuf>> {
        info!("Cleaning up backups older than {} days", self.retention_days);

        let mut removed_backups = Vec::new();
        let cutoff_time = Utc::now().timestamp() - (self.retention_days * 24 * 60 * 60) as i64;

        let entries = fs::read_dir(&self.backup_dir).map_err(|e| {
            anyhow!(
                "Failed to read backup directory {}: {}",
                self.backup_dir.display(),
                e
            )
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| anyhow!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            // Only process backup files
            if path.extension().is_some_and(|ext| ext == "gz" || ext == "db") {
                // Check if it's an ai-lot backup
                if path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("ai-lot-backup-"))
                {
                    // Get metadata
                    let metadata = entry.metadata().map_err(|e| {
                        anyhow!("Failed to get file metadata for {}: {}", path.display(), e)
                    })?;

                    // Check modified time
                    if let Ok(modified) = metadata.modified() {
                        let modified_timestamp = modified
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs() as i64;

                        if modified_timestamp < cutoff_time {
                            info!(
                                "Removing old backup: {} (created {} days ago)",
                                path.display(),
                                (cutoff_time - modified_timestamp) / (24 * 60 * 60)
                            );

                            fs::remove_file(&path).map_err(|e| {
                                anyhow!(
                                    "Failed to remove old backup {}: {}",
                                    path.display(),
                                    e
                                )
                            })?;

                            removed_backups.push(path);
                        }
                    }
                }
            }
        }

        info!(
            "Cleanup completed: {} old backup(s) removed",
            removed_backups.len()
        );

        Ok(removed_backups)
    }

    /// List all available backups
    pub async fn list_backups(&self) -> Result<Vec<BackupInfo>> {
        info!("Listing backups in {}", self.backup_dir.display());

        let mut backups = Vec::new();

        let entries = fs::read_dir(&self.backup_dir).map_err(|e| {
            anyhow!(
                "Failed to read backup directory {}: {}",
                self.backup_dir.display(),
                e
            )
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| anyhow!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            // Only process backup files
            if path.extension().is_some_and(|ext| ext == "gz" || ext == "db") {
                // Check if it's an ai-lot backup
                if path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("ai-lot-backup-"))
                {
                    let metadata = entry.metadata().map_err(|e| {
                        anyhow!("Failed to get file metadata for {}: {}", path.display(), e)
                    })?;

                    let compressed = path.extension().is_some_and(|ext| ext == "gz");

                    // Try to extract timestamp from filename
                    let created_at = if let Some(file_name) = path.file_stem().and_then(|n| n.to_str()) {
                        // Expected format: ai-lot-backup-YYYYMMDD_HHMMSS
                        if let Some(timestamp_str) = file_name.strip_prefix("ai-lot-backup-") {
                            parse_timestamp_from_filename(timestamp_str)
                                .unwrap_or_else(|| Utc::now().timestamp())
                        } else {
                            Utc::now().timestamp()
                        }
                    } else {
                        Utc::now().timestamp()
                    };

                    backups.push(BackupInfo {
                        path: path.to_string_lossy().to_string(),
                        size: metadata.len(),
                        created_at,
                        compressed,
                    });
                }
            }
        }

        // Sort by creation time (newest first)
        backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        info!("Found {} backup(s)", backups.len());

        Ok(backups)
    }

    /// Delete a specific backup
    pub async fn delete_backup(&self, backup_path: &Path) -> Result<()> {
        info!("Deleting backup: {}", backup_path.display());

        if !backup_path.exists() {
            bail!("Backup file not found: {}", backup_path.display());
        }

        fs::remove_file(backup_path).map_err(|e| {
            anyhow!(
                "Failed to delete backup file {}: {}",
                backup_path.display(),
                e
            )
        })?;

        info!("Backup deleted successfully");
        Ok(())
    }

    /// Start automatic backup task
    pub async fn start_auto_backup(&self, interval_hours: u64) -> Result<()> {
        let interval = Duration::from_secs(interval_hours * 3600);

        info!(
            "Starting auto-backup task with interval: {} hours",
            interval_hours
        );

        let db_path = self.db_path.clone();
        let backup_dir = self.backup_dir.clone();
        let retention_days = self.retention_days;

        let handle = tokio::spawn(async move {
            let mut timer = tokio::time::interval(interval);

            loop {
                timer.tick().await;

                info!("Auto-backup triggered");

                // Create backup
                let service = BackupService::new(db_path.clone(), backup_dir.clone(), retention_days);
                if let Err(e) = service.create_backup().await {
                    error!("Auto-backup failed: {}", e);
                }

                // Cleanup old backups
                if let Err(e) = service.cleanup_old_backups().await {
                    error!("Auto-cleanup failed: {}", e);
                }
            }
        });

        let mut guard = self._auto_backup_handle.write().await;

        // Abort existing task if any
        if let Some(old_handle) = guard.take() {
            old_handle.abort();
        }

        *guard = Some(handle);

        Ok(())
    }

    /// Stop automatic backup task
    pub async fn stop_auto_backup(&self) {
        let mut guard = self._auto_backup_handle.write().await;

        if let Some(handle) = guard.take() {
            handle.abort();
            info!("Auto-backup task stopped");
        }
    }
}

/// Backup information structure
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackupInfo {
    pub path: String,
    pub size: u64,
    pub created_at: i64,
    pub compressed: bool,
}

// ========== Helper functions ==========

/// Format file size for display
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Parse timestamp from filename format YYYYMMDD_HHMMSS
fn parse_timestamp_from_filename(s: &str) -> Option<i64> {
    use chrono::NaiveDate;

    // Parse YYYYMMDD_HHMMSS format
    if s.len() == 15 {
        let year = s[0..4].parse::<i32>().ok()?;
        let month = s[4..6].parse::<u32>().ok()?;
        let day = s[6..8].parse::<u32>().ok()?;
        let hour = s[9..11].parse::<u32>().ok()?;
        let minute = s[11..13].parse::<u32>().ok()?;
        let second = s[13..15].parse::<u32>().ok()?;

        let naive = NaiveDate::from_ymd_opt(year, month, day)?
            .and_hms_opt(hour, minute, second)?;

        Some(naive.timestamp())
    } else {
        None
    }
}

/// Try to restore database with verification
async fn try_restore(
    source_path: &Path,
    target_path: &Path,
    _rollback_path: &Path,
) -> Result<()> {
    // Copy backup to target
    fs::copy(source_path, target_path).map_err(|e| {
        anyhow!(
            "Failed to copy backup from {} to {}: {}",
            source_path.display(),
            target_path.display(),
            e
        )
    })?;

    // Verify integrity of restored database
    let service = BackupService::new(target_path.to_path_buf(), PathBuf::from("/tmp"), 30);
    service.verify_database(target_path).await?;

    Ok(())
}

/// Rollback to previous database version
async fn try_rollback(backup_path: &Path, target_path: &Path) -> Result<()> {
    fs::copy(backup_path, target_path).map_err(|e| {
        anyhow!(
            "Failed to rollback database from {} to {}: {}",
            backup_path.display(),
            target_path.display(),
            e
        )
    })?;

    // Clean up rollback backup
    fs::remove_file(backup_path).ok();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(2048), "2.00 KB");
        assert_eq!(format_size(2_097_152), "2.00 MB");
        assert_eq!(format_size(2_147_483_648), "2.00 GB");
    }

    #[test]
    fn test_parse_timestamp_from_filename() {
        let timestamp = parse_timestamp_from_filename("20231215_143045").unwrap();
        assert!(timestamp > 0);
    }
}
