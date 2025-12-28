<template>
  <div class="trade-console">
    <!-- Page Header -->
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">交易控制台</h2>
        <el-breadcrumb separator="/">
          <el-breadcrumb-item :to="{ path: '/' }">首页</el-breadcrumb-item>
          <el-breadcrumb-item>交易控制台</el-breadcrumb-item>
        </el-breadcrumb>
      </div>
      <div class="header-actions">
        <el-tag :type="balanceLoading ? 'info' : 'success'" size="large">
          账户余额: {{ balanceLoading ? '加载中...' : `$${accountBalance.toFixed(2)}` }}
        </el-tag>
        <el-tag type="info" size="large">
          未实现盈亏:
          <span :class="totalUnrealizedPnl >= 0 ? 'pnl-profit' : 'pnl-loss'">
            {{ totalUnrealizedPnl >= 0 ? '+$' : '-$' }}{{ Math.abs(totalUnrealizedPnl).toFixed(2) }}
          </span>
        </el-tag>
        <el-button
          type="danger"
          @click="closeAllPositions"
          :disabled="tradeStore.positions.length === 0"
          :loading="tradeStore.loading"
        >
          <el-icon><Close /></el-icon>
          全部平仓
        </el-button>
      </div>
    </div>

    <!-- Running Strategies Panel -->
    <el-card class="strategies-panel-card" shadow="never" v-if="runningInstances.length > 0">
      <template #header>
        <div class="card-header">
          <span>运行策略</span>
          <el-badge :value="runningInstances.length" :max="99" />
        </div>
      </template>
      <el-table
        :data="runningInstances"
        stripe
        :empty-text="strategyLoading ? '加载中...' : '暂无运行策略'"
        max-height="200"
      >
        <el-table-column prop="name" label="策略名称" width="150" />
        <el-table-column prop="symbols" label="交易对" width="100">
          <template #default="{ row }">
            {{ row.symbols?.[0] || '-' }}
          </template>
        </el-table-column>
        <el-table-column prop="status" label="状态" width="100" align="center">
          <template #default="{ row }">
            <el-tag :type="getInstanceStatusType(row.status)" size="small">
              {{ getInstanceStatusLabel(row.status) }}
            </el-tag>
          </template>
        </el-table-column>
        <el-table-column prop="timeframes" label="周期" width="80">
          <template #default="{ row }">
            {{ row.timeframes?.[0] || '-' }}
          </template>
        </el-table-column>
        <el-table-column label="操作" width="80" align="center">
          <template #default="{ row }">
            <el-button
              v-if="canStopInstance(row.status)"
              type="danger"
              size="small"
              link
              @click="handleStopInstance(row.id)"
            >
              停止
            </el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-card>

    <el-row :gutter="16" class="main-row" v-if="userStore.user">
      <!-- Left Column: Order Form -->
      <el-col :span="12">
        <el-card class="order-form-card" shadow="never">
          <OrderForm
            :user-id="userStore.user.id"
            default-symbol="BTCUSDT"
            @order-placed="handleOrderPlaced"
          />
        </el-card>
      </el-col>

      <!-- Right Column: Positions + Orders -->
      <el-col :span="12">
        <!-- Position List -->
        <el-card class="positions-card" shadow="never">
          <template #header>
            <div class="card-header">
              <span>当前持仓</span>
              <el-badge :value="tradeStore.positions.length" :max="99" />
            </div>
          </template>
          <div class="positions-list">
            <div v-if="tradeStore.positions.length === 0" class="empty-state">
              <el-empty description="暂无持仓" :image-size="60" />
            </div>
            <div v-for="pos in tradeStore.positions" :key="pos.id" class="position-item">
              <div class="position-header">
                <div class="position-symbol">{{ pos.symbol }}</div>
                <el-tag :type="pos.side === 'long' ? 'success' : 'danger'" size="small">
                  {{ pos.side === 'long' ? '多头' : '空头' }}
                </el-tag>
              </div>
              <div class="position-info">
                <div class="position-row">
                  <span class="row-label">数量</span>
                  <span class="row-value">{{ pos.quantity.toFixed(6) }}</span>
                </div>
                <div class="position-row">
                  <span class="row-label">开仓价</span>
                  <span class="row-value">{{ formatPrice(pos.entryPrice) }}</span>
                </div>
                <div class="position-row">
                  <span class="row-label">当前价</span>
                  <span class="row-value" :class="pos.unrealizedPnl >= 0 ? 'price-up' : 'price-down'">
                    {{ pos.currentPrice ? formatPrice(pos.currentPrice) : '-' }}
                  </span>
                </div>
                <div class="position-row">
                  <span class="row-label">未实现盈亏</span>
                  <span class="row-value" :class="pos.unrealizedPnl >= 0 ? 'profit' : 'loss'">
                    {{ pos.unrealizedPnl >= 0 ? '+' : '' }}{{ formatCurrency(pos.unrealizedPnl) }}
                  </span>
                </div>
              </div>
              <div class="position-actions">
                <el-button size="small" @click="closePosition(pos)" :loading="tradeStore.loading">
                  平仓
                </el-button>
              </div>
            </div>
          </div>
        </el-card>

        <!-- Order List -->
        <el-card class="orders-card" shadow="never">
          <template #header>
            <div class="card-header">
              <span>活跃订单</span>
              <el-badge :value="tradeStore.activeOrders.length" :max="99" />
            </div>
          </template>
          <div class="orders-list">
            <div v-if="tradeStore.activeOrders.length === 0" class="empty-state">
              <el-empty description="暂无活跃订单" :image-size="60" />
            </div>
            <div v-for="order in tradeStore.activeOrders" :key="order.id" class="order-item">
              <div class="order-header">
                <span class="order-symbol">{{ order.symbol }}</span>
                <el-tag :type="order.side === 'buy' ? 'success' : 'danger'" size="small">
                  {{ order.side === 'buy' ? '买入' : '卖出' }}
                </el-tag>
              </div>
              <div class="order-details">
                <div class="order-row">
                  <span class="row-label">类型</span>
                  <span class="row-value">{{ order.type === 'market' ? '市价' : '限价' }}</span>
                </div>
                <div class="order-row">
                  <span class="row-label">价格</span>
                  <span class="row-value">
                    {{ order.price ? formatPrice(order.price) : '市价' }}
                  </span>
                </div>
                <div class="order-row">
                  <span class="row-label">数量</span>
                  <span class="row-value">{{ order.quantity.toFixed(6) }}</span>
                </div>
                <div class="order-row">
                  <span class="row-label">已成交</span>
                  <span class="row-value">{{ order.filledQuantity.toFixed(6) }}</span>
                </div>
                <div class="order-row">
                  <span class="row-label">状态</span>
                  <span class="row-value">{{ getOrderStatusLabel(order.status) }}</span>
                </div>
              </div>
              <div class="order-actions">
                <el-button
                  size="small"
                  type="danger"
                  link
                  @click="cancelOrder(order.id)"
                  :loading="tradeStore.loading"
                >
                  撤单
                </el-button>
              </div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import { Close } from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { useUserStore } from '@/store/modules/user';
import { useTradeStore } from '@/store/modules/trade';
import { useMarketStore } from '@/store/modules/market';
import * as api from '@/api/tauri';
import OrderForm from '@/components/Trade/OrderForm.vue';
import type { InstanceStatus, OrderStatus } from '@/types';

// Router
const router = useRouter();

// Stores
const userStore = useUserStore();
const tradeStore = useTradeStore();
const marketStore = useMarketStore();

// State
const accountBalance = ref(0);
const balanceLoading = ref(false);
const runningInstances = ref<any[]>([]);
const strategyLoading = ref(false);

// Interval refs
const refreshInterval = ref<number | null>(null);

// Computed
const totalUnrealizedPnl = computed(() => {
  return tradeStore.positions.reduce((sum, p) => sum + p.unrealizedPnl, 0);
});

// Methods
function formatPrice(price: number): string {
  if (price >= 1000) return price.toFixed(2);
  if (price >= 1) return price.toFixed(4);
  return price.toFixed(6);
}

function formatCurrency(value: number): string {
  return value.toLocaleString('en-US', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
}

function getOrderStatusLabel(status: OrderStatus): string {
  switch (status) {
    case 'pending': return '待提交';
    case 'open': return '挂单';
    case 'partially_filled': return '部分成交';
    case 'filled': return '已成交';
    case 'canceled': return '已撤销';
    case 'rejected': return '已拒绝';
    default: return status;
  }
}

function getInstanceStatusLabel(status: InstanceStatus): string {
  switch (status) {
    case 'Starting': return '启动中';
    case 'Running': return '运行中';
    case 'Stopping': return '停止中';
    case 'Stopped': return '已停止';
    case 'Error': return '错误';
    default: return status;
  }
}

function getInstanceStatusType(status: InstanceStatus): string {
  switch (status) {
    case 'Running': return 'success';
    case 'Starting':
    case 'Stopping': return 'warning';
    case 'Stopped': return 'info';
    case 'Error': return 'danger';
    default: return '';
  }
}

function canStopInstance(status: InstanceStatus): boolean {
  return status === 'Running' || status === 'Starting' || status === 'Error';
}

async function loadAccountBalance() {
  balanceLoading.value = true;
  try {
    const balances = await tradeStore.getBalance();
    // Find USDT balance or calculate total balance
    const usdtBalance = balances.find((b: any) => b.asset === 'USDT');
    accountBalance.value = usdtBalance ? parseFloat(usdtBalance.free || '0') : 0;
  } catch (error) {
    console.error('Failed to load balance:', error);
    accountBalance.value = 0;
  } finally {
    balanceLoading.value = false;
  }
}

async function loadRunningInstances() {
  strategyLoading.value = true;
  try {
    runningInstances.value = await api.invoke('strategy_instance_list_all');
  } catch (error) {
    console.error('Failed to load running instances:', error);
    runningInstances.value = [];
  } finally {
    strategyLoading.value = false;
  }
}

async function handleStopInstance(instanceId: string) {
  try {
    await ElMessageBox.confirm(
      '确定要停止此策略吗？',
      '停止策略',
      { type: 'warning' }
    );

    await api.invoke('strategy_stop_instance', { instanceId });
    ElMessage.success('策略已停止');

    // Reload instances
    await loadRunningInstances();
  } catch (error) {
    if (error !== 'cancel') {
      ElMessage.error('停止策略失败：' + (error as Error).message);
    }
  }
}

function handleOrderPlaced(order: any) {
  ElMessage.success(`订单已提交: ${order.id}`);
}

async function closePosition(pos: any) {
  if (!userStore.user) return;

  try {
    await ElMessageBox.confirm('确定要平仓吗？', '确认平仓', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    });

    await tradeStore.closePosition(userStore.user.id, pos.symbol, pos.side, pos.quantity);
    ElMessage.success('平仓成功');
  } catch {
    // User canceled
  }
}

async function closeAllPositions() {
  if (!userStore.user) return;

  try {
    await ElMessageBox.confirm('确定要全部平仓吗？', '确认', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    });

    for (const pos of tradeStore.positions) {
      await tradeStore.closePosition(userStore.user.id, pos.symbol, pos.side, pos.quantity);
    }

    ElMessage.success('全部平仓成功');
  } catch {
    // User canceled
  }
}

async function cancelOrder(orderId: string) {
  if (!userStore.user) return;

  try {
    await tradeStore.cancelOrder(userStore.user.id, orderId);
    ElMessage.success('撤单成功');
  } catch (error) {
    ElMessage.error('撤单失败：' + (error as Error).message);
  }
}

async function refreshData() {
  if (!userStore.user) return;

  // Refresh account balance
  await loadAccountBalance();

  // Refresh positions and orders
  await tradeStore.initialize(userStore.user.id);

  // Refresh running instances
  await loadRunningInstances();
}

// Lifecycle
onMounted(async () => {
  // Check if user is logged in
  if (!userStore.user) {
    ElMessage.warning('请先登录');
    router.push('/login');
    return;
  }

  // Load symbols
  if (marketStore.symbols.length === 0) {
    await marketStore.loadSymbols();
  }

  // Initialize trade store
  await tradeStore.initialize(userStore.user.id);

  // Load initial data
  await loadAccountBalance();
  await loadRunningInstances();

  // Set up auto-refresh (every 5 seconds)
  refreshInterval.value = window.setInterval(() => {
    refreshData();
  }, 5000);
});

onUnmounted(() => {
  // Clear interval
  if (refreshInterval.value) {
    clearInterval(refreshInterval.value);
  }
});
</script>

<style scoped lang="scss">
.trade-console {
  padding: 20px;
  min-height: calc(100vh - 60px);
  background: var(--el-bg-color-page);
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.header-left {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.page-title {
  font-size: 20px;
  font-weight: 600;
  color: var(--el-text-color-primary);
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 12px;
  align-items: center;
}

.pnl-profit {
  color: #67c23a;
  font-weight: 600;
}

.pnl-loss {
  color: #f56c6c;
  font-weight: 600;
}

// Strategy panel
.strategies-panel-card {
  margin-bottom: 16px;
}

// 账户余额卡片
.account-row {
  margin-bottom: 16px;
}

.balance-card {
  display: flex;
  align-items: center;
  gap: 16px;
  padding: 20px;
  background: #fff;
  border-radius: 12px;
  border: 1px solid #ebeef5;
  transition: all 0.3s;

  &:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
    transform: translateY(-2px);
  }

  &.balance-total {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border: none;

    .balance-icon {
      background: rgba(255, 255, 255, 0.2);
      color: #fff;
    }

    .balance-label {
      color: rgba(255, 255, 255, 0.8);
    }

    .balance-value {
      color: #fff;
      font-size: 20px;
    }
  }
}

.balance-icon {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #fff;
  flex-shrink: 0;

  &.balance-icon-btc {
    background: linear-gradient(135deg, #f7931a 0%, #f7b92f 100%);
  }

  &.balance-icon-eth {
    background: linear-gradient(135deg, #627eea 0%, #7c8cf8 100%);
  }

  &.balance-icon-usdt {
    background: linear-gradient(135deg, #26a69a 0%, #2dd4bf 100%);
  }
}

.balance-info {
  flex: 1;
}

.balance-label {
  font-size: 12px;
  color: #909399;
  margin-bottom: 4px;
}

.balance-value {
  font-size: 18px;
  font-weight: 600;
  color: #303133;
}

// 主内容区
.main-row {
  margin-bottom: 16px;
}

// 交易对选择器
.symbol-card {
  :deep(.el-card__body) {
    padding: 0;
  }
}

.symbol-selector {
  .current-symbol {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px;
    cursor: pointer;
    transition: background 0.2s;

    &:hover {
      background: #f5f7fa;
    }
  }

  .symbol-info {
    flex: 1;
  }

  .symbol-name {
    font-size: 16px;
    font-weight: 600;
    color: #303133;
  }

  .symbol-price {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-top: 4px;
  }

  .price-value {
    font-size: 14px;
    font-weight: 600;
  }

  .price-change {
    font-size: 12px;
  }

  .expand-icon {
    color: #909399;
  }

  .quick-symbols {
    display: flex;
    gap: 8px;
    padding: 0 16px 16px;
    border-top: 1px solid #ebeef5;
    overflow-x: auto;

    .symbol-item {
      display: flex;
      flex-direction: column;
      padding: 8px 12px;
      background: #f5f7fa;
      border-radius: 6px;
      cursor: pointer;
      transition: all 0.2s;
      min-width: 80px;

      &:hover {
        background: #ecf5ff;
      }

      .symbol-pair {
        font-size: 12px;
        font-weight: 500;
        color: #606266;
        margin-bottom: 4px;
      }

      .symbol-price {
        font-size: 11px;
      }
    }
  }
}

// 深度图
.depth-card {
  :deep(.el-card__body) {
    padding: 0;
  }
}

.depth-chart {
  height: 300px;
}

// 交易面板
.trade-panel {
  margin-top: 16px;

  .trade-tabs {
    margin-bottom: 20px;

    .buy-tab,
    .sell-tab {
      display: flex;
      flex-direction: column;
      gap: 4px;
      padding: 12px 24px;
      border-radius: 8px;
      transition: all 0.2s;
    }

    .buy-tab {
      background: linear-gradient(135deg, rgba(103, 194, 58, 0.1) 0%, rgba(103, 194, 58, 0.05) 100%);
      border: 1px solid rgba(103, 194, 58, 0.2);

      &.is-active {
        background: linear-gradient(135deg, #67c23a 0%, #85ce61 100%);
        border-color: #67c23a;

        .tab-text {
          color: #fff;
        }

        .tab-price {
          color: #fff;
        }
      }
    }

    .sell-tab {
      background: linear-gradient(135deg, rgba(245, 108, 108, 0.1) 0%, rgba(245, 108, 108, 0.05) 100%);
      border: 1px solid rgba(245, 108, 108, 0.2);

      &.is-active {
        background: linear-gradient(135deg, #f56c6c 0%, #f78989 100%);
        border-color: #f56c6c;

        .tab-text {
          color: #fff;
        }

        .tab-price {
          color: #fff;
        }
      }
    }

    .tab-text {
      font-size: 14px;
      font-weight: 500;
    }

    .tab-price {
      font-size: 16px;
      font-weight: 600;
    }
  }
}

.trade-form-content {
  .el-form-item {
    margin-bottom: 16px;
  }

  .price-input-row,
  .amount-input-row {
    display: flex;
    flex-direction: column;
    gap: 8px;
    margin-bottom: 16px;
  }

  .price-label,
  .amount-label {
    display: flex;
    justify-content: space-between;
    font-size: 13px;
    color: #606266;
    font-weight: 500;
  }

  .price-label-desc,
  .amount-label-desc {
    color: #909399;
    font-size: 12px;
  }

  .price-input-wrapper,
  .amount-input-wrapper {
    display: flex;
    gap: 8px;

    .el-input-number {
      flex: 1;
    }
  }

  .price-presets,
  .amount-presets {
    display: flex;
    gap: 6px;

    .el-button {
      flex: 1;
    }
  }

  .trade-preview {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
    padding: 16px;
    background: #f5f7fa;
    border-radius: 8px;
    margin-bottom: 16px;
  }

  .preview-item {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .preview-label {
    font-size: 12px;
    color: #909399;
  }

  .preview-value {
    font-size: 14px;
    font-weight: 600;
    color: #303133;
  }

  .submit-button {
    width: 100%;
    height: 48px;
    font-size: 16px;
    font-weight: 600;

    &.buy-button {
      background: linear-gradient(135deg, #67c23a 0%, #85ce61 100%);
      border: none;

      &:hover {
        background: linear-gradient(135deg, #5daf34 0%, #67c23a 100%);
      }
    }

    &.sell-button {
      background: linear-gradient(135deg, #f56c6c 0%, #f78989 100%);
      border: none;

      &:hover {
        background: linear-gradient(135deg, #f4516c 0%, #f56c6c 100%);
      }
    }
  }
}

// 持仓和委托卡片
.positions-card,
.orders-card {
  height: calc(100% - 16px);
  display: flex;
  flex-direction: column;

  :deep(.el-card__body) {
    flex: 1;
    padding: 16px;
    overflow-y: auto;
  }
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 14px;
  font-weight: 600;
  color: #303133;
}

.positions-list,
.orders-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.empty-state {
  display: flex;
  justify-content: center;
  padding: 20px;
}

.position-item,
.order-item {
  padding: 12px;
  background: #f5f7fa;
  border-radius: 8px;
  border: 1px solid #ebeef5;
}

.position-header,
.order-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.position-symbol,
.order-symbol {
  font-size: 14px;
  font-weight: 600;
  color: #303133;
}

.position-info,
.order-details {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 8px;
}

.position-row,
.order-row {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
}

.row-label {
  color: #909399;
}

.row-value {
  font-weight: 500;
  color: #303133;
}

.position-actions,
.order-actions {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid #ebeef5;
}

.profit {
  color: #ef5350;
}

.loss {
  color: #26a69a;
}

.price-up {
  color: #ef5350;
}

.price-down {
  color: #26a69a;
}

// 响应式
@media (max-width: 1200px) {
  .main-row {
    flex-direction: column;

    > div {
      margin-bottom: 16px;
    }
  }
}
</style>
