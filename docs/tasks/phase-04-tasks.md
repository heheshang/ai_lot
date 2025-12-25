# Phase 4: 实盘交易模块 - 详细任务规范

## 目标

实现订单管理、持仓管理和策略实盘运行。

---

## 任务概览

| ID | 任务 | 估时 |
|----|------|------|
| P4-01 | 实现订单状态机 | 2h |
| P4-02 | 实现持仓管理 | 2h |
| P4-03 | 实现 TradeService | 3h |
| P4-04 | 实现 Tauri 交易命令 | 1h |
| P4-05 | 实现实盘策略循环 | 3h |
| P4-06 | 实现策略实例持久化 | 1h |
| P4-07 | 实现策略启停控制 | 1h |
| P4-08 | 实现策略状态查询 | 0.5h |
| P4-09 | 实现 TradeStore | 1h |
| P4-10 | 实现订单列表组件 | 1h |
| P4-11 | 实现持仓列表组件 | 1h |
| P4-12 | 实现手动下单表单 | 1h |
| P4-13 | 实现交易控制台页面 | 2h |

---

## 核心任务详解

### P4-01: 实现订单状态机

```rust
// src-tauri/src/core/trade/order/state.rs

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderState {
    Pending,
    Open,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
}

pub struct OrderStateMachine {
    state: OrderState,
}

impl OrderStateMachine {
    pub fn new() -> Self {
        Self {
            state: OrderState::Pending,
        }
    }

    pub fn transition_to(&mut self, new_state: OrderState) -> Result<()> {
        match (&self.state, &new_state) {
            (OrderState::Pending, OrderState::Open) => Ok(()),
            (OrderState::Pending, OrderState::Rejected) => Ok(()),
            (OrderState::Open, OrderState::PartiallyFilled) => Ok(()),
            (OrderState::Open, OrderState::Filled) => Ok(()),
            (OrderState::Open, OrderState::Canceled) => Ok(()),
            (OrderState::PartiallyFilled, OrderState::Filled) => Ok(()),
            (OrderState::PartiallyFilled, OrderState::Canceled) => Ok(()),
            _ => anyhow::bail!("Invalid state transition: {:?} -> {:?}", self.state, new_state),
        }?;
        self.state = new_state;
        Ok(())
    }

    pub fn state(&self) -> &OrderState {
        &self.state
    }
}
```

### P4-02: 实现持仓管理

```rust
// src-tauri/src/core/trade/position/manager.rs

use crate::core::trade::types::*;
use std::collections::HashMap;

pub struct PositionManager {
    positions: HashMap<String, Position>,
}

impl PositionManager {
    pub fn new() -> Self {
        Self {
            positions: HashMap::new(),
        }
    }

    pub fn update_position(&mut self, trade: &Trade) {
        let key = format!("{}_{}", trade.symbol, trade.side);
        let pos = self.positions.entry(key).or_insert_with(|| Position {
            id: Uuid::new_v4().to_string(),
            symbol: trade.symbol.clone(),
            side: trade.side.clone(),
            quantity: 0.0,
            entry_price: 0.0,
            unrealized_pnl: 0.0,
            realized_pnl: 0.0,
            opened_at: chrono::Utc::now().timestamp(),
        });

        // 更新持仓
        if trade.side == "buy" {
            // 加仓逻辑
            let total_cost = pos.quantity * pos.entry_price + trade.quantity * trade.price;
            pos.quantity += trade.quantity;
            pos.entry_price = total_cost / pos.quantity;
        } else {
            // 减仓逻辑
            pos.quantity -= trade.quantity;
        }

        // 计算未实现盈亏
        if let Some(current_price) = trade.price {
            pos.unrealized_pnl = if pos.side == "long" {
                (current_price - pos.entry_price) * pos.quantity;
            } else {
                (pos.entry_price - current_price) * pos.quantity;
            };
        }
    }
}
```

### P4-03: 实现 TradeService

```rust
// src-tauri/src/services/trade_service.rs

pub struct TradeService {
    exchange: Arc<dyn Exchange>,
    db: Database,
}

impl TradeService {
    pub async fn place_order(&self, request: OrderRequest) -> Result<Order> {
        let order = self.exchange.place_order(&request).await?;

        // 保存到数据库
        sqlx::query(
            r#"
            INSERT INTO orders (id, user_id, exchange_id, symbol, side, order_type,
                               price, quantity, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&order.id)
        .bind(&order.user_id)
        .bind(&order.exchange_id)
        .bind(&order.symbol)
        .bind(&order.side)
        .bind(&order.order_type)
        .bind(order.price)
        .bind(order.quantity)
        .bind(&order.status)
        .bind(order.created_at)
        .bind(order.updated_at)
        .execute(&self.db.pool)
        .await?;

        Ok(order)
    }

    pub async fn cancel_order(&self, order_id: &str) -> Result<()> {
        // 调用交易所撤单
        // 更新数据库状态
        Ok(())
    }

    pub async fn get_positions(&self) -> Result<Vec<Position>> {
        let positions = sqlx::query_as::<_, Position>(
            "SELECT * FROM positions WHERE status = 'open'"
        )
        .fetch_all(&self.db.pool)
        .await?;
        Ok(positions)
    }
}
```

### P4-05: 实现实盘策略循环

```rust
// 扩展策略引擎的运行方法

impl RunningInstance {
    pub async fn run(mut self) {
        let mut kline_stream = self.event_bus.subscribe_market();
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        self.shutdown_tx = Some(shutdown_tx);

        // 订阅指定交易对的行情
        self.exchange.subscribe_kline(
            vec![self.config.symbol.clone()],
            Interval::from_str(&self.config.timeframe).unwrap(),
        ).await.ok();

        // 执行策略初始化
        self.execute_callback("onInit", &[]).await;

        loop {
            tokio::select! {
                // 接收K线事件
                Ok(event) = kline_stream.recv() => {
                    if let MarketEvent::Kline(kline) = event {
                        // 只处理当前交易对和周期
                        if kline.symbol == self.config.symbol {
                            if let Ok(signal) = self.on_bar(&kline).await {
                                self.execute_signal(signal).await;
                            }
                        }
                    }
                }

                // 接收停止信号
                Ok(()) = &mut shutdown_rx => {
                    break;
                }
            }
        }

        // 清理
        let _ = self.execute_callback("onStop", &[]).await;
    }

    async fn execute_signal(&self, signal: Signal) {
        // 风控检查
        if !self.check_risk(&signal).await {
            log::warn!("Risk check failed, signal ignored");
            return;
        }

        // 执行订单
        let order_request = OrderRequest {
            symbol: signal.symbol.clone(),
            side: signal.action.parse().unwrap(),
            order_type: OrderType::Market,
            quantity: signal.quantity,
            price: signal.price,
        };

        match self.exchange.place_order(&order_request).await {
            Ok(order) => {
                log::info!("Order placed: {:?}", order);
                // 记录到数据库
            }
            Err(e) => {
                log::error!("Place order failed: {}", e);
            }
        }
    }
}
```

### P4-09 ~ P4-13: 交易前端

创建 `src/views/Trade/TradeConsole.vue`：

```vue
<template>
  <div class="trade-console">
    <el-row :gutter="20">
      <!-- 策略实例列表 -->
      <el-col :span="8">
        <el-card header="运行策略">
          <el-table :data="runningInstances" stripe>
            <el-table-column prop="strategyName" label="策略" />
            <el-table-column prop="symbol" label="交易对" />
            <el-table-column prop="status" label="状态">
              <template #default="{ row }">
                <el-tag :type="getStatusType(row.status)">
                  {{ row.status }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column label="操作" width="120">
              <template #default="{ row }">
                <el-button
                  v-if="row.status === 'running'"
                  type="warning"
                  size="small"
                  @click="stopInstance(row.id)"
                >
                  停止
                </el-button>
                <el-button
                  v-else
                  type="primary"
                  size="small"
                  @click="startInstance(row.id)"
                >
                  启动
                </el-button>
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>

      <!-- 手动下单 -->
      <el-col :span="16">
        <el-card header="手动下单">
          <el-form :model="orderForm" inline>
            <el-form-item label="交易对">
              <el-select v-model="orderForm.symbol" filterable>
                <el-option
                  v-for="sym in marketStore.symbols"
                  :key="sym"
                  :label="sym"
                  :value="sym"
                />
              </el-select>
            </el-form-item>

            <el-form-item label="方向">
              <el-radio-group v-model="orderForm.side">
                <el-radio-button label="buy">买入</el-radio-button>
                <el-radio-button label="sell">卖出</el-radio-button>
              </el-radio-group>
            </el-form-item>

            <el-form-item label="类型">
              <el-select v-model="orderForm.type">
                <el-option label="市价单" value="market" />
                <el-option label="限价单" value="limit" />
              </el-select>
            </el-form-item>

            <el-form-item label="数量">
              <el-input-number
                v-model="orderForm.quantity"
                :min="0"
                :precision="4"
                :step="0.0001"
              />
            </el-form-item>

            <el-form-item label="价格" v-if="orderForm.type === 'limit'">
              <el-input-number
                v-model="orderForm.price"
                :min="0"
                :precision="2"
                :step="0.01"
              />
            </el-form-item>

            <el-form-item>
              <el-button type="primary" @click="placeOrder" :loading="placing">
                下单
              </el-button>
            </el-form-item>
          </el-form>
        </el-card>

        <!-- 持仓列表 -->
        <el-card header="当前持仓" style="margin-top: 20px;">
          <el-table :data="tradeStore.positions" stripe>
            <el-table-column prop="symbol" label="交易对" />
            <el-table-column prop="side" label="方向" width="80">
              <template #default="{ row }">
                <el-tag :type="row.side === 'long' ? 'success' : 'danger'">
                  {{ row.side === 'long' ? '多' : '空' }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column prop="quantity" label="数量" />
            <el-table-column prop="entryPrice" label="均价" />
            <el-table-column prop="unrealizedPnl" label="未实现盈亏">
              <template #default="{ row }">
                <span :style="{ color: row.unrealizedPnl >= 0 ? '#67c23a' : '#f56c6c' }">
                  {{ row.unrealizedPnl >= 0 ? '+' : '' }}{{ row.unrealizedPnl.toFixed(2) }}
                </span>
              </template>
            </el-table-column>
          </el-table>
        </el-card>

        <!-- 订单列表 -->
        <el-card header="活跃订单" style="margin-top: 20px;">
          <el-table :data="activeOrders" stripe>
            <el-table-column prop="symbol" label="交易对" />
            <el-table-column prop="side" label="方向" width="80" />
            <el-table-column prop="orderType" label="类型" width="80" />
            <el-table-column prop="price" label="价格" />
            <el-table-column prop="quantity" label="数量" />
            <el-table-column prop="filledQuantity" label="已成交" />
            <el-table-column prop="status" label="状态" width="80">
              <template #default="{ row }">
                <el-tag :type="getStatusType(row.status)">
                  {{ row.status }}
                </el-tag>
              </template>
            </el-table-column>
            <el-table-column label="操作" width="80">
              <template #default="{ row }">
                <el-button
                  type="danger"
                  size="small"
                  @click="cancelOrder(row.id)"
                >
                  撤单
                </el-button>
              </template>
            </el-table-column>
          </el-table>
        </el-card>
      </el-col>
    </el-row>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { ElMessage, ElMessageBox } from 'element-plus';
import { useTradeStore } from '@/store/modules/trade';
import { useMarketStore } from '@/store/modules/market';
import * as api from '@/api/tauri';

const tradeStore = useTradeStore();
const marketStore = useMarketStore();

const runningInstances = ref([]);
const activeOrders = computed(() => tradeStore.activeOrders);

const orderForm = ref({
  symbol: 'BTCUSDT',
  side: 'buy',
  type: 'market',
  quantity: 0.001,
  price: 0,
});

const placing = ref(false);

async function placeOrder() {
  placing.value = true;
  try {
    await api.tradeApi.placeOrder(orderForm.value);
    ElMessage.success('下单成功');
  } catch (error) {
    ElMessage.error('下单失败：' + (error as Error).message);
  } finally {
    placing.value = false;
  }
}

async function cancelOrder(orderId: string) {
  try {
    await api.tradeApi.cancelOrder(orderId);
    ElMessage.success('撤单成功');
  } catch (error) {
    ElMessage.error('撤单失败：' + (error as Error).message);
  }
}

async function stopInstance(instanceId: string) {
  try {
    await ElMessageBox.confirm('确定要停止策略吗？', '提示');
    await api.invoke('strategy_stop_instance', { instanceId });
    ElMessage.success('策略已停止');
  } catch {
    // 取消操作
  }
}

onMounted(() => {
  marketStore.loadSymbols();
});
</script>

<style scoped>
.trade-console {
  padding: 0;
}
</style>
```

---

## Phase 4 验收标准

### 功能验收
- [ ] 手动下单功能正常
- [ ] 订单列表实时更新
- [ ] 持仓盈亏实时计算
- [ ] 策略可启动和停止

### 技术验收
- [ ] 订单状态转换正确
- [ ] 持仓计算准确
- [ ] 交易所API调用成功
