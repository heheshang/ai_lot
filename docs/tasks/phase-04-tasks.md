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

**任务描述**: 实现订单状态机和状态转换逻辑，确保订单状态转换的正确性和一致性。

**实现文件**:
- `src-tauri/src/core/trade/types.rs` - OrderState 枚举定义
- `src-tauri/src/core/trade/order/state.rs` - OrderStateMachine 状态机实现
- `src-tauri/src/core/trade/order/mod.rs` - 模块导出

**核心代码结构**:

```rust
// src-tauri/src/core/trade/types.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderState {
    Pending,           // 订单已创建，未提交到交易所
    Open,              // 订单在交易所活跃
    PartiallyFilled,   // 订单部分成交
    Filled,            // 订单完全成交
    Canceled,          // 订单已取消
    Rejected,          // 订单被交易所拒绝
}

impl OrderState {
    pub fn is_terminal(&self) -> bool { ... }
    pub fn is_active(&self) -> bool { ... }
}

// src-tauri/src/core/trade/order/state.rs

pub struct OrderStateMachine {
    state: OrderState,
}

impl OrderStateMachine {
    pub fn new() -> Self { ... }
    pub fn from_state(state: OrderState) -> Self { ... }
    pub fn transition_to(&mut self, new_state: OrderState) -> Result<()> { ... }
    pub fn state(&self) -> OrderState { ... }
    pub fn is_terminal(&self) -> bool { ... }
    pub fn is_active(&self) -> bool { ... }
    pub fn can_transition_to(&self, target_state: &OrderState) -> bool { ... }
}
```

**有效状态转换**:
```
Pending → Open
Pending → Rejected
Open → PartiallyFilled
Open → Filled
Open → Canceled
PartiallyFilled → Filled
PartiallyFilled → Canceled
```

**验收标准**:

1. **代码结构** ✅
   - [x] `OrderState` 枚举定义完整，包含6种状态
   - [x] 实现了 `Display` trait（状态转字符串）
   - [x] 实现了 `FromStr` trait（字符串转状态）
   - [x] 支持 serde 序列化/反序列化
   - [x] `OrderStateMachine` 结构体实现完整

2. **功能完整性** ✅
   - [x] `new()` 方法创建初始状态为 Pending 的状态机
   - [x] `transition_to()` 方法正确执行状态转换
   - [x] 无效的状态转换返回错误
   - [x] `is_terminal()` 正确识别终端状态（Filled/Canceled/Rejected）
   - [x] `is_active()` 正确识别活跃状态（Open/PartiallyFilled）
   - [x] `can_transition_to()` 预检查状态转换有效性

3. **单元测试覆盖** ✅
   - [x] 测试初始状态创建
   - [x] 测试所有有效状态转换路径
   - [x] 测试无效状态转换拒绝
   - [x] 测试幂等转换（相同状态）
   - [x] 测试终端状态判断
   - [x] 测试活跃状态判断
   - [x] 测试 Display 和 FromStr 实现

4. **类型安全** ✅
   - [x] `Order` 结构体的 `status` 字段使用 `OrderState` 类型
   - [x] 不再使用 String 类型表示状态
   - [x] 编译时保证状态值的有效性

### P4-02: 实现持仓管理

**任务描述**: 实现持仓管理器，跟踪交易持仓、计算盈亏、管理持仓生命周期。

**实现文件**:
- `src-tauri/src/core/trade/position/manager.rs` - PositionManager 核心实现
- `src-tauri/src/core/trade/position/mod.rs` - 模块导出

**核心代码结构**:

```rust
// Trade 记录结构
pub struct Trade {
    pub symbol: String,
    pub side: String,      // "buy" 或 "sell"
    pub quantity: f64,
    pub price: f64,
    pub timestamp: i64,
}

// 持仓管理器
pub struct PositionManager {
    positions: HashMap<String, Position>,
}

impl PositionManager {
    // 基础操作
    pub fn new() -> Self { ... }
    pub fn update_position(&mut self, trade: &Trade) { ... }

    // 查询操作
    pub fn get_position(&self, symbol: &str, side: &str) -> Option<&Position> { ... }
    pub fn get_all_positions(&self) -> Vec<&Position> { ... }
    pub fn get_open_positions(&self) -> Vec<&Position> { ... }

    // 持仓管理
    pub fn close_position(&mut self, symbol: &str, side: &str, close_price: f64) -> f64 { ... }
    pub fn remove_position(&mut self, symbol: &str, side: &str) -> Option<Position> { ... }
    pub fn clear(&mut self) { ... }

    // 盈亏计算
    pub fn get_total_unrealized_pnl(&self) -> f64 { ... }
    pub fn get_total_realized_pnl(&self) -> f64 { ... }

    // 统计
    pub fn active_position_count(&self) -> usize { ... }
}
```

**核心功能**:
1. **加仓逻辑**: 计算新的平均入场价格
   ```
   新均价 = (旧持仓量 * 旧均价 + 新交易量 * 新价格) / 总持仓量
   ```

2. **减仓逻辑**: 直接减少持仓量，入场价格不变

3. **未实现盈亏**:
   - 多头: `(当前价格 - 入场价格) * 持仓量`
   - 空头: `(入场价格 - 当前价格) * 持仓量`

4. **已实现盈亏**: 平仓时计算并记录

**验收标准**:

1. **代码结构** ✅
   - [x] `PositionManager` 结构体使用 HashMap 存储持仓
   - [x] `Trade` 结构体定义完整，包含所有必要字段
   - [x] 模块组织清晰，导出正确

2. **功能完整性** ✅
   - [x] `update_position()` 正确处理新持仓创建
   - [x] `update_position()` 正确处理加仓（均价重算）
   - [x] `update_position()` 正确处理减仓（数量减少）
   - [x] `close_position()` 正确计算已实现盈亏
   - [x] `get_open_positions()` 只返回持仓量 > 0 的仓位
   - [x] 多空持仓独立维护（同一交易对可同时持有多空）

3. **盈亏计算准确性** ✅
   - [x] 多头持仓盈亏计算正确（价格上涨盈利）
   - [x] 空头持仓盈亏计算正确（价格下跌盈利）
   - [x] `get_total_unrealized_pnl()` 正确汇总所有持仓
   - [x] `get_total_realized_pnl()` 正确汇总已平仓盈亏

4. **单元测试覆盖** ✅
   - [x] 测试新持仓创建
   - [x] 测试加仓逻辑（均价重算）
   - [x] 测试减仓逻辑
   - [x] 测试多头盈亏计算
   - [x] 测试空头盈亏计算
   - [x] 测试平仓功能
   - [x] 测试多持仓管理
   - [x] 测试总盈亏汇总
   - [x] 测试多空独立持仓
   - [x] 测试持仓清理

5. **边界情况处理** ✅
   - [x] 处理零数量交易（仅更新价格）
   - [x] 处理不存在的持仓查询
   - [x] 处理重复平仓
   - [x] 处理空管理器操作

### P4-03: 实现 TradeService

**任务描述**: 实现交易服务层，封装订单和持仓管理功能，连接交易所API和数据库。

**实现文件**:
- `src-tauri/src/core/trade/types.rs` - 添加 OrderType, OrderSide, OrderRequest, TimeInForce 类型
- `src-tauri/src/core/trade/exchange/trait.rs` - 扩展 Exchange trait 添加交易方法
- `src-tauri/src/services/trade_service.rs` - TradeService 核心实现
- `src-tauri/src/core/trade/exchange/binance.rs` - 实现 BinanceExchange 交易占位方法

**核心代码结构**:

```rust
// 订单类型枚举
pub enum OrderType {
    Market,
    Limit,
    StopLoss,
    StopLimit,
    OCO,
}

// 订单方向枚举
pub enum OrderSide {
    Buy,
    Sell,
}

// 订单请求
pub struct OrderRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub quantity: f64,
    pub client_order_id: Option<String>,
    pub time_in_force: Option<TimeInForce>,
}

// TradeService
pub struct TradeService {
    exchange: Arc<dyn Exchange>,
    db: Database,
    position_manager: Arc<RwLock<PositionManager>>,
}

impl TradeService {
    // 订单操作
    pub async fn place_order(&self, request: OrderRequest, user_id: &str) -> Result<Order>
    pub async fn cancel_order(&self, order_id: &str, user_id: &str) -> Result<()>
    pub async fn get_order(&self, order_id: &str, user_id: &str) -> Result<Order>
    pub async fn get_orders(&self, user_id: &str, symbol, status, limit) -> Result<Vec<Order>>
    pub async fn get_open_orders(&self, user_id: &str) -> Result<Vec<Order>>
    pub async fn sync_order_status(&self, order_id: &str, user_id: &str) -> Result<Order>

    // 持仓操作
    pub async fn get_positions(&self, user_id: &str) -> Result<Vec<Position>>

    // 账户操作
    pub async fn get_balance(&self) -> Result<Vec<Balance>>

    // 私有辅助方法
    fn validate_order_request(&self, request: &OrderRequest) -> Result<()>
    async fn save_order_to_db(&self, order: &Order, user_id: &str) -> Result<()>
    async fn get_order_from_db(&self, order_id: &str, user_id: &str) -> Result<Order>
    async fn update_order_status(&self, order_id: &str, status: OrderState) -> Result<()>
    async fn update_positions_from_order(&self, order: &Order)
}
```

**Exchange trait 扩展**:
```rust
#[async_trait]
pub trait Exchange: Send + Sync {
    // ... 原有方法 ...

    // ========== 交易操作 ==========
    async fn place_order(&self, request: &OrderRequest) -> Result<Order>;
    async fn cancel_order(&self, order_id: &str) -> Result<()>;
    async fn get_order(&self, order_id: &str) -> Result<Order>;
    async fn get_open_orders(&self, symbol: Option<&str>) -> Result<Vec<Order>>;
    async fn get_balance(&self) -> Result<Vec<Balance>>;
    async fn get_positions(&self) -> Result<Vec<Position>>;
}
```

**验收标准**:

1. **类型定义** ✅
   - [x] `OrderType` 枚举定义完整（Market, Limit, StopLoss, StopLimit, OCO）
   - [x] `OrderSide` 枚举定义完整（Buy, Sell）
   - [x] `OrderRequest` 结构体包含所有必要字段
   - [x] `TimeInForce` 枚举定义（GTC, IOC, FOK）
   - [x] 所有类型支持 serde 序列化/反序列化
   - [x] `Order` 结构体使用 `OrderSide` 和 `OrderType` 而非 String

2. **Exchange trait 扩展** ✅
   - [x] 添加 `place_order` 方法
   - [x] 添加 `cancel_order` 方法
   - [x] 添加 `get_order` 方法
   - [x] 添加 `get_open_orders` 方法
   - [x] 添加 `get_balance` 方法
   - [x] 添加 `get_positions` 方法
   - [x] `BinanceExchange` 实现所有新增方法（占位符）

3. **TradeService 功能** ✅
   - [x] `place_order()` 验证请求 → 调用交易所 → 保存数据库 → 发布事件
   - [x] `cancel_order()` 检查状态 → 调用交易所 → 更新数据库
   - [x] `get_order()` 从数据库查询订单
   - [x] `get_orders()` 支持按 symbol/status 过滤
   - [x] `get_open_orders()` 获取所有开放订单
   - [x] `sync_order_status()` 从交易所同步订单状态
   - [x] `get_positions()` 获取用户持仓
   - [x] `get_balance()` 获取账户余额

4. **订单验证** ✅
   - [x] 验证数量必须为正数
   - [x] 限价单必须有价格
   - [x] 止损单必须有止损价格
   - [x] 市价单不需要价格

5. **数据库操作** ✅
   - [x] `save_order_to_db()` 保存订单到数据库
   - [x] `get_order_from_db()` 从数据库获取订单
   - [x] `update_order_status()` 更新订单状态
   - [x] `update_order_in_db()` 更新订单信息
   - [x] 正确处理 OrderSide/OrderType 序列化

6. **持仓管理集成** ✅
   - [x] 订单成交后自动更新持仓
   - [x] 使用 `PositionManager` 进行持仓计算
   - [x] 区分多空持仓

7. **错误处理** ✅
   - [x] 交易所错误正确传播
   - [x] 数据库错误正确传播
   - [x] 订单不存在返回错误
   - [x] 无效状态转换返回错误

**TODO（后续实现）**:
- BinanceExchange 交易方法实现实际API调用
- 完善事件总线集成
- 添加单元测试（需要mock exchange和database）

### P4-04: 实现 Tauri 交易命令

**任务描述**: 实现 Tauri IPC 命令，将 TradeService 功能暴露给前端调用。

**实现文件**:
- `src-tauri/src/commands/trade.rs` - 交易命令实现
- `src-tauri/src/commands/mod.rs` - 导出交易命令
- `src-tauri/src/lib.rs` - 注册交易命令到 Tauri

**核心代码结构**:

```rust
// 前端请求结构
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequest {
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub price: Option<f64>,
    pub stop_price: Option<f64>,
    pub quantity: f64,
    pub client_order_id: Option<String>,
    pub time_in_force: Option<String>,
}

// 转换为内部 OrderRequest
impl TryFrom<PlaceOrderRequest> for OrderRequest {
    type Error = String;
    fn try_from(req: PlaceOrderRequest) -> Result<Self, Self::Error> { ... }
}

// Tauri 命令列表
#[tauri::command]
pub async fn trade_place_order(
    db: State<'_, Database>,
    user_id: String,
    request: PlaceOrderRequest,
) -> Result<Order, String> { ... }

#[tauri::command]
pub async fn trade_cancel_order(
    db: State<'_, Database>,
    user_id: String,
    order_id: String,
) -> Result<(), String> { ... }

#[tauri::command]
pub async fn trade_get_order(
    db: State<'_, Database>,
    user_id: String,
    order_id: String,
) -> Result<Order, String> { ... }

#[tauri::command]
pub async fn trade_get_orders(
    db: State<'_, Database>,
    user_id: String,
    symbol: Option<String>,
    status: Option<String>,
    limit: Option<usize>,
) -> Result<Vec<Order>, String> { ... }

#[tauri::command]
pub async fn trade_get_open_orders(
    db: State<'_, Database>,
    user_id: String,
) -> Result<Vec<Order>, String> { ... }

#[tauri::command]
pub async fn trade_sync_order_status(
    db: State<'_, Database>,
    user_id: String,
    order_id: String,
) -> Result<Order, String> { ... }

#[tauri::command]
pub async fn trade_get_positions(
    db: State<'_, Database>,
    user_id: String,
) -> Result<Vec<Position>, String> { ... }

#[tauri::command]
pub async fn trade_get_balance(
    db: State<'_, Database>,
) -> Result<Vec<Balance>, String> { ... }

#[tauri::command]
pub async fn trade_cancel_all_orders(
    db: State<'_, Database>,
    user_id: String,
    symbol: Option<String>,
) -> Result<usize, String> { ... }

#[tauri::command]
pub async fn trade_close_position(
    db: State<'_, Database>,
    user_id: String,
    symbol: String,
    side: String,
    quantity: Option<f64>,
) -> Result<f64, String> { ... }
```

**验收标准**:

1. **代码结构** ✅
   - [x] `commands/trade.rs` 文件创建并包含10个交易命令
   - [x] `PlaceOrderRequest` 结构体使用 camelCase 命名
   - [x] 实现 `TryFrom<PlaceOrderRequest> for OrderRequest` 转换
   - [x] `commands/mod.rs` 导出所有交易命令
   - [x] `lib.rs` 注册所有交易命令到 invoke_handler

2. **命令完整性** ✅
   - [x] `trade_place_order` - 下单命令
   - [x] `trade_cancel_order` - 取消订单
   - [x] `trade_get_order` - 获取单个订单
   - [x] `trade_get_orders` - 获取订单列表（支持过滤）
   - [x] `trade_get_open_orders` - 获取所有开放订单
   - [x] `trade_sync_order_status` - 同步订单状态
   - [x] `trade_get_positions` - 获取持仓
   - [x] `trade_get_balance` - 获取余额
   - [x] `trade_cancel_all_orders` - 批量撤单
   - [x] `trade_close_position` - 平仓

3. **参数处理** ✅
   - [x] 使用 `State<'_, Database>` 访问数据库状态
   - [x] `user_id` 参数用于用户身份验证
   - [x] 可选参数使用 `Option<T>` 类型（symbol, status, limit等）
   - [x] 默认值处理（limit 默认 100）
   - [x] 无 serde 属性在 Tauri 命令参数上

4. **类型转换** ✅
   - [x] `PlaceOrderRequest` 转换为 `OrderRequest` 正确处理类型
   - [x] 字符串类型的 side/order_type 转换为枚举类型
   - [x] 转换错误返回清晰的错误消息
   - [x] TimeInForce 字符串正确解析

5. **错误处理** ✅
   - [x] 所有命令返回 `Result<T, String>`
   - [x] 错误使用 `.map_err(|e| e.to_string())` 转换
   - [x] 占位实现返回合理的响应
   - [x] 日志记录所有命令调用

6. **单元测试** ✅
   - [x] 测试 `PlaceOrderRequest` 到 `OrderRequest` 转换
   - [x] 测试市价单转换
   - [x] 测试限价单转换
   - [x] 测试无效 side 字符串
   - [x] 测试无效 order_type 字符串

7. **编译通过** ✅
   - [x] 项目成功编译（无错误，仅有警告）
   - [x] 所有 Tauri 命令正确注册
   - [x] 类型定义一致

**TODO（后续实现）**:
- 连接实际的 TradeService 实现（当前为占位符）
- 添加集成测试
- 实现前端调用接口

### P4-05: 实现实盘策略循环

**任务描述**: 增强策略引擎以支持实盘交易，将策略信号自动转换为实际订单并执行。

**实现文件**:
- `src-tauri/src/core/strategy/engine.rs` - 策略引擎核心实现
- `src-tauri/src/infrastructure/database.rs` - 添加 Exchange 初始化
- `src-tauri/src/commands/strategy_engine.rs` - 更新 Tauri 命令

**核心代码结构**:

```rust
// RunningInstance 添加新字段
struct RunningInstance {
    id: String,
    config: StrategyConfig,
    executor: ScriptExecutor,
    event_bus: Arc<EventBus>,
    exchange: Arc<dyn Exchange>,         // 新增：交易所接口
    user_id: String,                      // 新增：用户ID
    shutdown_tx: Option<broadcast::Sender<()>>,
    history: HashMap<String, Vec<Kline>>,
}

impl RunningInstance {
    pub fn new(
        id: String,
        config: StrategyConfig,
        event_bus: Arc<EventBus>,
        exchange: Arc<dyn Exchange>,      // 新增参数
        user_id: String,                   // 新增参数
    ) -> Result<Self> { ... }

    // 执行交易信号（新增方法）
    async fn execute_signal(&self, signal: Signal) -> Result<()> {
        // 1. 风控检查
        if !self.check_risk(&signal).await {
            return Ok(());
        }

        // 2. 转换为订单请求
        let order_request = OrderRequest {
            symbol: signal.symbol.clone(),
            side: signal.action.parse::<OrderSide>()?,
            order_type: OrderType::Market,
            price: signal.price,
            stop_price: None,
            quantity: signal.quantity,
            client_order_id: Some(format!("{}-{}", self.id, uuid::Uuid::new_v4())),
            time_in_force: Some(TimeInForce::IOC),
        };

        // 3. 执行订单
        match self.exchange.place_order(&order_request).await {
            Ok(order) => {
                self.event_bus.publish_order_placed(order);
            }
            Err(e) => {
                self.event_bus.publish_strategy_error(format!("Order failed: {}", e));
            }
        }

        Ok(())
    }

    // 风控检查（新增方法）
    async fn check_risk(&self, signal: &Signal) -> bool {
        // 基本检查
        if signal.quantity <= 0.0 { return false; }
        if let Some(price) = signal.price {
            if price <= 0.0 { return false; }
        }

        // TODO: 添加更多风控规则
        // - 账户余额检查
        // - 单笔订单金额限制
        // - 持仓限制
        // - 日内交易次数限制
        // - 最大回撤限制

        true
    }
}

// StrategyEngine 更新
pub struct StrategyEngine {
    instances: Arc<RwLock<HashMap<String, Arc<RwLock<RunningInstance>>>>>,
    event_bus: Arc<EventBus>,
    exchange: Arc<dyn Exchange>,         // 新增字段
}

impl StrategyEngine {
    pub fn new(event_bus: Arc<EventBus>, exchange: Arc<dyn Exchange>) -> Self { ... }

    pub async fn start_instance(&self, config: StrategyConfig, user_id: String) -> Result<String> {
        // 传递 exchange 和 user_id
        let instance = RunningInstance::new(
            id.clone(),
            config,
            self.event_bus.clone(),
            self.exchange.clone(),
            user_id,
        )?;
        ...
    }
}
```

**验收标准**:

1. **代码结构** ✅
   - [x] `RunningInstance` 添加 `exchange: Arc<dyn Exchange>` 字段
   - [x] `RunningInstance` 添加 `user_id: String` 字段
   - [x] `StrategyEngine` 添加 `exchange` 字段
   - [x] `new()` 构造函数接受 exchange 和 user_id 参数
   - [x] `start_instance()` 方法接受 user_id 参数

2. **信号执行功能** ✅
   - [x] `execute_signal()` 方法实现完整
   - [x] 信号转换为 OrderRequest（action → OrderSide）
   - [x] 自动生成 client_order_id（包含策略实例ID）
   - [x] 使用市价单类型（OrderType::Market）
   - [x] 使用 IOC 时间限制（TimeInForce::IOC）
   - [x] 订单成功后发布 OrderPlaced 事件
   - [x] 订单失败后发布 Error 事件

3. **风控检查功能** ✅
   - [x] `check_risk()` 方法实现
   - [x] 检查数量必须为正数
   - [x] 检查价格必须为正数（如果提供）
   - [x] 风控失败记录日志并发布错误事件
   - [x] 风控失败阻止订单执行
   - [x] 代码中包含 TODO 注释标记扩展风控规则

4. **策略循环集成** ✅
   - [x] `on_kline()` 在生成信号后调用 `execute_signal()`
   - [x] 信号发布到事件总线（publish_signal）
   - [x] 信号执行成功/失败都有日志记录
   - [x] 执行错误不中断策略循环

5. **数据库和命令集成** ✅
   - [x] `Database::new()` 创建 BinanceExchange 实例
   - [x] Exchange 传递给 StrategyEngine
   - [x] `strategy_engine_start` 命令添加 user_id 参数
   - [x] user_id 正确传递给 start_instance 方法

6. **编译通过** ✅
   - [x] 项目成功编译（无错误，仅有警告）
   - [x] 所有类型定义一致
   - [x] 方法签名匹配

7. **事件流程完整性** ✅
   - [x] K线事件 → on_bar → 信号生成 → 信号发布
   - [x] 信号 → execute_signal → 风控检查 → 订单下单
   - [x] 订单结果 → 事件发布（成功或失败）

**TODO（后续实现）**:
- 完善风控规则（余额检查、持仓限制、交易次数限制）
- 添加订单执行结果持久化
- 实现订单状态跟踪和更新
- 添加策略性能统计
- 实现 Exchange 的真实 API 调用（当前为占位符）

### P4-06: 实现策略实例持久化

**任务描述**: 实现策略实例的数据库持久化功能，支持实例的创建、查询、更新和删除操作。

**实现文件**:
- `src-tauri/src/models/strategy_instance.rs` - StrategyInstance 数据模型
- `src-tauri/src/repository/strategy_instance_repo.rs` - StrategyInstanceRepository 仓储层
- `src-tauri/src/commands/strategy_instance.rs` - 策略实例 Tauri 命令
- `src-tauri/src/infrastructure/database.rs` - 添加 repository 获取方法

**核心代码结构**:

```rust
// 策略实例模型
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StrategyInstance {
    pub id: String,
    pub strategy_id: String,
    pub user_id: String,
    pub name: String,
    pub parameters: String,      // JSON string
    pub exchange_id: String,
    pub symbol: String,
    pub timeframe: String,
    pub mode: String,            // 'paper' or 'live'
    pub status: String,          // 'running', 'stopped', 'error', 'paused'
    pub error_message: Option<String>,
    pub start_time: Option<i64>,
    pub stop_time: Option<i64>,
    pub total_trades: i64,
    pub total_pnl: f64,
    pub max_drawdown: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

impl StrategyInstance {
    pub fn new(req: CreateInstanceRequest) -> Self { ... }
    pub fn mark_running(&mut self) { ... }
    pub fn mark_stopped(&mut self) { ... }
    pub fn mark_error(&mut self, error: String) { ... }
    pub fn update_stats(&mut self, trades: i64, pnl: f64, drawdown: f64) { ... }
}

// 策略实例仓储
pub struct StrategyInstanceRepository {
    pool: Pool<Sqlite>,
}

impl StrategyInstanceRepository {
    pub async fn find_by_user(&self, user_id: &str) -> Result<Vec<StrategyInstanceListItem>>
    pub async fn find_by_status(&self, status: &str) -> Result<Vec<StrategyInstance>>
    pub async fn find_running_by_user(&self, user_id: &str) -> Result<Vec<StrategyInstance>>
    pub async fn create(&self, req: CreateInstanceRequest) -> Result<StrategyInstance>
    pub async fn update_status(&self, id: &str, status: &str, error_message: Option<&str>) -> Result<()>
    pub async fn update_stats(&self, id: &str, trades: i64, pnl: f64, drawdown: f64) -> Result<()>
    pub async fn delete(&self, id: &str) -> Result<()>
    pub async fn name_exists(&self, user_id: &str, name: &str, exclude_id: Option<&str>) -> Result<bool>
}
```

**Tauri 命令**:
```rust
#[tauri::command]
pub async fn instance_list(db: State<'_, Database>, user_id: String)
    -> Result<Vec<StrategyInstanceListItem>, String>

#[tauri::command]
pub async fn instance_get(db: State<'_, Database>, id: String)
    -> Result<Option<StrategyInstance>, String>

#[tauri::command]
pub async fn instance_create(db: State<'_, Database>, request: CreateInstanceRequest)
    -> Result<StrategyInstance, String>

#[tauri::command]
pub async fn instance_update_status(
    db: State<'_, Database>,
    id: String,
    status: String,
    error_message: Option<String>,
) -> Result<(), String>

#[tauri::command]
pub async fn instance_update_stats(
    db: State<'_, Database>,
    id: String,
    trades: i64,
    pnl: f64,
    drawdown: f64,
) -> Result<(), String>

#[tauri::command]
pub async fn instance_delete(db: State<'_, Database>, id: String)
    -> Result<(), String>

#[tauri::command]
pub async fn instance_list_running(db: State<'_, Database>, user_id: String)
    -> Result<Vec<StrategyInstance>, String>
```

**验收标准**:

1. **数据模型** ✅
   - [x] `StrategyInstance` 结构体映射数据库表字段
   - [x] `StrategyInstanceListItem` 用于列表视图（简化字段）
   - [x] `CreateInstanceRequest` 用于创建请求
   - [x] 实现 `FromRow` trait 支持 SQLx 查询
   - [x] 实现 `Serialize/Deserialize` 支持 JSON

2. **模型辅助方法** ✅
   - [x] `new()` - 创建新实例（自动生成 ID 和时间戳）
   - [x] `mark_running()` - 标记为运行中状态
   - [x] `mark_stopped()` - 标记为已停止状态
   - [x] `mark_error()` - 标记为错误状态
   - [x] `update_stats()` - 更新统计数据

3. **Repository 功能** ✅
   - [x] `find_by_user()` - 查询用户的所有实例
   - [x] `find_by_status()` - 按状态查询实例
   - [x] `find_running_by_user()` - 查询用户运行中的实例
   - [x] `create()` - 创建新实例
   - [x] `update_status()` - 更新实例状态
   - [x] `update_stats()` - 更新统计数据
   - [x] `delete()` - 删除实例
   - [x] `name_exists()` - 检查名称是否存在

4. **Repository 实现 Trait** ✅
   - [x] 实现 `find_by_id()` - 通过 ID 查询
   - [x] 实现 `find_all()` - 查询所有
   - [x] 实现 `insert()` - 插入记录
   - [x] 实现 `update()` - 更新记录
   - [x] 实现 `delete()` - 删除记录

5. **Tauri 命令** ✅
   - [x] `instance_list` - 获取用户实例列表
   - [x] `instance_get` - 获取单个实例详情
   - [x] `instance_create` - 创建新实例（含名称重复检查）
   - [x] `instance_update_status` - 更新实例状态
   - [x] `instance_update_stats` - 更新统计信息
   - [x] `instance_delete` - 删除实例
   - [x] `instance_list_running` - 获取运行中实例列表

6. **数据库集成** ✅
   - [x] `Database::strategy_instance_repo()` 获取 repository
   - [x] `repository/mod.rs` 导出 StrategyInstanceRepository
   - [x] `models/mod.rs` 导出策略实例相关类型
   - [x] `lib.rs` 注册所有策略实例命令

7. **编译通过** ✅
   - [x] 项目成功编译（无错误，仅有警告）
   - [x] 所有类型导入正确
   - [x] 所有命令正确注册

8. **单元测试** ✅
   - [x] 测试 `new()` 创建实例
   - [x] 测试 `mark_running()` 状态转换
   - [x] 测试 `mark_stopped()` 状态转换
   - [x] 测试 `mark_error()` 错误标记
   - [x] 测试 `update_stats()` 统计更新

**TODO（后续实现）**:
- 实现实例统计信息实时更新
- 添加集成测试（需要测试数据库）
- 实现实例与策略的关联验证

### P4-07: 实现策略启停控制

**任务描述**: 实现策略启动/停止时的数据库状态持久化，确保策略实例状态与数据库保持同步。

**实现文件**:
- `src-tauri/src/core/strategy/engine.rs` - StrategyEngine 启停逻辑集成
- `src-tauri/src/infrastructure/database.rs` - Database 初始化更新

**核心代码结构**:

```rust
// StrategyEngine 结构体更新
pub struct StrategyEngine {
    instances: Arc<RwLock<HashMap<String, Arc<RwLock<RunningInstance>>>>>,
    event_bus: Arc<EventBus>,
    exchange: Arc<dyn Exchange>,
    instance_repo: Arc<StrategyInstanceRepository>,  // 新增字段
}

impl StrategyEngine {
    pub fn new(
        event_bus: Arc<EventBus>,
        exchange: Arc<dyn Exchange>,
        instance_repo: Arc<StrategyInstanceRepository>,  // 新增参数
    ) -> Self { ... }

    pub async fn start_instance(&self, config: StrategyConfig, user_id: String) -> Result<String> {
        // 1. 构建 CreateInstanceRequest
        // 2. 在数据库中创建实例记录（status = "stopped"）
        // 3. 更新实例状态为 "running"
        // 4. 创建并启动 RunningInstance
        // 5. 错误时清理数据库记录
    }

    pub async fn stop_instance(&self, id: &str) -> Result<()> {
        // 1. 停止策略实例
        // 2. 从内存中移除
        // 3. 更新数据库状态为 "stopped"
    }
}

// RunningInstance 结构体更新
struct RunningInstance {
    id: String,
    config: StrategyConfig,
    executor: ScriptExecutor,
    event_bus: Arc<EventBus>,
    exchange: Arc<dyn Exchange>,
    user_id: String,
    instance_repo: Arc<StrategyInstanceRepository>,  // 新增字段
    shutdown_tx: Option<broadcast::Sender<()>>,
    history: HashMap<String, Vec<Kline>>,
}

impl RunningInstance {
    pub async fn run(&mut self) -> Result<()> {
        // on_init 失败 → 更新数据库状态为 "error"
        // on_bar 错误 → 更新数据库状态为 "error"
        // on_stop 失败 → 更新数据库状态为 "error"
    }
}
```

**启动流程**:
```
1. start_instance() 被调用
2. 构建 CreateInstanceRequest:
   - strategy_id: 从 StrategyConfig.id 获取
   - user_id: 参数传入
   - name: 从 StrategyConfig.name 获取
   - parameters: 从 StrategyConfig.parameters 获取
   - exchange_id: "default_exchange" (默认值)
   - symbol: 从 StrategyConfig.symbols[0] 获取（取第一个）
   - timeframe: 从 StrategyConfig.timeframes[0] 获取（取第一个）
   - mode: "paper" (固定值)
3. 调用 instance_repo.create() 创建数据库记录
4. 调用 instance_repo.update_status(id, "running", None) 更新状态
5. 创建 RunningInstance 并启动
6. 失败时清理数据库记录
```

**停止流程**:
```
1. stop_instance() 被调用
2. 从内存中移除实例并调用 stop()
3. 调用 instance_repo.update_status(id, "stopped", None)
4. 记录日志
```

**错误处理**:
```
1. on_init 失败:
   - 记录错误日志
   - 更新数据库状态为 "error"
   - 返回错误

2. on_bar 处理失败:
   - 记录错误日志
   - 更新数据库状态为 "error"
   - 继续运行（不中断循环）

3. on_stop 失败:
   - 记录错误日志
   - 更新数据库状态为 "error"
   - 发布停止事件

4. 策略循环异常退出:
   - 在 tokio::spawn 的错误处理中
   - 更新数据库状态为 "error"
   - 记录错误消息
```

**Database 初始化更新**:
```rust
impl Database {
    pub async fn new(db_path: PathBuf) -> Result<Self> {
        // ... 创建连接池、EventBus、Exchange ...

        // 创建 StrategyInstanceRepository
        let instance_repo = Arc::new(StrategyInstanceRepository::new(pool.clone()));

        // 创建 StrategyEngine（传入 repository）
        let strategy_engine = Arc::new(StrategyEngine::new(
            event_bus.clone(),
            exchange,
            instance_repo,
        ));

        Ok(Self { pool, event_bus, strategy_engine })
    }
}
```

**验收标准**:

1. **代码结构** ✅
   - [x] `StrategyEngine` 添加 `instance_repo: Arc<StrategyInstanceRepository>` 字段
   - [x] `StrategyEngine::new()` 接受 `instance_repo` 参数
   - [x] `RunningInstance` 添加 `instance_repo` 字段
   - [x] `RunningInstance::new()` 接受 `instance_repo` 参数
   - [x] `Database::new()` 创建 StrategyInstanceRepository 并传递给 StrategyEngine

2. **start_instance 功能** ✅
   - [x] 从 StrategyConfig 构建 CreateInstanceRequest
   - [x] strategy_id 从 config.id 获取
   - [x] name 从 config.name 获取
   - [x] parameters 从 config.parameters 获取
   - [x] symbol 从 config.symbols[0] 获取（有默认值 "BTCUSDT"）
   - [x] timeframe 从 config.timeframes[0] 获取（有默认值 "1h"）
   - [x] exchange_id 使用 "default_exchange"
   - [x] mode 使用 "paper"
   - [x] 调用 instance_repo.create() 创建数据库记录
   - [x] 调用 instance_repo.update_status(id, "running", None)
   - [x] 创建失败时清理数据库记录
   - [x] 所有操作都有日志记录

3. **stop_instance 功能** ✅
   - [x] 停止策略实例（调用 instance.stop()）
   - [x] 从内存 HashMap 中移除实例
   - [x] 调用 instance_repo.update_status(id, "stopped", None)
   - [x] 所有操作都有日志记录

4. **run() 错误处理** ✅
   - [x] on_init 失败时更新数据库状态为 "error"
   - [x] on_bar 处理失败时更新数据库状态为 "error"
   - [x] on_stop 失败时更新数据库状态为 "error"
   - [x] tokio::spawn 错误处理中更新数据库状态
   - [x] 所有错误都记录详细的错误消息

5. **数据一致性** ✅
   - [x] 启动成功: 数据库状态 = "running"
   - [x] 停止成功: 数据库状态 = "stopped"
   - [x] 启动失败: 数据库记录被清理
   - [x] 运行错误: 数据库状态 = "error"，包含错误消息
   - [x] start_time/stop_time 正确更新

6. **日志记录** ✅
   - [x] 数据库记录创建时记录日志
   - [x] 状态更新时记录日志
   - [x] 错误发生时记录详细错误日志
   - [x] 使用适当的日志级别（info/error/warn）

7. **编译通过** ✅
   - [x] 项目成功编译（无错误）
   - [x] 所有类型匹配
   - [x] 所有权和借用正确
   - [x] async/await 正确使用

8. **边界情况处理** ✅
   - [x] symbols 为空时使用默认值
   - [x] timeframes 为空时使用默认值
   - [x] 数据库操作失败时正确处理
   - [x] 状态更新失败时记录日志

**TODO（后续实现）**:
- 实现实例统计信息实时更新（total_trades, total_pnl, max_drawdown）
- 添加集成测试验证数据库状态同步
- 实现实例恢复逻辑（应用重启后从数据库恢复运行中实例）
- 添加实例状态变化事件通知

### P4-08: 实现策略状态查询

**任务描述**: 实现策略实例状态查询功能，支持获取实例列表、单个实例详情和运行状态。

**实现文件**:
- `src-tauri/src/commands/strategy_instance.rs` - 添加实例查询命令
- `src-tauri/src/core/strategy/engine.rs` - 添加查询方法

**核心功能**:
```rust
// StrategyEngine 查询方法
impl StrategyEngine {
    pub async fn list_instances(&self) -> Vec<InstanceInfo> { ... }
    pub async fn get_instance(&self, id: &str) -> Option<InstanceInfo> { ... }
}

// Tauri 命令
#[tauri::command]
pub async fn strategy_instance_list_all(
    db: State<'_, Database>,
) -> Result<Vec<InstanceInfo>, String>

#[tauri::command]
pub async fn strategy_instance_get(
    db: State<'_, Database>,
    id: String,
) -> Result<Option<InstanceInfo>, String>
```

**验收标准**:
1. **查询功能** ✅
   - [x] list_instances() 返回所有运行中实例信息
   - [x] get_instance(id) 返回指定实例信息
   - [x] 实例不存在时返回 None

2. **信息完整性** ✅
   - [x] InstanceInfo 包含 id, name, status, symbols, timeframes
   - [x] 状态实时反映实例运行状态

3. **Tauri 命令** ✅
   - [x] strategy_instance_list_all 命令实现
   - [x] strategy_instance_get 命令实现
   - [x] 正确的错误处理

**实现详情**:
- `strategy_instance_list_all()` - 调用 `db.get_strategy_engine().list_instances()`
- `strategy_instance_get(id)` - 调用 `db.get_strategy_engine().get_instance(&id)`
- 返回类型: `InstanceInfo` (已在 `engine.rs` 中定义)
- 命令已注册到 Tauri invoke_handler
- 编译通过，无错误

**TODO** (后续实现):
- 添加单元测试
- 实现实例状态变化的实时通知机制

### P4-09: 实现 TradeStore

**任务描述**: 实现 Pinia store 用于管理前端交易状态，包括订单、持仓、交易历史和加载状态。

**实现文件**:
- `src/store/modules/trade.ts` - TradeStore 完整实现

**核心代码结构**:

```typescript
// Place Order Request Interface
export interface PlaceOrderRequest {
  symbol: string;
  side: OrderSide;
  orderType: OrderType;
  quantity: number;
  price?: number;
  stopPrice?: number;
  clientOrderId?: string;
  timeInForce?: 'GTC' | 'IOC' | 'FOK';
}

export const useTradeStore = defineStore('trade', () => {
  // ========== State ==========
  const orders = ref<Order[]>([]);
  const positions = ref<Position[]>([]);
  const trades = ref<any[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);

  // ========== Computed Getters ==========
  const activeOrders = computed(() =>
    orders.value.filter(
      o => o.status === 'open' || o.status === 'partially_filled'
    )
  );

  const completedOrders = computed(() =>
    orders.value.filter(
      o => o.status === 'filled' || o.status === 'canceled' || o.status === 'rejected'
    )
  );

  const getOrdersBySymbol = computed(() => {
    return (symbol: string) => orders.value.filter(o => o.symbol === symbol);
  });

  const getPositionsBySymbol = computed(() => {
    return (symbol: string) => positions.value.filter(p => p.symbol === symbol);
  });

  const totalUnrealizedPnl = computed(() =>
    positions.value.reduce((sum, p) => sum + p.unrealizedPnl, 0)
  );

  const totalRealizedPnl = computed(() =>
    positions.value.reduce((sum, p) => sum + p.realizedPnl, 0)
  );

  // ========== Actions ==========
  async function fetchOrders(userId: string, symbol?: string, status?: OrderStatus) { ... }
  async function fetchOpenOrders(userId: string) { ... }
  async function fetchPositions(userId: string) { ... }
  async function placeOrder(userId: string, request: PlaceOrderRequest) { ... }
  async function cancelOrder(userId: string, orderId: string) { ... }
  async function cancelAllOrders(userId: string, symbol?: string) { ... }
  async function syncOrderStatus(userId: string, orderId: string) { ... }
  async function closePosition(userId: string, symbol: string, side: 'long' | 'short', quantity?: number) { ... }
  async function getBalance() { ... }

  function updateOrder(order: Order) { ... }
  function updatePosition(position: Position) { ... }
  function addTrade(trade: any) { ... }
  function clear() { ... }
  async function initialize(userId: string) { ... }

  return {
    // State
    orders,
    positions,
    trades,
    loading,
    error,

    // Computed
    activeOrders,
    completedOrders,
    getOrdersBySymbol,
    getPositionsBySymbol,
    totalUnrealizedPnl,
    totalRealizedPnl,
    longPositions,
    shortPositions,

    // Actions
    fetchOrders,
    fetchOpenOrders,
    fetchPositions,
    placeOrder,
    cancelOrder,
    cancelAllOrders,
    syncOrderStatus,
    closePosition,
    getBalance,
    updateOrder,
    updatePosition,
    addTrade,
    clear,
    initialize,
  };
});
```

**验收标准**:

1. **代码结构** ✅
   - [x] 使用 Pinia composition API 风格
   - [x] 导出 PlaceOrderRequest 接口定义
   - [x] State 包含 orders, positions, trades, loading, error
   - [x] 所有必要的类型导入自 @/types

2. **State 管理** ✅
   - [x] orders 存储所有订单
   - [x] positions 存储所有持仓
   - [x] trades 存储交易历史
   - [x] loading 跟踪加载状态
   - [x] error 存储错误信息

3. **Computed Getters** ✅
   - [x] activeOrders - 过滤活跃订单（open 或 partially_filled）
   - [x] completedOrders - 过滤已完成订单（filled, canceled, rejected）
   - [x] getOrdersBySymbol - 按交易对过滤订单（返回函数）
   - [x] getPositionsBySymbol - 按交易对过滤持仓（返回函数）
   - [x] totalUnrealizedPnl - 计算总未实现盈亏
   - [x] totalRealizedPnl - 计算总已实现盈亏
   - [x] longPositions - 过滤多头持仓
   - [x] shortPositions - 过滤空头持仓

4. **Actions - 订单操作** ✅
   - [x] fetchOrders() - 获取订单列表（支持 symbol 和 status 过滤）
   - [x] fetchOpenOrders() - 获取开放订单
   - [x] placeOrder() - 下单（转换请求格式并更新本地状态）
   - [x] cancelOrder() - 取消订单（更新本地状态为 canceled）
   - [x] cancelAllOrders() - 批量撤单（支持可选 symbol 参数）
   - [x] syncOrderStatus() - 同步订单状态（更新本地订单）

5. **Actions - 持仓操作** ✅
   - [x] fetchPositions() - 获取持仓列表
   - [x] closePosition() - 平仓（支持可选数量参数）
   - [x] closePosition() 完成后刷新持仓列表

6. **Actions - 账户操作** ✅
   - [x] getBalance() - 获取账户余额

7. **Actions - 本地更新** ✅
   - [x] updateOrder() - 更新本地订单（用于 WebSocket 事件）
   - [x] updatePosition() - 更新本地持仓（用于 WebSocket 事件）
   - [x] addTrade() - 添加交易记录（保持最多 1000 条）
   - [x] clear() - 清除所有数据

8. **Actions - 初始化** ✅
   - [x] initialize() - 并行加载开放订单和持仓
   - [x] initialize() 处理错误

9. **错误处理** ✅
   - [x] 所有 async 函数有 try-catch 错误处理
   - [x] 错误时设置 error.value
   - [x] 错误时记录 console.error
   - [x] finally 块确保 loading.value 重置
   - [x] 所有错误抛出让调用者处理

10. **类型安全** ✅
    - [x] 正确使用 Order, Position, OrderSide, OrderType, OrderStatus 类型
    - [x] PlaceOrderRequest 接口定义完整
    - [x] 使用 api.invokeRaw 调用 Tauri 命令
    - [x] 所有返回值有正确的类型注解

11. **与后端 API 集成** ✅
    - [x] fetchOrders → trade_get_orders
    - [x] fetchOpenOrders → trade_get_open_orders
    - [x] fetchPositions → trade_get_positions
    - [x] placeOrder → trade_place_order
    - [x] cancelOrder → trade_cancel_order
    - [x] cancelAllOrders → trade_cancel_all_orders
    - [x] syncOrderStatus → trade_sync_order_status
    - [x] closePosition → trade_close_position
    - [x] getBalance → trade_get_balance

12. **状态一致性** ✅
    - [x] placeOrder 成功后添加到本地 orders 列表
    - [x] cancelOrder 成功后更新本地订单状态
    - [x] cancelAllOrders 批量更新本地订单状态
    - [x] syncOrderStatus 更新本地订单数据
    - [x] closePosition 刷新持仓列表

13. **导出完整性** ✅
    - [x] 导出所有 state
    - [x] 导出所有 computed getters
    - [x] 导出所有 actions
    - [x] store/index.ts 已导出 trade store

14. **编译通过** ✅
    - [x] TypeScript 编译无错误
    - [x] 所有类型定义正确
    - [x] 与其他 store 模式一致（参考 market.ts）

**使用示例**:
```typescript
import { useTradeStore } from '@/store/modules/trade';

const tradeStore = useTradeStore();

// 初始化
await tradeStore.initialize(userId);

// 下单
await tradeStore.placeOrder(userId, {
  symbol: 'BTCUSDT',
  side: OrderSide.BUY,
  orderType: OrderType.MARKET,
  quantity: 0.001,
});

// 撤单
await tradeStore.cancelOrder(userId, orderId);

// 获取活跃订单
const activeOrders = tradeStore.activeOrders;

// 获取总盈亏
const totalPnl = tradeStore.totalUnrealizedPnl + tradeStore.totalRealizedPnl;
```

**TODO（后续实现）**:
- 添加单元测试（需要 mock Tauri API）
- 实现 WebSocket 事件监听集成
- 添加交易历史筛选和分页功能
- 实现订单和持仓的实时更新机制

### P4-10: 实现订单列表组件

**任务描述**: 实现订单列表组件，显示所有订单信息，支持筛选、撤单等操作。

**实现文件**:
- `src/components/Trade/OrderList.vue` - 订单列表组件

**验收标准**:

1. **组件结构** ✅
   - [x] 使用 Vue 3 Composition API (`<script setup>`)
   - [x] Props: userId, symbol?, autoRefresh?
   - [x] Emits: order-canceled
   - [x] 集成 useTradeStore

2. **显示功能** ✅
   - [x] 显示订单 ID（前12位省略显示）
   - [x] 显示交易对、方向（买入/卖出标签）、类型（市价/限价）
   - [x] 显示数量、价格、已成交数量、均价
   - [x] 显示状态（带颜色标签）
   - [x] 显示创建时间（相对时间）

3. **筛选功能** ✅
   - [x] 全部/活跃/已完成 状态筛选
   - [x] 按交易对筛选（通过 symbol prop）
   - [x] 按创建时间倒序排序

4. **操作功能** ✅
   - [x] 撤单按钮（仅活跃订单显示）
   - [x] 撤单前显示确认对话框
   - [x] 撤单成功后显示消息并触发事件
   - [x] 刷新按钮

5. **样式** ✅
   - [x] 买入绿色标签，卖出红色标签
   - [x] 状态标签带颜色区分
   - [x] 已完成订单半透明显示
   - [x] 已撤销订单删除线显示
   - [x] 响应式布局

6. **类型安全** ✅
   - [x] 正确使用 Order, OrderSide, OrderType, OrderStatus 类型
   - [x] Props 类型定义完整
   - [x] Emits 类型定义完整

**使用示例**:
```vue
<OrderList
  :user-id="userId"
  symbol="BTCUSDT"
  :auto-refresh="true"
  @order-canceled="handleOrderCanceled"
/>
```

### P4-11: 实现持仓列表组件

**任务描述**: 实现持仓列表组件，显示所有持仓信息，支持平仓操作。

**实现文件**:
- `src/components/Trade/PositionList.vue` - 持仓列表组件

**验收标准**:

1. **组件结构** ✅
   - [x] 使用 Vue 3 Composition API (`<script setup>`)
   - [x] Props: userId, symbol?, autoRefresh?
   - [x] Emits: position-closed
   - [x] 集成 useTradeStore

2. **显示功能** ✅
   - [x] 显示交易对、方向（多头/空头标签）
   - [x] 显示数量、开仓价、当前价
   - [x] 显示未实现盈亏（带颜色）
   - [x] 显示已实现盈亏（带颜色）
   - [x] 显示开仓时间（相对时间）
   - [x] 顶部汇总显示总盈亏

3. **操作功能** ✅
   - [x] 平仓按钮
   - [x] 平仓前显示确认对话框
   - [x] 平仓成功后显示消息并触发事件
   - [x] 刷新按钮

4. **样式** ✅
   - [x] 多头绿色标签，空头红色标签
   - [x] 盈利显示绿色，亏损显示红色
   - [x] 盈利持仓浅绿背景，亏损持仓浅红背景
   - [x] 按未实现盈亏绝对值排序
   - [x] 响应式布局

5. **类型安全** ✅
   - [x] 正确使用 Position 类型
   - [x] Props 类型定义完整
   - [x] Emits 类型定义完整

**使用示例**:
```vue
<PositionList
  :user-id="userId"
  symbol="BTCUSDT"
  :auto-refresh="true"
  @position-closed="handlePositionClosed"
/>
```

### P4-12: 实现手动下单表单

**任务描述**: 实现手动下单表单组件，支持市价单和限价单下单。

**实现文件**:
- `src/components/Trade/OrderForm.vue` - 下单表单组件

**验收标准**:

1. **组件结构** ✅
   - [x] 使用 Vue 3 Composition API (`<script setup>`)
   - [x] Props: userId, defaultSymbol?
   - [x] Emits: order-placed
   - [x] 使用 Element Plus 表单组件
   - [x] 集成 useTradeStore 和 useMarketStore

2. **表单字段** ✅
   - [x] 交易对选择器（可筛选）
   - [x] 买卖方向（单选按钮）
   - [x] 订单类型（市价/限价下拉）
   - [x] 价格输入（仅限价单，带精度控制）
   - [x] 数量输入（带精度控制）
   - [x] 快捷数量按钮（25%, 50%, 75%, 100%）

3. **功能** ✅
   - [x] 表单验证（必填字段、正数检查）
   - [x] 限价单价格验证
   - [x] 显示当前价格（从 MarketStore）
   - [x] 显示可用数量（买入/卖出分别计算）
   - [x] 显示账户余额
   - [x] 显示订单预估价值和手续费
   - [x] 下单成功后清空表单并显示消息

4. **样式** ✅
   - [x] 买入绿色样式，卖出红色样式
   - [x] 提交按钮颜色随方向变化
   - [x] 加载状态显示
   - [x] 响应式布局

5. **类型安全** ✅
   - [x] 正确使用 OrderSide, OrderType 类型
   - [x] PlaceOrderRequest 接口定义
   - [x] Props 类型定义完整

**使用示例**:
```vue
<OrderForm
  :user-id="userId"
  default-symbol="BTCUSDT"
  @order-placed="handleOrderPlaced"
/>
```

### P4-13: 实现交易控制台页面

**任务描述**: 实现交易控制台主页面，整合订单、持仓、下单等所有交易组件。

**实现文件**:
- `src/views/Trade/TradeConsole.vue` - 交易控制台页面

**验收标准**:

1. **页面结构** ✅
   - [x] 使用 Vue 3 Composition API (`<script setup>`)
   - [x] 页面头部（标题、余额、盈亏）
   - [x] 策略实例面板（有运行策略时显示）
   - [x] 下单表单（左列）
   - [x] 持仓列表（右列）
   - [x] 订单列表（右列）

2. **组件集成** ✅
   - [x] 使用 OrderForm 组件
   - [x] 使用 PositionList 组件（内联实现）
   - [x] 使用 OrderList 组件（内联实现）
   - [x] 运行策略列表（显示实例状态）

3. **功能** ✅
   - [x] 显示账户余额
   - [x] 显示总未实现盈亏（带颜色）
   - [x] 显示运行策略列表
   - [x] 停止策略按钮
   - [x] 全部平仓按钮
   - [x] 撤单功能
   - [x] 平仓功能
   - [x] 自动刷新（每5秒）

4. **Store 集成** ✅
   - [x] 使用 useUserStore
   - [x] 使用 useTradeStore
   - [x] 使用 useMarketStore
   - [x] 调用 Tauri 命令

5. **样式** ✅
   - [x] 响应式两列布局
   - [x] 盈亏颜色显示
   - [x] 卡片阴影效果
   - [x] 徽章显示数量
   - [x] 加载状态处理

6. **类型安全** ✅
   - [x] 正确使用所有类型
   - [x] InstanceStatus, OrderStatus 枚举
   - [x] 正确的 API 调用

**使用示例**:
```typescript
// 在路由配置中注册
{
  path: '/trade',
  component: () => import('@/views/Trade/TradeConsole.vue'),
  meta: { requiresAuth: true }
}
```

---

**P4-09 ~ P4-13 完成总结**:

所有前端交易组件已完成实现：
- ✅ P4-09: TradeStore (435 行，14 个 actions)
- ✅ P4-10: OrderList 组件 (订单列表)
- ✅ P4-11: PositionList 组件 (持仓列表)
- ✅ P4-12: OrderForm 组件 (下单表单)
- ✅ P4-13: TradeConsole 页面 (交易控制台)

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
