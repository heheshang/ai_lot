<template>
  <div class="risk-dashboard">
    <!-- Page Header -->
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">
          <el-icon><Warning /></el-icon>
          风险监控仪表盘
        </h2>
        <el-breadcrumb separator="/">
          <el-breadcrumb-item :to="{ path: '/dashboard' }">首页</el-breadcrumb-item>
          <el-breadcrumb-item :to="{ path: '/risk' }">风险监控</el-breadcrumb-item>
          <el-breadcrumb-item>仪表盘</el-breadcrumb-item>
        </el-breadcrumb>
      </div>
      <div class="header-actions">
        <span class="last-update">
          <el-icon><Clock /></el-icon>
          上次更新: {{ formatTime(lastUpdate) }}
        </span>
        <el-button :icon="Refresh" @click="refreshData" :loading="loading">刷新</el-button>
      </div>
    </div>

    <!-- Real-time Risk Overview Cards -->
    <div class="overview-section">
      <el-row :gutter="16">
        <el-col :xs="24" :sm="12" :md="8" :lg="6" :xl="6">
          <div class="overview-card balance-card">
            <div class="card-icon">
              <el-icon><Wallet /></el-icon>
            </div>
            <div class="card-content">
              <div class="card-label">账户余额</div>
              <div class="card-value">{{ formatCurrency(overview?.balance || 0) }}</div>
            </div>
          </div>
        </el-col>

        <el-col :xs="24" :sm="12" :md="8" :lg="6" :xl="6">
          <div class="overview-card pnl-card">
            <div class="card-icon">
              <el-icon><TrendCharts /></el-icon>
            </div>
            <div class="card-content">
              <div class="card-label">今日盈亏</div>
              <div class="card-value" :class="(overview?.todayPnl ?? 0) >= 0 ? 'positive' : 'negative'">
                {{ formatCurrency(overview?.todayPnl || 0) }}
              </div>
            </div>
          </div>
        </el-col>

        <el-col :xs="24" :sm="12" :md="8" :lg="6" :xl="6">
          <div class="overview-card position-card">
            <div class="card-icon">
              <el-icon><ShoppingCart /></el-icon>
            </div>
            <div class="card-content">
              <div class="card-label">持仓价值</div>
              <div class="card-value">{{ formatCurrency(overview?.totalPositionValue || 0) }}</div>
            </div>
          </div>
        </el-col>

        <el-col :xs="24" :sm="12" :md="8" :lg="6" :xl="6">
          <div class="overview-card alert-card">
            <div class="card-icon">
              <el-icon><Bell /></el-icon>
            </div>
            <div class="card-content">
              <div class="card-label">活跃预警</div>
              <div class="card-value" :class="getAlertCountClass(overview?.activeAlertCount || 0)">
                {{ overview?.activeAlertCount || 0 }}
              </div>
            </div>
          </div>
        </el-col>
      </el-row>

      <!-- Drawdown and Peak Equity -->
      <el-row :gutter="16" style="margin-top: 16px">
        <el-col :xs="24" :sm="12" :md="12" :lg="12" :xl="12">
          <div class="overview-card drawdown-card">
            <div class="drawdown-header">
              <span class="drawdown-label">回撤率</span>
              <span class="drawdown-value" :class="getDrawdownClass(overview?.currentDrawdownPct || 0)">
                {{ (overview?.currentDrawdownPct || 0).toFixed(2) }}%
              </span>
            </div>
            <div class="drawdown-progress">
              <el-progress
                :percentage="Math.min(overview?.currentDrawdownPct || 0, 20) * 5"
                :color="getDrawdownProgressColor(overview?.currentDrawdownPct || 0)"
                :show-text="false"
              />
            </div>
            <div class="drawdown-footer">
              <span class="peak-equity">峰值权益: {{ formatCurrency(overview?.peakEquity || 0) }}</span>
            </div>
          </div>
        </el-col>

        <el-col :xs="24" :sm="12" :md="12" :lg="12" :xl="12">
          <div class="overview-card equity-card">
            <div class="equity-header">
              <span class="equity-label">权益曲线</span>
            </div>
            <div class="equity-chart" ref="equityChartRef" style="height: 80px"></div>
          </div>
        </el-col>
      </el-row>
    </div>

    <!-- Risk Rule Status Panel -->
    <div class="rule-status-section">
      <div class="section-header">
        <h3 class="section-title">
          <el-icon><DocumentChecked /></el-icon>
          风险规则状态
        </h3>
        <el-tag :type="getOverallStatusType()" size="small">
          {{ getOverallStatusText() }}
        </el-tag>
      </div>

      <div class="rule-status-grid">
        <div
          v-for="(status, ruleName) in overview?.ruleStatus"
          :key="ruleName"
          class="rule-status-card"
          :class="`status-${status}`"
        >
          <div class="rule-status-header">
            <span class="rule-name">{{ getRuleDisplayName(ruleName) }}</span>
            <div class="rule-indicator">
              <span v-if="status === 'ok'" class="indicator-dot ok"></span>
              <span v-else-if="status === 'warning'" class="indicator-dot warning"></span>
              <span v-else class="indicator-dot critical"></span>
            </div>
          </div>
          <div class="rule-status-text">
            <el-tag v-if="status === 'ok'" type="success" size="small">正常</el-tag>
            <el-tag v-else-if="status === 'warning'" type="warning" size="small">警告</el-tag>
            <el-tag v-else type="danger" size="small">严重</el-tag>
          </div>
        </div>
      </div>
    </div>

    <!-- Active Alerts Panel -->
    <div class="alerts-section">
      <div class="section-header">
        <h3 class="section-title">
          <el-icon><Bell /></el-icon>
          活跃预警
          <el-badge :value="activeAlerts.length" :max="99" class="alert-badge" />
        </h3>
        <el-radio-group v-model="alertFilter" size="small" @change="filterAlerts">
          <el-radio-button label="all">全部</el-radio-button>
          <el-radio-button label="critical">严重</el-radio-button>
          <el-radio-button label="high">高</el-radio-button>
          <el-radio-button label="medium">中</el-radio-button>
        </el-radio-group>
      </div>

      <div class="alerts-list" v-loading="alertsLoading">
        <div
          v-for="alert in filteredAlerts"
          :key="alert.id"
          class="alert-item"
          :class="`alert-${alert.severity}`"
        >
          <div class="alert-icon">
            <el-icon v-if="alert.severity === 'critical'"><CircleClose /></el-icon>
            <el-icon v-else-if="alert.severity === 'high'"><Warning /></el-icon>
            <el-icon v-else><InfoFilled /></el-icon>
          </div>
          <div class="alert-content">
            <div class="alert-header">
              <div class="alert-title-row">
                <span class="alert-title">{{ alert.rule_name }}</span>
                <el-tag :type="getSeverityTagType(alert.severity)" size="small">
                  {{ getSeverityText(alert.severity) }}
                </el-tag>
              </div>
              <span class="alert-time">{{ formatTime(alert.created_at) }}</span>
            </div>
            <div class="alert-message">{{ alert.message }}</div>
            <div class="alert-footer">
              <div class="alert-meta">
                <span v-if="alert.instance_id" class="alert-instance">{{ alert.instance_id }}</span>
                <span v-if="alert.symbol" class="alert-symbol">{{ alert.symbol }}</span>
              </div>
              <div class="alert-actions">
                <el-button text size="small" @click="onHandleAlert(alert.id)">处理</el-button>
                <el-button text size="small" @click="onIgnoreAlert(alert.id)">忽略</el-button>
              </div>
            </div>
          </div>
        </div>

        <div v-if="filteredAlerts.length === 0 && !alertsLoading" class="alerts-empty">
          <el-empty description="暂无活跃预警" :image-size="80" />
        </div>
      </div>
    </div>

    <!-- Position Utilization Gauge -->
    <div class="utilization-section">
      <div class="section-header">
        <h3 class="section-title">
          <el-icon><PieChart /></el-icon>
          仓位利用率
        </h3>
      </div>

      <div class="utilization-content">
        <div class="utilization-gauge" ref="gaugeChartRef" style="height: 200px"></div>
        <div class="utilization-legend">
          <div class="legend-item">
            <span class="legend-dot" style="background: #67c23a"></span>
            <span class="legend-text">安全 (0-50%)</span>
          </div>
          <div class="legend-item">
            <span class="legend-dot" style="background: #e6a23c"></span>
            <span class="legend-text">警告 (50-80%)</span>
          </div>
          <div class="legend-item">
            <span class="legend-dot" style="background: #f56c6c"></span>
            <span class="legend-text">危险 (80-100%)</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { ElMessage } from 'element-plus';
import { invoke } from '@tauri-apps/api/core';
import type { RiskOverview, RiskAlertListItem } from '@/types';
import * as echarts from 'echarts';
import type { EChartsOption } from 'echarts';
import {
  Warning,
  Refresh,
  Clock,
  Wallet,
  TrendCharts,
  ShoppingCart,
  Bell,
  DocumentChecked,
  CircleClose,
  InfoFilled,
  PieChart,
} from '@element-plus/icons-vue';

// State
const loading = ref(false);
const alertsLoading = ref(false);
const overview = ref<RiskOverview | null>(null);
const activeAlerts = ref<RiskAlertListItem[]>([]);
const alertFilter = ref<'all' | 'critical' | 'high' | 'medium'>('all');
const lastUpdate = ref(Date.now());
const equityChartRef = ref<HTMLElement>();
const gaugeChartRef = ref<HTMLElement>();

let equityChart: echarts.ECharts | null = null;
let gaugeChart: echarts.ECharts | null = null;
let refreshTimer: number | null = null;

// Computed
const filteredAlerts = computed(() => {
  if (alertFilter.value === 'all') return activeAlerts.value;
  return activeAlerts.value.filter((a) => a.severity === alertFilter.value);
});

// Methods
async function fetchOverview() {
  try {
    const data = await invoke<RiskOverview>('get_risk_overview');
    overview.value = data;
    lastUpdate.value = Date.now();
  } catch (error) {
    console.error('Failed to fetch risk overview:', error);
    ElMessage.error('获取风险概览失败');
  }
}

async function fetchAlerts() {
  try {
    alertsLoading.value = true;
    const data = await invoke<RiskAlertListItem[]>('get_active_alerts');
    activeAlerts.value = data;
  } catch (error) {
    console.error('Failed to fetch alerts:', error);
    ElMessage.error('获取预警列表失败');
  } finally {
    alertsLoading.value = false;
  }
}

async function refreshData() {
  loading.value = true;
  try {
    await Promise.all([fetchOverview(), fetchAlerts()]);
    ElMessage.success('数据已刷新');
  } finally {
    loading.value = false;
  }
}

function filterAlerts() {
  // Filtering is handled by computed property
}

async function onHandleAlert(alertId: string) {
  try {
    await invoke('handle_alert', { alertId });
    ElMessage.success('预警已处理');
    await fetchAlerts();
    await fetchOverview();
  } catch (error) {
    console.error('Failed to handle alert:', error);
    ElMessage.error('处理预警失败');
  }
}

async function onIgnoreAlert(alertId: string) {
  try {
    await invoke('ignore_alert', { alertId });
    ElMessage.info('预警已忽略');
    await fetchAlerts();
    await fetchOverview();
  } catch (error) {
    console.error('Failed to ignore alert:', error);
    ElMessage.error('忽略预警失败');
  }
}

function formatCurrency(value: number): string {
  return `$${value.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
}

function formatTime(timestamp: number): string {
  const now = Date.now();
  const diff = Math.floor((now - timestamp) / 1000);

  if (diff < 60) return `${diff}秒前`;
  if (diff < 3600) return `${Math.floor(diff / 60)}分钟前`;
  if (diff < 86400) return `${Math.floor(diff / 3600)}小时前`;
  return `${Math.floor(diff / 86400)}天前`;
}

function getAlertCountClass(count: number): string {
  if (count === 0) return 'no-alerts';
  if (count < 3) return 'few-alerts';
  return 'many-alerts';
}

function getDrawdownClass(drawdown: number): string {
  if (drawdown < 5) return 'low';
  if (drawdown < 10) return 'medium';
  return 'high';
}

function getDrawdownProgressColor(drawdown: number): string {
  if (drawdown < 5) return '#67c23a';
  if (drawdown < 10) return '#e6a23c';
  return '#f56c6c';
}

function getOverallStatusType(): 'success' | 'warning' | 'danger' {
  if (!overview?.value?.ruleStatus) return 'success';

  const statuses = Object.values(overview.value.ruleStatus);
  if (statuses.includes('critical')) return 'danger';
  if (statuses.includes('warning')) return 'warning';
  return 'success';
}

function getOverallStatusText(): string {
  if (!overview?.value?.ruleStatus) return '正常';

  const statuses = Object.values(overview.value.ruleStatus);
  if (statuses.includes('critical')) return '存在严重风险';
  if (statuses.includes('warning')) return '存在警告';
  return '全部正常';
}

function getRuleDisplayName(ruleName: string): string {
  const names: Record<string, string> = {
    position_limit: '持仓限制',
    drawdown_limit: '回撤限制',
    loss_limit: '亏损限制',
  };
  return names[ruleName] || ruleName;
}

function getSeverityTagType(severity: string): 'success' | 'warning' | 'danger' | 'info' {
  switch (severity) {
    case 'critical':
      return 'danger';
    case 'high':
      return 'danger';
    case 'medium':
      return 'warning';
    default:
      return 'info';
  }
}

function getSeverityText(severity: string): string {
  const texts: Record<string, string> = {
    critical: '严重',
    high: '高',
    medium: '中',
    low: '低',
  };
  return texts[severity] || severity;
}

function renderEquityChart() {
  if (!equityChartRef.value) return;

  if (!equityChart) {
    equityChart = echarts.init(equityChartRef.value);
  }

  // Generate mock equity curve data
  const data = [];
  let value = 10000;
  for (let i = 0; i < 24; i++) {
    value += (Math.random() - 0.45) * 200;
    data.push(value);
  }

  const option: EChartsOption = {
    grid: {
      left: '0%',
      right: '0%',
      bottom: '0%',
      top: '10%',
      containLabel: true,
    },
    xAxis: {
      type: 'category',
      show: false,
      data: Array.from({ length: 24 }, (_, i) => i.toString()),
    },
    yAxis: {
      type: 'value',
      show: false,
    },
    series: [
      {
        type: 'line',
        data,
        smooth: true,
        symbol: 'none',
        lineStyle: {
          width: 2,
          color: '#409eff',
        },
        areaStyle: {
          color: {
            type: 'linear',
            x: 0,
            y: 0,
            x2: 0,
            y2: 1,
            colorStops: [
              { offset: 0, color: 'rgba(64, 158, 255, 0.3)' },
              { offset: 1, color: 'rgba(64, 158, 255, 0)' },
            ],
          },
        },
      },
    ],
  };

  equityChart.setOption(option);
}

function renderGaugeChart() {
  if (!gaugeChartRef.value) return;

  if (!gaugeChart) {
    gaugeChart = echarts.init(gaugeChartRef.value);
  }

  const utilization = overview.value
    ? ((overview.value.totalPositionValue / overview.value.balance) * 100)
    : 0;

  const option: EChartsOption = {
    series: [
      {
        type: 'gauge',
        startAngle: 180,
        endAngle: 0,
        min: 0,
        max: 100,
        radius: '80%',
        center: ['50%', '60%'],
        itemStyle: {
          color: {
            type: 'linear',
            x: 0,
            y: 0,
            x2: 1,
            y2: 0,
            colorStops: [
              { offset: 0, color: '#67c23a' },
              { offset: 0.5, color: '#e6a23c' },
              { offset: 1, color: '#f56c6c' },
            ],
          },
        },
        progress: {
          show: true,
          width: 20,
        },
        pointer: {
          show: true,
          length: '60%',
          width: 6,
          itemStyle: {
            color: 'auto',
          },
        },
        axisLine: {
          lineStyle: {
            width: 20,
            color: [[1, '#e5e7eb']],
          },
        },
        axisTick: {
          distance: -20,
          length: 5,
          lineStyle: {
            color: '#fff',
            width: 1,
          },
        },
        splitLine: {
          distance: -20,
          length: 10,
          lineStyle: {
            color: '#fff',
            width: 2,
          },
        },
        axisLabel: {
          distance: -35,
          color: '#999',
          fontSize: 12,
          formatter: '{value}%',
        },
        detail: {
          valueAnimation: true,
          formatter: '{value}%',
          color: 'inherit',
          fontSize: 24,
          offsetCenter: [0, '20%'],
        },
        data: [{ value: Math.min(utilization, 100) }],
      },
    ],
  };

  gaugeChart.setOption(option);
}

// Lifecycle
onMounted(async () => {
  await refreshData();
  renderEquityChart();
  renderGaugeChart();

  // Auto-refresh every 10 seconds
  refreshTimer = window.setInterval(async () => {
    await fetchOverview();
    await fetchAlerts();
  }, 10000);
});

onUnmounted(() => {
  if (refreshTimer) {
    clearInterval(refreshTimer);
  }
  if (equityChart) {
    equityChart.dispose();
    equityChart = null;
  }
  if (gaugeChart) {
    gaugeChart.dispose();
    gaugeChart = null;
  }
});
</script>

<style scoped lang="scss">
.risk-dashboard {
  padding: 0;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  flex-wrap: wrap;
  gap: 16px;

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
    align-items: center;
    gap: 12px;

    .last-update {
      display: flex;
      align-items: center;
      gap: 6px;
      font-size: 13px;
      color: #909399;

      .el-icon {
        font-size: 14px;
      }
    }
  }
}

// Overview Cards
.overview-section {
  margin-bottom: 20px;
}

.overview-card {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);
  display: flex;
  align-items: center;
  gap: 16px;
  transition: all 0.3s ease;
  height: 100%;

  &:hover {
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    transform: translateY(-2px);
  }

  .card-icon {
    width: 50px;
    height: 50px;
    border-radius: 10px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 24px;
    flex-shrink: 0;
  }

  .card-content {
    flex: 1;
    min-width: 0;
  }

  .card-label {
    font-size: 13px;
    color: #909399;
    margin-bottom: 6px;
  }

  .card-value {
    font-size: 22px;
    font-weight: 700;
    color: #303133;

    &.positive {
      color: #67c23a;
    }

    &.negative {
      color: #f56c6c;
    }

    &.no-alerts {
      color: #67c23a;
    }

    &.few-alerts {
      color: #e6a23c;
    }

    &.many-alerts {
      color: #f56c6c;
    }
  }
}

.balance-card .card-icon {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #fff;
}

.pnl-card .card-icon {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
  color: #fff;
}

.position-card .card-icon {
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
  color: #fff;
}

.alert-card .card-icon {
  background: linear-gradient(135deg, #fa709a 0%, #fee140 100%);
  color: #fff;
}

// Drawdown Card
.drawdown-card {
  display: block !important;

  .drawdown-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;

    .drawdown-label {
      font-size: 14px;
      font-weight: 600;
      color: #303133;
    }

    .drawdown-value {
      font-size: 20px;
      font-weight: 700;

      &.low {
        color: #67c23a;
      }

      &.medium {
        color: #e6a23c;
      }

      &.high {
        color: #f56c6c;
      }
    }
  }

  .drawdown-progress {
    margin-bottom: 8px;
  }

  .drawdown-footer {
    .peak-equity {
      font-size: 12px;
      color: #909399;
    }
  }
}

// Equity Card
.equity-card {
  display: block !important;

  .equity-header {
    margin-bottom: 8px;

    .equity-label {
      font-size: 14px;
      font-weight: 600;
      color: #303133;
    }
  }
}

// Rule Status Section
.rule-status-section {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 20px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;

  .section-title {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 16px;
    font-weight: 600;
    color: #303133;
    margin: 0;

    .el-icon {
      color: #409eff;
    }

    .alert-badge {
      margin-left: 4px;
    }
  }
}

.rule-status-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 12px;
}

.rule-status-card {
  border: 1px solid #e5e7eb;
  border-radius: 8px;
  padding: 16px;
  transition: all 0.2s ease;

  &:hover {
    border-color: #409eff;
    box-shadow: 0 2px 12px rgba(64, 158, 255, 0.1);
  }

  &.status-ok {
    border-left: 3px solid #67c23a;
  }

  &.status-warning {
    border-left: 3px solid #e6a23c;
  }

  &.status-critical {
    border-left: 3px solid #f56c6c;
  }

  .rule-status-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;

    .rule-name {
      font-size: 14px;
      font-weight: 600;
      color: #303133;
    }

    .indicator-dot {
      width: 10px;
      height: 10px;
      border-radius: 50%;
      display: inline-block;

      &.ok {
        background: #67c23a;
      }

      &.warning {
        background: #e6a23c;
      }

      &.critical {
        background: #f56c6c;
      }
    }
  }

  .rule-status-text {
    display: flex;
    justify-content: flex-end;
  }
}

// Alerts Section
.alerts-section {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 20px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);
}

.alerts-list {
  max-height: 500px;
  overflow-y: auto;

  &::-webkit-scrollbar {
    width: 4px;
  }

  &::-webkit-scrollbar-thumb {
    background: #e5e7eb;
    border-radius: 2px;
  }
}

.alert-item {
  display: flex;
  gap: 12px;
  padding: 16px;
  border-radius: 8px;
  border-left: 3px solid;
  margin-bottom: 12px;
  background: #f9fafb;
  transition: all 0.2s ease;

  &:last-child {
    margin-bottom: 0;
  }

  &:hover {
    background: #f3f4f6;
  }

  &.alert-critical {
    border-left-color: #f56c6c;
    background: #fef2f2;

    &:hover {
      background: #fee;
    }

    .alert-icon {
      color: #f56c6c;
    }
  }

  &.alert-high {
    border-left-color: #e6a23c;
    background: #fffbeb;

    &:hover {
      background: #fef3c7;
    }

    .alert-icon {
      color: #e6a23c;
    }
  }

  &.alert-medium {
    border-left-color: #409eff;
    background: #eff6ff;

    &:hover {
      background: #dbeafe;
    }

    .alert-icon {
      color: #409eff;
    }
  }

  .alert-icon {
    flex-shrink: 0;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 20px;
  }

  .alert-content {
    flex: 1;
    min-width: 0;

    .alert-header {
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      margin-bottom: 8px;
      flex-wrap: wrap;
      gap: 8px;

      .alert-title-row {
        display: flex;
        align-items: center;
        gap: 8px;
      }

      .alert-title {
        font-size: 14px;
        font-weight: 600;
        color: #303133;
      }

      .alert-time {
        font-size: 12px;
        color: #909399;
      }
    }

    .alert-message {
      font-size: 13px;
      color: #606266;
      margin-bottom: 8px;
      line-height: 1.5;
    }

    .alert-footer {
      display: flex;
      justify-content: space-between;
      align-items: center;
      flex-wrap: wrap;
      gap: 8px;

      .alert-meta {
        display: flex;
        align-items: center;
        gap: 8px;

        .alert-instance,
        .alert-symbol {
          font-size: 12px;
          color: #909399;
        }
      }

      .alert-actions {
        display: flex;
        gap: 4px;
      }
    }
  }
}

.alerts-empty {
  padding: 40px 20px;
  text-align: center;
}

// Utilization Section
.utilization-section {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);
}

.utilization-content {
  display: flex;
  align-items: center;
  justify-content: space-around;
  flex-wrap: wrap;
  gap: 32px;
}

.utilization-gauge {
  flex: 1;
  min-width: 250px;
}

.utilization-legend {
  display: flex;
  flex-direction: column;
  gap: 16px;

  .legend-item {
    display: flex;
    align-items: center;
    gap: 10px;

    .legend-dot {
      width: 12px;
      height: 12px;
      border-radius: 50%;
    }

    .legend-text {
      font-size: 14px;
      color: #606266;
    }
  }
}

@media (max-width: 768px) {
  .page-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .header-actions {
    width: 100%;
    justify-content: space-between;
  }

  .utilization-content {
    flex-direction: column;
  }
}
</style>
