<template>
  <div class="alert-history">
    <!-- Page Header -->
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">
          <el-icon><Warning /></el-icon>
          预警历史
        </h2>
        <el-breadcrumb separator="/">
          <el-breadcrumb-item :to="{ path: '/dashboard' }">首页</el-breadcrumb-item>
          <el-breadcrumb-item :to="{ path: '/risk' }">风险监控</el-breadcrumb-item>
          <el-breadcrumb-item>预警历史</el-breadcrumb-item>
        </el-breadcrumb>
      </div>
      <div class="header-actions">
        <el-button :icon="Download" @click="exportToCSV" :disabled="alerts.length === 0">
          导出 CSV
        </el-button>
        <el-button type="primary" :icon="Refresh" @click="loadAlerts" :loading="loading">
          刷新
        </el-button>
      </div>
    </div>

    <!-- Filters Section -->
    <el-card class="filter-card" shadow="never">
      <el-form :model="filter" inline>
        <el-form-item label="日期范围">
          <el-date-picker
            v-model="dateRange"
            type="daterange"
            range-separator="至"
            start-placeholder="开始日期"
            end-placeholder="结束日期"
            value-format="X"
            :clearable="true"
            @change="handleDateChange"
          />
        </el-form-item>

        <el-form-item label="严重程度">
          <el-select v-model="filter.severity" placeholder="全部" clearable @change="handleFilterChange">
            <el-option label="全部" value="" />
            <el-option label="低" value="low" />
            <el-option label="中" value="medium" />
            <el-option label="高" value="high" />
            <el-option label="严重" value="critical" />
          </el-select>
        </el-form-item>

        <el-form-item label="状态">
          <el-select v-model="filter.status" placeholder="全部" clearable @change="handleFilterChange">
            <el-option label="全部" value="" />
            <el-option label="活跃" value="active" />
            <el-option label="已处理" value="handled" />
            <el-option label="已忽略" value="ignored" />
          </el-select>
        </el-form-item>

        <el-form-item label="规则类型">
          <el-select v-model="filter.rule_name" placeholder="全部" clearable @change="handleFilterChange">
            <el-option label="全部" value="" />
            <el-option label="仓位限制" value="PositionLimit" />
            <el-option label="回撤限制" value="DrawdownLimit" />
            <el-option label="亏损限制" value="LossLimit" />
          </el-select>
        </el-form-item>

        <el-form-item label="搜索">
          <el-input
            v-model="filter.search_text"
            placeholder="搜索预警信息"
            clearable
            @clear="handleFilterChange"
            @keyup.enter="handleFilterChange"
          >
            <template #append>
              <el-button :icon="Search" @click="handleFilterChange" />
            </template>
          </el-input>
        </el-form-item>

        <el-form-item>
          <el-button type="primary" @click="handleFilterChange">查询</el-button>
          <el-button @click="resetFilters">重置</el-button>
        </el-form-item>
      </el-form>
    </el-card>

    <!-- Bulk Actions -->
    <div class="bulk-actions" v-if="selectedAlerts.length > 0">
      <el-alert
        :title="`已选择 ${selectedAlerts.length} 条预警`"
        type="info"
        :closable="false"
      >
        <template #default>
          <el-button
            type="success"
            size="small"
            @click="bulkMarkHandled"
            :loading="bulkActionLoading"
          >
            批量标记已处理
          </el-button>
          <el-button
            type="danger"
            size="small"
            @click="bulkDelete"
            :loading="bulkActionLoading"
          >
            批量删除
          </el-button>
          <el-button size="small" @click="clearSelection">取消选择</el-button>
        </template>
      </el-alert>
    </div>

    <!-- Alert Table -->
    <el-card class="table-card" shadow="never">
      <el-table
        ref="tableRef"
        :data="alerts"
        v-loading="loading"
        stripe
        @row-click="showAlertDetail"
        @selection-change="handleSelectionChange"
        :default-sort="{ prop: 'created_at', order: 'descending' }"
      >
        <el-table-column type="selection" width="55" />

        <el-table-column prop="id" label="ID" width="180">
          <template #default="{ row }">
            <span class="alert-id">{{ row.id.substring(0, 8) }}...</span>
          </template>
        </el-table-column>

        <el-table-column prop="rule_name" label="规则名称" width="150" />

        <el-table-column prop="severity" label="严重程度" width="100" align="center">
          <template #default="{ row }">
            <el-tag :type="getSeverityTagType(row.severity)" size="small">
              {{ getSeverityLabel(row.severity) }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="status" label="状态" width="100" align="center">
          <template #default="{ row }">
            <el-tag :type="getStatusTagType(row.status)" size="small">
              {{ getStatusLabel(row.status) }}
            </el-tag>
          </template>
        </el-table-column>

        <el-table-column prop="message" label="预警信息" min-width="250" show-overflow-tooltip />

        <el-table-column prop="symbol" label="交易对" width="120" align="center">
          <template #default="{ row }">
            <span v-if="row.symbol">{{ row.symbol }}</span>
            <span v-else class="text-muted">-</span>
          </template>
        </el-table-column>

        <el-table-column prop="created_at" label="创建时间" width="180" sortable>
          <template #default="{ row }">
            <span :title="formatFullTime(row.created_at)">
              {{ formatRelativeTime(row.created_at) }}
            </span>
          </template>
        </el-table-column>

        <el-table-column label="操作" width="120" align="center" fixed="right">
          <template #default="{ row }">
            <el-button
              v-if="row.status === 'active'"
              type="success"
              size="small"
              link
              @click.stop="markAsHandled(row)"
            >
              标记已处理
            </el-button>
            <el-button
              type="danger"
              size="small"
              link
              @click.stop="deleteAlert(row)"
            >
              删除
            </el-button>
          </template>
        </el-table-column>
      </el-table>

      <!-- Pagination -->
      <div class="pagination-container">
        <el-pagination
          v-model:current-page="filter.page"
          v-model:page-size="filter.page_size"
          :page-sizes="[10, 20, 50, 100]"
          :total="total"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handlePageChange"
        />
      </div>
    </el-card>

    <!-- Alert Detail Dialog -->
    <el-dialog
      v-model="detailDialogVisible"
      title="预警详情"
      width="600px"
      @close="closeDetailDialog"
    >
      <div v-if="currentAlert" class="alert-detail">
        <div class="detail-row">
          <span class="detail-label">规则名称</span>
          <span class="detail-value">{{ currentAlert.rule_name }}</span>
        </div>

        <div class="detail-row">
          <span class="detail-label">严重程度</span>
          <el-tag :type="getSeverityTagType(currentAlert.severity)" size="small">
            {{ getSeverityLabel(currentAlert.severity) }}
          </el-tag>
        </div>

        <div class="detail-row">
          <span class="detail-label">状态</span>
          <el-tag :type="getStatusTagType(currentAlert.status)" size="small">
            {{ getStatusLabel(currentAlert.status) }}
          </el-tag>
        </div>

        <div class="detail-row">
          <span class="detail-label">预警信息</span>
          <span class="detail-value">{{ currentAlert.message }}</span>
        </div>

        <div class="detail-row" v-if="currentAlert.symbol">
          <span class="detail-label">交易对</span>
          <span class="detail-value">{{ currentAlert.symbol }}</span>
        </div>

        <div class="detail-row">
          <span class="detail-label">当前值</span>
          <span class="detail-value">{{ currentAlert.current_value?.toFixed(2) || '-' }}</span>
        </div>

        <div class="detail-row">
          <span class="detail-label">阈值</span>
          <span class="detail-value">{{ currentAlert.threshold_value?.toFixed(2) || '-' }}</span>
        </div>

        <div class="detail-row">
          <span class="detail-label">创建时间</span>
          <span class="detail-value">{{ formatFullTime(currentAlert.created_at) }}</span>
        </div>

        <div class="detail-row" v-if="currentAlert.handled_at">
          <span class="detail-label">处理时间</span>
          <span class="detail-value">{{ formatFullTime(currentAlert.handled_at) }}</span>
        </div>

        <div class="detail-row" v-if="currentAlert.handled_by">
          <span class="detail-label">处理人</span>
          <span class="detail-value">{{ currentAlert.handled_by }}</span>
        </div>

        <div class="detail-section" v-if="currentAlert.status === 'handled' && !currentAlert.handling_note">
          <el-divider />
          <div class="add-note-section">
            <h4>添加处理备注</h4>
            <el-input
              v-model="handlingNote"
              type="textarea"
              :rows="3"
              placeholder="请输入处理备注..."
            />
            <div class="note-actions">
              <el-button type="primary" @click="addHandlingNote" :loading="noteLoading">
                保存备注
              </el-button>
            </div>
          </div>
        </div>

        <div class="detail-section" v-if="currentAlert.handling_note">
          <el-divider />
          <div class="handling-note">
            <h4>处理备注</h4>
            <p>{{ currentAlert.handling_note }}</p>
          </div>
        </div>
      </div>

      <template #footer>
        <el-button @click="closeDetailDialog">关闭</el-button>
        <el-button
          v-if="currentAlert && currentAlert.status === 'active'"
          type="success"
          @click="markAsHandled(currentAlert)"
          :loading="actionLoading"
        >
          标记已处理
        </el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import {
  Warning,
  Download,
  Refresh,
  Search,
} from '@element-plus/icons-vue';
import * as api from '@/api/tauri';
import type { AlertFilter, RiskAlertListItem, RiskAlertHistory } from '@/types';

// State
const loading = ref(false);
const bulkActionLoading = ref(false);
const actionLoading = ref(false);
const noteLoading = ref(false);
const alerts = ref<RiskAlertListItem[]>([]);
const total = ref(0);
const selectedAlerts = ref<RiskAlertListItem[]>([]);
const dateRange = ref<[number, number] | null>(null);

// Detail dialog
const detailDialogVisible = ref(false);
const currentAlert = ref<RiskAlertHistory | null>(null);
const handlingNote = ref('');

// Filter
const filter = reactive<AlertFilter>({
  start_date: undefined,
  end_date: undefined,
  severity: '',
  status: '',
  rule_name: '',
  search_text: '',
  page: 1,
  page_size: 20,
});

// Table ref
const tableRef = ref();

// Methods
function getSeverityTagType(severity: string): string {
  switch (severity) {
    case 'low': return 'info';
    case 'medium': return 'warning';
    case 'high': return 'danger';
    case 'critical': return 'danger';
    default: return '';
  }
}

function getSeverityLabel(severity: string): string {
  switch (severity) {
    case 'low': return '低';
    case 'medium': return '中';
    case 'high': return '高';
    case 'critical': return '严重';
    default: return severity;
  }
}

function getStatusTagType(status: string): string {
  switch (status) {
    case 'active': return 'danger';
    case 'handled': return 'success';
    case 'ignored': return 'info';
    default: return '';
  }
}

function getStatusLabel(status: string): string {
  switch (status) {
    case 'active': return '活跃';
    case 'handled': return '已处理';
    case 'ignored': return '已忽略';
    default: return status;
  }
}

function formatRelativeTime(timestamp: number): string {
  const now = Math.floor(Date.now() / 1000);
  const diff = now - timestamp;

  if (diff < 60) return `${diff}秒前`;
  if (diff < 3600) return `${Math.floor(diff / 60)}分钟前`;
  if (diff < 86400) return `${Math.floor(diff / 3600)}小时前`;
  if (diff < 604800) return `${Math.floor(diff / 86400)}天前`;

  const date = new Date(timestamp * 1000);
  return date.toLocaleDateString('zh-CN');
}

function formatFullTime(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
    second: '2-digit',
  });
}

function handleDateChange(values: [number, number] | null) {
  if (values) {
    filter.start_date = values[0];
    filter.end_date = values[1];
  } else {
    filter.start_date = undefined;
    filter.end_date = undefined;
  }
  handleFilterChange();
}

function handleFilterChange() {
  filter.page = 1;
  loadAlerts();
}

function resetFilters() {
  filter.start_date = undefined;
  filter.end_date = undefined;
  filter.severity = '';
  filter.status = '';
  filter.rule_name = '';
  filter.search_text = '';
  filter.page = 1;
  dateRange.value = null;
  loadAlerts();
}

function handlePageChange(page: number) {
  filter.page = page;
  loadAlerts();
}

function handleSizeChange(size: number) {
  filter.page_size = size;
  filter.page = 1;
  loadAlerts();
}

async function loadAlerts() {
  loading.value = true;
  try {
    const result = await api.invoke<{ items: RiskAlertListItem[]; total: number }>(
      'get_alert_history',
      { filter }
    );
    alerts.value = result.items || [];
    total.value = result.total || 0;
  } catch (error) {
    console.error('Failed to load alerts:', error);
    ElMessage.error('加载预警历史失败：' + (error as Error).message);
    alerts.value = [];
    total.value = 0;
  } finally {
    loading.value = false;
  }
}

function handleSelectionChange(selection: RiskAlertListItem[]) {
  selectedAlerts.value = selection;
}

function clearSelection() {
  tableRef.value?.clearSelection();
}

async function showAlertDetail(row: RiskAlertListItem) {
  try {
    const alert = await api.invoke<RiskAlertHistory>('get_alert_detail', { id: row.id });
    currentAlert.value = alert;
    handlingNote.value = alert.handling_note || '';
    detailDialogVisible.value = true;
  } catch (error) {
    console.error('Failed to load alert detail:', error);
    ElMessage.error('加载预警详情失败：' + (error as Error).message);
  }
}

function closeDetailDialog() {
  detailDialogVisible.value = false;
  currentAlert.value = null;
  handlingNote.value = '';
}

async function markAsHandled(alert: RiskAlertListItem | RiskAlertHistory) {
  try {
    await ElMessageBox.confirm('确定要将此预警标记为已处理吗？', '确认操作', {
      type: 'warning',
    });

    actionLoading.value = true;
    await api.invoke('add_alert_note', {
      id: alert.id,
      note: handlingNote.value || '已处理',
    });

    ElMessage.success('预警已标记为已处理');
    detailDialogVisible.value = false;
    loadAlerts();
  } catch (error) {
    if (error !== 'cancel') {
      console.error('Failed to mark alert as handled:', error);
      ElMessage.error('操作失败：' + (error as Error).message);
    }
  } finally {
    actionLoading.value = false;
  }
}

async function addHandlingNote() {
  if (!currentAlert.value || !handlingNote.value.trim()) {
    ElMessage.warning('请输入处理备注');
    return;
  }

  noteLoading.value = true;
  try {
    await api.invoke('add_alert_note', {
      id: currentAlert.value.id,
      note: handlingNote.value,
    });

    ElMessage.success('备注已保存');

    // Reload alert detail
    const alert = await api.invoke<RiskAlertHistory>('get_alert_detail', {
      id: currentAlert.value.id,
    });
    currentAlert.value = alert;
  } catch (error) {
    console.error('Failed to add handling note:', error);
    ElMessage.error('保存备注失败：' + (error as Error).message);
  } finally {
    noteLoading.value = false;
  }
}

async function deleteAlert(alert: RiskAlertListItem) {
  try {
    await ElMessageBox.confirm('确定要删除此预警吗？此操作不可恢复。', '确认删除', {
      type: 'warning',
      confirmButtonText: '删除',
      cancelButtonText: '取消',
    });

    await api.invoke('delete_alert', { id: alert.id });
    ElMessage.success('预警已删除');
    loadAlerts();
  } catch (error) {
    if (error !== 'cancel') {
      console.error('Failed to delete alert:', error);
      ElMessage.error('删除失败：' + (error as Error).message);
    }
  }
}

async function bulkMarkHandled() {
  if (selectedAlerts.value.length === 0) return;

  try {
    await ElMessageBox.confirm(
      `确定要将选中的 ${selectedAlerts.value.length} 条预警标记为已处理吗？`,
      '批量操作',
      { type: 'warning' }
    );

    bulkActionLoading.value = true;

    for (const alert of selectedAlerts.value) {
      if (alert.status === 'active') {
        await api.invoke('add_alert_note', {
          id: alert.id,
          note: '批量处理',
        });
      }
    }

    ElMessage.success(`已成功处理 ${selectedAlerts.value.length} 条预警`);
    clearSelection();
    loadAlerts();
  } catch (error) {
    if (error !== 'cancel') {
      console.error('Failed to bulk mark as handled:', error);
      ElMessage.error('批量操作失败：' + (error as Error).message);
    }
  } finally {
    bulkActionLoading.value = false;
  }
}

async function bulkDelete() {
  if (selectedAlerts.value.length === 0) return;

  try {
    await ElMessageBox.confirm(
      `确定要删除选中的 ${selectedAlerts.value.length} 条预警吗？此操作不可恢复。`,
      '批量删除',
      {
        type: 'warning',
        confirmButtonText: '删除',
        cancelButtonText: '取消',
      }
    );

    bulkActionLoading.value = true;

    for (const alert of selectedAlerts.value) {
      await api.invoke('delete_alert', { id: alert.id });
    }

    ElMessage.success(`已成功删除 ${selectedAlerts.value.length} 条预警`);
    clearSelection();
    loadAlerts();
  } catch (error) {
    if (error !== 'cancel') {
      console.error('Failed to bulk delete:', error);
      ElMessage.error('批量删除失败：' + (error as Error).message);
    }
  } finally {
    bulkActionLoading.value = false;
  }
}

function exportToCSV() {
  if (alerts.value.length === 0) {
    ElMessage.warning('没有数据可导出');
    return;
  }

  // CSV Header
  let csv = '\uFEFF'; // BOM for Excel
  csv += 'ID,规则名称,严重程度,状态,预警信息,交易对,当前值,阈值,创建时间\n';

  // CSV Rows
  alerts.value.forEach((alert) => {
    csv += `${alert.id},`;
    csv += `${alert.rule_name},`;
    csv += `${getSeverityLabel(alert.severity)},`;
    csv += `${getStatusLabel(alert.status)},`;
    csv += `"${alert.message.replace(/"/g, '""')}",`;
    csv += `${alert.symbol || '-'},`;
    csv += `${alert.current_value || '-'},`;
    csv += `${alert.threshold_value || '-'},`;
    csv += `${formatFullTime(alert.created_at)},`;
    csv += '\n';
  });

  // Create download link
  const blob = new Blob([csv], { type: 'text/csv;charset=utf-8;' });
  const link = document.createElement('a');
  const url = URL.createObjectURL(blob);
  link.setAttribute('href', url);
  link.setAttribute('download', `alert_history_${Date.now()}.csv`);
  link.style.visibility = 'hidden';
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);

  ElMessage.success('导出成功');
}

// Lifecycle
onMounted(() => {
  loadAlerts();
});
</script>

<style scoped lang="scss">
.alert-history {
  padding: 20px;
  min-height: calc(100vh - 60px);
  background: var(--el-bg-color-page);
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.page-title {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 22px;
  font-weight: 700;
  color: #303133;
  margin: 0;

  .el-icon {
    color: #f56c6c;
  }
}

.header-actions {
  display: flex;
  gap: 12px;
}

.filter-card {
  margin-bottom: 20px;

  :deep(.el-card__body) {
    padding: 16px;
  }

  .el-form {
    margin-bottom: 0;

    .el-form-item {
      margin-bottom: 12px;

      &:last-child {
        margin-bottom: 0;
      }
    }
  }
}

.bulk-actions {
  margin-bottom: 16px;

  :deep(.el-alert__content) {
    display: flex;
    align-items: center;
    gap: 12px;
  }
}

.table-card {
  :deep(.el-card__body) {
    padding: 0;
  }
}

.alert-id {
  font-family: 'Courier New', monospace;
  font-size: 12px;
  color: #909399;
}

.text-muted {
  color: #909399;
}

.pagination-container {
  display: flex;
  justify-content: center;
  padding: 16px;
  border-top: 1px solid #ebeef5;
}

// Alert Detail Dialog
.alert-detail {
  .detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 0;
    border-bottom: 1px solid #f5f7fa;

    &:last-child {
      border-bottom: none;
    }
  }

  .detail-label {
    font-size: 14px;
    color: #909399;
    font-weight: 500;
  }

  .detail-value {
    font-size: 14px;
    color: #303133;
    font-weight: 600;
  }

  .detail-section {
    margin-top: 16px;

    h4 {
      font-size: 14px;
      color: #303133;
      margin-bottom: 12px;
    }
  }

  .add-note-section {
    .note-actions {
      margin-top: 12px;
      text-align: right;
    }
  }

  .handling-note {
    p {
      font-size: 14px;
      color: #606266;
      line-height: 1.6;
      padding: 12px;
      background: #f5f7fa;
      border-radius: 4px;
      white-space: pre-wrap;
    }
  }
}

// Responsive
@media (max-width: 768px) {
  .page-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }

  .filter-card {
    :deep(.el-form) {
      .el-form-item {
        display: block;
        width: 100%;

        .el-form-item__content {
          width: 100%;

          .el-date-editor,
          .el-select,
          .el-input {
            width: 100%;
          }
        }
      }
    }
  }
}
</style>
