<template>
  <div class="order-form">
    <el-card shadow="never">
      <template #header>
        <div class="card-header">
          <span>手动下单</span>
          <el-tag :type="accountBalanceLoading ? 'info' : 'success'" size="small">
            余额: {{ accountBalanceLoading ? '加载中...' : formatBalance(accountBalance) }}
          </el-tag>
        </div>
      </template>

      <el-form
        ref="formRef"
        :model="form"
        :rules="rules"
        label-width="80px"
        label-position="left"
        @submit.prevent="handleSubmit"
      >
        <!-- 交易对选择 -->
        <el-form-item label="交易对" prop="symbol">
          <el-select
            v-model="form.symbol"
            filterable
            placeholder="选择交易对"
            style="width: 100%"
          >
            <el-option
              v-for="symbol in symbols"
              :key="symbol"
              :label="symbol"
              :value="symbol"
            >
              <span class="symbol-option">{{ symbol }}</span>
            </el-option>
          </el-select>
        </el-form-item>

        <!-- 买卖方向 -->
        <el-form-item label="方向" prop="side">
          <el-radio-group v-model="form.side" size="large" style="width: 100%">
            <el-radio-button
              value="buy"
              class="side-button buy-button"
            >
              买入
            </el-radio-button>
            <el-radio-button
              value="sell"
              class="side-button sell-button"
            >
              卖出
            </el-radio-button>
          </el-radio-group>
        </el-form-item>

        <!-- 订单类型 -->
        <el-form-item label="类型" prop="type">
          <el-select v-model="form.type" style="width: 100%">
            <el-option label="市价单" value="market" />
            <el-option label="限价单" value="limit" />
          </el-select>
        </el-form-item>

        <!-- 价格（仅限价单） -->
        <el-form-item
          v-if="form.type === 'limit'"
          label="价格"
          prop="price"
        >
          <el-input-number
            v-model="form.price"
            :min="0"
            :precision="getPricePrecision(form.symbol)"
            :step="getPriceStep(form.symbol)"
            :style="{ width: '100%' }"
            :controls-position="'right'"
            placeholder="输入价格"
          />
          <div class="form-hint">
            当前价格: {{ currentPrice ? formatPrice(currentPrice) : '-' }}
          </div>
        </el-form-item>

        <!-- 数量 -->
        <el-form-item label="数量" prop="quantity">
          <el-input-number
            v-model="form.quantity"
            :min="getMinQuantity(form.symbol)"
            :max="getMaxQuantity(form.symbol)"
            :precision="getQuantityPrecision(form.symbol)"
            :step="getQuantityStep(form.symbol)"
            :style="{ width: '100%' }"
            :controls-position="'right'"
            placeholder="输入数量"
          />
          <div class="form-hint">
            可用: {{ form.side === 'buy' ? getBuyAvailable() : getSellAvailable() }}
          </div>
        </el-form-item>

        <!-- 快速选择数量 -->
        <el-form-item label="快捷">
          <div class="quick-amounts">
            <el-button
              v-for="percent in [25, 50, 75, 100]"
              :key="percent"
              size="small"
              @click="handleQuickAmount(percent)"
            >
              {{ percent }}%
            </el-button>
          </div>
        </el-form-item>

        <!-- 订单预览 -->
        <el-form-item label="预估">
          <div class="order-preview">
            <div class="preview-row">
              <span class="preview-label">订单价值:</span>
              <span class="preview-value">{{ formatOrderValue() }}</span>
            </div>
            <div class="preview-row">
              <span class="preview-label">手续费:</span>
              <span class="preview-value">{{ formatEstCommission() }}</span>
            </div>
          </div>
        </el-form-item>

        <!-- 提交按钮 -->
        <el-form-item>
          <el-button
            type="primary"
            :style="{ width: '100%' }"
            :loading="submitting"
            :class="{
              'buy-button-bg': form.side === 'buy',
              'sell-button-bg': form.side === 'sell'
            }"
            @click="handleSubmit"
          >
            {{ form.side === 'buy' ? '买入 ' : '卖出 ' }}
            {{ form.symbol || '-' }}
          </el-button>
        </el-form-item>
      </el-form>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue';
import { ElMessage, type FormInstance, type FormRules } from 'element-plus';
import { useTradeStore } from '@/store/modules/trade';
import { useMarketStore } from '@/store/modules/market';
import type { OrderSide, OrderType } from '@/types';

// Props
interface Props {
  userId: string;
  defaultSymbol?: string;
}

const props = withDefaults(defineProps<Props>(), {
  defaultSymbol: 'BTCUSDT',
});

// Emits
const emit = defineEmits<{
  (e: 'order-placed', order: any): void;
}>();

// Stores
const tradeStore = useTradeStore();
const marketStore = useMarketStore();

// Refs
const formRef = ref<FormInstance>();

// State
const submitting = ref(false);
const accountBalance = ref(0);
const accountBalanceLoading = ref(false);

// Form
const form = reactive({
  symbol: props.defaultSymbol,
  side: 'buy' as OrderSide,
  type: 'market' as OrderType,
  price: 0,
  quantity: 0,
});

// Validation rules
const rules: FormRules = {
  symbol: [{ required: true, message: '请选择交易对', trigger: 'change' }],
  side: [{ required: true, message: '请选择方向', trigger: 'change' }],
  type: [{ required: true, message: '请选择订单类型', trigger: 'change' }],
  price: [
    {
      validator: (_rule, value, callback) => {
        if (form.type === 'limit' && (!value || value <= 0)) {
          callback(new Error('请输入有效价格'));
        } else {
          callback();
        }
      },
      trigger: 'blur'
    }
  ],
  quantity: [
    { required: true, message: '请输入数量', trigger: 'blur' },
    {
      validator: (_rule, value, callback) => {
        if (!value || value <= 0) {
          callback(new Error('请输入有效数量'));
        } else {
          callback();
        }
      },
      trigger: 'blur'
    }
  ],
};

// Computed
const symbols = computed(() => marketStore.symbols);

const currentPrice = computed(() => {
  if (!form.symbol) return 0;
  const ticker = marketStore.tickers.get(form.symbol);
  return ticker?.price || 0;
});

const currentPosition = computed(() => {
  if (!form.symbol) return null;
  return tradeStore.positions.find(p => p.symbol === form.symbol);
});

// Methods
function formatBalance(balance: number): string {
  return `$${balance.toFixed(2)}`;
}

function formatPrice(price: number): string {
  if (price >= 1000) return price.toFixed(2);
  if (price >= 1) return price.toFixed(4);
  return price.toFixed(6);
}

function getPricePrecision(symbol: string): number {
  if (!symbol) return 2;
  if (symbol.includes('USDT')) {
    if (symbol.startsWith('BTC')) return 2;
    if (symbol.startsWith('ETH')) return 2;
    return 4;
  }
  return 6;
}

function getPriceStep(symbol: string): number {
  if (!symbol) return 0.01;
  if (symbol.includes('USDT')) {
    if (symbol.startsWith('BTC')) return 1;
    if (symbol.startsWith('ETH')) return 0.01;
    return 0.0001;
  }
  return 0.000001;
}

function getQuantityPrecision(symbol: string): number {
  if (!symbol) return 4;
  if (symbol.includes('USDT')) return 4;
  return 6;
}

function getQuantityStep(symbol: string): number {
  if (!symbol) return 0.0001;
  if (symbol.includes('USDT')) return 0.0001;
  return 0.000001;
}

function getMinQuantity(symbol: string): number {
  if (symbol?.includes('USDT')) return 0.0001;
  return 0.000001;
}

function getMaxQuantity(_symbol: string): number {
  // This would come from exchange info in a real implementation
  return 1000000;
}

function getBuyAvailable(): string {
  if (!currentPrice.value || currentPrice.value === 0) return '-';
  const available = accountBalance.value / currentPrice.value;
  return `${available.toFixed(6)} ${form.symbol?.replace('USDT', '') || ''}`;
}

function getSellAvailable(): string {
  if (!currentPosition.value || currentPosition.value.quantity === 0) return '0';
  return `${currentPosition.value.quantity.toFixed(6)} ${form.symbol?.replace('USDT', '') || ''}`;
}

function formatOrderValue(): string {
  if (!form.quantity) return '-';
  const price = form.type === 'limit' ? form.price : currentPrice.value;
  if (!price || price === 0) return '-';
  const value = form.quantity * price;
  return `$${value.toFixed(2)}`;
}

function formatEstCommission(): string {
  if (!form.quantity) return '-';
  const price = form.type === 'limit' ? form.price : currentPrice.value;
  if (!price || price === 0) return '-';
  const commission = form.quantity * price * 0.001; // 0.1% commission
  return `$${commission.toFixed(4)}`;
}

function handleQuickAmount(percent: number) {
  if (form.side === 'buy') {
    // Buy: use available balance
    if (!currentPrice.value || currentPrice.value === 0) {
      ElMessage.warning('当前价格不可用');
      return;
    }
    const available = accountBalance.value / currentPrice.value;
    form.quantity = (available * percent / 100);
  } else {
    // Sell: use current position
    if (!currentPosition.value || currentPosition.value.quantity === 0) {
      ElMessage.warning('无可用持仓');
      return;
    }
    form.quantity = (currentPosition.value.quantity * percent / 100);
  }
}

async function handleSubmit() {
  if (!formRef.value) return;

  try {
    await formRef.value.validate();
  } catch {
    return;
  }

  submitting.value = true;

  try {
    const orderRequest = {
      symbol: form.symbol,
      side: form.side,
      orderType: form.type,
      quantity: form.quantity,
      price: form.type === 'limit' ? form.price : undefined,
    };

    const order = await tradeStore.placeOrder(props.userId, orderRequest);

    ElMessage.success('下单成功');
    emit('order-placed', order);

    // Reset form
    form.quantity = 0;
    if (form.type === 'limit') {
      form.price = 0;
    }

    // Refresh orders
    await tradeStore.fetchOrders(props.userId);
  } catch (error) {
    ElMessage.error('下单失败：' + (error as Error).message);
  } finally {
    submitting.value = false;
  }
}

async function loadAccountBalance() {
  accountBalanceLoading.value = true;
  try {
    const balances = await tradeStore.getBalance();
    // Find USDT balance or use first available balance
    const usdtBalance = balances.find((b: any) => b.asset === 'USDT');
    accountBalance.value = usdtBalance ? parseFloat(usdtBalance.free || '0') : 0;
  } catch (error) {
    console.error('Failed to load balance:', error);
    accountBalance.value = 0;
  } finally {
    accountBalanceLoading.value = false;
  }
}

// Lifecycle
onMounted(async () => {
  // Load symbols
  if (marketStore.symbols.length === 0) {
    await marketStore.loadSymbols();
  }

  // Subscribe to ticker
  await marketStore.subscribeTicker([form.symbol]);

  // Load account balance
  await loadAccountBalance();

  // Load positions for sell available calculation
  await tradeStore.fetchPositions(props.userId);
});
</script>

<style scoped>
.order-form {
  height: 100%;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-weight: 600;
}

.form-hint {
  font-size: 12px;
  color: var(--el-text-color-secondary);
  margin-top: 4px;
}

.quick-amounts {
  display: flex;
  gap: 8px;
  width: 100%;
}

.quick-amounts .el-button {
  flex: 1;
}

.order-preview {
  background-color: var(--el-fill-color-light);
  padding: 12px;
  border-radius: 4px;
}

.preview-row {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
}

.preview-row:last-child {
  margin-bottom: 0;
}

.preview-label {
  color: var(--el-text-color-secondary);
}

.preview-value {
  font-weight: 600;
}

.side-button {
  flex: 1;
}

.side-button :deep(.el-radio-button__inner) {
  width: 100%;
  padding: 12px 20px;
  font-weight: 600;
}

.buy-button :deep(.el-radio-button__original:checked + .el-radio-button__inner) {
  background-color: #67c23a;
  border-color: #67c23a;
  color: white;
}

.sell-button :deep(.el-radio-button__original:checked + .el-radio-button__inner) {
  background-color: #f56c6c;
  border-color: #f56c6c;
  color: white;
}

.buy-button-bg {
  background-color: #67c23a;
  border-color: #67c23a;
}

.sell-button-bg {
  background-color: #f56c6c;
  border-color: #f56c6c;
}

.symbol-option {
  font-weight: 600;
}

:deep(.el-input-number) {
  width: 100%;
}

:deep(.el-input-number .el-input__inner) {
  text-align: left;
}
</style>
