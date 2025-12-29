<template>
  <div class="backtest-view">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">
          <el-icon><DataAnalysis /></el-icon>
          策略回测
        </h2>
        <el-breadcrumb separator="/">
          <el-breadcrumb-item :to="{ path: '/dashboard' }">首页</el-breadcrumb-item>
          <el-breadcrumb-item>策略回测</el-breadcrumb-item>
        </el-breadcrumb>
      </div>
      <div class="header-actions">
        <el-button type="primary" :icon="VideoPlay" @click="runBacktest" :loading="isRunning">
          {{ isRunning ? '回测中...' : '开始回测' }}
        </el-button>
      </div>
    </div>

    <!-- 参数配置 -->
    <div class="config-section">
      <div class="section-header">
        <h3 class="section-title">
          <el-icon><Setting /></el-icon>
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
                style="width: 100%"
              />
              <span class="unit-text">%</span>
            </el-form-item>
          </el-col>
        </el-row>
      </el-form>
    </div>

    <!-- 回测进度 -->
    <div v-if="isRunning" class="progress-section">
      <div class="progress-content">
        <div class="progress-info">
          <span class="progress-label">回测进度</span>
          <span class="progress-percent">{{ progressPercent }}%</span>
        </div>
        <el-progress :percentage="progressPercent" :stroke-width="8" striped />
        <div class="progress-detail">
          <span>已处理 {{ processedBars }} 条 K线数据</span>
          <span>预计剩余 {{ estimatedTime }}</span>
        </div>
      </div>
    </div>

    <!-- 回测结果 -->
    <div v-if="backtestResult && !isRunning" class="result-section">
      <!-- 统计卡片 -->
      <div class="stats-cards">
        <div class="stat-card stat-primary">
          <div class="stat-icon">
            <el-icon :size="24"><TrendCharts /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-label">总收益率</div>
            <div class="stat-value" :class="getReturnClass(backtestResult.totalReturn)">
              {{ formatPercent(backtestResult.totalReturn) }}
            </div>
          </div>
          <div class="stat-chart">
            <svg viewBox="0 0 100 40" class="mini-chart">
              <path
                :d="getTrendPath(backtestResult.equityCurve)"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
              />
            </svg>
          </div>
        </div>

        <div class="stat-card stat-success">
          <div class="stat-icon">
            <el-icon :size="24"><Coin /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-label">最终资金</div>
            <div class="stat-value">
              {{ formatCurrency(backtestResult.finalCapital) }}
            </div>
          </div>
          <div class="stat-change">
            <span :class="getReturnClass(backtestResult.profit)">
              {{ formatCurrency(backtestResult.profit) }}
            </span>
          </div>
        </div>

        <div class="stat-card stat-warning">
          <div class="stat-icon">
            <el-icon :size="24"><Warning /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-label">最大回撤</div>
            <div class="stat-value danger">
              {{ formatPercent(backtestResult.maxDrawdown) }}
            </div>
          </div>
          <div class="stat-change">
            <span class="text-secondary">风险控制</span>
          </div>
        </div>

        <div class="stat-card stat-info">
          <div class="stat-icon">
            <el-icon :size="24"><Odometer /></el-icon>
          </div>
          <div class="stat-content">
            <div class="stat-label">夏普比率</div>
            <div class="stat-value">
              {{ backtestResult.sharpeRatio.toFixed(2) }}
            </div>
          </div>
          <div class="stat-change">
            <span :class="backtestResult.sharpeRatio >= 1 ? 'text-success' : 'text-warning'">
              {{ backtestResult.sharpeRatio >= 1 ? '优秀' : '一般' }}
            </span>
          </div>
        </div>
      </div>

      <!-- 详细指标 -->
      <div class="metrics-grid">
        <div class="metric-card">
          <div class="metric-header">
            <span class="metric-title">交易统计</span>
            <el-icon class="metric-icon"><ShoppingCart /></el-icon>
          </div>
          <div class="metric-body">
            <div class="metric-item">
              <span class="metric-label">总交易次数</span>
              <span class="metric-value">{{ backtestResult.totalTrades }}</span>
            </div>
            <div class="metric-item">
              <span class="metric-label">盈利次数</span>
              <span class="metric-value text-success">{{ backtestResult.winningTrades }}</span>
            </div>
            <div class="metric-item">
              <span class="metric-label">亏损次数</span>
              <span class="metric-value danger">{{ backtestResult.losingTrades }}</span>
            </div>
            <div class="metric-item">
              <span class="metric-label">胜率</span>
              <span class="metric-value" :class="getWinRateClass(backtestResult.winRate)">
                {{ formatPercent(backtestResult.winRate) }}
              </span>
            </div>
          </div>
        </div>

        <div class="metric-card">
          <div class="metric-header">
            <span class="metric-title">盈亏分析</span>
            <el-icon class="metric-icon"><DataLine /></el-icon>
          </div>
          <div class="metric-body">
            <div class="metric-item">
              <span class="metric-label">平均盈利</span>
              <span class="metric-value text-success">
                {{ formatCurrency(backtestResult.avgWin) }}
              </span>
            </div>
            <div class="metric-item">
              <span class="metric-label">平均亏损</span>
              <span class="metric-value danger">
                {{ formatCurrency(backtestResult.avgLoss) }}
              </span>
            </div>
            <div class="metric-item">
              <span class="metric-label">盈亏比</span>
              <span class="metric-value">
                {{ backtestResult.profitFactor.toFixed(2) }}
              </span>
            </div>
            <div class="metric-item">
              <span class="metric-label">期望收益</span>
              <span class="metric-value" :class="getReturnClass(backtestResult.expectedValue)">
                {{ formatCurrency(backtestResult.expectedValue) }}
              </span>
            </div>
          </div>
        </div>

        <div class="metric-card">
          <div class="metric-header">
            <span class="metric-title">风险指标</span>
            <el-icon class="metric-icon"><Monitor /></el-icon>
          </div>
          <div class="metric-body">
            <div class="metric-item">
              <span class="metric-label">最大连续盈利</span>
              <span class="metric-value text-success">{{ backtestResult.maxConsecutiveWins }}次</span>
            </div>
            <div class="metric-item">
              <span class="metric-label">最大连续亏损</span>
              <span class="metric-value danger">{{ backtestResult.maxConsecutiveLosses }}次</span>
            </div>
            <div class="metric-item">
              <span class="metric-label">最大单笔盈利</span>
              <span class="metric-value text-success">
                {{ formatCurrency(backtestResult.maxSingleWin) }}
              </span>
            </div>
            <div class="metric-item">
              <span class="metric-label">最大单笔亏损</span>
              <span class="metric-value danger">
                {{ formatCurrency(backtestResult.maxSingleLoss) }}
              </span>
            </div>
          </div>
        </div>

        <div class="metric-card">
          <div class="metric-header">
            <span class="metric-title">资金分析</span>
            <el-icon class="metric-icon"><Wallet /></el-icon>
          </div>
          <div class="metric-body">
            <div class="metric-item">
              <span class="metric-label">初始资金</span>
              <span class="metric-value">{{ formatCurrency(backtestResult.initialCapital) }}</span>
            </div>
            <div class="metric-item">
              <span class="metric-label">最大资金</span>
              <span class="metric-value text-success">
                {{ formatCurrency(backtestResult.peakCapital) }}
              </span>
            </div>
            <div class="metric-item">
              <span class="metric-label">最小资金</span>
              <span class="metric-value danger">
                {{ formatCurrency(backtestResult.troughCapital) }}
              </span>
            </div>
            <div class="metric-item">
              <span class="metric-label">平均资金利用率</span>
              <span class="metric-value">
                {{ formatPercent(backtestResult.avgCapitalUtilization) }}
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- 收益曲线 -->
      <div class="chart-section">
        <div class="chart-header">
          <h3 class="chart-title">
            <el-icon><TrendCharts /></el-icon>
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
          <div ref="equityChartRef" class="chart" style="height: 350px"></div>
        </div>
      </div>

      <!-- 交易记录 -->
      <div class="trades-section">
        <div class="section-header">
          <h3 class="section-title">
            <el-icon><List /></el-icon>
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
              <el-tag :type="row.side === 'buy' ? 'danger' : 'success'" size="small">
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

    <!-- 空状态 -->
    <div v-if="!backtestResult && !isRunning" class="empty-state">
      <el-empty description="配置参数后开始回测">
        <template #image>
          <div class="empty-icon">
            <el-icon :size="80"><DataAnalysis /></el-icon>
          </div>
        </template>
      </el-empty>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted, nextTick } from 'vue';
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
} from '@element-plus/icons-vue';
import { backtestApi, strategyApi, type BacktestJob, type Strategy } from '@/api/tauri';

// 图表引用
const equityChartRef = ref<HTMLElement>();
let equityChart: echarts.ECharts | null = null;

// 状态
const isRunning = ref(false);
const progressPercent = ref(0);
const processedBars = ref(0);
const estimatedTime = ref('');
const chartType = ref<'equity' | 'drawdown'>('equity');

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

// 加载策略列表
async function loadStrategies() {
  loadingStrategies.value = true;
  try {
    // 使用默认用户 ID，实际应用中应该从用户状态获取
    const userId = 'default';
    const result = await strategyApi.list(userId);
    strategies.value = result;
  } catch (error) {
    ElMessage.error(`加载策略列表失败: ${error}`);
  } finally {
    loadingStrategies.value = false;
  }
}

// 回测结果
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
  progressPercent.value = 0;
  processedBars.value = 0;

  try {
    // 构建回测配置
    const config = {
      strategy_id: backtestConfig.value.strategyId,
      symbol: backtestConfig.value.symbol,
      timeframe: backtestConfig.value.interval,
      start_time: Math.floor(backtestConfig.value.dateRange[0].getTime() / 1000),
      end_time: Math.floor(backtestConfig.value.dateRange[1].getTime() / 1000),
      initial_capital: backtestConfig.value.initialCapital,
      commission_rate: backtestConfig.value.feeRate / 100,
      slippage: backtestConfig.value.slippage / 100,
      max_positions: backtestConfig.value.maxPositions,
      max_position_ratio: backtestConfig.value.maxPositionRatio / 100,
      stop_loss_ratio: backtestConfig.value.stopLossRatio / 100,
    };

    // 直接运行回测
    const result = await backtestApi.run(config);

    // 转换结果格式
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
    }

    ElMessage.success('回测完成');
  } catch (error) {
    ElMessage.error(`回测失败: ${error}`);
  } finally {
    isRunning.value = false;
    nextTick(() => {
      renderChart();
    });
  }
}

function generateMockResult() {
  const initialCapital = backtestConfig.value.initialCapital;
  const trades: Trade[] = [];
  const equityCurve: number[] = [initialCapital];
  const drawdownCurve: number[] = [0];

  let balance = initialCapital;
  let peak = balance;
  const tradeCount = 50 + Math.floor(Math.random() * 100);

  for (let i = 0; i < tradeCount; i++) {
    const isBuy = Math.random() > 0.5;
    const price = 40000 + Math.random() * 20000;
    const amount = 0.1 + Math.random() * 2;
    const value = price * amount;
    const fee = value * (backtestConfig.value.feeRate / 100);

    const pnl = isBuy ? null : (Math.random() - 0.45) * 2000;
    if (pnl !== null) {
      balance += pnl;
    }
    balance -= fee;

    trades.push({
      id: i + 1,
      time: new Date(Date.now() - (tradeCount - i) * 3600 * 1000),
      side: isBuy ? 'buy' : 'sell',
      price,
      amount,
      value,
      fee,
      pnl,
      balance,
    });

    equityCurve.push(balance);
    peak = Math.max(peak, balance);
    const drawdown = ((peak - balance) / peak) * 100;
    drawdownCurve.push(-drawdown);
  }

  const winningTrades = trades.filter((t) => t.pnl && t.pnl > 0);
  const losingTrades = trades.filter((t) => t.pnl && t.pnl < 0);

  backtestResult.value = {
    initialCapital,
    finalCapital: balance,
    profit: balance - initialCapital,
    totalReturn: ((balance - initialCapital) / initialCapital) * 100,
    maxDrawdown: Math.abs(Math.min(...drawdownCurve)),
    sharpeRatio: 1.2 + Math.random() * 1.5,
    totalTrades: trades.length,
    winningTrades: winningTrades.length,
    losingTrades: losingTrades.length,
    winRate: (winningTrades.length / trades.length) * 100,
    avgWin: winningTrades.reduce((sum, t) => sum + t.pnl!, 0) / winningTrades.length || 0,
    avgLoss: losingTrades.reduce((sum, t) => sum + t.pnl!, 0) / losingTrades.length || 0,
    profitFactor:
      Math.abs(winningTrades.reduce((sum, t) => sum + t.pnl!, 0) /
        losingTrades.reduce((sum, t) => sum + t.pnl!, 0)) || 0,
    expectedValue: trades.reduce((sum, t) => sum + (t.pnl || 0), 0) / trades.length,
    maxConsecutiveWins: Math.floor(Math.random() * 10) + 3,
    maxConsecutiveLosses: Math.floor(Math.random() * 5) + 2,
    maxSingleWin: Math.max(...winningTrades.map((t) => t.pnl!), 0),
    maxSingleLoss: Math.min(...losingTrades.map((t) => t.pnl!), 0),
    peakCapital: peak,
    troughCapital: Math.min(...equityCurve),
    avgCapitalUtilization: 60 + Math.random() * 30,
    equityCurve,
    drawdownCurve,
    trades,
  };

  nextTick(() => {
    renderChart();
  });
}

function renderChart() {
  if (!equityChartRef.value || !backtestResult.value) return;

  if (!equityChart) {
    equityChart = echarts.init(equityChartRef.value);
  }

  const option: EChartsOption = {
    grid: {
      left: '3%',
      right: '3%',
      bottom: '10%',
      top: '10%',
      containLabel: true,
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'cross',
      },
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
      axisLine: { lineStyle: { color: '#e5e7eb' } },
      axisLabel: {
        color: '#6b7280',
        formatter: (value: number) => {
          if (chartType.value === 'equity') {
            return formatCurrency(value);
          }
          return `${Math.abs(value).toFixed(1)}%`;
        },
      },
      splitLine: { lineStyle: { color: '#f3f4f6' } },
    },
    series: [
      {
        type: 'line',
        data: chartType.value === 'equity'
          ? backtestResult.value.equityCurve
          : backtestResult.value.drawdownCurve,
        smooth: true,
        symbol: 'none',
        lineStyle: {
          width: 2,
          color: chartType.value === 'equity' ? '#3b82f6' : '#ef5350',
        },
        areaStyle: chartType.value === 'equity' ? {
          color: {
            type: 'linear',
            x: 0, y: 0, x2: 0, y2: 1,
            colorStops: [
              { offset: 0, color: 'rgba(59, 130, 246, 0.3)' },
              { offset: 1, color: 'rgba(59, 130, 246, 0)' },
            ],
          },
        } : {
          color: {
            type: 'linear',
            x: 0, y: 0, x2: 0, y2: 1,
            colorStops: [
              { offset: 0, color: 'rgba(239, 83, 80, 0.3)' },
              { offset: 1, color: 'rgba(239, 83, 80, 0)' },
            ],
          },
        },
      },
    ],
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
watch(chartType, () => {
  renderChart();
});

// 生命周期
onMounted(async () => {
  // 设置默认日期范围（最近一个月）
  const end = new Date();
  const start = new Date();
  start.setTime(start.getTime() - 3600 * 1000 * 24 * 30);
  backtestConfig.value.dateRange = [start, end];

  // 加载策略列表
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
      color: #409eff;
    }
  }
}

.config-section {
  background: #fff;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 20px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
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
      }
    }
  }

  .config-form {
    .strategy-option {
      display: flex;
      justify-content: space-between;
      align-items: center;

      .strategy-category {
        font-size: 12px;
        color: #909399;
        padding: 2px 6px;
        background: #f5f7fa;
        border-radius: 4px;
      }
    }

    .unit-text {
      margin-left: 8px;
      font-size: 12px;
      color: #909399;
    }
  }
}

.progress-section {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 20px;

  .progress-content {
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
        font-size: 24px;
        font-weight: 700;
        color: #fff;
      }
    }

    .progress-detail {
      display: flex;
      justify-content: space-between;
      margin-top: 12px;
      font-size: 12px;
      color: rgba(255, 255, 255, 0.7);
    }

    :deep(.el-progress-bar__outer) {
      background: rgba(255, 255, 255, 0.2);
    }

    :deep(.el-progress-bar__inner) {
      background: linear-gradient(90deg, #fff 0%, rgba(255, 255, 255, 0.8) 100%);
    }
  }
}

.result-section {
  .stats-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
    gap: 16px;
    margin-bottom: 20px;
  }

  .stat-card {
    background: #fff;
    border-radius: 12px;
    padding: 20px;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);
    display: flex;
    align-items: center;
    gap: 16px;
    transition: all 0.3s ease;

    &:hover {
      transform: translateY(-4px);
      box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    }

    .stat-icon {
      width: 56px;
      height: 56px;
      border-radius: 12px;
      display: flex;
      align-items: center;
      justify-content: center;
      color: #fff;
      flex-shrink: 0;
    }

    &.stat-primary .stat-icon {
      background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
    }

    &.stat-success .stat-icon {
      background: linear-gradient(135deg, #26a69a 0%, #00897b 100%);
    }

    &.stat-warning .stat-icon {
      background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
    }

    &.stat-info .stat-icon {
      background: linear-gradient(135deg, #8b5cf6 0%, #7c3aed 100%);
    }

    .stat-content {
      flex: 1;

      .stat-label {
        font-size: 12px;
        color: #909399;
        margin-bottom: 4px;
      }

      .stat-value {
        font-size: 22px;
        font-weight: 700;
        color: #303133;

        &.text-success {
          color: #26a69a;
        }

        &.danger {
          color: #ef5350;
        }
      }
    }

    .stat-change,
    .stat-chart {
      flex-shrink: 0;
      text-align: right;

      .mini-chart {
        width: 60px;
        height: 40px;
        color: #3b82f6;
      }

      span {
        font-size: 12px;
      }
    }
  }

  .metrics-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 16px;
    margin-bottom: 20px;
  }

  .metric-card {
    background: #fff;
    border-radius: 12px;
    padding: 20px;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);

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
        font-size: 20px;
        color: #909399;
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

  .chart-section {
    background: #fff;
    border-radius: 12px;
    padding: 20px;
    margin-bottom: 20px;
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);

    .chart-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 16px;

      .chart-title {
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

    .chart-container {
      .chart {
        width: 100%;
      }
    }
  }

  .trades-section {
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

    .trades-table {
      :deep(.el-table__body-wrapper) {
        max-height: 400px;
        overflow-y: auto;
      }
    }
  }
}

.empty-state {
  background: #fff;
  border-radius: 12px;
  padding: 80px 20px;
  text-align: center;

  .empty-icon {
    color: #c0c4cc;
    margin-bottom: 20px;
  }
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
</style>
