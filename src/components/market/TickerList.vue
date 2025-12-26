<template>
  <div class="ticker-list">
    <el-table
      :data="tickerData"
      stripe
      height="100%"
      :empty-text="loading ? '加载中...' : '暂无数据'"
      @row-click="handleRowClick"
      :row-class-name="getRowClassName"
    >
      <el-table-column prop="symbol" label="交易对" width="120">
        <template #default="{ row }">
          <span class="symbol">{{ row.symbol }}</span>
        </template>
      </el-table-column>

      <el-table-column prop="price" label="最新价" width="120" align="right">
        <template #default="{ row }">
          <span :class="getPriceClass(row)">
            {{ formatPrice(row.price) }}
          </span>
        </template>
      </el-table-column>

      <el-table-column prop="priceChangePercent" label="涨跌幅" width="100" align="right">
        <template #default="{ row }">
          <span :class="getChangeClass(row)">
            {{ formatPercent(row.priceChangePercent) }}
          </span>
        </template>
      </el-table-column>

      <el-table-column prop="high24h" label="24h最高" width="120" align="right">
        <template #default="{ row }">
          {{ formatPrice(row.high24h) }}
        </template>
      </el-table-column>

      <el-table-column prop="low24h" label="24h最低" width="120" align="right">
        <template #default="{ row }">
          {{ formatPrice(row.low24h) }}
        </template>
      </el-table-column>

      <el-table-column prop="volume24h" label="24h成交量" width="140" align="right">
        <template #default="{ row }">
          {{ formatVolume(row.volume24h) }}
        </template>
      </el-table-column>

      <el-table-column prop="timestamp" label="更新时间" width="100" align="center">
        <template #default="{ row }">
          {{ formatTime(row.timestamp) }}
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue';
import { useMarketStore } from '@/store/modules/market';
import type { Ticker } from '@/types';

// Props
interface Props {
  symbols?: string[];
  autoSubscribe?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  symbols: undefined,
  autoSubscribe: true,
});

// Emits
const emit = defineEmits<{
  (e: 'select', symbol: string): void;
}>();

// Store
const marketStore = useMarketStore();

// Computed
const tickerData = computed(() => {
  const tickers = Array.from(marketStore.tickers.values());

  if (props.symbols && props.symbols.length > 0) {
    return tickers.filter(t => props.symbols!.includes(t.symbol));
  }

  return tickers.sort((a, b) => a.symbol.localeCompare(b.symbol));
});

const loading = computed(() => marketStore.loading);

// Methods
function formatPrice(price: number): string {
  if (price >= 1000) {
    return price.toFixed(2);
  } else if (price >= 1) {
    return price.toFixed(4);
  } else {
    return price.toFixed(6);
  }
}

function formatPercent(percent: number): string {
  const sign = percent >= 0 ? '+' : '';
  return `${sign}${percent.toFixed(2)}%`;
}

function formatVolume(volume: number): string {
  if (volume >= 1e9) {
    return `${(volume / 1e9).toFixed(2)}B`;
  } else if (volume >= 1e6) {
    return `${(volume / 1e6).toFixed(2)}M`;
  } else if (volume >= 1e3) {
    return `${(volume / 1e3).toFixed(2)}K`;
  }
  return volume.toFixed(2);
}

function formatTime(timestamp: number): string {
  if (!timestamp) return '-';
  const date = new Date(timestamp);
  return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
}

function getPriceClass(ticker: Ticker): string {
  if (ticker.priceChangePercent > 0) return 'price-up';
  if (ticker.priceChangePercent < 0) return 'price-down';
  return '';
}

function getChangeClass(ticker: Ticker): string {
  if (ticker.priceChangePercent > 0) return 'change-up';
  if (ticker.priceChangePercent < 0) return 'change-down';
  return '';
}

function getRowClassName({ row }: { row: Ticker }): string {
  if (row.symbol === marketStore.currentSymbol) {
    return 'current-row';
  }
  return '';
}

function handleRowClick(row: Ticker) {
  emit('select', row.symbol);
}

// Lifecycle
onMounted(async () => {
  // Load symbols if not loaded
  if (marketStore.symbols.length === 0) {
    await marketStore.loadSymbols();
  }

  // Subscribe to ticker updates
  if (props.autoSubscribe) {
    const symbolsToSubscribe = props.symbols || marketStore.symbols.slice(0, 20);
    if (symbolsToSubscribe.length > 0) {
      await marketStore.subscribeTicker(symbolsToSubscribe);
    }
  }
});

onUnmounted(() => {
  // Unsubscribe if using specific symbols
  if (props.symbols && props.symbols.length > 0) {
    marketStore.unsubscribeTicker(props.symbols);
  }
});
</script>

<style scoped>
.ticker-list {
  height: 100%;
}

:deep(.el-table) {
  height: 100%;
}

:deep(.el-table__body-wrapper) {
  height: calc(100% - 40px);
  overflow-y: auto;
}

.symbol {
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.price-up {
  color: #f56c6c;
}

.price-down {
  color: #67c23a;
}

.change-up {
  color: #f56c6c;
  font-weight: 600;
}

.change-down {
  color: #67c23a;
  font-weight: 600;
}

:deep(.current-row) {
  background-color: var(--el-color-primary-light-9) !important;
}

:deep(.el-table__row) {
  cursor: pointer;
}

:deep(.el-table__row:hover) {
  background-color: var(--el-fill-color-light);
}
</style>
