<template>
  <div class="audit-logs">
    <div class="audit-logs__header">
      <h2>审计日志</h2>
      <div class="audit-logs__filters">
        <el-select
          v-model="filters.event_types"
          multiple
          placeholder="选择事件类型"
          clearable
          collapse-tags
          style="width: 280px"
        >
          <el-option
            v-for="type in eventTypes"
            :key="type.value"
            :label="type.label"
            :value="type.value"
          />
        </el-select>

        <el-select
          v-model="filters.user_id"
          placeholder="选择用户"
          clearable
          filterable
          style="width: 200px"
        >
          <el-option
            v-for="user in users"
            :key="user.id"
            :label="user.username"
            :value="user.id"
          />
        </el-select>

        <el-input-number
          v-model="filters.limit"
          :min="10"
          :max="1000"
          :step="10"
          placeholder="数量限制"
          style="width: 150px"
        />

        <el-button type="primary" @click="handleSearch">
          <el-icon><Search /></el-icon>
          查询
        </el-button>

        <el-button @click="handleExport">
          <el-icon><Download /></el-icon>
          导出
        </el-button>

        <el-button @click="handleReset">
          <el-icon><RefreshLeft /></el-icon>
          重置
        </el-button>
      </div>
    </div>

    <div class="audit-logs__content">
      <el-table
        :data="logs"
        v-loading="loading"
        stripe
        border
        style="width: 100%"
        :default-sort="{ prop: 'timestamp', order: 'descending' }"
      >
        <el-table-column prop="timestamp" label="时间" width="180" sortable>
          <template #default="{ row }">
            {{ formatTimestamp(row.timestamp) }}
          </template>
        </el-table-column>

        <el-table-column prop="event_type" label="事件类型" width="150">
          <template #default="{ row }">
            <el-tag :type="getEventTypeColor(row.event_type)">
              {{ getEventTypeLabel(row.event_type) }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="user_id" label="用户" width="120">
          <template #default="{ row }">
            {{ row.user_id || '-' }}
          </template>
        </el-table-column>

        <el-table-column prop="event_data" label="事件详情" min-width="300">
          <template #default="{ row }">
            <div class="event-data">
              {{ formatEventData(row.event_data) }}
            </div>
          </template>
        </el-table-column>

        <el-table-column label="操作" width="100" fixed="right">
          <template #default="{ row }">
            <el-button
              type="primary"
              link
              size="small"
              @click="handleViewDetail(row)"
            >
              详情
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <div class="audit-logs__empty" v-if="!loading && logs.length === 0">
        <el-empty description="暂无审计日志" />
      </div>
    </div>

    <el-dialog
      v-model="detailDialogVisible"
      title="审计日志详情"
      width="600px"
    >
      <div v-if="selectedLog" class="audit-log-detail">
        <div class="detail-item">
          <label>时间：</label>
          <span>{{ formatTimestamp(selectedLog.timestamp) }}</span>
        </div>
        <div class="detail-item">
          <label>事件类型：</label>
          <el-tag :type="getEventTypeColor(selectedLog.event_type)">
            {{ getEventTypeLabel(selectedLog.event_type) }}
          </el-tag>
        </div>
        <div class="detail-item">
          <label>用户 ID：</label>
          <span>{{ selectedLog.user_id || '-' }}</span>
        </div>
        <div class="detail-item">
          <label>事件数据：</label>
          <pre class="event-data-raw">{{ formatEventDataRaw(selectedLog.event_data) }}</pre>
        </div>
      </div>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { Search, Download, RefreshLeft } from '@element-plus/icons-vue';
import { auditApi } from '@/api/tauri';
import type { AuditLog, AuditFilter } from '@/types';
import { useUserStore } from '@/store';

const userStore = useUserStore();

const loading = ref(false);
const logs = ref<AuditLog[]>([]);
const users = ref<{ id: string; username: string }[]>([]);
const detailDialogVisible = ref(false);
const selectedLog = ref<AuditLog | null>(null);

const filters = reactive<AuditFilter>({
  event_types: [],
  user_id: undefined,
  limit: 100,
});

const eventTypes = [
  { label: '用户登录', value: 'UserLogin' },
  { label: '用户登出', value: 'UserLogout' },
  { label: '创建策略', value: 'StrategyCreated' },
  { label: '更新策略', value: 'StrategyUpdated' },
  { label: '删除策略', value: 'StrategyDeleted' },
  { label: '下单', value: 'OrderPlaced' },
  { label: '风险预警', value: 'RiskAlertTriggered' },
  { label: '系统启动', value: 'SystemStarted' },
];

const eventTypeColors: Record<string, any> = {
  UserLogin: 'success',
  UserLogout: 'info',
  StrategyCreated: 'success',
  StrategyUpdated: 'warning',
  StrategyDeleted: 'danger',
  OrderPlaced: 'primary',
  RiskAlertTriggered: 'danger',
  SystemStarted: 'info',
};

const eventTypeLabels: Record<string, string> = {
  UserLogin: '用户登录',
  UserLogout: '用户登出',
  StrategyCreated: '创建策略',
  StrategyUpdated: '更新策略',
  StrategyDeleted: '删除策略',
  OrderPlaced: '下单',
  RiskAlertTriggered: '风险预警',
  SystemStarted: '系统启动',
};

async function loadLogs() {
  loading.value = true;
  try {
    const filter: AuditFilter = {
      event_types: filters.event_types?.length ? filters.event_types : undefined,
      user_id: filters.user_id,
      limit: filters.limit || 100,
    };
    logs.value = await auditApi.getLogs(filter);
  } catch (error) {
    console.error('Failed to load audit logs:', error);
    ElMessage.error('加载审计日志失败');
  } finally {
    loading.value = false;
  }
}

function handleSearch() {
  loadLogs();
}

function handleReset() {
  filters.event_types = [];
  filters.user_id = undefined;
  filters.limit = 100;
  loadLogs();
}

async function handleExport() {
  try {
    await ElMessageBox.confirm('确定要导出审计日志吗？', '确认', {
      type: 'warning',
    });

    const filter: AuditFilter = {
      event_types: filters.event_types?.length ? filters.event_types : undefined,
      user_id: filters.user_id,
      limit: filters.limit || 100,
    };

    const csvPath = await auditApi.exportToCsv(filter);
    ElMessage.success(`审计日志已导出到: ${csvPath}`);
  } catch (error) {
    if (error !== 'cancel') {
      console.error('Failed to export audit logs:', error);
      ElMessage.error('导出审计日志失败');
    }
  }
}

function handleViewDetail(log: AuditLog) {
  selectedLog.value = log;
  detailDialogVisible.value = true;
}

function formatTimestamp(timestamp: number): string {
  return new Date(timestamp * 1000).toLocaleString('zh-CN');
}

function getEventTypeColor(type: string): string {
  return eventTypeColors[type] || 'info';
}

function getEventTypeLabel(type: string): string {
  return eventTypeLabels[type] || type;
}

function formatEventData(data: string): string {
  try {
    const parsed = JSON.parse(data);
    const parts: string[] = [];

    if (parsed.username) parts.push(`用户: ${parsed.username}`);
    if (parsed.strategy_name) parts.push(`策略: ${parsed.strategy_name}`);
    if (parsed.symbol) parts.push(`交易对: ${parsed.symbol}`);
    if (parsed.alert_type) parts.push(`类型: ${parsed.alert_type}`);
    if (parsed.severity) parts.push(`级别: ${parsed.severity}`);
    if (parsed.message) parts.push(`消息: ${parsed.message}`);

    return parts.length > 0 ? parts.join(' | ') : data;
  } catch {
    return data;
  }
}

function formatEventDataRaw(data: string): string {
  try {
    return JSON.stringify(JSON.parse(data), null, 2);
  } catch {
    return data;
  }
}

onMounted(() => {
  if (userStore.user) {
    users.value = [{ id: userStore.user.id, username: userStore.user.username }];
  }
  loadLogs();
});
</script>

<style scoped lang="scss">
.audit-logs {
  padding: 20px;
  height: 100%;
  display: flex;
  flex-direction: column;

  &__header {
    margin-bottom: 20px;

    h2 {
      margin: 0 0 16px 0;
      font-size: 24px;
      font-weight: 600;
    }
  }

  &__filters {
    display: flex;
    gap: 12px;
    flex-wrap: wrap;
    align-items: center;
  }

  &__content {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  &__empty {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }
}

.event-data {
  font-size: 13px;
  color: #606266;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.audit-log-detail {
  .detail-item {
    margin-bottom: 16px;

    label {
      font-weight: 600;
      margin-right: 8px;
      color: #303133;
    }

    span {
      color: #606266;
    }
  }

  .event-data-raw {
    background: #f5f7fa;
    padding: 12px;
    border-radius: 4px;
    font-size: 13px;
    line-height: 1.6;
    max-height: 300px;
    overflow: auto;
  }
}
</style>