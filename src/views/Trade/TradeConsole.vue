<template>
  <div class="trade-console">
    <!-- 页面头部 -->
    <div class="page-header">
      <div class="header-left">
        <h2 class="page-title">交易控制台</h2>
        <el-breadcrumb separator="/">
          <el-breadcrumb-item :to="{ path: '/' }">首页</el-breadcrumb-item>
          <el-breadcrumb-item>交易控制台</el-breadcrumb-item>
        </el-breadcrumb>
      </div>
      <div class="header-actions">
        <el-button type="danger" @click="closeAllPositions" :disabled="positions.length === 0">
          <el-icon><Close /></el-icon>
          全部平仓
        </el-button>
      </div>
    </div>

    <!-- 账户信息 -->
    <el-row :gutter="16" class="account-row">
      <el-col :span="6">
        <div class="balance-card">
          <div class="balance-icon balance-icon-btc">
            <el-icon :size="20"><Wallet /></el-icon>
          </div>
          <div class="balance-info">
            <div class="balance-label">BTC 可用</div>
            <div class="balance-value">1.2345</div>
          </div>
        </div>
      </el-col>
      <el-col :span="6">
        <div class="balance-card">
          <div class="balance-icon balance-icon-eth">
            <el-icon :size="20"><Wallet /></el-icon>
          </div>
          <div class="balance-info">
            <div class="balance-label">ETH 可用</div>
            <div class="balance-value">12.5678</div>
          </div>
        </div>
      </el-col>
      <el-col :span="6">
        <div class="balance-card">
          <div class="balance-icon balance-icon-usdt">
            <el-icon :size="20"><Wallet /></el-icon>
          </div>
          <div class="balance-info">
            <div class="balance-label">USDT 可用</div>
            <div class="balance-value">12,456.78</div>
          </div>
        </div>
      </el-col>
      <el-col :span="6">
        <div class="balance-card balance-total">
          <div class="balance-icon">
            <el-icon :size="20"><TrendCharts /></el-icon>
          </div>
          <div class="balance-info">
            <div class="balance-label">总资产估值</div>
            <div class="balance-value">45,678.90</div>
          </div>
        </div>
      </el-col>
    </el-row>

    <el-row :gutter="16" class="main-row">
      <!-- 左侧：深度图 + 交易面板 -->
      <el-col :span="16">
        <!-- 交易对选择 -->
        <el-card class="symbol-card" shadow="never">
          <div class="symbol-selector">
            <div class="current-symbol">
              <el-avatar :size="32" src="https://cryptologos.cc/logos/btc.png" />
              <div class="symbol-info">
                <div class="symbol-name">BTC/USDT</div>
                <div class="symbol-price">
                  <span class="price-value price-up">43,256.78</span>
                  <span class="price-change price-up">+2.34%</span>
                </div>
              </div>
              <el-icon class="expand-icon"><ArrowDown /></el-icon>
            </div>
            <div class="quick-symbols">
              <div class="symbol-item" v-for="s in hotSymbols" :key="s.symbol">
                <span class="symbol-pair">{{ s.symbol }}</span>
                <span class="symbol-price" :class="s.change >= 0 ? 'price-up' : 'price-down'">
                  {{ s.change >= 0 ? '+' : '' }}{{ s.change }}%
                </span>
              </div>
            </div>
          </div>
        </el-card>

        <!-- 深度图 -->
        <el-card class="depth-card" shadow="never">
          <template #header>
            <div class="card-header">
              <span>订单簿深度</span>
              <el-radio-group v-model="depthType" size="small">
                <el-radio-button label="limit">限价</el-radio-button>
                <el-radio-button label="market">市价</el-radio-button>
              </el-radio-group>
            </div>
          </template>
          <div ref="depthChartRef" class="depth-chart"></div>
        </el-card>

        <!-- 交易面板 -->
        <el-card class="trade-panel" shadow="never">
          <template #header>
            <span>快速交易</span>
          </template>
          <div class="trade-tabs">
            <el-radio-group v-model="tradeSide" size="large">
              <el-radio-button value="buy" class="buy-tab">
                <span class="tab-text">买入</span>
                <span class="tab-price price-up">43,256.78</span>
              </el-radio-button>
              <el-radio-button value="sell" class="sell-tab">
                <span class="tab-text">卖出</span>
                <span class="tab-price price-down">43,254.32</span>
              </el-radio-button>
            </el-radio-group>
          </div>

          <div class="trade-form">
            <el-form :model="tradeForm" label-width="0" class="trade-form-content">
              <el-row :gutter="16">
                <el-col :span="12">
                  <el-form-item label="订单类型">
                    <el-radio-group v-model="tradeForm.orderType" size="small">
                      <el-radio-button label="limit">限价</el-radio-button>
                      <el-radio-button label="market">市价</el-radio-button>
                      <el-radio-button label="stop">止损</el-radio-button>
                    </el-radio-group>
                  </el-form-item>
                </el-col>
                <el-col :span="12">
                  <el-form-item label="价格类型">
                    <el-radio-group v-model="tradeForm.priceType" size="small">
                      <el-radio-button label="limit">限价</el-radio-button>
                      <el-radio-button label="market">市价</el-radio-button>
                    </el-radio-group>
                  </el-form-item>
                </el-col>
              </el-row>

              <!-- 价格输入 -->
              <div v-if="tradeForm.orderType !== 'market'" class="price-input-row">
                <div class="price-label">
                  <span>价格</span>
                  <span class="price-label-desc">({{ tradeForm.priceType === 'limit' ? '限价' : '市价' }})</span>
                </div>
                <div class="price-input-wrapper">
                  <el-input-number
                    v-model="tradeForm.price"
                    :precision="2"
                    :step="0.01"
                    :controls-position="right"
                    class="price-input"
                    @change="updateTotalAmount"
                  >
                    <template #append>
                      <span>USDT</span>
                    </template>
                  </el-input-number>
                </div>
                <div class="price-presets">
                  <el-button size="small" @click="setPricePercent(-0.1)">-0.1%</el-button>
                  <el-button size="small" @click="setPricePercent(-1)">-1%</el-button>
                  <el-button size="small" @click="setPricePercent(0)">市价</el-button>
                  <el-button size="small" @click="setPricePercent(1)">+1%</el-button>
                  <el-button size="small" @click="setPricePercent(0.1)">+0.1%</el-button>
                </div>
              </div>

              <!-- 数量输入 -->
              <div class="amount-input-row">
                <div class="amount-label">
                  <span>数量</span>
                  <span class="amount-label-desc">(BTC)</span>
                </div>
                <div class="amount-input-wrapper">
                  <el-input-number
                    v-model="tradeForm.amount"
                    :precision="6"
                    :step="0.000001"
                    :controls-position="right"
                    class="amount-input"
                    @change="updateTotalAmount"
                  >
                    <template #append>
                      <span>BTC</span>
                    </template>
                  </el-input-number>
                </div>
                <div class="amount-presets">
                  <el-button size="small" @click="setAmountPercent(25)">25%</el-button>
                  <el-button size="small" @click="setAmountPercent(50)">50%</el-button>
                  <el-button size="small" @click="setAmountPercent(75)">75%</el-button>
                  <el-button size="small" @click="setAmountPercent(100)">100%</el-button>
                </div>
              </div>

              <!-- 交易预览 -->
              <div class="trade-preview">
                <div class="preview-item">
                  <span class="preview-label">交易额</span>
                  <span class="preview-value">{{ formatCurrency(tradeForm.total) }} USDT</span>
                </div>
                <div class="preview-item">
                  <span class="preview-label">手续费</span>
                  <span class="preview-value">{{ tradeForm.fee }} USDT</span>
                </div>
              </div>

              <!-- 提交按钮 -->
              <el-button
                type="primary"
                size="large"
                :class="tradeSide === 'buy' ? 'buy-button' : 'sell-button'"
                :loading="trading"
                @click="submitTrade"
                class="submit-button"
              >
                {{ tradeSide === 'buy' ? '买入 BTC' : '卖出 BTC' }}
              </el-button>
            </el-form>
          </div>
        </el-card>
      </el-col>

      <!-- 右侧：持仓 + 订单 -->
      <el-col :span="8">
        <!-- 当前持仓 -->
        <el-card class="positions-card" shadow="never">
          <template #header>
            <div class="card-header">
              <span>当前持仓</span>
              <el-badge :value="positions.length" :max="99" />
            </div>
          </template>
          <div class="positions-list">
            <div v-if="positions.length === 0" class="empty-state">
              <el-empty description="暂无持仓" :image-size="60" />
            </div>
            <div v-for="pos in positions" :key="pos.id" class="position-item">
              <div class="position-header">
                <div class="position-symbol">{{ pos.symbol }}</div>
                <el-tag :type="pos.side === 'long' ? 'success' : 'danger'" size="small">
                  {{ pos.side === 'long' ? '做多' : '做空' }}
                </el-tag>
              </div>
              <div class="position-info">
                <div class="position-row">
                  <span class="row-label">数量</span>
                  <span class="row-value">{{ pos.amount }}</span>
                </div>
                <div class="position-row">
                  <span class="row-label">均价</span>
                  <span class="row-value">{{ pos.avgPrice }}</span>
                </div>
                <div class="position-row">
                  <span class="row-label">当前价</span>
                  <span class="row-value" :class="pos.pnl >= 0 ? 'price-up' : 'price-down'">
                    {{ pos.currentPrice }}
                  </span>
                </div>
                <div class="position-row">
                  <span class="row-label">未实现盈亏</span>
                  <span class="row-value" :class="pos.pnl >= 0 ? 'profit' : 'loss'">
                    {{ pos.pnl >= 0 ? '+' : '' }}{{ pos.pnl }}
                  </span>
                </div>
              </div>
              <div class="position-actions">
                <el-button size="small" @click="closePosition(pos.id)">平仓</el-button>
              </div>
            </div>
          </div>
        </el-card>

        <!-- 当前委托 -->
        <el-card class="orders-card" shadow="never">
          <template #header>
            <div class="card-header">
              <span>当前委托</span>
              <el-badge :value="orders.length" :max="99" />
            </div>
          </template>
          <div class="orders-list">
            <div v-if="orders.length === 0" class="empty-state">
              <el-empty description="暂无委托" :image-size="60" />
            </div>
            <div v-for="order in orders" :key="order.id" class="order-item">
              <div class="order-header">
                <span class="order-symbol">{{ order.symbol }}</span>
                <el-tag :type="order.side === 'buy' ? 'success' : 'danger'" size="small">
                  {{ order.side === 'buy' ? '买入' : '卖出' }}
                </el-tag>
              </div>
              <div class="order-details">
                <div class="order-row">
                  <span class="row-label">类型</span>
                  <span class="row-value">{{ order.type }}</span>
                </div>
                <div class="order-row">
                  <span class="row-label">价格</span>
                  <span class="row-value">{{ order.price }}</span>
                </div>
                <div class="order-row">
                  <span class="row-label">数量</span>
                  <span class="row-value">{{ order.amount }}</span>
                </div>
                <div class="order-row">
                  <span class="row-label">已成交</span>
                  <span class="row-value">{{ order.filled }}</span>
                </div>
              </div>
              <div class="order-actions">
                <el-button size="small" type="danger" link @click="cancelOrder(order.id)">撤单</el-button>
              </div>
            </div>
          </div>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue';
import * as echarts from 'echarts';
import {
  Close,
  Wallet,
  TrendCharts,
  ArrowDown,
  ShoppingCart,
} from '@element-plus/icons-vue';
import { ElMessage, ElMessageBox } from 'element-plus';

// 状态
const depthType = ref('limit');
const tradeSide = ref('buy');
const trading = ref(false);
const depthChartRef = ref<HTMLElement>();
let depthChart: echarts.ECharts | null = null;

// 交易表单
const tradeForm = ref({
  orderType: 'limit',
  priceType: 'limit',
  price: 43256.78,
  amount: 0.1,
  total: 4325.68,
  fee: 4.33,
});

// 热门交易对
const hotSymbols = ref([
  { symbol: 'ETH/USDT', change: 1.23 },
  { symbol: 'BNB/USDT', change: -0.56 },
  { symbol: 'SOL/USDT', change: 3.45 },
  { symbol: 'ADA/USDT', change: -1.23 },
  { symbol: 'DOT/USDT', change: 0.89 },
]);

// 模拟持仓数据
const positions = ref([
  {
    id: 1,
    symbol: 'BTC/USDT',
    side: 'long',
    amount: '0.5',
    avgPrice: 42000,
    currentPrice: 43256,
    pnl: 628,
  },
  {
    id: 2,
    symbol: 'ETH/USDT',
    side: 'short',
    amount: '5',
    avgPrice: 2600,
    currentPrice: 2550,
    pnl: 250,
  },
]);

// 模拟委托数据
const orders = ref([
  {
    id: 1,
    symbol: 'BTC/USDT',
    side: 'buy',
    type: 'limit',
    price: 43000,
    amount: 0.5,
    filled: 0,
  },
  {
    id: 2,
    symbol: 'ETH/USDT',
    side: 'sell',
    type: 'limit',
    price: 2600,
    amount: 2,
    filled: 0,
  },
]);

// 初始化深度图
function initDepthChart() {
  if (!depthChartRef.value) return;

  depthChart = echarts.init(depthChartRef.value);

  // 模拟深度数据
  const bids = Array.from({ length: 20 }, (_, i) => ({
    price: 43200 - i * 10,
    amount: Math.random() * 100 + 10,
  }));

  const asks = Array.from({ length: 20 }, (_, i) => ({
    price: 43300 + i * 10,
    amount: Math.random() * 100 + 10,
  }));

  const option = {
    grid: { left: 10, right: 10 },
    xAxis: {
      type: 'category',
      data: [...bids.reverse().map(d => d.price), ...asks.map(d => d.price)],
      axisLine: { show: false },
      axisTick: { show: false },
      splitLine: { show: false },
      axisLabel: { show: false },
    },
    yAxis: {
      type: 'value',
      axisLine: { show: false },
      axisTick: { show: false },
      splitLine: { show: false },
      axisLabel: { show: false },
    },
    series: [
      {
        type: 'bar',
        name: '买单',
        data: [...bids.reverse().map(d => d.amount), ...asks.map(() => 0)],
        itemStyle: {
          color: '#26a69a',
        },
        stack: 'total',
      },
      {
        type: 'bar',
        name: '卖单',
        data: [...bids.reverse().map(() => 0), ...asks.map(d => d.amount)],
        itemStyle: {
          color: '#ef5350',
        },
        stack: 'total',
      },
    ],
  };

  depthChart.setOption(option);

  window.addEventListener('resize', handleDepthResize);
}

function handleDepthResize() {
  depthChart?.resize();
}

function updateTotalAmount() {
  tradeForm.value.total = tradeForm.value.price * tradeForm.value.amount;
  tradeForm.value.fee = tradeForm.value.total * 0.001;
}

function setPricePercent(percent: number) {
  const currentPrice = 43256.78;
  tradeForm.value.price = currentPrice * (1 + percent / 100);
  updateTotalAmount();
}

function setAmountPercent(percent: number) {
  tradeForm.value.amount = 1 * (percent / 100);
  updateTotalAmount();
}

function formatCurrency(value: number): string {
  return value.toLocaleString('en-US', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  });
}

async function submitTrade() {
  trading.value = true;
  // 模拟交易
  await new Promise(resolve => setTimeout(resolve, 1000));
  trading.value = false;
  ElMessage.success(tradeSide.value === 'buy' ? '买入成功' : '卖出成功');
}

async function closePosition(id: number) {
  try {
    await ElMessageBox.confirm('确定要平仓吗？', '确认平仓', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    });
    ElMessage.success('平仓成功');
  } catch {}
}

async function closeAllPositions() {
  try {
    await ElMessageBox.confirm('确定要全部平仓吗？', '确认', {
      confirmButtonText: '确定',
      cancelButtonText: '取消',
      type: 'warning',
    });
    ElMessage.success('全部平仓成功');
  } catch {}
}

async function cancelOrder(id: number) {
  ElMessage.success('撤单成功');
}

onMounted(() => {
  nextTick(() => {
    initDepthChart();
  });
});

onUnmounted(() => {
  depthChart?.dispose();
  window.removeEventListener('resize', handleDepthResize);
});
</script>

<style scoped lang="scss">
.trade-console {
  padding: 0;
  min-height: calc(100vh - 60px);
  background: #f5f7fa;
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
  color: #303133;
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 12px;
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
