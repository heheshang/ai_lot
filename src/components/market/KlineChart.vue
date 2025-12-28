<template>
  <div class="kline-chart" :style="{ height: props.height }">
    <!-- 工具栏 -->
    <div class="chart-toolbar">
      <div class="toolbar-group">
        <span class="toolbar-label">时间周期:</span>
        <el-button-group size="small">
          <el-button
            v-for="tf in timeframes"
            :key="tf.value"
            :type="currentTimeframe === tf.value ? 'primary' : ''"
            @click="handleTimeframeChange(tf.value)"
          >
            {{ tf.label }}
          </el-button>
        </el-button-group>
      </div>

      <div class="toolbar-group">
        <span class="toolbar-label">指标:</span>
        <el-select
          v-model="selectedIndicators"
          multiple
          size="small"
          placeholder="选择指标"
          style="width: 200px"
          @change="updateChart"
        >
          <el-option label="MA均线" value="ma" />
          <el-option label="EMA均线" value="ema" />
          <el-option label="BOLL布林带" value="boll" />
          <el-option label="MACD" value="macd" />
          <el-option label="KDJ" value="kdj" />
          <el-option label="RSI" value="rsi" />
          <el-option label="VOL成交量" value="vol" />
        </el-select>
      </div>

      <div class="toolbar-group">
        <el-button size="small" @click="refreshData">
          <el-icon><Refresh /></el-icon>
          刷新
        </el-button>
        <el-button size="small" @click="toggleFullscreen">
          <el-icon><FullScreen /></el-icon>
          全屏
        </el-button>
      </div>
    </div>

    <!-- 图表内容 -->
    <div class="chart-content">
      <div v-if="loading" class="chart-loading">
        <div class="loading-spinner">
          <el-icon class="is-loading" :size="40"><Loading /></el-icon>
        </div>
        <span class="loading-text">加载K线数据中...</span>
      </div>

      <div v-else-if="error" class="chart-error">
        <el-icon :size="48" color="#f56c6c"><WarningFilled /></el-icon>
        <span class="error-text">{{ error }}</span>
        <el-button type="primary" size="small" @click="refreshData">重试</el-button>
      </div>

      <div v-else-if="chartData.length === 0" class="chart-empty">
        <el-empty description="暂无K线数据" :image-size="120">
          <el-button type="primary" @click="refreshData">加载数据</el-button>
        </el-empty>
      </div>

      <div v-else ref="chartRef" class="chart-container" />
    </div>

    <!-- 价格信息栏 -->
    <div v-if="currentPrice" class="price-info-bar">
      <div class="price-item">
        <span class="price-label">最新价</span>
        <span class="price-value" :class="priceClass">{{ currentPrice.close }}</span>
      </div>
      <div class="price-item">
        <span class="price-label">涨跌额</span>
        <span class="price-value" :class="priceClass">
          {{ priceChange }}
        </span>
      </div>
      <div class="price-item">
        <span class="price-label">涨跌幅</span>
        <span class="price-value" :class="priceClass">
          {{ priceChangePercent }}%
        </span>
      </div>
      <div class="price-item">
        <span class="price-label">24h最高</span>
        <span class="price-value text-up">{{ currentPrice.high }}</span>
      </div>
      <div class="price-item">
        <span class="price-label">24h最低</span>
        <span class="price-value text-down">{{ currentPrice.low }}</span>
      </div>
      <div class="price-item">
        <span class="price-label">24h成交量</span>
        <span class="price-value">{{ formatVolume(currentPrice.volume) }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue';
import * as echarts from 'echarts';
import { Loading, WarningFilled, Refresh, FullScreen } from '@element-plus/icons-vue';
import { useMarketStore } from '@/store/modules/market';
import type { Kline } from '@/types';
import type { EChartsOption } from 'echarts';

// Props
interface Props {
  symbol?: string;
  timeframe?: string;
  height?: string;
  autoLoad?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  symbol: undefined,
  timeframe: undefined,
  height: '600px',
  autoLoad: true,
});

// 时间周期选项
const timeframes = [
  { label: '1m', value: '1m' },
  { label: '5m', value: '5m' },
  { label: '15m', value: '15m' },
  { label: '30m', value: '30m' },
  { label: '1h', value: '1h' },
  { label: '4h', value: '4h' },
  { label: '1d', value: '1d' },
  { label: '1w', value: '1w' },
];

// Store
const marketStore = useMarketStore();

// Refs
const chartRef = ref<HTMLDivElement>();
const currentTimeframe = ref(props.timeframe || marketStore.currentTimeframe);
const selectedIndicators = ref<string[]>(['ma', 'vol']);
let chartInstance: echarts.ECharts | null = null;

// Computed
const currentSymbol = computed(() => props.symbol || marketStore.currentSymbol);
const chartData = computed(() => {
  const key = `${currentSymbol.value}_${currentTimeframe.value}`;
  return marketStore.klines.get(key) || [];
});
const loading = computed(() => marketStore.loading);
const error = computed(() => marketStore.error);

// 当前价格信息
const currentPrice = computed(() => {
  if (chartData.value.length === 0) return null;
  const latest = chartData.value[chartData.value.length - 1];
  const prev = chartData.value.length > 1 ? chartData.value[chartData.value.length - 2] : latest;

  return {
    close: latest.close.toFixed(2),
    open: latest.open,
    high: latest.high.toFixed(2),
    low: latest.low.toFixed(2),
    volume: latest.volume,
    prevClose: prev.close,
    closeNum: latest.close,
    prevCloseNum: prev.close,
  };
});

const priceChange = computed(() => {
  if (!currentPrice.value) return '--';
  const change = (currentPrice.value.closeNum || 0) - (currentPrice.value.prevCloseNum || 0);
  return (change >= 0 ? '+' : '') + change.toFixed(2);
});

const priceChangePercent = computed(() => {
  if (!currentPrice.value) return '--';
  const percent = ((currentPrice.value.closeNum || 0) - (currentPrice.value.prevCloseNum || 0)) / (currentPrice.value.prevCloseNum || 1) * 100;
  return (percent >= 0 ? '+' : '') + percent.toFixed(2);
});

const priceClass = computed(() => {
  if (!currentPrice.value) return '';
  return (currentPrice.value.closeNum || 0) >= (currentPrice.value.prevCloseNum || 0) ? 'price-up' : 'price-down';
});

// 颜色配置
const colors = {
  up: '#ef5350',
  down: '#26a69a',
  ma5: '#f5ce3a',
  ma10: '#36cfc9',
  ma20: '#3ba272',
  ma30: '#9a60b4',
  ma60: '#ea7ccc',
  bollUpper: 'rgba(64, 158, 255, 0.5)',
  bollLower: 'rgba(64, 158, 255, 0.5)',
};

// 计算移动平均线
function calculateMA(data: number[][], dayCount: number) {
  let result = [];
  for (let i = 0, len = data.length; i < len; i++) {
    if (i < dayCount - 1) {
      result.push('-');
      continue;
    }
    let sum = 0;
    for (let j = 0; j < dayCount; j++) {
      sum += data[i - j][1]; // 使用收盘价
    }
    result.push((sum / dayCount).toFixed(2));
  }
  return result;
}

// 计算布林带
function calculateBOLL(data: number[][], dayCount: number = 20) {
  const upper: (string | number)[] = [];
  const lower: (string | number)[] = [];
  const mid: (string | number)[] = [];

  for (let i = 0; i < data.length; i++) {
    if (i < dayCount - 1) {
      upper.push('-');
      lower.push('-');
      mid.push('-');
      continue;
    }

    const closePrices: number[] = [];
    for (let j = 0; j < dayCount; j++) {
      closePrices.push(data[i - j][1]);
    }

    const ma = closePrices.reduce((a, b) => a + b, 0) / dayCount;
    const squaredDiffs = closePrices.map(p => Math.pow(p - ma, 2));
    const stdDev = Math.sqrt(squaredDiffs.reduce((a, b) => a + b, 0) / dayCount);

    upper.push((ma + 2 * stdDev).toFixed(2));
    lower.push((ma - 2 * stdDev).toFixed(2));
    mid.push(ma.toFixed(2));
  }

  return { upper, lower, mid };
}

// 计算MACD
function calculateMACD(data: number[][], shortPeriod = 12, longPeriod = 26, signalPeriod = 9) {
  const closePrices = data.map(d => d[1]);

  function calculateEMA(prices: number[], period: number) {
    const k = 2 / (period + 1);
    const ema: number[] = [prices[0]];
    for (let i = 1; i < prices.length; i++) {
      ema.push(prices[i] * k + ema[i - 1] * (1 - k));
    }
    return ema;
  }

  const emaShort = calculateEMA(closePrices, shortPeriod);
  const emaLong = calculateEMA(closePrices, longPeriod);
  const dif: number[] = [];
  const dea: number[] = [];
  const macd: number[] = [];

  for (let i = 0; i < closePrices.length; i++) {
    dif.push(emaShort[i] - emaLong[i]);
  }

  dea.push(dif[0]);
  for (let i = 1; i < dif.length; i++) {
    dea.push(dif[i] * (2 / (signalPeriod + 1)) + dea[i - 1] * (1 - 2 / (signalPeriod + 1)));
  }

  for (let i = 0; i < dif.length; i++) {
    macd.push((dif[i] - dea[i]) * 2);
  }

  return { dif, dea, macd };
}

// 格式化成交量
function formatVolume(vol: number): string {
  if (vol >= 1e9) return (vol / 1e9).toFixed(2) + 'B';
  if (vol >= 1e6) return (vol / 1e6).toFixed(2) + 'M';
  if (vol >= 1e3) return (vol / 1e3).toFixed(2) + 'K';
  return vol.toFixed(2);
}

// 创建图表配置
function createChartOption(): EChartsOption {
  const rawData = chartData.value.map((kline: Kline) => [
    kline.timestamp,
    kline.open,
    kline.close,
    kline.low,
    kline.high,
  ]);

  const dates = rawData.map((item: any) => {
    const date = new Date(item[0]);
    return date.toLocaleTimeString('zh-CN', {
      hour: '2-digit',
      minute: '2-digit',
    });
  });

  const volumes = rawData.map((item: any) => item[0]);
  const volumeData = chartData.value.map((kline: Kline) => [
    kline.timestamp,
    kline.volume,
    kline.close > kline.open ? 1 : -1,
  ]);

  // 计算指标数据
  const ma5 = calculateMA(rawData, 5);
  const ma10 = calculateMA(rawData, 10);
  const ma20 = calculateMA(rawData, 20);
  const ma30 = calculateMA(rawData, 30);
  const boll = calculateBOLL(rawData, 20);
  const macdData = calculateMACD(rawData);

  // 构建series
  const series: any[] = [];
  const yAxis: any[] = [];
  const xAxis: any[] = [];
  const grid: any[] = [];

  // 主图 - K线
  grid.push({
    left: '10%',
    right: '8%',
    top: '5%',
    height: selectedIndicators.value.includes('macd') ? '45%' : '60%',
  });
  xAxis.push({
    type: 'category',
    data: dates,
    boundaryGap: false,
    axisLine: { onZero: false, lineStyle: { color: '#dcdfe6' } },
    axisLabel: { color: '#606266' },
    splitLine: { show: false },
    axisPointer: { z: 100 },
  });
  yAxis.push({
    scale: true,
    splitLine: { lineStyle: { color: '#ebeef5', type: 'dashed' } },
    axisLabel: { color: '#606266' },
    axisLine: { lineStyle: { color: '#dcdfe6' } },
  });

  // K线
  series.push({
    name: 'K线',
    type: 'candlestick',
    data: rawData,
    itemStyle: {
      color: colors.up,
      color0: colors.down,
      borderColor: colors.up,
      borderColor0: colors.down,
    },
  });

  // MA均线
  if (selectedIndicators.value.includes('ma')) {
    series.push(
      {
        name: 'MA5',
        type: 'line',
        data: ma5,
        smooth: true,
        lineStyle: { opacity: 0.8, width: 1 },
        itemStyle: { color: colors.ma5 },
      },
      {
        name: 'MA10',
        type: 'line',
        data: ma10,
        smooth: true,
        lineStyle: { opacity: 0.8, width: 1 },
        itemStyle: { color: colors.ma10 },
      },
      {
        name: 'MA20',
        type: 'line',
        data: ma20,
        smooth: true,
        lineStyle: { opacity: 0.8, width: 1 },
        itemStyle: { color: colors.ma20 },
      },
      {
        name: 'MA30',
        type: 'line',
        data: ma30,
        smooth: true,
        lineStyle: { opacity: 0.8, width: 1 },
        itemStyle: { color: colors.ma30 },
      }
    );
  }

  // 布林带
  if (selectedIndicators.value.includes('boll')) {
    series.push({
      name: 'BOLL',
      type: 'line',
      data: boll.mid,
      smooth: true,
      lineStyle: { opacity: 0.5, width: 1, type: 'dashed' },
      itemStyle: { color: '#409eff' },
    });
    series.push({
      name: 'UPPER',
      type: 'line',
      data: boll.upper,
      smooth: true,
      lineStyle: { opacity: 0.3, width: 1 },
      itemStyle: { color: '#409eff' },
      areaStyle: {
        color: new echarts.graphic.LinearGradient(0, 0, 0, 1, [
          { offset: 0, color: 'rgba(64, 158, 255, 0.1)' },
          { offset: 1, color: 'rgba(64, 158, 255, 0.05)' },
        ]),
      },
    });
    series.push({
      name: 'LOWER',
      type: 'line',
      data: boll.lower,
      smooth: true,
      lineStyle: { opacity: 0.3, width: 1 },
      itemStyle: { color: '#409eff' },
    });
  }

  // MACD指标
  if (selectedIndicators.value.includes('macd')) {
    grid.push({
      left: '10%',
      right: '8%',
      top: '55%',
      height: '15%',
    });
    xAxis.push({
      type: 'category',
      gridIndex: 1,
      data: dates,
      axisLabel: { show: false },
      axisLine: { show: false },
      axisTick: { show: false },
      splitLine: { show: false },
    });
    yAxis.push({
      scale: true,
      gridIndex: 1,
      splitNumber: 2,
      axisLabel: { show: false },
      axisLine: { show: false },
      axisTick: { show: false },
      splitLine: { show: false },
    });

    series.push({
      name: 'MACD',
      type: 'bar',
      xAxisIndex: 1,
      yAxisIndex: 1,
      data: macdData.macd.map((v, i) => ({
        value: [volumes[i], v],
        itemStyle: { color: v >= 0 ? colors.up : colors.down },
      })),
    });
    series.push({
      name: 'DIF',
      type: 'line',
      xAxisIndex: 1,
      yAxisIndex: 1,
      data: macdData.dif,
      lineStyle: { width: 1, color: '#ffffff' },
      symbol: 'none',
    });
    series.push({
      name: 'DEA',
      type: 'line',
      xAxisIndex: 1,
      yAxisIndex: 1,
      data: macdData.dea,
      lineStyle: { width: 1, color: '#ffff00' },
      symbol: 'none',
    });
  }

  // 成交量
  if (selectedIndicators.value.includes('vol')) {
    const volGridIndex = selectedIndicators.value.includes('macd') ? 2 : 1;
    grid.push({
      left: '10%',
      right: '8%',
      top: selectedIndicators.value.includes('macd') ? '75%' : '70%',
      height: '12%',
    });
    xAxis.push({
      type: 'category',
      gridIndex: volGridIndex,
      data: dates,
      axisLabel: { color: '#606266' },
      axisLine: { show: false },
      axisTick: { show: false },
      splitLine: { show: false },
    });
    yAxis.push({
      scale: true,
      gridIndex: volGridIndex,
      splitNumber: 2,
      axisLabel: { show: false },
      axisLine: { show: false },
      axisTick: { show: false },
      splitLine: { show: false },
    });

    series.push({
      name: 'Volume',
      type: 'bar',
      xAxisIndex: volGridIndex,
      yAxisIndex: volGridIndex,
      data: volumeData,
      itemStyle: {
        color: (params: any) => (params.data[2] === 1 ? colors.up : colors.down),
      },
    });
  }

  return {
    animation: false,
    legend: {
      bottom: 0,
      left: 'center',
      data: series.map((s: any) => s.name).filter((n: string) => n),
      textStyle: { color: '#606266', fontSize: 12 },
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: { type: 'cross' },
      backgroundColor: 'rgba(0, 0, 0, 0.8)',
      borderColor: '#409eff',
      borderWidth: 1,
      textStyle: { color: '#fff', fontSize: 12 },
      formatter: (params: any) => {
        const kline = params.find((p: any) => p.seriesName === 'K线');
        if (!kline) return '';

        const data = kline.data;
        const date = new Date(data[0]);
        const dateStr = date.toLocaleString('zh-CN', {
          month: '2-digit',
          day: '2-digit',
          hour: '2-digit',
          minute: '2-digit',
        });

        return `
          <div style="font-weight: 600; margin-bottom: 8px; color: #409eff;">${dateStr}</div>
          <div style="display: flex; justify-content: space-between; min-width: 140px;">
            <span style="color: #909399;">开盘:</span>
            <span style="color: ${data[1] <= data[2] ? colors.up : colors.down};">${data[1].toFixed(2)}</span>
          </div>
          <div style="display: flex; justify-content: space-between;">
            <span style="color: #909399;">收盘:</span>
            <span style="color: ${data[2] >= data[1] ? colors.up : colors.down};">${data[2].toFixed(2)}</span>
          </div>
          <div style="display: flex; justify-content: space-between;">
            <span style="color: #909399;">最低:</span>
            <span style="color: ${colors.down};">${data[3].toFixed(2)}</span>
          </div>
          <div style="display: flex; justify-content: space-between;">
            <span style="color: #909399;">最高:</span>
            <span style="color: ${colors.up};">${data[4].toFixed(2)}</span>
          </div>
        `;
      },
    },
    axisPointer: {
      link: [{ xAxisIndex: 'all' }],
      label: { backgroundColor: '#777' },
    },
    grid,
    xAxis,
    yAxis,
    dataZoom: [
      {
        type: 'inside',
        xAxisIndex: xAxis.map((_: any, i: number) => i),
        start: 0,
        end: 100,
      },
      {
        show: true,
        xAxisIndex: xAxis.map((_: any, i: number) => i),
        type: 'slider',
        bottom: selectedIndicators.value.includes('macd') ? '2%' : '5%',
        start: 0,
        end: 100,
        height: 20,
        borderColor: '#dcdfe6',
        fillerColor: 'rgba(64, 158, 255, 0.2)',
        handleStyle: { color: '#409eff' },
      },
    ],
    series,
  };
}

// 初始化图表
function initChart() {
  if (!chartRef.value) return;

  if (chartInstance) {
    chartInstance.dispose();
  }

  chartInstance = echarts.init(chartRef.value);
  chartInstance.setOption(createChartOption());

  window.addEventListener('resize', handleResize);
}

// 更新图表
function updateChart() {
  if (!chartInstance) {
    initChart();
    return;
  }
  chartInstance.setOption(createChartOption(), true);
}

// 刷新数据
async function refreshData() {
  await marketStore.loadKlines(currentSymbol.value, currentTimeframe.value, 500);
  updateChart();
}

// 切换时间周期
async function handleTimeframeChange(tf: string) {
  currentTimeframe.value = tf;
  await marketStore.loadKlines(currentSymbol.value, tf, 500);
  updateChart();
}

// 全屏切换
function toggleFullscreen() {
  if (!document.fullscreenElement) {
    chartRef.value?.requestFullscreen();
  } else {
    document.exitFullscreen();
  }
}

// 处理窗口大小变化
function handleResize() {
  chartInstance?.resize();
}

// Lifecycle
onMounted(async () => {
  await nextTick();

  if (props.autoLoad && chartData.value.length === 0) {
    await marketStore.loadKlines(currentSymbol.value, currentTimeframe.value, 500);
  }

  initChart();
});

onUnmounted(() => {
  if (chartInstance) {
    chartInstance.dispose();
    chartInstance = null;
  }
  window.removeEventListener('resize', handleResize);
});

// 监听变化
watch(
  () => [props.symbol, chartData.value],
  () => {
    updateChart();
  },
  { deep: true }
);
</script>

<style scoped lang="scss">
.kline-chart {
  display: flex;
  flex-direction: column;
  background: #fff;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 2px 12px rgba(0, 0, 0, 0.1);
}

// 工具栏
.chart-toolbar {
  display: flex;
  align-items: center;
  gap: 20px;
  padding: 12px 16px;
  background: #f5f7fa;
  border-bottom: 1px solid #ebeef5;
  flex-wrap: wrap;

  .toolbar-group {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .toolbar-label {
    font-size: 13px;
    color: #606266;
    font-weight: 500;
  }
}

// 图表内容
.chart-content {
  flex: 1;
  position: relative;
  overflow: hidden;
}

.chart-container {
  width: 100%;
  height: 100%;
}

.chart-loading,
.chart-error,
.chart-empty {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  color: #909399;
}

.loading-spinner {
  .el-icon {
    color: #409eff;
  }
}

.loading-text {
  font-size: 14px;
}

.error-text {
  font-size: 14px;
  color: #f56c6c;
}

// 价格信息栏
.price-info-bar {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  background: #fafbfc;
  border-top: 1px solid #ebeef5;
  gap: 24px;
  overflow-x: auto;

  .price-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
    min-width: 100px;
  }

  .price-label {
    font-size: 12px;
    color: #909399;
  }

  .price-value {
    font-size: 16px;
    font-weight: 600;
    color: #303133;

    &.price-up {
      color: #ef5350;
    }

    &.price-down {
      color: #26a69a;
    }
  }
}

.text-up {
  color: #ef5350;
}

.text-down {
  color: #26a69a;
}
</style>
