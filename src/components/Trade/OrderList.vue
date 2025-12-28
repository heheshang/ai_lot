<template>
  <div class="order-list">
    <div class="order-list-header">
      <el-radio-group v-model="statusFilter" size="small" @change="handleFilterChange">
        <el-radio-button value="all">全部</el-radio-button>
        <el-radio-button value="active">活跃</el-radio-button>
        <el-radio-button value="completed">已完成</el-radio-button>
      </el-radio-group>
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
      :data="displayedOrders"
      stripe
      height="100%"
      :empty-text="loading ? '加载中...' : '暂无订单'"
      :row-class-name="getRowClassName"
    >
      <el-table-column prop="id" label="订单ID" width="180" show-overflow-tooltip>
        <template #default="{ row }">
          <span class="order-id">{{ row.id.slice(0, 12) }}...</span>
        </template>
      </el-table-column>

      <el-table-column prop="symbol" label="交易对" width="120">
        <template #default="{ row }">
          <span class="symbol">{{ row.symbol }}</span>
        </template>
      </el-table-column>

      <el-table-column prop="side" label="方向" width="80" align="center">
        <template #default="{ row }">
          <el-tag :type="row.side === OrderSide.BUY ? 'success' : 'danger'" size="small">
            {{ row.side === OrderSide.BUY ? '买入' : '卖出' }}
          </el-tag>
        </template>
      </el-table-column>

      <el-table-column prop="type" label="类型" width="80" align="center">
        <template #default="{ row }">
          <span>{{ getOrderTypeLabel(row.type) }}</span>
        </template>
      </el-table-column>

      <el-table-column prop="quantity" label="数量" width="120" align="right">
        <template #default="{ row }">
          {{ formatQuantity(row.quantity, row.symbol) }}
        </template>
      </el-table-column>

      <el-table-column prop="price" label="价格" width="120" align="right">
        <template #default="{ row }">
          <span v-if="row.type === OrderType.MARKET">市价</span>
          <span v-else>{{ formatPrice(row.price) }}</span>
        </template>
      </el-table-column>

      <el-table-column prop="filledQuantity" label="成交" width="100" align="right">
        <template #default="{ row }">
          {{ formatQuantity(row.filledQuantity, row.symbol) }}
        </template>
      </el-table-column>

      <el-table-column prop="avgPrice" label="均价" width="120" align="right">
        <template #default="{ row }">
          <span v-if="row.avgPrice">{{ formatPrice(row.avgPrice) }}</span>
          <span v-else>-</span>
        </template>
      </el-table-column>

      <el-table-column prop="status" label="状态" width="100" align="center">
        <template #default="{ row }">
          <el-tag :type="getStatusType(row.status)" size="small">
            {{ getStatusLabel(row.status) }}
          </el-tag>
        </template>
      </el-table-column>

      <el-table-column prop="createdAt" label="时间" width="100" align="center">
        <template #default="{ row }">
          {{ formatTime(row.createdAt) }}
        </template>
      </el-table-column>

      <el-table-column label="操作" width="80" align="center" fixed="right">
        <template #default="{ row }">
          <el-button
            v-if="canCancelOrder(row)"
            type="danger"
            size="small"
            link
            @click="handleCancel(row)"
          >
            撤单
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
import type { Order } from '@/types';
import { OrderSide, OrderType, OrderStatus } from '@/types';

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
  (e: 'order-canceled', orderId: string): void;
}>();

// Store
const tradeStore = useTradeStore();

// State
const statusFilter = ref<'all' | 'active' | 'completed'>('all');
const loading = ref(false);

// Computed
const displayedOrders = computed(() => {
  let orders = tradeStore.orders;

  // Filter by symbol if provided
  if (props.symbol) {
    orders = orders.filter(o => o.symbol === props.symbol);
  }

  // Filter by status
  if (statusFilter.value === 'active') {
    orders = orders.filter(o =>
      o.status === OrderStatus.OPEN ||
      o.status === OrderStatus.PARTIALLY_FILLED ||
      o.status === OrderStatus.PENDING
    );
  } else if (statusFilter.value === 'completed') {
    orders = orders.filter(o =>
      o.status === OrderStatus.FILLED ||
      o.status === OrderStatus.CANCELED ||
      o.status === OrderStatus.REJECTED
    );
  }

  // Sort by created time (newest first)
  return orders.sort((a, b) => b.createdAt - a.createdAt);
});

// Methods
function formatPrice(price?: number): string {
  if (!price) return '-';
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

function formatTime(timestamp: number): string {
  const date = new Date(timestamp);
  const now = new Date();
  const diff = now.getTime() - date.getTime();

  if (diff < 60000) return '刚刚';
  if (diff < 3600000) return `${Math.floor(diff / 60000)}分钟前`;
  if (diff < 86400000) return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
  return date.toLocaleDateString('zh-CN', { month: '2-digit', day: '2-digit' });
}

function getOrderTypeLabel(type: OrderType): string {
  switch (type) {
    case OrderType.MARKET: return '市价';
    case OrderType.LIMIT: return '限价';
    default: return type;
  }
}

function getStatusLabel(status: OrderStatus): string {
  switch (status) {
    case OrderStatus.PENDING: return '待提交';
    case OrderStatus.OPEN: return '挂单';
    case OrderStatus.PARTIALLY_FILLED: return '部分成交';
    case OrderStatus.FILLED: return '已成交';
    case OrderStatus.CANCELED: return '已撤销';
    case OrderStatus.REJECTED: return '已拒绝';
    default: return status;
  }
}

function getStatusType(status: OrderStatus): string {
  switch (status) {
    case OrderStatus.OPEN:
    case OrderStatus.PARTIALLY_FILLED:
      return 'warning';
    case OrderStatus.FILLED:
      return 'success';
    case OrderStatus.CANCELED:
    case OrderStatus.REJECTED:
      return 'info';
    default:
      return '';
  }
}

function getRowClassName({ row }: { row: Order }): string {
  if (row.status === OrderStatus.FILLED) return 'order-filled';
  if (row.status === OrderStatus.CANCELED) return 'order-canceled';
  return '';
}

function canCancelOrder(order: Order): boolean {
  return order.status === OrderStatus.OPEN ||
         order.status === OrderStatus.PARTIALLY_FILLED ||
         order.status === OrderStatus.PENDING;
}

async function handleRefresh() {
  loading.value = true;
  try {
    await tradeStore.fetchOrders(props.userId);
    ElMessage.success('订单列表已刷新');
  } catch (error) {
    ElMessage.error('刷新失败：' + (error as Error).message);
  } finally {
    loading.value = false;
  }
}

async function handleCancel(order: Order) {
  try {
    await ElMessageBox.confirm(
      `确定要撤销订单 ${order.id.slice(0, 12)}... 吗？`,
      '撤销订单',
      { type: 'warning' }
    );

    await tradeStore.cancelOrder(props.userId, order.id);
    ElMessage.success('撤单成功');
    emit('order-canceled', order.id);
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('撤单失败：' + (error as Error).message);
    }
  }
}

function handleFilterChange() {
  // Filter changes handled by computed
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
.order-list {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.order-list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px;
  background-color: var(--el-fill-color-light);
  border-bottom: 1px solid var(--el-border-color-light);
}

:deep(.el-table) {
  flex: 1;
}

:deep(.el-table__body-wrapper) {
  overflow-y: auto;
}

.order-id {
  font-family: 'Monaco', 'Consolas', monospace;
  font-size: 12px;
  color: var(--el-text-color-secondary);
}

.symbol {
  font-weight: 600;
  color: var(--el-text-color-primary);
}

:deep(.order-filled) {
  opacity: 0.6;
}

:deep(.order-canceled) {
  opacity: 0.5;
  text-decoration: line-through;
}
</style>
