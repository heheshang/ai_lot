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
        <div class="update-indicator" :class="{ active: isUpdating }">
          <span class="indicator-dot"></span>
          <span class="indicator-text">{{ isUpdating ? '更新中...' : '实时更新' }}</span>
        </div>
        <el-button :icon="Setting" @click="showSettings = true">风险设置</el-button>
        <el-button type="primary" :icon="Refresh" :loading="refreshing" @click="refreshData">刷新</el-button>
      </div>
    </div>

    <!-- 风险评分仪表盘 -->
    <div class="gauge-section">
      <TransitionGroup name="gauge-card" tag="div" class="gauge-cards">
        <div
          v-for="(card, index) in gaugeCards"
          :key="card.id"
          class="gauge-card"
          :style="{ transitionDelay: `${index * 100}ms` }"
        >
          <div class="gauge-header">
            <span class="gauge-title">{{ card.title }}</span>
            <el-icon class="gauge-icon" :class="card.iconClass">
              <component :is="card.icon" />
            </el-icon>
          </div>
          <div class="gauge-body">
            <div :ref="el => setGaugeRef(card.id, el)" class="gauge-chart" style="height: 200px"></div>
            <div class="risk-score-value">
              <span class="score" :class="getRiskLevelClass(card.value)">
                <span :ref="el => setScoreRef(card.id, el)">{{ displayScore(card.id, card.value) }}</span>
              </span>
              <span v-if="card.showLabel" class="label">/ 100</span>
              <span v-else class="label">%</span>
            </div>
          </div>
          <div class="gauge-footer">
            <span v-if="card.showLevel" class="risk-level" :class="getRiskLevelClass(card.value)">
              {{ getRiskLevelText(card.value) }}
            </span>
            <span v-else class="risk-desc">{{ card.desc }}</span>
          </div>
        </div>
      </TransitionGroup>
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
            <el-radio-button value="all">全部</el-radio-button>
            <el-radio-button value="critical">严重</el-radio-button>
            <el-radio-button value="warning">警告</el-radio-button>
            <el-radio-button value="info">提示</el-radio-button>
          </el-radio-group>
        </div>
      </div>

      <div class="alerts-list">
        <TransitionGroup name="alert-item">
          <div
            v-for="alert in filteredAlerts"
            :key="alert.id"
            class="alert-item"
            :class="[`alert-${alert.level}`, { 'alert-pulse': alert.level === 'critical' }]"
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
                  <el-button text size="small" @click="handleAlert(alert)">
                    <el-icon><Check /></el-icon>
                    处理
                  </el-button>
                  <el-button text size="small" @click="dismissAlert(alert.id)">
                    <el-icon><Close /></el-icon>
                    忽略
                  </el-button>
                </div>
              </div>
            </div>
          </div>
        </TransitionGroup>

        <div v-if="filteredAlerts.length === 0" class="alerts-empty">
          <div class="empty-state">
            <svg class="empty-icon" viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
              <circle cx="32" cy="32" r="28" fill="#E8F5E9"/>
              <path d="M32 18V32M32 38V40" stroke="#4CAF50" stroke-width="3" stroke-linecap="round"/>
              <circle cx="32" cy="32" r="24" stroke="#4CAF50" stroke-width="2" stroke-dasharray="4 4"/>
            </svg>
            <p class="empty-text">暂无预警信息</p>
            <p class="empty-desc">系统运行正常，所有风险指标在安全范围内</p>
          </div>
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

      <TransitionGroup name="position-card" tag="div" class="position-grid">
        <div
          v-for="(position, index) in positionRisks"
          :key="position.symbol"
          class="position-card"
          :style="{ transitionDelay: `${index * 80}ms` }"
        >
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
              <span
                class="pnl-value"
                :class="position.unrealizedPnl >= 0 ? 'text-success' : 'danger'"
                :ref="el => setPnlRef(position.symbol, el)"
              >
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
      </TransitionGroup>
    </div>

    <!-- 风险趋势图 -->
    <div class="risk-trend-section">
      <div class="section-header">
        <h3 class="section-title">
          <el-icon><TrendCharts /></el-icon>
          风险趋势
        </h3>
        <div class="section-actions">
          <el-radio-group v-model="trendPeriod" size="small" @change="updateTrendChart">
            <el-radio-button value="1h">1小时</el-radio-button>
            <el-radio-button value="4h">4小时</el-radio-button>
            <el-radio-button value="1d">1天</el-radio-button>
            <el-radio-button value="1w">1周</el-radio-button>
          </el-radio-group>
        </div>
      </div>
      <div class="trend-chart-container">
        <div ref="trendChartRef" class="trend-chart" style="height: 300px"></div>
      </div>
    </div>

    <!-- 风险设置对话框 -->
    <el-dialog v-model="showSettings" title="风险阈值设置" width="600px" :close-on-click-modal="false">
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
import { ref, computed, onMounted, onUnmounted, reactive } from 'vue';
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
  Check,
  Close,
} from '@element-plus/icons-vue';

// ============== 类型定义 ==============
interface GaugeCard {
  id: string;
  title: string;
  icon: any;
  iconClass: string;
  value: number;
  displayValue: number;
  showLabel: boolean;
  showLevel: boolean;
  desc?: string;
}

interface Alert {
  id: number;
  level: 'critical' | 'warning' | 'info';
  title: string;
  message: string;
  source: string;
  symbol: string;
  time: Date;
}

interface PositionRisk {
  symbol: string;
  value: number;
  amount: number;
  avgPrice: number;
  currentPrice: number;
  unrealizedPnl: number;
  unrealizedPnlPercent: number;
  riskPercent: number;
  displayPnl: number;
}

// ============== 图表引用管理 ==============
const gaugeRefs = reactive<Record<string, HTMLElement>>({});
const scoreRefs = reactive<Record<string, HTMLElement>>({});
const pnlRefs = reactive<Record<string, HTMLElement>>({});

function setGaugeRef(id: string, el: HTMLElement | null) {
  if (el) gaugeRefs[id] = el;
}

function setScoreRef(id: string, el: HTMLElement | null) {
  if (el) scoreRefs[id] = el;
}

function setPnlRef(symbol: string, el: HTMLElement | null) {
  if (el) pnlRefs[symbol] = el;
}

// ============== 状态管理 ==============
const showSettings = ref(false);
const alertFilter = ref<'all' | 'critical' | 'warning' | 'info'>('all');
const trendPeriod = ref<'1h' | '4h' | '1d' | '1w'>('1d');
const refreshing = ref(false);
const isUpdating = ref(false);

// ============== 仪表盘卡片数据 ==============
const gaugeCards = reactive<GaugeCard[]>([
  {
    id: 'riskScore',
    title: '整体风险评分',
    icon: Monitor,
    iconClass: 'icon-primary',
    value: 72,
    displayValue: 0,
    showLabel: true,
    showLevel: true,
  },
  {
    id: 'positionRisk',
    title: '仓位风险',
    icon: DataLine,
    iconClass: 'icon-success',
    value: 65,
    displayValue: 0,
    showLabel: false,
    showLevel: false,
    desc: '仓位利用率',
  },
  {
    id: 'exposureRisk',
    title: '暴露度风险',
    icon: TrendCharts,
    iconClass: 'icon-warning',
    value: 45,
    displayValue: 0,
    showLabel: false,
    showLevel: false,
    desc: '市场暴露度',
  },
  {
    id: 'volatilityRisk',
    title: '波动率风险',
    icon: DataAnalysis,
    iconClass: 'icon-danger',
    value: 38,
    displayValue: 0,
    showLabel: false,
    showLevel: false,
    desc: '价格波动率',
  },
]);

// 显示分数（用于动画）
function displayScore(id: string, value: number): number {
  const card = gaugeCards.find(c => c.id === id);
  return card?.displayValue ?? value;
}

// ============== 预警数据 ==============
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

// ============== 仓位风险数据 ==============
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
    displayPnl: 0,
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
    displayPnl: 0,
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
    displayPnl: 0,
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
    displayPnl: 0,
  },
]);

// ============== 风险设置 ==============
const riskSettings = ref({
  maxPositionRatio: 30,
  maxTotalPosition: 80,
  stopLossRatio: 5,
  takeProfitRatio: 15,
  maxDrawdown: 10,
  dailyLossLimit: 5,
});

// ============== 计算属性 ==============
const filteredAlerts = computed(() => {
  if (alertFilter.value === 'all') return alerts.value;
  return alerts.value.filter((a) => a.level === alertFilter.value);
});

const alertCount = computed(() => {
  return alerts.value.filter((a) => a.level === 'critical' || a.level === 'warning').length;
});

// ============== 工具函数 ==============
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

// ============== 交互处理 ==============
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

async function refreshData() {
  refreshing.value = true;
  isUpdating.value = true;

  // 模拟数据刷新
  await new Promise(resolve => setTimeout(resolve, 1500));

  // 随机更新风险评分
  gaugeCards.forEach(card => {
    const change = Math.floor(Math.random() * 10) - 5;
    card.value = Math.max(0, Math.min(100, card.value + change));
    animateScore(card.id, card.value);
  });

  // 随机更新仓位数据
  positionRisks.value.forEach(pos => {
    const change = (Math.random() - 0.5) * 500;
    pos.unrealizedPnl = Math.round(pos.unrealizedPnl + change);
    pos.displayPnl = pos.displayPnl || 0;
    animatePnl(pos.symbol, pos.unrealizedPnl);
  });

  updateTrendChart();

  refreshing.value = false;
  isUpdating.value = false;
  ElMessage.success('数据已刷新');
}

function saveSettings() {
  showSettings.value = false;
  ElMessage.success('风险设置已保存');
}

// ============== 动画函数 ==============
// 数字滚动动画 (easeOutExpo)
function easeOutExpo(t: number): number {
  return t === 1 ? 1 : 1 - Math.pow(2, -10 * t);
}

function animateScore(id: string, targetValue: number, duration: number = 1500) {
  const card = gaugeCards.find(c => c.id === id);
  if (!card) return;

  const startValue = card.displayValue;
  const startTime = performance.now();

  function update(currentTime: number) {
    const elapsed = currentTime - startTime;
    const progress = Math.min(elapsed / duration, 1);
    const eased = easeOutExpo(progress);
    const currentValue = startValue + (targetValue - startValue) * eased;

    card.displayValue = Math.round(currentValue);

    if (progress < 1) {
      requestAnimationFrame(update);
    } else {
      card.displayValue = targetValue;
    }
  }

  requestAnimationFrame(update);
}

function animatePnl(symbol: string, targetValue: number) {
  const position = positionRisks.find(p => p.symbol === symbol);
  if (!position) return;

  const startValue = position.displayPnl;
  const duration = 800;
  const startTime = performance.now();

  function update(currentTime: number) {
    const elapsed = currentTime - startTime;
    const progress = Math.min(elapsed / duration, 1);
    const eased = easeOutExpo(progress);
    const currentValue = startValue + (targetValue - startValue) * eased;

    position.displayPnl = currentValue;

    if (progress < 1) {
      requestAnimationFrame(update);
    } else {
      position.displayPnl = targetValue;
    }
  }

  requestAnimationFrame(update);
}

// ============== ECharts 图表 ==============
const trendChartRef = ref<HTMLElement>();
let trendChart: echarts.ECharts | null = null;

function renderGaugeChart(container: HTMLElement, value: number): echarts.ECharts {
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

function renderTrendChart() {
  if (!trendChartRef.value) return;

  if (!trendChart) {
    trendChart = echarts.init(trendChartRef.value);
  }

  const data = [];
  const now = new Date();
  const points = 24;

  for (let i = points; i >= 0; i--) {
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
        animationDuration: 1500,
        animationEasing: 'cubicOut',
      },
    ],
  };

  trendChart.setOption(option, true);
}

function updateTrendChart() {
  renderTrendChart();
}

// ============== 生命周期 ==============
let updateInterval: number | null = null;

onMounted(() => {
  // 渲染仪表盘
  setTimeout(() => {
    Object.entries(gaugeRefs).forEach(([id, container]) => {
      const card = gaugeCards.find(c => c.id === id);
      if (card && container) {
        renderGaugeChart(container, card.value);
        animateScore(id, card.value);
      }
    });
  }, 100);

  // 渲染趋势图
  renderTrendChart();

  // 初始化盈亏显示值
  positionRisks.value.forEach(pos => {
    pos.displayPnl = 0;
    setTimeout(() => {
      animatePnl(pos.symbol, pos.unrealizedPnl);
    }, 500 + positionRisks.value.indexOf(pos) * 100);
  });

  // 模拟实时更新 (每30秒)
  updateInterval = window.setInterval(() => {
    isUpdating.value = true;
    setTimeout(() => {
      // 轻微随机更新
      gaugeCards.forEach(card => {
        const change = Math.floor(Math.random() * 6) - 3;
        card.value = Math.max(0, Math.min(100, card.value + change));
        animateScore(card.id, card.value);
      });
      isUpdating.value = false;
    }, 500);
  }, 30000);
});

onUnmounted(() => {
  if (trendChart) {
    trendChart.dispose();
    trendChart = null;
  }
  if (updateInterval !== null) {
    clearInterval(updateInterval);
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
  margin-bottom: 24px;
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
      font-size: 24px;
    }
  }

  .header-actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }
}

// 实时更新指示器
.update-indicator {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  background: #f0f9ff;
  border-radius: 20px;
  font-size: 12px;
  color: #409eff;
  transition: all 0.3s ease;

  .indicator-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #409eff;
    animation: pulse 2s ease-in-out infinite;
  }

  &.active {
    background: #e6f7ff;

    .indicator-dot {
      animation: pulse-fast 0.8s ease-in-out infinite;
    }
  }
}

@keyframes pulse {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.6; transform: scale(1.1); }
}

@keyframes pulse-fast {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.5; transform: scale(1.2); }
}

// ============== 仪表盘区域 ==============
.gauge-section {
  margin-bottom: 24px;

  .gauge-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 20px;
  }
}

.gauge-card-enter-active {
  transition: all 0.5s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.gauge-card-enter-from {
  opacity: 0;
  transform: translateY(30px) scale(0.9);
}

.gauge-card {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  box-shadow: var(--shadow-2, 0 4px 16px rgba(0, 0, 0, 0.08));
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  border: 1px solid transparent;

  &:hover {
    box-shadow: var(--shadow-hover, 0 6px 20px rgba(0, 0, 0, 0.12));
    transform: translateY(-4px);
    border-color: #409eff;
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
      font-size: 20px;
      padding: 8px;
      border-radius: 8px;

      &.icon-primary { color: #409eff; background: #ecf5ff; }
      &.icon-success { color: #26a69a; background: #e6f7f7; }
      &.icon-warning { color: #e6a23c; background: #fef8f0; }
      &.icon-danger { color: #ef5350; background: #fef2f2; }
    }
  }

  .gauge-body {
    position: relative;

    .risk-score-value {
      position: absolute;
      bottom: 5px;
      left: 50%;
      transform: translateX(-50%);
      text-align: center;

      .score {
        font-size: 32px;
        font-weight: 700;
        display: inline-block;

        &.risk-low { color: #26a69a; }
        &.risk-medium { color: #e6a23c; }
        &.risk-high { color: #ef5350; }
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

      &.risk-low { color: #26a69a; }
      &.risk-medium { color: #e6a23c; }
      &.risk-high { color: #ef5350; }
    }

    .risk-desc {
      font-size: 12px;
      color: #909399;
    }
  }
}

// ============== 预警列表 ==============
.alerts-section {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: var(--shadow-2, 0 4px 16px rgba(0, 0, 0, 0.08));

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
    flex-wrap: wrap;
    gap: 12px;

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
        font-size: 20px;
      }

      .alert-badge {
        margin-left: 4px;
      }
    }
  }

  .alerts-list {
    max-height: 450px;
    overflow-y: auto;

    &::-webkit-scrollbar {
      width: 6px;
    }

    &::-webkit-scrollbar-thumb {
      background: #e5e7eb;
      border-radius: 3px;

      &:hover {
        background: #d1d5db;
      }
    }
  }
}

// 预警条目动画
.alert-item-enter-active {
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.alert-item-enter-from {
  opacity: 0;
  transform: translateX(-20px);
}

.alert-item-leave-active {
  transition: all 0.3s ease;
}

.alert-item-leave-to {
  opacity: 0;
  transform: translateX(20px);
}

.alert-item {
  display: flex;
  gap: 16px;
  padding: 16px;
  border-radius: 10px;
  border-left: 4px solid;
  margin-bottom: 12px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  position: relative;
  overflow: hidden;

  &:last-child {
    margin-bottom: 0;
  }

  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
    transform: translateX(-100%);
    transition: transform 0.6s ease;
  }

  &:hover {
    transform: translateX(4px);

    &::before {
      transform: translateX(100%);
    }
  }

  &.alert-critical {
    border-left-color: #ef5350;
    background: linear-gradient(135deg, #fef2f2 0%, #ffffff 100%);

    .alert-icon { color: #ef5350; }
  }

  &.alert-warning {
    border-left-color: #e6a23c;
    background: linear-gradient(135deg, #fffbeb 0%, #ffffff 100%);

    .alert-icon { color: #e6a23c; }
  }

  &.alert-info {
    border-left-color: #409eff;
    background: linear-gradient(135deg, #eff6ff 0%, #ffffff 100%);

    .alert-icon { color: #409eff; }
  }

  // 严重预警脉冲动画
  &.alert-pulse {
    animation: alert-pulse 2s ease-in-out infinite;

    @keyframes alert-pulse {
      0%, 100% { box-shadow: 0 0 0 0 rgba(239, 83, 80, 0); }
      50% { box-shadow: 0 0 0 8px rgba(239, 83, 80, 0.1); }
    }
  }

  .alert-icon {
    flex-shrink: 0;
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 24px;
    background: #fff;
    border-radius: 50%;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
  }

  .alert-content {
    flex: 1;
    min-width: 0;

    .alert-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 6px;

      .alert-title {
        font-size: 14px;
        font-weight: 600;
        color: #303133;
      }

      .alert-time {
        font-size: 12px;
        color: #909399;
        white-space: nowrap;
      }
    }

    .alert-message {
      font-size: 13px;
      color: #606266;
      margin-bottom: 10px;
      line-height: 1.5;
    }

    .alert-footer {
      display: flex;
      justify-content: space-between;
      align-items: center;
      flex-wrap: wrap;
      gap: 8px;

      .alert-source {
        display: flex;
        align-items: center;
        gap: 8px;

        .alert-symbol {
          font-size: 12px;
          color: #909399;
        }
      }

      .alert-actions {
        display: flex;
        gap: 4px;

        .el-button {
          display: flex;
          align-items: center;
          gap: 4px;

          .el-icon {
            font-size: 14px;
          }
        }
      }
    }
  }
}

// 空状态
.alerts-empty {
  padding: 40px 20px;
  text-align: center;

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;

    .empty-icon {
      width: 120px;
      height: 120px;
      animation: float 3s ease-in-out infinite;
    }

    .empty-text {
      font-size: 16px;
      font-weight: 600;
      color: #303133;
      margin: 0;
    }

    .empty-desc {
      font-size: 13px;
      color: #909399;
      margin: 0;
    }
  }
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-10px); }
}

// ============== 仓位风险分析 ==============
.position-risk-section {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: var(--shadow-2, 0 4px 16px rgba(0, 0, 0, 0.08));

  .section-header {
    margin-bottom: 20px;

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
        font-size: 20px;
      }
    }
  }

  .position-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 20px;
  }
}

// 仓位卡片动画
.position-card-enter-active {
  transition: all 0.5s cubic-bezier(0.34, 1.56, 0.64, 1);
}

.position-card-enter-from {
  opacity: 0;
  transform: translateY(30px) scale(0.95);
}

.position-card {
  border: 1px solid #e5e7eb;
  border-radius: 12px;
  padding: 18px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  background: linear-gradient(135deg, #ffffff 0%, #fafbfc 100%);
  position: relative;
  overflow: hidden;

  &::after {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.5), transparent);
    transition: left 0.6s ease;
  }

  &:hover {
    border-color: #409eff;
    box-shadow: 0 4px 20px rgba(64, 158, 255, 0.15);
    transform: translateY(-4px);

    &::after {
      left: 100%;
    }
  }

  .position-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 14px;

    .position-symbol {
      font-size: 16px;
      font-weight: 700;
      color: #303133;
    }

    .position-value {
      font-size: 15px;
      font-weight: 600;
      color: #409eff;
    }
  }

  .position-body {
    margin-bottom: 14px;

    .position-metrics {
      display: flex;
      justify-content: space-between;
      margin-bottom: 10px;

      .metric {
        display: flex;
        flex-direction: column;
        align-items: center;
        flex: 1;

        .metric-label {
          font-size: 11px;
          color: #909399;
          margin-bottom: 3px;
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
      padding-top: 10px;
      border-top: 1px solid #f3f4f6;

      .pnl-label {
        font-size: 12px;
        color: #909399;
      }

      .pnl-value {
        font-size: 14px;
        font-weight: 600;
        transition: color 0.3s ease;
      }
    }
  }

  .position-footer {
    .risk-bar {
      display: flex;
      align-items: center;
      gap: 10px;

      .risk-bar-label {
        font-size: 11px;
        color: #909399;
        white-space: nowrap;
      }

      .risk-bar-track {
        flex: 1;
        height: 8px;
        background: #f3f4f6;
        border-radius: 4px;
        overflow: hidden;
        position: relative;

        .risk-bar-fill {
          height: 100%;
          border-radius: 4px;
          transition: width 0.8s cubic-bezier(0.4, 0, 0.2, 1);
          position: relative;

          &::after {
            content: '';
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.3), transparent);
            animation: shimmer 2s infinite;
          }

          &.risk-low { background: linear-gradient(90deg, #26a69a, #4db6ac); }
          &.risk-medium { background: linear-gradient(90deg, #e6a23c, #f5ba7c); }
          &.risk-high { background: linear-gradient(90deg, #ef5350, #f07170); }
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

@keyframes shimmer {
  0% { transform: translateX(-100%); }
  100% { transform: translateX(100%); }
}

// ============== 风险趋势 ==============
.risk-trend-section {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  box-shadow: var(--shadow-2, 0 4px 16px rgba(0, 0, 0, 0.08));

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;
    flex-wrap: wrap;
    gap: 12px;

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
        font-size: 20px;
      }
    }
  }

  .trend-chart-container {
    .trend-chart {
      width: 100%;
    }
  }
}

// ============== 设置对话框 ==============
.setting-hint {
  margin-left: 12px;
  font-size: 12px;
  color: #909399;
}

// ============== 工具类 ==============
.text-success {
  color: #26a69a !important;
}

.danger {
  color: #ef5350 !important;
}

// ============== 响应式 ==============
@media (max-width: 768px) {
  .page-header {
    flex-direction: column;
    align-items: flex-start;

    .header-actions {
      width: 100%;
      justify-content: space-between;

      .update-indicator {
        display: none;
      }
    }
  }

  .gauge-section .gauge-cards {
    grid-template-columns: 1fr;
  }

  .position-risk-section .position-grid {
    grid-template-columns: 1fr;
  }

  .alerts-section .section-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .alert-item {
    flex-direction: column;
    gap: 12px;

    .alert-icon {
      align-self: flex-start;
    }

    .alert-content .alert-footer {
      flex-direction: column;
      align-items: flex-start;
    }
  }
}
</style>
