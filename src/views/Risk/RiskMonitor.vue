<template>
  <div class="risk-monitor">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">
          <el-icon><Warning /></el-icon>
          风险监控
        </h2>
        <el-breadcrumb separator="/">
          <el-breadcrumb-item :to="{ path: '/dashboard' }">首页</el-breadcrumb-item>
          <el-breadcrumb-item>风险监控</el-breadcrumb-item>
        </el-breadcrumb>
      </div>
      <div class="header-actions">
        <el-button :icon="Setting" @click="showSettings = true">风险设置</el-button>
        <el-button type="primary" :icon="Refresh" @click="refreshData">刷新</el-button>
      </div>
    </div>

    <!-- 风险评分仪表盘 -->
    <div class="gauge-section">
      <div class="gauge-cards">
        <div class="gauge-card">
          <div class="gauge-header">
            <span class="gauge-title">整体风险评分</span>
            <el-icon class="gauge-icon"><Monitor /></el-icon>
          </div>
          <div class="gauge-body">
            <div ref="riskScoreRef" class="gauge-chart" style="height: 200px"></div>
            <div class="risk-score-value">
              <span class="score" :class="getRiskLevelClass(riskScore)">{{ riskScore }}</span>
              <span class="label">/ 100</span>
            </div>
          </div>
          <div class="gauge-footer">
            <span class="risk-level" :class="getRiskLevelClass(riskScore)">
              {{ getRiskLevelText(riskScore) }}
            </span>
          </div>
        </div>

        <div class="gauge-card">
          <div class="gauge-header">
            <span class="gauge-title">仓位风险</span>
            <el-icon class="gauge-icon"><DataLine /></el-icon>
          </div>
          <div class="gauge-body">
            <div ref="positionRiskRef" class="gauge-chart" style="height: 200px"></div>
            <div class="risk-score-value">
              <span class="score" :class="getRiskLevelClass(100 - positionRisk)">{{ positionRisk }}%</span>
            </div>
          </div>
          <div class="gauge-footer">
            <span class="risk-desc">仓位利用率</span>
          </div>
        </div>

        <div class="gauge-card">
          <div class="gauge-header">
            <span class="gauge-title">暴露度风险</span>
            <el-icon class="gauge-icon"><TrendCharts /></el-icon>
          </div>
          <div class="gauge-body">
            <div ref="exposureRiskRef" class="gauge-chart" style="height: 200px"></div>
            <div class="risk-score-value">
              <span class="score" :class="getRiskLevelClass(100 - exposureRisk)">{{ exposureRisk }}%</span>
            </div>
          </div>
          <div class="gauge-footer">
            <span class="risk-desc">市场暴露度</span>
          </div>
        </div>

        <div class="gauge-card">
          <div class="gauge-header">
            <span class="gauge-title">波动率风险</span>
            <el-icon class="gauge-icon"><DataAnalysis /></el-icon>
          </div>
          <div class="gauge-body">
            <div ref="volatilityRiskRef" class="gauge-chart" style="height: 200px"></div>
            <div class="risk-score-value">
              <span class="score" :class="getRiskLevelClass(100 - volatilityRisk)">{{ volatilityRisk }}%</span>
            </div>
          </div>
          <div class="gauge-footer">
            <span class="risk-desc">价格波动率</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 实时预警列表 -->
    <div class="alerts-section">
      <div class="section-header">
        <h3 class="section-title">
          <el-icon><Bell /></el-icon>
          实时预警
          <el-badge :value="alertCount" :max="99" class="alert-badge" />
        </h3>
        <div class="section-actions">
          <el-radio-group v-model="alertFilter" size="small">
            <el-radio-button label="all">全部</el-radio-button>
            <el-radio-button label="critical">严重</el-radio-button>
            <el-radio-button label="warning">警告</el-radio-button>
            <el-radio-button label="info">提示</el-radio-button>
          </el-radio-group>
        </div>
      </div>

      <div class="alerts-list">
        <div
          v-for="alert in filteredAlerts"
          :key="alert.id"
          class="alert-item"
          :class="`alert-${alert.level}`"
        >
          <div class="alert-icon">
            <el-icon v-if="alert.level === 'critical'"><CircleClose /></el-icon>
            <el-icon v-else-if="alert.level === 'warning'"><Warning /></el-icon>
            <el-icon v-else><InfoFilled /></el-icon>
          </div>
          <div class="alert-content">
            <div class="alert-header">
              <span class="alert-title">{{ alert.title }}</span>
              <span class="alert-time">{{ formatTime(alert.time) }}</span>
            </div>
            <div class="alert-message">{{ alert.message }}</div>
            <div class="alert-footer">
              <div class="alert-source">
                <el-tag size="small" type="info">{{ alert.source }}</el-tag>
                <span class="alert-symbol">{{ alert.symbol }}</span>
              </div>
              <div class="alert-actions">
                <el-button text size="small" @click="handleAlert(alert)">处理</el-button>
                <el-button text size="small" @click="dismissAlert(alert.id)">忽略</el-button>
              </div>
            </div>
          </div>
        </div>

        <div v-if="filteredAlerts.length === 0" class="alerts-empty">
          <el-empty description="暂无预警" :image-size="100" />
        </div>
      </div>
    </div>

    <!-- 仓位风险分析 -->
    <div class="position-risk-section">
      <div class="section-header">
        <h3 class="section-title">
          <el-icon><ShoppingCart /></el-icon>
          仓位风险分析
        </h3>
      </div>

      <div class="position-grid">
        <div v-for="position in positionRisks" :key="position.symbol" class="position-card">
          <div class="position-header">
            <div class="position-symbol">{{ position.symbol }}</div>
            <div class="position-value">
              {{ formatCurrency(position.value) }}
            </div>
          </div>
          <div class="position-body">
            <div class="position-metrics">
              <div class="metric">
                <span class="metric-label">数量</span>
                <span class="metric-value">{{ position.amount }}</span>
              </div>
              <div class="metric">
                <span class="metric-label">均价</span>
                <span class="metric-value">{{ formatPrice(position.avgPrice) }}</span>
              </div>
              <div class="metric">
                <span class="metric-label">现价</span>
                <span class="metric-value" :class="getPriceClass(position.currentPrice, position.avgPrice)">
                  {{ formatPrice(position.currentPrice) }}
                </span>
              </div>
            </div>
            <div class="position-pnl">
              <span class="pnl-label">未实现盈亏</span>
              <span class="pnl-value" :class="position.unrealizedPnl >= 0 ? 'text-success' : 'danger'">
                {{ formatCurrency(position.unrealizedPnl) }}
                ({{ formatPercent(position.unrealizedPnlPercent) }})
              </span>
            </div>
          </div>
          <div class="position-footer">
            <div class="risk-bar">
              <div class="risk-bar-label">风险占比</div>
              <div class="risk-bar-track">
                <div
                  class="risk-bar-fill"
                  :style="{ width: position.riskPercent + '%' }"
                  :class="getRiskBarClass(position.riskPercent)"
                ></div>
              </div>
              <div class="risk-bar-value">{{ position.riskPercent }}%</div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 风险趋势图 -->
    <div class="risk-trend-section">
      <div class="section-header">
        <h3 class="section-title">
          <el-icon><TrendCharts /></el-icon>
          风险趋势
        </h3>
        <div class="section-actions">
          <el-radio-group v-model="trendPeriod" size="small">
            <el-radio-button label="1h">1小时</el-radio-button>
            <el-radio-button label="4h">4小时</el-radio-button>
            <el-radio-button label="1d">1天</el-radio-button>
            <el-radio-button label="1w">1周</el-radio-button>
          </el-radio-group>
        </div>
      </div>
      <div class="trend-chart-container">
        <div ref="trendChartRef" class="trend-chart" style="height: 300px"></div>
      </div>
    </div>

    <!-- 风险设置对话框 -->
    <el-dialog v-model="showSettings" title="风险阈值设置" width="600px">
      <el-form :model="riskSettings" label-width="120px">
        <el-form-item label="最大持仓比例">
          <el-slider v-model="riskSettings.maxPositionRatio" :min="1" :max="100" :step="1" show-input />
          <span class="setting-hint">单个币种最大持仓占比</span>
        </el-form-item>
        <el-form-item label="总仓位限制">
          <el-slider v-model="riskSettings.maxTotalPosition" :min="1" :max="100" :step="1" show-input />
          <span class="setting-hint">所有持仓总资金占比上限</span>
        </el-form-item>
        <el-form-item label="止损比例">
          <el-slider v-model="riskSettings.stopLossRatio" :min="1" :max="50" :step="1" show-input />
          <span class="setting-hint">单个仓位触发止损的比例</span>
        </el-form-item>
        <el-form-item label="止盈比例">
          <el-slider v-model="riskSettings.takeProfitRatio" :min="1" :max="100" :step="1" show-input />
          <span class="setting-hint">单个仓位触发止盈的比例</span>
        </el-form-item>
        <el-form-item label="最大回撤限制">
          <el-slider v-model="riskSettings.maxDrawdown" :min="1" :max="50" :step="1" show-input />
          <span class="setting-hint">账户最大可接受回撤比例</span>
        </el-form-item>
        <el-form-item label="日亏损限制">
          <el-slider v-model="riskSettings.dailyLossLimit" :min="1" :max="20" :step="1" show-input />
          <span class="setting-hint">单日最大可接受亏损比例</span>
        </el-form-item>
      </el-form>
      <template #footer>
        <el-button @click="showSettings = false">取消</el-button>
        <el-button type="primary" @click="saveSettings">保存设置</el-button>
      </template>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { ElMessage } from 'element-plus';
import * as echarts from 'echarts';
import type { EChartsOption } from 'echarts';
import {
  Warning,
  Setting,
  Refresh,
  Monitor,
  DataLine,
  TrendCharts,
  DataAnalysis,
  Bell,
  CircleClose,
  InfoFilled,
  ShoppingCart,
} from '@element-plus/icons-vue';

// 图表引用
const riskScoreRef = ref<HTMLElement>();
const positionRiskRef = ref<HTMLElement>();
const exposureRiskRef = ref<HTMLElement>();
const volatilityRiskRef = ref<HTMLElement>();
const trendChartRef = ref<HTMLElement>();

let riskScoreChart: echarts.ECharts | null = null;
let positionRiskChart: echarts.ECharts | null = null;
let exposureRiskChart: echarts.ECharts | null = null;
let volatilityRiskChart: echarts.ECharts | null = null;
let trendChart: echarts.ECharts | null = null;

// 状态
const showSettings = ref(false);
const alertFilter = ref<'all' | 'critical' | 'warning' | 'info'>('all');
const trendPeriod = ref<'1h' | '4h' | '1d' | '1w'>('1d');

// 风险数据
const riskScore = ref(72);
const positionRisk = ref(65);
const exposureRisk = ref(45);
const volatilityRisk = ref(38);

// 预警数据
interface Alert {
  id: number;
  level: 'critical' | 'warning' | 'info';
  title: string;
  message: string;
  source: string;
  symbol: string;
  time: Date;
}

const alerts = ref<Alert[]>([
  {
    id: 1,
    level: 'critical',
    title: '止损触发',
    message: 'ETH/USDT 触发止损条件，已自动平仓',
    source: '止损系统',
    symbol: 'ETH/USDT',
    time: new Date(Date.now() - 5 * 60 * 1000),
  },
  {
    id: 2,
    level: 'warning',
    title: '仓位过高',
    message: 'BTC/USDT 仓位占比已达 35%，超过阈值 30%',
    source: '仓位监控',
    symbol: 'BTC/USDT',
    time: new Date(Date.now() - 15 * 60 * 1000),
  },
  {
    id: 3,
    level: 'warning',
    title: '波动率异常',
    message: 'SOL/USDT 近1小时波动率达到 8.5%，请关注市场',
    source: '波动率监控',
    symbol: 'SOL/USDT',
    time: new Date(Date.now() - 30 * 60 * 1000),
  },
  {
    id: 4,
    level: 'info',
    title: '资金提醒',
    message: '可用资金不足 20%，建议补充资金或减少仓位',
    source: '资金管理',
    symbol: '全部',
    time: new Date(Date.now() - 60 * 60 * 1000),
  },
  {
    id: 5,
    level: 'info',
    title: '策略信号',
    message: '双均线策略产生卖出信号，请关注',
    source: '策略引擎',
    symbol: 'BNB/USDT',
    time: new Date(Date.now() - 2 * 60 * 60 * 1000),
  },
]);

// 仓位风险数据
interface PositionRisk {
  symbol: string;
  value: number;
  amount: number;
  avgPrice: number;
  currentPrice: number;
  unrealizedPnl: number;
  unrealizedPnlPercent: number;
  riskPercent: number;
}

const positionRisks = ref<PositionRisk[]>([
  {
    symbol: 'BTC/USDT',
    value: 45000,
    amount: 1.2,
    avgPrice: 42000,
    currentPrice: 43500,
    unrealizedPnl: 1800,
    unrealizedPnlPercent: 3.57,
    riskPercent: 35,
  },
  {
    symbol: 'ETH/USDT',
    value: 28000,
    amount: 15,
    avgPrice: 1950,
    currentPrice: 1880,
    unrealizedPnl: -1050,
    unrealizedPnlPercent: -3.59,
    riskPercent: 22,
  },
  {
    symbol: 'BNB/USDT',
    value: 15000,
    amount: 25,
    avgPrice: 620,
    currentPrice: 635,
    unrealizedPnl: 375,
    unrealizedPnlPercent: 2.42,
    riskPercent: 12,
  },
  {
    symbol: 'SOL/USDT',
    value: 8500,
    amount: 50,
    avgPrice: 175,
    currentPrice: 168,
    unrealizedPnl: -350,
    unrealizedPnlPercent: -4.0,
    riskPercent: 8,
  },
]);

// 风险设置
const riskSettings = ref({
  maxPositionRatio: 30,
  maxTotalPosition: 80,
  stopLossRatio: 5,
  takeProfitRatio: 15,
  maxDrawdown: 10,
  dailyLossLimit: 5,
});

// 计算属性
const filteredAlerts = computed(() => {
  if (alertFilter.value === 'all') return alerts.value;
  return alerts.value.filter((a) => a.level === alertFilter.value);
});

const alertCount = computed(() => {
  return alerts.value.filter((a) => a.level === 'critical' || a.level === 'warning').length;
});

// 方法
function formatTime(date: Date): string {
  const now = new Date();
  const diff = Math.floor((now.getTime() - date.getTime()) / 1000);

  if (diff < 60) return `${diff}秒前`;
  if (diff < 3600) return `${Math.floor(diff / 60)}分钟前`;
  if (diff < 86400) return `${Math.floor(diff / 3600)}小时前`;
  return `${Math.floor(diff / 86400)}天前`;
}

function formatCurrency(value: number): string {
  return `$${value.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
}

function formatPrice(value: number): string {
  return `$${value.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
}

function formatPercent(value: number): string {
  return `${value >= 0 ? '+' : ''}${value.toFixed(2)}%`;
}

function getRiskLevelClass(score: number): string {
  if (score >= 80) return 'risk-low';
  if (score >= 60) return 'risk-medium';
  return 'risk-high';
}

function getRiskLevelText(score: number): string {
  if (score >= 80) return '风险低';
  if (score >= 60) return '风险中等';
  return '风险高';
}

function getPriceClass(current: number, avg: number): string {
  if (current >= avg) return 'text-success';
  return 'danger';
}

function getRiskBarClass(percent: number): string {
  if (percent >= 50) return 'risk-high';
  if (percent >= 30) return 'risk-medium';
  return 'risk-low';
}

function handleAlert(alert: Alert) {
  ElMessage.success(`已处理预警: ${alert.title}`);
  const index = alerts.value.findIndex((a) => a.id === alert.id);
  if (index > -1) {
    alerts.value.splice(index, 1);
  }
}

function dismissAlert(id: number) {
  const index = alerts.value.findIndex((a) => a.id === id);
  if (index > -1) {
    alerts.value.splice(index, 1);
    ElMessage.info('已忽略该预警');
  }
}

function refreshData() {
  ElMessage.success('数据已刷新');
}

function saveSettings() {
  showSettings.value = false;
  ElMessage.success('风险设置已保存');
}

// 渲染仪表盘图表
function renderGaugeChart(container: HTMLElement, value: number, _color: string): echarts.ECharts {
  const chart = echarts.init(container);
  const option: EChartsOption = {
    series: [
      {
        type: 'gauge',
        startAngle: 180,
        endAngle: 0,
        min: 0,
        max: 100,
        radius: '80%',
        center: ['50%', '70%'],
        itemStyle: {
          color: {
            type: 'linear',
            x: 0, y: 0, x2: 1, y2: 0,
            colorStops: [
              { offset: 0, color: '#ef5350' },
              { offset: 0.5, color: '#e6a23c' },
              { offset: 1, color: '#26a69a' },
            ],
          },
        },
        progress: {
          show: true,
          width: 12,
        },
        pointer: {
          show: false,
        },
        axisLine: {
          lineStyle: {
            width: 12,
            color: [[1, '#e5e7eb']],
          },
        },
        axisTick: {
          show: false,
        },
        splitLine: {
          show: false,
        },
        axisLabel: {
          show: false,
        },
        detail: {
          show: false,
        },
        data: [{ value }],
      },
    ],
  };
  chart.setOption(option);
  return chart;
}

// 渲染趋势图
function renderTrendChart() {
  if (!trendChartRef.value) return;

  if (!trendChart) {
    trendChart = echarts.init(trendChartRef.value);
  }

  // 生成模拟数据
  const data = [];
  const now = new Date();
  for (let i = 24; i >= 0; i--) {
    const time = new Date(now.getTime() - i * 3600 * 1000);
    const score = 60 + Math.random() * 30 + Math.sin(i / 4) * 10;
    data.push({
      time: time.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' }),
      value: Math.round(score),
    });
  }

  const option: EChartsOption = {
    grid: {
      left: '3%',
      right: '4%',
      bottom: '10%',
      top: '15%',
      containLabel: true,
    },
    tooltip: {
      trigger: 'axis',
      formatter: (params: any) => {
        const data = params[0];
        return `${data.axisValue}<br/>风险评分: ${data.value}`;
      },
    },
    xAxis: {
      type: 'category',
      data: data.map((d) => d.time),
      axisLine: { lineStyle: { color: '#e5e7eb' } },
      axisLabel: { color: '#6b7280' },
    },
    yAxis: {
      type: 'value',
      min: 0,
      max: 100,
      axisLine: { lineStyle: { color: '#e5e7eb' } },
      axisLabel: { color: '#6b7280' },
      splitLine: { lineStyle: { color: '#f3f4f6' } },
    },
    series: [
      {
        type: 'line',
        data: data.map((d) => d.value),
        smooth: true,
        symbol: 'circle',
        symbolSize: 6,
        lineStyle: {
          width: 3,
          color: {
            type: 'linear',
            x: 0, y: 0, x2: 1, y2: 0,
            colorStops: [
              { offset: 0, color: '#ef5350' },
              { offset: 0.5, color: '#e6a23c' },
              { offset: 1, color: '#26a69a' },
            ],
          },
        },
        itemStyle: {
          color: '#409eff',
        },
        areaStyle: {
          color: {
            type: 'linear',
            x: 0, y: 0, x2: 0, y2: 1,
            colorStops: [
              { offset: 0, color: 'rgba(64, 158, 255, 0.3)' },
              { offset: 1, color: 'rgba(64, 158, 255, 0)' },
            ],
          },
        },
      },
    ],
  };

  trendChart.setOption(option);
}

// 生命周期
onMounted(() => {
  // 渲染仪表盘
  if (riskScoreRef.value) {
    riskScoreChart = renderGaugeChart(riskScoreRef.value, riskScore.value, '#409eff');
  }
  if (positionRiskRef.value) {
    positionRiskChart = renderGaugeChart(positionRiskRef.value, positionRisk.value, '#67c23a');
  }
  if (exposureRiskRef.value) {
    exposureRiskChart = renderGaugeChart(exposureRiskRef.value, exposureRisk.value, '#e6a23c');
  }
  if (volatilityRiskRef.value) {
    volatilityRiskChart = renderGaugeChart(volatilityRiskRef.value, volatilityRisk.value, '#f56c6c');
  }

  // 渲染趋势图
  renderTrendChart();
});

onUnmounted(() => {
  if (riskScoreChart) {
    riskScoreChart.dispose();
    riskScoreChart = null;
  }
  if (positionRiskChart) {
    positionRiskChart.dispose();
    positionRiskChart = null;
  }
  if (exposureRiskChart) {
    exposureRiskChart.dispose();
    exposureRiskChart = null;
  }
  if (volatilityRiskChart) {
    volatilityRiskChart.dispose();
    volatilityRiskChart = null;
  }
  if (trendChart) {
    trendChart.dispose();
    trendChart = null;
  }
});
</script>

<style scoped lang="scss">
.risk-monitor {
  padding: 0;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;

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
}

// 仪表盘区域
.gauge-section {
  margin-bottom: 20px;

  .gauge-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 16px;
  }

  .gauge-card {
    background: #fff;
    border-radius: 12px;
    padding: 20px;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);
    transition: all 0.3s ease;

    &:hover {
      box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    }

    .gauge-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 16px;

      .gauge-title {
        font-size: 14px;
        font-weight: 600;
        color: #303133;
      }

      .gauge-icon {
        font-size: 18px;
        color: #909399;
      }
    }

    .gauge-body {
      position: relative;

      .risk-score-value {
        position: absolute;
        bottom: 10px;
        left: 50%;
        transform: translateX(-50%);
        text-align: center;

        .score {
          font-size: 32px;
          font-weight: 700;

          &.risk-low {
            color: #26a69a;
          }

          &.risk-medium {
            color: #e6a23c;
          }

          &.risk-high {
            color: #ef5350;
          }
        }

        .label {
          font-size: 14px;
          color: #909399;
        }
      }
    }

    .gauge-footer {
      text-align: center;
      margin-top: 8px;

      .risk-level {
        font-size: 14px;
        font-weight: 600;

        &.risk-low {
          color: #26a69a;
        }

        &.risk-medium {
          color: #e6a23c;
        }

        &.risk-high {
          color: #ef5350;
        }
      }

      .risk-desc {
        font-size: 12px;
        color: #909399;
      }
    }
  }
}

// 预警列表
.alerts-section {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 20px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);

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
        color: #f56c6c;
      }

      .alert-badge {
        margin-left: 4px;
      }
    }
  }

  .alerts-list {
    max-height: 400px;
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
      border-left-color: #ef5350;
      background: #fef2f2;

      &:hover {
        background: #fee;
      }

      .alert-icon {
        color: #ef5350;
      }
    }

    &.alert-warning {
      border-left-color: #e6a23c;
      background: #fffbeb;

      &:hover {
        background: #fef3c7;
      }

      .alert-icon {
        color: #e6a23c;
      }
    }

    &.alert-info {
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
        align-items: center;
        margin-bottom: 4px;

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
      }

      .alert-footer {
        display: flex;
        justify-content: space-between;
        align-items: center;

        .alert-source {
          display: flex;
          align-items: center;
          gap: 8px;

          .alert-symbol {
            font-size: 12px;
            color: #909399;
          }
        }
      }
    }
  }

  .alerts-empty {
    padding: 40px 20px;
    text-align: center;
  }
}

// 仓位风险分析
.position-risk-section {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 20px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);

  .section-header {
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
    }
  }

  .position-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 16px;
  }

  .position-card {
    border: 1px solid #e5e7eb;
    border-radius: 10px;
    padding: 16px;
    transition: all 0.2s ease;

    &:hover {
      border-color: #409eff;
      box-shadow: 0 2px 12px rgba(64, 158, 255, 0.1);
    }

    .position-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 12px;

      .position-symbol {
        font-size: 16px;
        font-weight: 700;
        color: #303133;
      }

      .position-value {
        font-size: 14px;
        font-weight: 600;
        color: #409eff;
      }
    }

    .position-body {
      margin-bottom: 12px;

      .position-metrics {
        display: flex;
        justify-content: space-between;
        margin-bottom: 8px;

        .metric {
          display: flex;
          flex-direction: column;
          align-items: center;

          .metric-label {
            font-size: 11px;
            color: #909399;
            margin-bottom: 2px;
          }

          .metric-value {
            font-size: 13px;
            font-weight: 500;
            color: #303133;
          }
        }
      }

      .position-pnl {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding-top: 8px;
        border-top: 1px solid #f3f4f6;

        .pnl-label {
          font-size: 12px;
          color: #909399;
        }

        .pnl-value {
          font-size: 14px;
          font-weight: 600;
        }
      }
    }

    .position-footer {
      .risk-bar {
        display: flex;
        align-items: center;
        gap: 8px;

        .risk-bar-label {
          font-size: 11px;
          color: #909399;
          white-space: nowrap;
        }

        .risk-bar-track {
          flex: 1;
          height: 6px;
          background: #f3f4f6;
          border-radius: 3px;
          overflow: hidden;

          .risk-bar-fill {
            height: 100%;
            border-radius: 3px;
            transition: all 0.3s ease;

            &.risk-low {
              background: #26a69a;
            }

            &.risk-medium {
              background: #e6a23c;
            }

            &.risk-high {
              background: #ef5350;
            }
          }
        }

        .risk-bar-value {
          font-size: 12px;
          font-weight: 600;
          color: #303133;
          min-width: 35px;
          text-align: right;
        }
      }
    }
  }
}

// 风险趋势
.risk-trend-section {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);

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
    }
  }

  .trend-chart-container {
    .trend-chart {
      width: 100%;
    }
  }
}

// 风险设置对话框
.setting-hint {
  margin-left: 12px;
  font-size: 12px;
  color: #909399;
}

// 工具类
.text-success {
  color: #26a69a !important;
}

.danger {
  color: #ef5350 !important;
}
</style>
