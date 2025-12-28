<template>
  <div class="backup-view">
    <el-card class="backup-header">
      <template #header>
        <div class="header-content">
          <span>Database Backup & Restore</span>
          <div class="header-actions">
            <el-button
              type="primary"
              :icon="Plus"
              :loading="creating"
              @click="handleCreateBackup"
            >
              Create Backup
            </el-button>
            <el-button
              :icon="Delete"
              :loading="cleaning"
              @click="handleCleanup"
            >
              Cleanup Old
            </el-button>
          </div>
        </div>
      </template>
      <div class="backup-stats">
        <div class="stat-item">
          <span class="stat-label">Total Backups:</span>
          <span class="stat-value">{{ backups.length }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">Total Size:</span>
          <span class="stat-value">{{ formatSize(totalSize) }}</span>
        </div>
        <div class="stat-item">
          <span class="stat-label">Retention Days:</span>
          <span class="stat-value">{{ retentionDays }}</span>
        </div>
      </div>
    </el-card>

    <el-card class="backup-list">
      <template #header>
        <span>Backup History</span>
      </template>

      <el-table
        v-loading="loading"
        :data="backups"
        stripe
        style="width: 100%"
      >
        <el-table-column prop="path" label="File Name" min-width="200">
          <template #default="{ row }">
            <el-icon><Document /></el-icon>
            {{ getFileName(row.path) }}
          </template>
        </el-table-column>

        <el-table-column prop="size" label="Size" width="120">
          <template #default="{ row }">
            {{ formatSize(row.size) }}
          </template>
        </el-table-column>

        <el-table-column prop="compressed" label="Type" width="100">
          <template #default="{ row }">
            <el-tag :type="row.compressed ? 'success' : 'info'" size="small">
              {{ row.compressed ? 'Compressed' : 'Raw' }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="createdAt" label="Created" width="180">
          <template #default="{ row }">
            {{ formatDate(row.createdAt) }}
          </template>
        </el-table-column>

        <el-table-column label="Actions" width="200" fixed="right">
          <template #default="{ row }">
            <el-button
              type="primary"
              size="small"
              :icon="RefreshLeft"
              @click="handleRestore(row)"
            >
              Restore
            </el-button>
            <el-button
              type="danger"
              size="small"
              :icon="Delete"
              @click="handleDelete(row)"
            >
              Delete
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <el-empty
        v-if="!loading && backups.length === 0"
        description="No backups found"
      />
    </el-card>

    <!-- Restore Confirmation Dialog -->
    <el-dialog
      v-model="restoreDialogVisible"
      title="Restore Backup"
      width="500px"
    >
      <el-alert
        type="warning"
        :closable="false"
        show-icon
      >
        <template #title>
          Warning: Database Restore
        </template>
        <p>This action will replace your current database with the selected backup.</p>
        <p>All changes made after this backup will be lost.</p>
      </el-alert>

      <div v-if="selectedBackup" class="restore-info">
        <p><strong>Backup:</strong> {{ getFileName(selectedBackup.path) }}</p>
        <p><strong>Created:</strong> {{ formatDate(selectedBackup.createdAt) }}</p>
        <p><strong>Size:</strong> {{ formatSize(selectedBackup.size) }}</p>
      </div>

      <template #footer>
        <el-button @click="restoreDialogVisible = false">Cancel</el-button>
        <el-button
          type="primary"
          :loading="restoring"
          @click="confirmRestore"
        >
          Confirm Restore
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import {
  Plus,
  Delete,
  RefreshLeft,
  Document,
} from '@element-plus/icons-vue';
import { invoke } from '@tauri-apps/api/core';
import type { BackupInfo } from '@/types/backup';

// State
const backups = ref<BackupInfo[]>([]);
const loading = ref(false);
const creating = ref(false);
const restoring = ref(false);
const cleaning = ref(false);
const restoreDialogVisible = ref(false);
const selectedBackup = ref<BackupInfo | null>(null);
const retentionDays = ref(30);

// Computed
const totalSize = computed(() => {
  return backups.value.reduce((sum, b) => sum + b.size, 0);
});

// Methods
const loadBackups = async () => {
  loading.value = true;
  try {
    const result = await invoke<BackupInfo[]>('backup_list', {
      backupDir: getBackupDir(),
    });
    backups.value = result;
  } catch (error) {
    ElMessage.error(`Failed to load backups: ${error}`);
  } finally {
    loading.value = false;
  }
};

const handleCreateBackup = async () => {
  creating.value = true;
  try {
    const result = await invoke<BackupInfo>('backup_create', {
      dbPath: getDbPath(),
      backupDir: getBackupDir(),
      retentionDays: retentionDays.value,
    });
    ElMessage.success(`Backup created: ${getFileName(result.path)}`);
    await loadBackups();
  } catch (error) {
    ElMessage.error(`Failed to create backup: ${error}`);
  } finally {
    creating.value = false;
  }
};

const handleRestore = (backup: BackupInfo) => {
  selectedBackup.value = backup;
  restoreDialogVisible.value = true;
};

const confirmRestore = async () => {
  if (!selectedBackup.value) return;

  restoring.value = true;
  try {
    await invoke('backup_restore', {
      dbPath: getDbPath(),
      backupDir: getBackupDir(),
      backupPath: selectedBackup.value.path,
      retentionDays: retentionDays.value,
    });
    ElMessage.success('Database restored successfully');
    restoreDialogVisible.value = false;
    // Optionally reload the app or notify user to restart
  } catch (error) {
    ElMessage.error(`Failed to restore backup: ${error}`);
  } finally {
    restoring.value = false;
  }
};

const handleDelete = async (backup: BackupInfo) => {
  try {
    await ElMessageBox.confirm(
      `Are you sure you want to delete backup ${getFileName(backup.path)}?`,
      'Delete Backup',
      {
        confirmButtonText: 'Delete',
        cancelButtonText: 'Cancel',
        type: 'warning',
      }
    );

    await invoke('backup_delete', {
      backupDir: getBackupDir(),
      backupPath: backup.path,
    });
    ElMessage.success('Backup deleted');
    await loadBackups();
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error(`Failed to delete backup: ${error}`);
    }
  }
};

const handleCleanup = async () => {
  cleaning.value = true;
  try {
    const removed = await invoke<string[]>('backup_cleanup', {
      dbPath: getDbPath(),
      backupDir: getBackupDir(),
      retentionDays: retentionDays.value,
    });
    if (removed.length > 0) {
      ElMessage.success(`Cleaned up ${removed.length} old backup(s)`);
    } else {
      ElMessage.info('No old backups to clean up');
    }
    await loadBackups();
  } catch (error) {
    ElMessage.error(`Failed to cleanup backups: ${error}`);
  } finally {
    cleaning.value = false;
  }
};

// Utility functions
const getBackupDir = (): string => {
  // This should be configurable or obtained from Tauri
  return '/backup'; // Placeholder - actual path would come from config
};

const getDbPath = (): string => {
  // This should be obtained from Tauri's app data dir
  return '/data/ai-lot.db'; // Placeholder
};

const getFileName = (path: string): string => {
  return path.split('/').pop() || path;
};

const formatSize = (bytes: number): string => {
  const KB = 1024;
  const MB = KB * 1024;
  const GB = MB * 1024;

  if (bytes >= GB) {
    return `${(bytes / GB).toFixed(2)} GB`;
  } else if (bytes >= MB) {
    return `${(bytes / MB).toFixed(2)} MB`;
  } else if (bytes >= KB) {
    return `${(bytes / KB).toFixed(2)} KB`;
  } else {
    return `${bytes} B`;
  }
};

const formatDate = (timestamp: number): string => {
  return new Date(timestamp * 1000).toLocaleString();
};

// Lifecycle
onMounted(() => {
  loadBackups();
});
</script>

<style scoped>
.backup-view {
  display: flex;
  flex-direction: column;
  gap: 16px;
  height: 100%;
}

.backup-header {
  flex-shrink: 0;
}

.header-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-actions {
  display: flex;
  gap: 8px;
}

.backup-stats {
  display: flex;
  gap: 32px;
}

.stat-item {
  display: flex;
  gap: 8px;
}

.stat-label {
  color: var(--el-text-color-secondary);
}

.stat-value {
  font-weight: 600;
}

.backup-list {
  flex: 1;
  overflow: hidden;
}

.backup-list :deep(.el-card__body) {
  height: calc(100% - 60px);
  overflow-y: auto;
}

.restore-info {
  margin-top: 16px;
  padding: 12px;
  background-color: var(--el-fill-color-light);
  border-radius: 4px;
}

.restore-info p {
  margin: 4px 0;
}
</style>
