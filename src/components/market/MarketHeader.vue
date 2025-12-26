<template>
  <div class="market-header">
    <div class="header-left">
      <h2 class="title">行情数据</h2>

      <!-- Connection Status -->
      <div class="status-section">
        <el-tag
          :type="isConnected ? 'success' : 'info'"
          size="small"
          :icon="isConnected ? Connection : Loading"
        >
          {{ isConnected ? '已连接' : '未连接' }}
        </el-tag>
        <span v-if="marketStatus" class="status-info">
          {{ marketStatus.exchangesCount }} 个交易所 |
          {{ marketStatus.subscriptionsCount }} 个订阅
        </span>
      </div>
    </div>

    <div class="header-center">
      <!-- Symbol Selector -->
      <SymbolSelector
        v-model="currentSymbol"
        @update:model-value="handleSymbolChange"
      />

      <!-- Timeframe Selector -->
      <el-button-group class="timeframe-group">
        <el-button
          v-for="tf in timeframes"
          :key="tf.value"
          :type="currentTimeframe === tf.value ? 'primary' : ''"
          size="small"
          @click="handleTimeframeChange(tf.value)"
        >
          {{ tf.label }}
        </el-button>
      </el-button-group>
    </div>

    <div class="header-right">
      <!-- Refresh Button -->
      <el-button
        :icon="Refresh"
        :loading="loading"
        circle
        @click="handleRefresh"
      />

      <!-- Settings Button -->
      <el-dropdown trigger="click">
        <el-button :icon="Setting" circle />
        <template #dropdown>
          <el-dropdown-menu>
            <el-dropdown-item @click="handleSubscribeAll">
              订阅全部交易对
            </el-dropdown-item>
            <el-dropdown-item @click="handleUnsubscribeAll">
              取消所有订阅
            </el-dropdown-item>
            <el-dropdown-item divided @click="handleClearData">
              清除缓存数据
            </el-dropdown-item>
          </el-dropdown-menu>
        </template>
      </el-dropdown>
    </div>
  </div>

  <!-- Current Ticker Info -->
  <div v-if="currentTicker" class="ticker-info">
    <div class="ticker-main">
      <span class="ticker-symbol">{{ currentTicker.symbol }}</span>
      <span class="ticker-price" :class="getPriceClass()">
        {{ formatPrice(currentTicker.price) }}
      </span>
      <span class="ticker-change" :class="getChangeClass()">
        {{ formatChange(currentTicker.priceChange, currentTicker.priceChangePercent) }}
      </span>
    </div>
    <div class="ticker-details">
      <span>24h最高: {{ formatPrice(currentTicker.high24h) }}</span>
      <span>24h最低: {{ formatPrice(currentTicker.low24h) }}</span>
      <span>24h成交量: {{ formatVolume(currentTicker.volume24h) }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { Connection, Loading, Refresh, Setting } from '@element-plus/icons-vue';
import { useMarketStore } from '@/store/modules/market';
import { ElMessage } from 'element-plus';
import SymbolSelector from './SymbolSelector.vue';

// Store
const marketStore = useMarketStore();

// Timeframes
const timeframes = [
  { label: '1m', value: '1m' },
  { label: '5m', value: '5m' },
  { label: '15m', value: '15m' },
  { label: '1h', value: '1h' },
  { label: '4h', value: '4h' },
  { label: '1d', value: '1d' },
];

// Computed
const currentSymbol = computed({
  get: () => marketStore.currentSymbol,
  set: (val) => marketStore.setCurrentSymbol(val),
});

const currentTimeframe = computed({
  get: () => marketStore.currentTimeframe,
  set: (val) => marketStore.setCurrentTimeframe(val),
});

const currentTicker = computed(() => marketStore.currentTicker);
const marketStatus = computed(() => marketStore.marketStatus);
const isConnected = computed(() => marketStore.isConnected);
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

function formatChange(change: number, percent: number): string {
  const sign = change >= 0 ? '+' : '';
  return `${sign}${change.toFixed(2)} (${sign}${percent.toFixed(2)}%)`;
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

function getPriceClass(): string {
  if (!currentTicker.value) return '';
  if (currentTicker.value.priceChangePercent > 0) return 'price-up';
  if (currentTicker.value.priceChangePercent < 0) return 'price-down';
  return '';
}

function getChangeClass(): string {
  if (!currentTicker.value) return '';
  if (currentTicker.value.priceChangePercent > 0) return 'change-up';
  if (currentTicker.value.priceChangePercent < 0) return 'change-down';
  return '';
}

function handleSymbolChange(symbol: string) {
  currentSymbol.value = symbol;
}

function handleTimeframeChange(timeframe: string) {
  currentTimeframe.value = timeframe;
}

async function handleRefresh() {
  try {
    await Promise.all([
      marketStore.loadSymbols(),
      marketStore.loadKlines(currentSymbol.value, currentTimeframe.value),
      marketStore.getMarketStatus(),
    ]);
    ElMessage.success('刷新成功');
  } catch (error) {
    ElMessage.error('刷新失败');
  }
}

async function handleSubscribeAll() {
  try {
    await marketStore.subscribeTicker(marketStore.symbols.slice(0, 50));
    ElMessage.success('已订阅前50个交易对');
  } catch (error) {
    ElMessage.error('订阅失败');
  }
}

async function handleUnsubscribeAll() {
  try {
    const symbols = Array.from(marketStore.tickers.keys());
    if (symbols.length > 0) {
      await marketStore.unsubscribeTicker(symbols);
      ElMessage.success('已取消所有订阅');
    }
  } catch (error) {
    ElMessage.error('取消订阅失败');
  }
}

function handleClearData() {
  marketStore.clear();
  ElMessage.success('已清除缓存数据');
}

// Lifecycle
onMounted(async () => {
  await marketStore.initialize();
  await marketStore.subscribeTicker([currentSymbol.value]);
});
</script>

<style scoped>
.market-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  padding: 16px 0;
  border-bottom: 1px solid var(--el-border-color);
  margin-bottom: 16px;
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.title {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
}

.status-section {
  display: flex;
  align-items: center;
  gap: 12px;
}

.status-info {
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.header-center {
  display: flex;
  align-items: center;
  gap: 12px;
}

.timeframe-group {
  margin-left: 8px;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.ticker-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding: 12px 16px;
  background: var(--el-fill-color-light);
  border-radius: 4px;
  margin-bottom: 16px;
}

.ticker-main {
  display: flex;
  align-items: baseline;
  gap: 16px;
}

.ticker-symbol {
  font-size: 20px;
  font-weight: 600;
}

.ticker-price {
  font-size: 24px;
  font-weight: 600;
}

.ticker-change {
  font-size: 14px;
  font-weight: 600;
}

.ticker-details {
  display: flex;
  gap: 24px;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.price-up,
.change-up {
  color: #f56c6c;
}

.price-down,
.change-down {
  color: #67c23a;
}
</style>
