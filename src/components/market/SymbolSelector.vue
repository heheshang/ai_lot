<template>
  <div class="symbol-selector">
    <el-select
      :model-value="modelValue"
      @update:model-value="handleChange"
      filterable
      placeholder="选择交易对"
      :loading="loading"
      clearable
    >
      <el-option
        v-for="symbol in symbols"
        :key="symbol"
        :label="symbol"
        :value="symbol"
      >
        <div class="symbol-option">
          <span class="symbol-label">{{ symbol }}</span>
          <span v-if="tickers.has(symbol)" class="symbol-price">
            {{ formatTickerPrice(tickers.get(symbol)!) }}
          </span>
        </div>
      </el-option>
    </el-select>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue';
import { useMarketStore } from '@/store/modules/market';
import type { Ticker } from '@/types';

// Props
interface Props {
  modelValue?: string;
}

defineProps<Props>();

// Emits
const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void;
}>();

// Store
const marketStore = useMarketStore();

// Computed
const symbols = computed(() => marketStore.symbols);
const tickers = computed(() => marketStore.tickers);
const loading = computed(() => marketStore.loading);

// Methods
function handleChange(value: string) {
  emit('update:modelValue', value);
}

function formatTickerPrice(ticker: Ticker): string {
  if (ticker.price >= 1000) {
    return ticker.price.toFixed(2);
  } else if (ticker.price >= 1) {
    return ticker.price.toFixed(4);
  } else {
    return ticker.price.toFixed(6);
  }
}

// Lifecycle
onMounted(async () => {
  if (symbols.value.length === 0) {
    await marketStore.loadSymbols();
  }
});
</script>

<style scoped>
.symbol-selector {
  display: inline-block;
}

:deep(.el-select) {
  width: 200px;
}

.symbol-option {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.symbol-label {
  font-weight: 600;
}

.symbol-price {
  color: var(--el-text-color-secondary);
  font-size: 0.9em;
}
</style>
