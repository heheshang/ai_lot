<template>
  <div class="position-list">
    <div class="position-list-header">
      <div class="summary">
        <span class="summary-label">持仓盈亏:</span>
        <span :class="getPnlClass(totalUnrealizedPnl)" class="summary-value">
          {{ formatPnl(totalUnrealizedPnl) }}
        </span>
        <span class="summary-divider">|</span>
        <span class="summary-label">已实现:</span>
        <span :class="getPnlClass(totalRealizedPnl)" class="summary-value">
          {{ formatPnl(totalRealizedPnl) }}
        </span>
      </div>
      <el-button
        type="primary"
        size="small"
        :icon="RefreshRight"
        :loading="loading"
        @click="handleRefresh"
      >
        刷新
      </el-button>
    </div>

    <el-table
      :data="displayedPositions"
      stripe
      height="100%"
      :empty-text="loading ? '加载中...' : '暂无持仓'"
      :row-class-name="getRowClassName"
    >
      <el-table-column prop="symbol" label="交易对" width="120">
        <template #default="{ row }">
          <span class="symbol">{{ row.symbol }}</span>
        </template>
      </el-table-column>

      <el-table-column prop="side" label="方向" width="80" align="center">
        <template #default="{ row }">
          <el-tag :type="row.side === 'long' ? 'success' : 'danger'" size="small">
            {{ row.side === 'long' ? '多头' : '空头' }}
          </el-tag>
        </template>
      </el-table-column>

      <el-table-column prop="quantity" label="数量" width="120" align="right">
        <template #default="{ row }">
          {{ formatQuantity(row.quantity, row.symbol) }}
        </template>
      </el-table-column>

      <el-table-column prop="entryPrice" label="开仓价" width="120" align="right">
        <template #default="{ row }">
          {{ formatPrice(row.entryPrice) }}
        </template>
      </el-table-column>

      <el-table-column prop="currentPrice" label="当前价" width="120" align="right">
        <template #default="{ row }">
          <span v-if="row.currentPrice">{{ formatPrice(row.currentPrice) }}</span>
          <span v-else class="text-muted">-</span>
        </template>
      </el-table-column>

      <el-table-column prop="unrealizedPnl" label="未实现盈亏" width="140" align="right">
        <template #default="{ row }">
          <span :class="getPnlClass(row.unrealizedPnl)">
            {{ formatPnl(row.unrealizedPnl) }}
          </span>
        </template>
      </el-table-column>

      <el-table-column prop="realizedPnl" label="已实现盈亏" width="140" align="right">
        <template #default="{ row }">
          <span :class="getPnlClass(row.realizedPnl)">
            {{ formatPnl(row.realizedPnl) }}
          </span>
        </template>
      </el-table-column>

      <el-table-column prop="openedAt" label="开仓时间" width="100" align="center">
        <template #default="{ row }">
          {{ formatTime(row.openedAt) }}
        </template>
      </el-table-column>

      <el-table-column label="操作" width="80" align="center" fixed="right">
        <template #default="{ row }">
          <el-button
            type="danger"
            size="small"
            link
            @click="handleClose(row)"
          >
            平仓
          </el-button>
        </template>
      </el-table-column>
    </el-table>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { RefreshRight } from '@element-plus/icons-vue';
import { useTradeStore } from '@/store/modules/trade';
import type { Position } from '@/types';

// Props
interface Props {
  userId: string;
  symbol?: string;
  autoRefresh?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  symbol: undefined,
  autoRefresh: true,
});

// Emits
const emit = defineEmits<{
  (e: 'position-closed', position: Position): void;
}>();

// Store
const tradeStore = useTradeStore();

// State
const loading = ref(false);

// Computed
const displayedPositions = computed(() => {
  let positions = tradeStore.positions;

  // Filter by symbol if provided
  if (props.symbol) {
    positions = positions.filter(p => p.symbol === props.symbol);
  }

  // Sort by unrealized PnL (absolute value, highest first)
  return positions.sort((a, b) =>
    Math.abs(b.unrealizedPnl) - Math.abs(a.unrealizedPnl)
  );
});

const totalUnrealizedPnl = computed(() => {
  return displayedPositions.value.reduce((sum, p) => sum + p.unrealizedPnl, 0);
});

const totalRealizedPnl = computed(() => {
  return displayedPositions.value.reduce((sum, p) => sum + p.realizedPnl, 0);
});

// Methods
function formatPrice(price: number): string {
  if (price >= 1000) return price.toFixed(2);
  if (price >= 1) return price.toFixed(4);
  return price.toFixed(6);
}

function formatQuantity(quantity: number, symbol: string): string {
  if (symbol.includes('USDT')) {
    return quantity.toFixed(4);
  }
  return quantity.toFixed(6);
}

function formatPnl(pnl: number): string {
  const sign = pnl >= 0 ? '+$' : '-$';
  return `${sign}${Math.abs(pnl).toFixed(2)}`;
}

function formatTime(timestamp: number): string {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - date.getTime();

  if (diff < 3600000) return `${Math.floor(diff / 60000)}分钟前`;
  if (diff < 86400000) return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
  return date.toLocaleDateString('zh-CN', { month: '2-digit', day: '2-digit' });
}

function getPnlClass(pnl: number): string {
  if (pnl > 0) return 'pnl-profit';
  if (pnl < 0) return 'pnl-loss';
  return 'pnl-neutral';
}

function getRowClassName({ row }: { row: Position }): string {
  if (row.unrealizedPnl > 0) return 'position-profit';
  if (row.unrealizedPnl < 0) return 'position-loss';
  return '';
}

async function handleRefresh() {
  loading.value = true;
  try {
    await tradeStore.fetchPositions(props.userId);
    ElMessage.success('持仓列表已刷新');
  } catch (error) {
    ElMessage.error('刷新失败：' + (error as Error).message);
  } finally {
    loading.value = false;
  }
}

async function handleClose(position: Position) {
  try {
    await ElMessageBox.confirm(
      `确定要平仓 ${position.symbol} (${position.side === 'long' ? '多头' : '空头'}) 吗？`,
      '平仓确认',
      { type: 'warning' }
    );

    await tradeStore.closePosition(
      props.userId,
      position.symbol,
      position.side,
      position.quantity
    );

    ElMessage.success('平仓成功');
    emit('position-closed', position);
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('平仓失败：' + (error as Error).message);
    }
  }
}

// Lifecycle
onMounted(async () => {
  await handleRefresh();
});

// Watch for symbol changes
watch(() => props.symbol, async () => {
  if (props.autoRefresh) {
    await handleRefresh();
  }
});
</script>

<style scoped>
.position-list {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.position-list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background-color: var(--el-fill-color-light);
  border-bottom: 1px solid var(--el-border-color-light);
}

.summary {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
}

.summary-label {
  color: var(--el-text-color-secondary);
}

.summary-value {
  font-weight: 600;
  font-size: 15px;
}

.summary-divider {
  color: var(--el-border-color);
  margin: 0 4px;
}

:deep(.el-table) {
  flex: 1;
}

:deep(.el-table__body-wrapper) {
  overflow-y: auto;
}

.symbol {
  font-weight: 600;
  color: var(--el-text-color-primary);
}

.pnl-profit {
  color: #67c23a;
  font-weight: 600;
}

.pnl-loss {
  color: #f56c6c;
  font-weight: 600;
}

.pnl-neutral {
  color: var(--el-text-color-secondary);
}

.text-muted {
  color: var(--el-text-color-placeholder);
}

:deep(.position-profit) {
  background-color: rgba(103, 194, 58, 0.05);
}

:deep(.position-loss) {
  background-color: rgba(245, 108, 108, 0.05);
}
</style>
