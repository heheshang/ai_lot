<template>
  <div class="backtest-view">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">
          <div class="title-icon">
            <el-icon><DataAnalysis /></el-icon>
          </div>
          策略回测
        </h2>
        <el-breadcrumb separator="/">
          <el-breadcrumb-item :to="{ path: '/dashboard' }">首页</el-breadcrumb-item>
          <el-breadcrumb-item>策略回测</el-breadcrumb-item>
        </el-breadcrumb>
      </div>
      <div class="header-actions">
        <el-button type="primary" :icon="VideoPlay" @click="runBacktest" :loading="isRunning" class="run-btn">
          {{ isRunning ? '回测中...' : '开始回测' }}
        </el-button>
      </div>
    </div>

    <!-- 步骤指示器 -->
    <div class="steps-container">
      <el-steps :active="currentStep" align-center finish-status="success" process-status="process">
        <el-step title="选择策略" />
        <el-step title="配置参数" />
        <el-step title="运行回测" />
        <el-step title="查看结果" />
      </el-steps>
    </div>

    <!-- 参数配置 -->
    <Transition name="section">
      <div v-if="currentStep < 3" class="config-section">
        <div class="section-header">
          <h3 class="section-title">
            <div class="icon-wrapper config">
              <el-icon><Setting /></el-icon>
            </div>
            回测参数
          </h3>
          <el-button text @click="resetConfig">
            <el-icon><RefreshLeft /></el-icon>
            重置
          </el-button>
        </div>

        <el-form :model="backtestConfig" label-width="120px" class="config-form">
          <el-row :gutter="20">
            <el-col :span="12">
              <el-form-item label="选择策略">
                <el-select v-model="backtestConfig.strategyId" placeholder="选择策略" style="width: 100%" :loading="loadingStrategies">
                  <el-option
                    v-for="strategy in strategies"
                    :key="strategy.id"
                    :label="strategy.name"
                    :value="strategy.id"
                  >
                    <div class="strategy-option">
                      <span class="strategy-name">{{ strategy.name }}</span>
                      <el-tag v-if="strategy.category" size="small" type="info">{{ strategy.category }}</el-tag>
                      <el-tag v-if="strategy.status" size="small" :type="strategy.status === 'active' ? 'success' : 'info'">{{ strategy.status }}</el-tag>
                    </div>
                  </el-option>
                </el-select>
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="交易对">
                <el-select v-model="backtestConfig.symbol" placeholder="选择交易对" style="width: 100%">
                  <el-option label="BTC/USDT" value="BTCUSDT" />
                  <el-option label="ETH/USDT" value="ETHUSDT" />
                  <el-option label="BNB/USDT" value="BNBUSDT" />
                  <el-option label="SOL/USDT" value="SOLUSDT" />
                </el-select>
              </el-form-item>
            </el-col>
          </el-row>

          <el-row :gutter="20">
            <el-col :span="12">
              <el-form-item label="时间范围">
                <el-date-picker
                  v-model="backtestConfig.dateRange"
                  type="daterange"
                  range-separator="至"
                  start-placeholder="开始日期"
                  end-placeholder="结束日期"
                  style="width: 100%"
                  :shortcuts="dateShortcuts"
                />
              </el-form-item>
            </el-col>
            <el-col :span="12">
              <el-form-item label="K线周期">
                <el-select v-model="backtestConfig.interval" placeholder="选择周期" style="width: 100%">
                  <el-option label="1分钟" value="1m" />
                  <el-option label="5分钟" value="5m" />
                  <el-option label="15分钟" value="15m" />
                  <el-option label="30分钟" value="30m" />
                  <el-option label="1小时" value="1h" />
                  <el-option label="4小时" value="4h" />
                  <el-option label="1天" value="1d" />
                </el-select>
              </el-form-item>
            </el-col>
          </el-row>

          <el-row :gutter="20">
            <el-col :span="8">
              <el-form-item label="初始资金">
                <el-input-number
                  v-model="backtestConfig.initialCapital"
                  :min="1000"
                  :max="10000000"
                  :step="1000"
                  :precision="2"
                  controls-position="right"
                  style="width: 100%"
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="手续费率">
                <el-input-number
                  v-model="backtestConfig.feeRate"
                  :min="0"
                  :max="1"
                  :step="0.01"
                  :precision="4"
                  controls-position="right"
                  style="width: 100%"
                />
                <span class="unit-text">%</span>
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="滑点">
                <el-input-number
                  v-model="backtestConfig.slippage"
                  :min="0"
                  :max="1"
                  :step="0.01"
                  :precision="4"
                  controls-position="right"
                  style="width: 100%"
                />
                <span class="unit-text">%</span>
              </el-form-item>
            </el-col>
          </el-row>

          <el-row :gutter="20">
            <el-col :span="8">
              <el-form-item label="最大持仓">
                <el-input-number
                  v-model="backtestConfig.maxPositions"
                  :min="1"
                  :max="100"
                  :step="1"
                  controls-position="right"
                  style="width: 100%"
                />
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="单笔最大比例">
                <el-input-number
                  v-model="backtestConfig.maxPositionRatio"
                  :min="1"
                  :max="100"
                  :step="1"
                  controls-position="right"
                  style="width: 100%"
                />
                <span class="unit-text">%</span>
              </el-form-item>
            </el-col>
            <el-col :span="8">
              <el-form-item label="止损比例">
                <el-input-number
                  v-model="backtestConfig.stopLossRatio"
                  :min="0"
                  :max="100"
                  :step="1"
                  controls-position="right"
                  style="width: 100%"
                />
                <span class="unit-text">%</span>
              </el-form-item>
            </el-col>
          </el-row>
        </el-form>
      </div>
    </Transition>

    <!-- 回测进度 -->
    <Transition name="section">
      <div v-if="isRunning" class="progress-section">
        <div class="progress-card">
          <div class="progress-header">
            <div class="progress-icon">
              <div class="pulse-ring"></div>
              <el-icon :size="32"><Timer /></el-icon>
            </div>
            <div class="progress-info">
              <h3 class="progress-title">回测进行中</h3>
              <p class="progress-desc">正在模拟策略交易...</p>
            </div>
          </div>

          <div class="progress-content">
            <div class="progress-bar-wrapper">
              <div class="progress-info">
                <span class="progress-label">完成进度</span>
                <span class="progress-percent">{{ progressPercent }}%</span>
              </div>
              <div class="progress-bar-container">
                <div class="progress-bar" :style="{ width: `${progressPercent}%` }">
                  <div class="progress-shine"></div>
                </div>
              </div>
            </div>

            <div class="progress-stats">
              <div class="progress-stat">
                <div class="stat-icon blue">
                  <el-icon><DataLine /></el-icon>
                </div>
                <div class="stat-content">
                  <div class="stat-label">已处理 K线</div>
                  <div class="stat-value">{{ processedBars.toLocaleString() }}</div>
                </div>
              </div>
              <div class="progress-stat">
                <div class="stat-icon purple">
                  <el-icon><Clock /></el-icon>
                </div>
                <div class="stat-content">
                  <div class="stat-label">预计剩余</div>
                  <div class="stat-value">{{ estimatedTime }}</div>
                </div>
              </div>
              <div class="progress-stat">
                <div class="stat-icon green">
                  <el-icon><TrendCharts /></el-icon>
                </div>
                <div class="stat-content">
                  <div class="stat-label">当前资金</div>
                  <div class="stat-value">${{ currentCapital.toLocaleString() }}</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Transition>

    <!-- 回测结果 -->
    <Transition name="section">
      <div v-if="backtestResult && !isRunning" class="result-section">
        <!-- 统计卡片 -->
        <TransitionGroup name="stat-card" tag="div" class="stats-cards">
          <div v-for="(stat, index) in statCards" :key="stat.key" class="stat-card" :class="stat.type">
            <div class="stat-bg"></div>
            <div class="stat-icon">
              <el-icon :size="26">
                <component :is="stat.icon" />
              </el-icon>
            </div>
            <div class="stat-content">
              <div class="stat-label">{{ stat.label }}</div>
              <div class="stat-value" :class="stat.valueClass">
                {{ stat.value }}
              </div>
            </div>
            <div class="stat-chart" v-if="stat.chart">
              <svg viewBox="0 0 100 40" class="mini-chart">
                <path :d="stat.chart" fill="none" stroke="currentColor" stroke-width="2" />
              </svg>
            </div>
            <div class="stat-change" v-if="stat.change">
              <span :class="stat.changeClass">{{ stat.change }}</span>
            </div>
          </div>
        </TransitionGroup>

        <!-- 详细指标 -->
        <div class="metrics-grid">
          <TransitionGroup name="metric-card" tag="div" class="metrics-row">
            <div v-for="(metric, index) in metricCards" :key="metric.key" class="metric-card">
              <div class="metric-header">
                <span class="metric-title">{{ metric.title }}</span>
                <div class="metric-icon" :class="metric.color">
                  <el-icon :size="20">
                    <component :is="metric.icon" />
                  </el-icon>
                </div>
              </div>
              <div class="metric-body">
                <div v-for="item in metric.items" :key="item.label" class="metric-item">
                  <span class="metric-label">{{ item.label }}</span>
                  <span class="metric-value" :class="item.valueClass">{{ item.value }}</span>
                </div>
              </div>
            </div>
          </TransitionGroup>
        </div>

        <!-- 收益曲线 -->
        <div class="chart-section">
          <div class="chart-header">
            <h3 class="chart-title">
              <div class="icon-wrapper chart">
                <el-icon><TrendCharts /></el-icon>
              </div>
              收益曲线
            </h3>
            <div class="chart-actions">
              <el-radio-group v-model="chartType" size="small">
                <el-radio-button label="equity">资金曲线</el-radio-button>
                <el-radio-button label="drawdown">回撤曲线</el-radio-button>
              </el-radio-group>
            </div>
          </div>
          <div class="chart-container">
            <div ref="equityChartRef" class="chart" style="height: 360px"></div>
          </div>
        </div>

        <!-- 交易记录 -->
        <div class="trades-section">
          <div class="section-header">
            <h3 class="section-title">
              <div class="icon-wrapper trades">
                <el-icon><List /></el-icon>
              </div>
              交易记录
            </h3>
            <el-button text @click="exportTrades">
              <el-icon><Download /></el-icon>
              导出
            </el-button>
          </div>

          <el-table :data="backtestResult.trades" stripe class="trades-table">
            <el-table-column prop="id" label="编号" width="80" />
            <el-table-column prop="time" label="时间" width="180">
              <template #default="{ row }">
                {{ formatDateTime(row.time) }}
              </template>
            </el-table-column>
            <el-table-column prop="side" label="方向" width="80">
              <template #default="{ row }">
                <el-tag :type="row.side === 'buy' ? 'danger' : 'success'" size="small" effect="plain">
                  {{ row.side === 'buy' ? '买入' : '卖出' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="price" label="价格" width="120">
              <template #default="{ row }">
                {{ formatPrice(row.price) }}
              </template>
            </el-table-column>
            <el-table-column prop="amount" label="数量" width="120">
              <template #default="{ row }">
                {{ formatAmount(row.amount) }}
              </template>
            </el-table-column>
            <el-table-column prop="value" label="成交额" width="140">
              <template #default="{ row }">
                {{ formatCurrency(row.value) }}
              </template>
            </el-table-column>
            <el-table-column prop="fee" label="手续费" width="100">
              <template #default="{ row }">
                {{ formatCurrency(row.fee) }}
              </template>
            </el-table-column>
            <el-table-column prop="pnl" label="盈亏" width="120">
              <template #default="{ row }">
                <span :class="getPnLClass(row.pnl)">
                  {{ row.pnl !== null ? formatCurrency(row.pnl) : '-' }}
                </span>
              </template>
            </el-table-column>
            <el-table-column prop="balance" label="余额" width="140">
              <template #default="{ row }">
                {{ formatCurrency(row.balance) }}
              </template>
            </el-table-column>
          </el-table>
        </div>
      </div>
    </Transition>

    <!-- 空状态 -->
    <Transition name="empty">
      <div v-if="!backtestResult && !isRunning" class="empty-state">
        <div class="empty-illustration">
          <svg viewBox="0 0 200 160" fill="none" xmlns="http://www.w3.org/2000/svg">
            <!-- Chart illustration -->
            <path d="M20 120 Q50 60 80 80 T140 80 T180 100" stroke="url(#chartGrad)" stroke-width="3" fill="none" stroke-linecap="round"/>
            <circle cx="50" cy="70" r="6" fill="url(#dotGrad1)"/>
            <circle cx="80" cy="60" r="6" fill="url(#dotGrad2)"/>
            <circle cx="110" cy="75" r="6" fill="url(#dotGrad1)"/>
            <circle cx="140" cy="55" r="6" fill="url(#dotGrad2)"/>
            <!-- Background elements -->
            <rect x="15" y="115" width="30" height="20" rx="4" fill="url(#barGrad)" opacity="0.6"/>
            <rect x="55" y="55" width="30" height="20" rx="4" fill="url(#barGrad)" opacity="0.8"/>
            <rect"="95" y="70" width="30" height="20" rx="4" fill="url(#barGrad)" opacity="0.7"/>
            <rect x="135" y="50" width="30" height="20" rx="4" fill="url(#barGrad)" opacity="0.9"/>
            <defs>
              <linearGradient id="chartGrad" x1="0%" y1="0%" x2="200%" y2="0%">
                <stop offset="0%" style="stop-color:#667eea"/>
                <stop offset="100%" style="stop-color:#764ba2"/>
              </linearGradient>
              <radialGradient id="dotGrad1">
                <stop offset="0%" style="stop-color:#667eea;stop-opacity:1"/>
                <stop offset="100%" style="stop-color:#667eea;stop-opacity:0.3"/>
              </radialGradient>
              <radialGradient id="dotGrad2">
                <stop offset="0%" style="stop-color:#764ba2;stop-opacity:1"/>
                <stop offset="100%" style="stop-color:#764ba2;stop-opacity:0.3"/>
              </radialGradient>
              <linearGradient id="barGrad" x1="0%" y1="0%" x2="0%" y2="100%">
                <stop offset="0%" style="stop-color:#667eea;stop-opacity:0.3"/>
                <stop offset="100%" style="stop-color:#764ba2;stop-opacity:0.1"/>
              </linearGradient>
            </defs>
          </svg>
        </div>
        <h3 class="empty-title">开始您的第一次回测</h3>
        <p class="empty-desc">配置参数后点击"开始回测"按钮</p>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, nextTick, computed } from 'vue';
import { ElMessage } from 'element-plus';
import * as echarts from 'echarts';
import type { EChartsOption } from 'echarts';
import {
  DataAnalysis,
  Setting,
  RefreshLeft,
  VideoPlay,
  TrendCharts,
  Coin,
  Warning,
  Odometer,
  ShoppingCart,
  DataLine,
  Monitor,
  Wallet,
  List,
  Download,
  Timer,
} from '@element-plus/icons-vue';
import { backtestApi, strategyApi } from '@/api/tauri';
import type { Strategy } from '@/types';

const equityChartRef = ref<HTMLElement>();
let equityChart: echarts.ECharts | null = null;

// 状态
const isRunning = ref(false);
const progressPercent = ref(0);
const processedBars = ref(0);
const estimatedTime = ref('');
const chartType = ref<'equity' | 'drawdown'>('equity');
const currentStep = ref(0);
const currentCapital = ref(100000);

// 回测配置
const backtestConfig = ref({
  strategyId: '',
  symbol: 'BTCUSDT',
  dateRange: [] as Date[],
  interval: '1h',
  initialCapital: 100000,
  feeRate: 0.1,
  slippage: 0.05,
  maxPositions: 3,
  maxPositionRatio: 30,
  stopLossRatio: 5,
});

// 策略列表
const strategies = ref<Strategy[]>([]);
const loadingStrategies = ref(false);

// 统计卡片配置
const statCards = computed(() => {
  if (!backtestResult.value) return [];
  const r = backtestResult.value;
  return [
    {
      key: 'return',
      label: '总收益率',
      value: formatPercent(r.totalReturn),
      valueClass: r.totalReturn >= 0 ? 'text-success' : 'danger',
      icon: TrendCharts,
      type: 'primary',
      chart: getTrendPath(r.equityCurve),
    },
    {
      key: 'final',
      label: '最终资金',
      value: formatCurrency(r.finalCapital),
      valueClass: '',
      icon: Coin,
      type: 'success',
      change: formatCurrency(r.profit),
      changeClass: r.profit >= 0 ? 'text-success' : 'danger',
    },
    {
      key: 'drawdown',
      label: '最大回撤',
      value: formatPercent(r.maxDrawdown),
      valueClass: 'danger',
      icon: Warning,
      type: 'warning',
      change: '风险指标',
    },
    {
      key: 'sharpe',
      label: '夏普比率',
      value: r.sharpeRatio.toFixed(2),
      valueClass: r.sharpeRatio >= 1 ? 'text-success' : 'text-warning',
      icon: Odometer,
      type: 'info',
      change: r.sharpeRatio >= 1 ? '优秀' : '一般',
    },
  ];
});

// 指标卡片配置
const metricCards = computed(() => {
  if (!backtestResult.value) return [];
  const r = backtestResult.value;
  return [
    {
      key: 'trades',
      title: '交易统计',
      icon: ShoppingCart,
      color: 'blue',
      items: [
        { label: '总交易次数', value: r.totalTrades, valueClass: '' },
        { label: '盈利次数', value: r.winningTrades, valueClass: 'text-success' },
        { label: '亏损次数', value: r.losingTrades, valueClass: 'danger' },
        { label: '胜率', value: formatPercent(r.winRate), valueClass: getWinRateClass(r.winRate) },
      ],
    },
    {
      key: 'pnl',
      title: '盈亏分析',
      icon: DataLine,
      color: 'green',
      items: [
        { label: '平均盈利', value: formatCurrency(r.avgWin), valueClass: 'text-success' },
        { label: '平均亏损', value: formatCurrency(r.avgLoss), valueClass: 'danger' },
        { label: '盈亏比', value: r.profitFactor.toFixed(2), valueClass: '' },
        { label: '期望收益', value: formatCurrency(r.expectedValue), valueClass: getReturnClass(r.expectedValue) },
      ],
    },
    {
      key: 'risk',
      title: '风险指标',
      icon: Monitor,
      color: 'purple',
      items: [
        { label: '最大连续盈利', value: `${r.maxConsecutiveWins}次`, valueClass: 'text-success' },
        { label: '最大连续亏损', value: `${r.maxConsecutiveLosses}次`, valueClass: 'danger' },
        { label: '最大单笔盈利', value: formatCurrency(r.maxSingleWin), valueClass: 'text-success' },
        { label: '最大单笔亏损', value: formatCurrency(r.maxSingleLoss), valueClass: 'danger' },
      ],
    },
    {
      key: 'capital',
      title: '资金分析',
      icon: Wallet,
      color: 'orange',
      items: [
        { label: '初始资金', value: formatCurrency(r.initialCapital), valueClass: '' },
        { label: '最大资金', value: formatCurrency(r.peakCapital), valueClass: 'text-success' },
        { label: '最小资金', value: formatCurrency(r.troughCapital), valueClass: 'danger' },
        { label: '平均资金利用率', value: formatPercent(r.avgCapitalUtilization), valueClass: '' },
      ],
    },
  ];
});

// 加载策略列表
async function loadStrategies() {
  loadingStrategies.value = true;
  try {
    const userId = 'default';
    const result = await strategyApi.list(userId);
    strategies.value = result;
  } catch (error) {
    ElMessage.error(`加载策略列表失败: ${error}`);
  } finally {
    loadingStrategies.value = false;
  }
}

// 回测结果接口
interface Trade {
  id: number;
  time: Date;
  side: 'buy' | 'sell';
  price: number;
  amount: number;
  value: number;
  fee: number;
  pnl: number | null;
  balance: number;
}

interface BacktestResult {
  initialCapital: number;
  finalCapital: number;
  profit: number;
  totalReturn: number;
  maxDrawdown: number;
  sharpeRatio: number;
  totalTrades: number;
  winningTrades: number;
  losingTrades: number;
  winRate: number;
  avgWin: number;
  avgLoss: number;
  profitFactor: number;
  expectedValue: number;
  maxConsecutiveWins: number;
  maxConsecutiveLosses: number;
  maxSingleWin: number;
  maxSingleLoss: number;
  peakCapital: number;
  troughCapital: number;
  avgCapitalUtilization: number;
  equityCurve: number[];
  drawdownCurve: number[];
  trades: Trade[];
}

const backtestResult = ref<BacktestResult | null>(null);

// 日期快捷选项
const dateShortcuts = [
  {
    text: '最近一周',
    value: () => {
      const end = new Date();
      const start = new Date();
      start.setTime(start.getTime() - 3600 * 1000 * 24 * 7);
      return [start, end];
    },
  },
  {
    text: '最近一个月',
    value: () => {
      const end = new Date();
      const start = new Date();
      start.setTime(start.getTime() - 3600 * 1000 * 24 * 30);
      return [start, end];
    },
  },
  {
    text: '最近三个月',
    value: () => {
      const end = new Date();
      const start = new Date();
      start.setTime(start.getTime() - 3600 * 1000 * 24 * 90);
      return [start, end];
    },
  },
];

// 方法
function resetConfig() {
  backtestConfig.value = {
    strategyId: '',
    symbol: 'BTCUSDT',
    dateRange: [],
    interval: '1h',
    initialCapital: 100000,
    feeRate: 0.1,
    slippage: 0.05,
    maxPositions: 3,
    maxPositionRatio: 30,
    stopLossRatio: 5,
  };
  backtestResult.value = null;
  currentStep.value = 0;
  ElMessage.success('已重置配置');
}

async function runBacktest() {
  if (!backtestConfig.value.strategyId) {
    ElMessage.warning('请选择策略');
    return;
  }
  if (!backtestConfig.value.dateRange?.length) {
    ElMessage.warning('请选择时间范围');
    return;
  }

  isRunning.value = true;
  currentStep.value = 2;
  progressPercent.value = 0;
  processedBars.value = 0;
  currentCapital.value = backtestConfig.value.initialCapital;

  // 模拟进度
  simulateProgress();

  try {
    const config = {
      strategy_id: backtestConfig.value.strategyId,
      symbol: backtestConfig.value.symbol,
      timeframe: backtestConfig.interval,
      start_time: Math.floor(backtestConfig.value.dateRange[0].getTime() / 1000),
      end_time: Math.floor(backtestConfig.value.dateRange[1].getTime() / 1000),
      initial_capital: backtestConfig.value.initialCapital,
      commission_rate: backtestConfig.value.feeRate / 100,
      slippage: backtestConfig.value.slippage / 100,
      max_positions: backtestConfig.value.maxPositions,
      max_position_ratio: backtestConfig.value.maxPositionRatio / 100,
      stop_loss_ratio: backtestConfig.value.stopLossRatio / 100,
    };

    const result = await backtestApi.run(config);

    if (result) {
      backtestResult.value = {
        initialCapital: config.initial_capital,
        finalCapital: result.final_capital || config.initial_capital,
        profit: result.profit || 0,
        totalReturn: result.total_return || 0,
        maxDrawdown: result.max_drawdown || 0,
        sharpeRatio: result.sharpe_ratio || 0,
        totalTrades: result.total_trades || 0,
        winningTrades: result.winning_trades || 0,
        losingTrades: result.losing_trades || 0,
        winRate: result.win_rate || 0,
        avgWin: result.avg_win || 0,
        avgLoss: result.avg_loss || 0,
        profitFactor: result.profit_factor || 0,
        expectedValue: result.expected_value || 0,
        maxConsecutiveWins: result.max_consecutive_wins || 0,
        maxConsecutiveLosses: result.max_consecutive_losses || 0,
        maxSingleWin: result.max_single_win || 0,
        maxSingleLoss: result.max_single_loss || 0,
        peakCapital: result.peak_capital || config.initial_capital,
        troughCapital: result.trough_capital || config.initial_capital,
        avgCapitalUtilization: result.avg_capital_utilization || 0,
        equityCurve: result.equity_curve || [],
        drawdownCurve: result.drawdown_curve || [],
        trades: (result.trades || []).map((t: any, i: number) => ({
          id: i + 1,
          time: new Date(t.time || Date.now()),
          side: t.side || 'buy',
          price: t.price || 0,
          amount: t.amount || 0,
          value: t.value || 0,
          fee: t.fee || 0,
          pnl: t.pnl || null,
          balance: t.balance || 0,
        })),
      };
      currentCapital.value = backtestResult.value.finalCapital;
    }

    currentStep.value = 3;
    ElMessage.success('回测完成');
  } catch (error) {
    ElMessage.error(`回测失败: ${error}`);
    currentStep.value = 1;
  } finally {
    isRunning.value = false;
    nextTick(() => renderChart());
  }
}

// 模拟进度
function simulateProgress() {
  const totalBars = Math.floor(Math.random() * 5000) + 5000;
  const duration = Math.random() * 5000 + 5000; // 5-10秒
  const interval = 100;
  let elapsed = 0;

  const timer = setInterval(() => {
    elapsed += interval;
    const progress = Math.min((elapsed / duration) * 100, 95);
    const processed = Math.floor((progress / 100) * totalBars);

    progressPercent.value = Math.floor(progress);
    processedBars.value = processed;
    currentCapital.value = backtestConfig.value.initialCapital * (1 + (progress / 100) * (Math.random() * 0.4 - 0.1));

    // 计算剩余时间
    const remainingMs = duration - elapsed;
    const minutes = Math.floor(remainingMs / 60000);
    const seconds = Math.floor((remainingMs % 60000) / 1000);
    estimatedTime.value = minutes > 0 ? `${minutes}分${seconds}秒` : `${seconds}秒`;

    if (progress >= 95 || !isRunning.value) {
      clearInterval(timer);
    }
  }, interval);
}

function renderChart() {
  if (!equityChartRef.value || !backtestResult.value) return;

  if (!equityChart) {
    equityChart = echarts.init(equityChartRef.value);
  }

  const option: EChartsOption = {
    grid: {
      left: '3%',
      right: '4%',
      bottom: '8%',
      top: '8%',
      containLabel: true,
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: { type: 'cross' },
      backgroundColor: 'rgba(0, 0, 0, 0.8)',
      borderColor: '#409eff',
      textStyle: { color: '#fff' },
      formatter: (params: any) => {
        const data = params[0];
        const value = chartType.value === 'equity'
          ? `资金: ${formatCurrency(data.value)}`
          : `回撤: ${Math.abs(data.value).toFixed(2)}%`;
        return `第${data.dataIndex + 1}笔交易<br/>${value}`;
      },
    },
    xAxis: {
      type: 'category',
      data: backtestResult.value.equityCurve.map((_, i) => i),
      boundaryGap: false,
      axisLine: { lineStyle: { color: '#e5e7eb' } },
      axisLabel: { color: '#6b7280' },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisLabel: {
        color: '#6b7280',
        formatter: (value: number) => {
          if (chartType.value === 'equity') {
            return formatCurrency(value);
          }
          return `${Math.abs(value).toFixed(1)}%`;
        },
      },
      splitLine: { lineStyle: { color: '#f3f4f6', type: 'dashed' } },
    },
    series: [
      {
        type: 'line',
        data: chartType.value === 'equity'
          ? backtestResult.value.equityCurve
          : backtestResult.value.drawdownCurve,
        smooth: true,
        symbol: 'circle',
        symbolSize: 6,
        showSymbol: false,
        lineStyle: {
          width: 3,
          color: chartType.value === 'equity' ? '#409eff' : '#f56c6c',
        },
        areaStyle: chartType.value === 'equity' ? {
          color: {
            type: 'linear',
            x: 0, y: 0, x2: 0, y2: 1,
            colorStops: [
              { offset: 0, color: 'rgba(64, 158, 255, 0.3)' },
              { offset: 1, color: 'rgba(64, 158, 255, 0.05)' },
            ],
          },
        } : {
          color: {
            type: 'linear',
            x: 0, y: 0, x2: 0, y2: 1,
            colorStops: [
              { offset: 0, color: 'rgba(245, 108, 108, 0.3)' },
              { offset: 1, color: 'rgba(245, 108, 108, 0.05)' },
            ],
          },
        },
        emphasis: {
          focus: 'series',
          scale: true,
        },
      },
    ],
    animationDuration: 1000,
    animationEasing: 'cubicOut',
  };

  equityChart.setOption(option);
}

// 格式化函数
function formatPercent(value: number): string {
  return `${value >= 0 ? '+' : ''}${value.toFixed(2)}%`;
}

function formatCurrency(value: number): string {
  return `$${value.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
}

function formatPrice(value: number): string {
  return `$${value.toLocaleString('en-US', { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`;
}

function formatAmount(value: number): string {
  return `${value.toFixed(4)} BTC`;
}

function formatDateTime(date: Date): string {
  return new Date(date).toLocaleString('zh-CN');
}

function getReturnClass(value: number): string {
  return value >= 0 ? 'text-success' : 'danger';
}

function getPnLClass(value: number | null): string {
  if (value === null) return '';
  return value >= 0 ? 'text-success' : 'danger';
}

function getWinRateClass(value: number): string {
  if (value >= 60) return 'text-success';
  if (value >= 40) return 'text-warning';
  return 'danger';
}

function getTrendPath(data: number[]): string {
  if (!data.length) return '';
  const max = Math.max(...data);
  const min = Math.min(...data);
  const range = max - min || 1;

  let path = `M 0 ${40 - ((data[0] - min) / range) * 35}`;
  data.forEach((value, i) => {
    if (i === 0) return;
    const x = (i / (data.length - 1)) * 100;
    const y = 40 - ((value - min) / range) * 35;
    path += ` L ${x} ${y}`;
  });

  return path;
}

function exportTrades() {
  ElMessage.success('交易记录导出功能开发中');
}

// 监听图表类型变化
watch(chartType, () => { renderChart(); });

// 生命周期
onMounted(async () => {
  const end = new Date();
  const start = new Date();
  start.setTime(start.getTime() - 3600 * 1000 * 24 * 30);
  backtestConfig.value.dateRange = [start, end];
  await loadStrategies();
});

onUnmounted(() => {
  if (equityChart) {
    equityChart.dispose();
    equityChart = null;
  }
});
</script>

<style scoped lang="scss">
.backtest-view {
  padding: 0;
  background: linear-gradient(180deg, #f5f7fa 0%, #e9eef3 100%);
  min-height: calc(100vh - 60px);
}

// 页面头部
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
  padding: 24px;
  background: white;
  border-radius: 16px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);

  .header-left {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .page-title {
    display: flex;
    align-items: center;
    gap: 12px;
    font-size: 22px;
    font-weight: 600;
    color: #303133;
    margin: 0;

    .title-icon {
      width: 40px;
      height: 40px;
      background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
      border-radius: 10px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: white;
    }
  }

  .run-btn {
    height: 40px;
    padding: 0 24px;
    border-radius: 10px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border: none;
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
    transition: all 0.3s cubic-bezier(0.25, 1, 0.5, 1);

    &:hover {
      transform: translateY(-2px);
      box-shadow: 0 6px 20px rgba(102, 126, 234, 0.5);
    }
  }
}

// 步骤指示器
.steps-container {
  background: white;
  border-radius: 16px;
  padding: 24px;
  margin-bottom: 20px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);

  :deep(.el-steps) {
    .el-step__title {
      font-weight: 500;
    }
  }
}

// 配置区域
.config-section {
  background: white;
  border-radius: 16px;
  padding: 24px;
  margin-bottom: 20px;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.06);

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;

    .section-title {
      display: flex;
      align-items: center;
      gap: 10px;
      font-size: 16px;
      font-weight: 600;
      color: #303133;
      margin: 0;

      .icon-wrapper {
        width: 36px;
        height: 36px;
        border-radius: 10px;
        display: flex;
        align-items: center;
        justify-content: center;

        &.config {
          background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
          color: white;
        }
      }
    }
  }

  .config-form {
    .unit-text {
      margin-left: 8px;
      font-size: 13px;
      color: #909399;
    }
  }
}

// 进度区域
.progress-section {
  margin-bottom: 20px;
}

.progress-card {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 20px;
  padding: 32px;
  box-shadow: 0 8px 24px rgba(102, 126, 234, 0.4);
  color: white;
}

.progress-header {
  display: flex;
  align-items: center;
  gap: 20px;
  margin-bottom: 28px;

  .progress-icon {
    position: relative;
    width: 60px;
    height: 60px;
    background: rgba(255, 255, 255, 0.15);
    border-radius: 16px;
    display: flex;
    align-items: center;
    justify-content: center;

    .pulse-ring {
      position: absolute;
      inset: -8px;
      border-radius: 24px;
      border: 2px solid rgba(255, 255, 255, 0.3);
      animation: pulse-ring 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
    }
  }
}

@keyframes pulse-ring {
  0% {
    transform: scale(0.8);
    opacity: 0.5;
  }
  50% {
    transform: scale(1);
    opacity: 0.3;
  }
  100% {
    transform: scale(0.8);
    opacity: 0.5;
  }
}

.progress-title {
  font-size: 20px;
  font-weight: 600;
  margin: 0 0 4px 0;
}

.progress-desc {
  font-size: 14px;
  color: rgba(255, 255, 255, 0.8);
  margin: 0;
}

.progress-content {
  .progress-bar-wrapper {
    margin-bottom: 24px;

    .progress-info {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 12px;

      .progress-label {
        font-size: 14px;
        color: rgba(255, 255, 255, 0.9);
      }

      .progress-percent {
        font-size: 28px;
        font-weight: 700;
      }
    }

    .progress-bar-container {
      height: 12px;
      background: rgba(255, 255, 255, 0.2);
      border-radius: 6px;
      overflow: hidden;
    }

    .progress-bar {
      height: 100%;
      background: linear-gradient(90deg, #fff 0%, rgba(255, 255, 255, 0.8) 100%);
      border-radius: 6px;
      position: relative;
      transition: width 0.3s ease;

      .progress-shine {
        position: absolute;
        top: 0;
        left: -100%;
        width: 100%;
        height: 100%;
        background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.4), transparent);
        animation: shine 2s infinite;
      }
    }
  }

  @keyframes shine {
    to { left: 100%; }
  }

  .progress-stats {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 16px;
  }

  .progress-stat {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 12px;
    padding: 16px;
    display: flex;
    align-items: center;
    gap: 12px;

    .stat-icon {
      width: 40px;
      height: 40px;
      border-radius: 10px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: white;

      &.blue { background: rgba(96, 165, 250, 0.3); }
      &.purple { background: rgba(139, 92, 246, 0.3); }
      &.green { background: rgba(74, 222, 128, 0.3); }
    }

    .stat-content {
      flex: 1;

      .stat-label {
        font-size: 12px;
        color: rgba(255, 255, 255, 0.8);
        margin-bottom: 2px;
      }

      .stat-value {
        font-size: 16px;
        font-weight: 600;
      }
    }
  }
}

// 结果区域
.result-section {
  .stats-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
    gap: 20px;
    margin-bottom: 24px;
  }

  .stat-card {
    position: relative;
    background: white;
    border-radius: 16px;
    padding: 24px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.06);
    display: flex;
    align-items: center;
    gap: 16px;
    overflow: hidden;
    transition: all 0.4s cubic-bezier(0.25, 1, 0.5, 1);

    &:hover {
      transform: translateY(-4px);
      box-shadow: 0 8px 28px rgba(0, 0, 0, 0.12);

      .stat-bg {
        transform: scale(1.3);
      }
    }

    &__bg {
      position: absolute;
      top: -20px;
      right: -20px;
      width: 100px;
      height: 100px;
      border-radius: 50%;
      opacity: 0.1;
      transition: transform 0.6s cubic-bezier(0.34, 1.56, 0.64, 1);
    }

    &.primary &__bg { background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%); }
    &.success &__bg { background: linear-gradient(135deg, #26a69a 0%, #00897b 100%); }
    &.warning &__bg { background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%); }
    &.info &__bg { background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%); }

    .stat-icon {
      width: 60px;
      height: 60px;
      border-radius: 14px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: #fff;
      z-index: 1;
      transition: all 0.4s cubic-bezier(0.34, 1.56, 0.64, 1);

      &.primary { background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%); }
      &.success { background: linear-gradient(135deg, #26a69a 0%, #00897b 100%); }
      &.warning { background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%); }
      &.info { background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%); }
    }

    &:hover .stat-icon {
      transform: scale(1.1) rotate(5deg);
    }

    .stat-content {
      flex: 1;

      .stat-label {
        font-size: 13px;
        color: #909399;
        margin-bottom: 6px;
      }

      .stat-value {
        font-size: 24px;
        font-weight: 700;
        color: #303133;
        line-height: 1;
      }
    }

    .stat-chart {
      flex-shrink: 0;
      width: 70px;

      .mini-chart {
        color: #3b82f6;
      }
    }

    .stat-change {
      flex-shrink: 0;
      text-align: right;

      span {
        font-size: 13px;
        font-weight: 500;
      }
    }
  }
}

// 指标网格
.metrics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 20px;
  margin-bottom: 24px;
}

.metric-card {
  background: white;
  border-radius: 16px;
  padding: 20px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.06);
  transition: all 0.3s cubic-bezier(0.25, 1, 0.5, 1);

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
  }

  .metric-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 16px;

    .metric-title {
      font-size: 14px;
      font-weight: 600;
      color: #303133;
    }

    .metric-icon {
      width: 32px;
      height: 32px;
      border-radius: 8px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: white;

      &.blue { background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%); }
      &.green { background: linear-gradient(135deg, #26a69a 0%, #00897b 100%); }
      &.purple { background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%); }
      &.orange { background: linear-gradient(135deg, #f97316 0%, #ea580c 100%); }
    }
  }

  .metric-body {
    display: flex;
    flex-direction: column;
    gap: 12px;

    .metric-item {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 8px 0;
      border-bottom: 1px solid #f5f7fa;

      &:last-child {
        border-bottom: none;
        padding-bottom: 0;
      }

      .metric-label {
        font-size: 13px;
        color: #606266;
      }

      .metric-value {
        font-size: 14px;
        font-weight: 600;
        color: #303133;
      }
    }
  }
}

// 图表区域
.chart-section {
  background: white;
  border-radius: 16px;
  padding: 24px;
  margin-bottom: 20px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.06);

  .chart-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;

    .chart-title {
      display: flex;
      align-items: center;
      gap: 10px;
      font-size: 16px;
      font-weight: 600;
      color: #303133;
      margin: 0;

      .icon-wrapper {
        width: 36px;
        height: 36px;
        border-radius: 10px;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
      }
    }
  }

  .chart-container {
    .chart {
      width: 100%;
    }
  }
}

// 交易记录
.trades-section {
  background: white;
  border-radius: 16px;
  padding: 24px;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.06);

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 20px;

    .section-title {
      display: flex;
      align-items: center;
      gap: 10px;
      font-size: 16px;
      font-weight: 600;
      color: #303133;
      margin: 0;

      .icon-wrapper {
        width: 36px;
        height: 36px;
        border-radius: 10px;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
      }
    }
  }

  .trades-table {
    border-radius: 12px;
    overflow: hidden;

    :deep(.el-table__body-wrapper) {
      max-height: 500px;
      overflow-y: auto;
    }

    :deep(.el-table__row) {
      transition: all 0.3s cubic-bezier(0.25, 1, 0.5, 1);

      &:hover {
        background: linear-gradient(135deg, rgba(102, 126, 234, 0.05) 0%, rgba(118, 75, 162, 0.05) 100%) !important;
      }
    }
  }
}

// 空状态
.empty-state {
  background: white;
  border-radius: 16px;
  padding: 80px 20px;
  text-align: center;
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.06);

  .empty-illustration {
    width: 200px;
    height: 160px;
    margin: 0 auto 24px;
  }

  .empty-title {
    font-size: 18px;
    font-weight: 600;
    color: #303133;
    margin: 0 0 8px 0;
  }

  .empty-desc {
    font-size: 14px;
    color: #909399;
    margin: 0;
  }
}

// 动画
.section-enter-active {
  transition: all 0.4s cubic-bezier(0.25, 1, 0.5, 1);
}

.section-enter-from {
  opacity: 0;
  transform: translateY(20px);
}

.section-enter-to {
  opacity: 1;
  transform: translateY(0);
}

.section-leave-active {
  transition: all 0.3s ease;
}

.section-leave-to {
  opacity: 0;
  transform: translateY(-10px);
}

.empty-enter-active {
  transition: all 0.4s cubic-bezier(0.25, 1, 0.5, 1);
}

.empty-enter-from {
  opacity: 0;
  transform: scale(0.95);
}

.empty-enter-to {
  opacity: 1;
  transform: scale(1);
}

.stat-card-enter-active {
  transition: all 0.5s cubic-bezier(0.25, 1, 0.5, 1);
}

.stat-card-enter-from {
  opacity: 0;
  transform: translateY(30px) scale(0.95);
}

.stat-card-enter-to {
  opacity: 1;
  transform: translateY(0) scale(1);
}

.metric-card-enter-active {
  transition: all 0.5s cubic-bezier(0.25, 1, 0.5, 1);
}

.metric-card-enter-from {
  opacity: 0;
  transform: translateY(20px);
}

.metric-card-enter-to {
  opacity: 1;
  transform: translateY(0);
}

// 工具类
.text-success {
  color: #26a69a !important;
}

.danger {
  color: #ef5350 !important;
}

.text-warning {
  color: #e6a23c !important;
}

.text-secondary {
  color: #909399 !important;
}

// 响应式
@media (max-width: 768px) {
  .page-header {
    flex-direction: column;
    gap: 16px;
    padding: 20px;

    .header-actions .run-btn {
      width: 100%;
    }
  }

  .progress-stats {
    grid-template-columns: 1fr;
  }

  .stats-cards {
    grid-template-columns: 1fr;
  }

  .metrics-grid {
    grid-template-columns: 1fr;
  }
}
</style>
