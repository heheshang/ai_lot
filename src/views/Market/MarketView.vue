<template>
  <div class="market-view">
    <MarketHeader />

    <div class="market-content">
      <!-- Left: Ticker List -->
      <div class="ticker-section">
        <el-card class="ticker-card">
          <template #header>
            <div class="card-header">
              <span>行情列表</span>
              <el-badge :value="tickerCount" :max="99" type="primary" />
            </div>
          </template>
          <div class="ticker-list-wrapper">
            <TickerList
              :symbols="displaySymbols"
              @select="handleSelectSymbol"
            />
          </div>
        </el-card>
      </div>

      <!-- Right: Chart Section -->
      <div class="chart-section">
        <el-card class="chart-card">
          <template #header>
            <div class="card-header">
              <span>K线图表</span>
              <span class="subtitle">{{ currentSymbol }} - {{ currentTimeframe }}</span>
            </div>
          </template>
          <div class="chart-wrapper">
            <KlineChart :symbol="currentSymbol" :timeframe="currentTimeframe" />
          </div>
        </el-card>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue';
import { useMarketStore } from '@/store/modules/market';
import { useMarketSubscription } from '@/composables/useMarketEvents';
import MarketHeader from '@/components/market/MarketHeader.vue';
import TickerList from '@/components/market/TickerList.vue';
import KlineChart from '@/components/market/KlineChart.vue';

// Store
const marketStore = useMarketStore();

// Computed
const currentSymbol = computed(() => marketStore.currentSymbol);
const currentTimeframe = computed(() => marketStore.currentTimeframe);
const tickerCount = computed(() => marketStore.tickers.size);

// Display top 50 symbols by default
const displaySymbols = computed(() => {
  return marketStore.symbols.slice(0, 50);
});

// Real-time subscription
const { startSubscription, stopSubscription } = useMarketSubscription(
  undefined, // Use default symbols
  false // Don't auto-init, we'll do it manually
);

// Methods
function handleSelectSymbol(symbol: string) {
  marketStore.setCurrentSymbol(symbol);
}

// Lifecycle
onMounted(async () => {
  // Initialize market data
  await marketStore.initialize();

  // Start real-time subscription
  await startSubscription();
});

onUnmounted(() => {
  stopSubscription();
});
</script>

<style scoped>
.market-view {
  height: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.market-content {
  flex: 1;
  display: flex;
  gap: 16px;
  overflow: hidden;
  padding-bottom: 16px;
}

.ticker-section {
  flex: 0 0 400px;
  display: flex;
  flex-direction: column;
}

.ticker-card {
  height: 100%;
  display: flex;
  flex-direction: column;
}

:deep(.ticker-card .el-card__body) {
  flex: 1;
  padding: 0;
  overflow: hidden;
}

.ticker-list-wrapper {
  height: 100%;
}

.chart-section {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.chart-card {
  height: 100%;
  display: flex;
  flex-direction: column;
}

:deep(.chart-card .el-card__body) {
  flex: 1;
  padding: 0;
  overflow: hidden;
}

.card-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;
}

.subtitle {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  font-weight: normal;
}

.chart-wrapper {
  width: 100%;
  height: 100%;
}
</style>
