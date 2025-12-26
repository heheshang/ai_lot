<template>
  <div class="kline-chart">
    <div v-if="loading" class="chart-loading">
      <el-icon class="is-loading" :size="32"><Loading /></el-icon>
      <span>加载中...</span>
    </div>
    <div v-else-if="error" class="chart-error">
      <el-icon :size="32"><WarningFilled /></el-icon>
      <span>{{ error }}</span>
    </div>
    <div v-else-if="chartData.length === 0" class="chart-empty">
      <el-empty description="暂无K线数据" />
    </div>
    <div v-else ref="chartRef" class="chart-container" />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue';
import * as echarts from 'echarts';
import { Loading, WarningFilled } from '@element-plus/icons-vue';
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
  height: '100%',
  autoLoad: true,
});

// Store
const marketStore = useMarketStore();

// Refs
const chartRef = ref<HTMLDivElement>();
let chartInstance: echarts.ECharts | null = null;

// Computed
const currentSymbol = computed(() => props.symbol || marketStore.currentSymbol);
const currentTimeframe = computed(() => props.timeframe || marketStore.currentTimeframe);
const chartData = computed(() => {
  if (props.symbol && props.timeframe) {
    // Use specific symbol/timeframe combination
    const key = `${props.symbol}_${props.timeframe}`;
    return marketStore.klines.get(key) || [];
  }
  // Use current selection
  return marketStore.currentKlines;
});
const loading = computed(() => marketStore.loading);
const error = computed(() => marketStore.error);

// Methods
function createChartOption(): EChartsOption {
  // Prepare data: [timestamp, open, close, low, high]
  const data = chartData.value.map((kline: Kline) => [
    kline.timestamp,
    kline.open,
    kline.close,
    kline.low,
    kline.high,
  ]);

  // Calculate min/max for y-axis
  const allValues = chartData.value.flatMap(k => [k.high, k.low]);
  const minValue = Math.min(...allValues);
  const maxValue = Math.max(...allValues);
  const padding = (maxValue - minValue) * 0.1;

  // Volume data
  const volumeData = chartData.value.map((kline: Kline) => [
    kline.timestamp,
    kline.volume,
    kline.close > kline.open ? 1 : -1, // Up or Down
  ]);

  // Get colors based on trend
  const upColor = '#ef5350';
  const downColor = '#26a69a';

  return {
    animation: false,
    legend: {
      bottom: 0,
      left: 'center',
      data: ['K线', '成交量'],
    },
    tooltip: {
      trigger: 'axis',
      axisPointer: {
        type: 'cross',
      },
      borderWidth: 1,
      borderColor: '#ccc',
      padding: 10,
      textStyle: {
        color: '#000',
      },
      position: function (pos: any, _params: any, _el: any, elRect: any, _dataSize: any) {
        const obj = { top: '10%' } as any;
        obj[['left', 'right'][+(pos[0] < elRect.width / 2)]] = 30;
        return obj;
      },
      formatter: function (params: any) {
        const kline = params[0];
        const volume = params[1];
        if (!kline || !kline.data) return '';

        const data = kline.data;
        const date = new Date(data[0]);
        const dateStr = date.toLocaleString('zh-CN', {
          year: 'numeric',
          month: '2-digit',
          day: '2-digit',
          hour: '2-digit',
          minute: '2-digit',
        });

        return `
          <div style="font-weight: 600; margin-bottom: 8px;">${dateStr}</div>
          <div>开盘: ${data[1].toFixed(4)}</div>
          <div>收盘: ${data[2].toFixed(4)}</div>
          <div>最低: ${data[3].toFixed(4)}</div>
          <div>最高: ${data[4].toFixed(4)}</div>
          <div>涨跌: ${(data[2] - data[1]).toFixed(4)}</div>
          <div>涨跌幅: ${((data[2] - data[1]) / data[1] * 100).toFixed(2)}%</div>
          <div>成交量: ${volume ? volume.data[1].toFixed(2) : 'N/A'}</div>
        `;
      },
    },
    axisPointer: {
      link: [{ xAxisIndex: 'all' }],
      label: {
        backgroundColor: '#777',
      },
    },
    grid: [
      {
        left: '10%',
        right: '8%',
        top: '10%',
        height: '50%',
      },
      {
        left: '10%',
        right: '8%',
        top: '70%',
        height: '15%',
      },
    ],
    xAxis: [
      {
        type: 'category',
        data: data.map((item: any) => {
          const date = new Date(item[0]);
          return date.toLocaleTimeString('zh-CN', {
            hour: '2-digit',
            minute: '2-digit',
          });
        }),
        boundaryGap: false,
        axisLine: { onZero: false },
        splitLine: { show: false },
        min: 'dataMin',
        max: 'dataMax',
        axisPointer: {
          z: 100,
        },
      },
      {
        type: 'category',
        gridIndex: 1,
        data: data.map((item: any) => {
          const date = new Date(item[0]);
          return date.toLocaleTimeString('zh-CN', {
            hour: '2-digit',
            minute: '2-digit',
          });
        }),
        boundaryGap: false,
        axisLine: { onZero: false },
        axisTick: { show: false },
        splitLine: { show: false },
        axisLabel: { show: false },
        min: 'dataMin',
        max: 'dataMax',
      },
    ],
    yAxis: [
      {
        scale: true,
        splitArea: {
          show: true,
        },
        min: minValue - padding,
        max: maxValue + padding,
        axisLabel: {
          formatter: function (value: number) {
            return value.toFixed(4);
          },
        },
      },
      {
        scale: true,
        gridIndex: 1,
        splitNumber: 2,
        axisLabel: { show: false },
        axisLine: { show: false },
        axisTick: { show: false },
        splitLine: { show: false },
      },
    ],
    dataZoom: [
      {
        type: 'inside',
        xAxisIndex: [0, 1],
        start: 0,
        end: 100,
      },
      {
        show: true,
        xAxisIndex: [0, 1],
        type: 'slider',
        top: '90%',
        start: 0,
        end: 100,
      },
    ],
    series: [
      {
        name: 'K线',
        type: 'candlestick',
        data: data,
        itemStyle: {
          color: upColor,
          color0: downColor,
          borderColor: upColor,
          borderColor0: downColor,
        },
      },
      {
        name: '成交量',
        type: 'bar',
        xAxisIndex: 1,
        yAxisIndex: 1,
        data: volumeData,
        itemStyle: {
          color: function (params: any) {
            return params.data[2] === 1 ? upColor : downColor;
          },
        },
      },
    ],
  };
}

function initChart() {
  if (!chartRef.value) return;

  // Dispose existing chart
  if (chartInstance) {
    chartInstance.dispose();
  }

  // Create new chart
  chartInstance = echarts.init(chartRef.value);
  chartInstance.setOption(createChartOption());

  // Resize handler
  window.addEventListener('resize', handleResize);
}

function updateChart() {
  if (!chartInstance) {
    initChart();
    return;
  }

  chartInstance.setOption(createChartOption(), true);
}

function handleResize() {
  if (chartInstance) {
    chartInstance.resize();
  }
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

// Watch for changes
watch(
  () => [props.symbol, props.timeframe, chartData.value],
  async () => {
    if (props.autoLoad) {
      const symbol = props.symbol || currentSymbol.value;
      const timeframe = props.timeframe || currentTimeframe.value;

      // Reload if props changed
      if (props.symbol || props.timeframe) {
        await marketStore.loadKlines(symbol, timeframe, 500);
      }
    }
    updateChart();
  },
  { deep: true }
);
</script>

<style scoped>
.kline-chart {
  width: 100%;
  height: v-bind(height);
  position: relative;
}

.chart-container {
  width: 100%;
  height: 100%;
  min-height: 400px;
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
  color: var(--el-text-color-secondary);
}
</style>
