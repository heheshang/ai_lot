# AI-LOT 量化交易系统架构设计

## 1. 系统概述

### 1.1 系统定位

AI-LOT 是一个**本地优先**的量化交易桌面应用，核心特点：

| 特点 | 说明 |
|------|------|
| 本地化 | 所有数据存储在本地，API密钥不离开用户设备 |
| 低延迟 | 本地执行策略，直连交易所，无中间服务器 |
| 跨平台 | 支持 Windows / macOS / Linux |
| 桌面应用 | Tauri 框架，体积小、资源占用低 |

### 1.2 核心业务流程

```
用户登录 → 配置交易所 → 编写/导入策略 → 回测验证 → 部署实盘 → 监控运行
```

### 1.3 技术栈

```
┌─────────────────────────────────────────────────────────────┐
│                    技术栈总览                               │
├─────────────────────────────────────────────────────────────┤
│  前端    │ Vue 3 + TypeScript + Pinia + Element Plus      │
│  桌面    │ Tauri 2.x (Rust后端 + WebView前端)             │
│  后端    │ Rust + Tokio (异步运行时)                       │
│  数据库  │ SQLite + sqlx (编译时SQL检查)                   │
│  加密    │ AES-256-GCM + argon2 (密码哈希)                 │
│  图表    │ ECharts (K线图、深度图)                         │
│  编辑器  │ Monaco Editor (VS Code同款)                    │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. 系统架构

### 2.1 分层架构（自上而下）

```
┌──────────────────────────────────────────────────────────────┐
│                      表现层 (Presentation)                    │
│                         Vue 3 SPA                           │
│  ┌──────────┬──────────┬──────────┬──────────┬─────────────┐ │
│  │ Dashboard│  Market  │ Strategy │  Trade   │    Risk     │ │
│  │  (仪表盘) │ (行情)   │ (策略)   │ (交易)   │  (风控)     │ │
│  └──────────┴──────────┴──────────┴──────────┴─────────────┘ │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼ Tauri IPC (进程间通信)
┌──────────────────────────────────────────────────────────────┐
│                      应用层 (Application)                     │
│                         Rust Backend                         │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │                   Commands (Tauri命令)                   │ │
│  │  接收前端请求，调用Service层处理，返回结果               │ │
│  └─────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│                       服务层 (Service)                        │
│  ┌─────────────┬─────────────┬─────────────┬───────────────┐ │
│  │UserService  │MarketService│StrategySvc  │ TradeService  │ │
│  │(用户认证)   │(行情数据)   │(策略执行)   │ (订单管理)    │ │
│  ├─────────────┼─────────────┼─────────────┼───────────────┤ │
│  │AuthService  │DataService  │BacktestSvc  │ RiskService   │ │
│  │(权限控制)   │(数据访问)   │(回测引擎)   │ (风控监控)    │ │
│  └─────────────┴─────────────┴─────────────┴───────────────┘ │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│                     领域层 (Domain)                           │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │                    核心业务逻辑                          │ │
│  │  ┌──────────────┬──────────────┬──────────────────────┐ │ │
│  │  │  Exchange    │  Strategy    │     Backtest         │ │ │
│  │  │  (交易所抽象) │  (策略引擎)  │    (回测引擎)        │ │ │
│  │  └──────────────┴──────────────┴──────────────────────┘ │ │
│  │  ┌──────────────┬──────────────┬──────────────────────┐ │ │
│  │  │   OrderBook  │   Position   │      RiskRule        │ │ │
│  │  │  (订单簿管理) │  (持仓管理)  │    (风控规则)        │ │ │
│  │  └──────────────┴──────────────┴──────────────────────┘ │ │
│  └─────────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│                   基础设施层 (Infrastructure)                 │
│  ┌─────────────┬─────────────┬─────────────┬───────────────┐ │
│  │   Database  │    Crypto   │   Logger    │   Notifier    │ │
│  │  (SQLite)   │  (AES加密)  │  (日志记录)  │  (消息通知)   │ │
│  └─────────────┴─────────────┴─────────────┴───────────────┘ │
└──────────────────────────────────────────────────────────────┘
```

### 2.2 模块职责说明

| 层级 | 职责 | 示例 |
|------|------|------|
| **表现层** | UI展示、用户交互 | 渲染K线图、表单验证、路由跳转 |
| **应用层** | 请求路由、参数校验 | Tauri命令接收、参数解析、错误转换 |
| **服务层** | 业务编排、事务管理 | 启动策略（涉及多个表的读写） |
| **领域层** | 核心业务逻辑、实体 | 订单状态机、持仓计算、风控规则判定 |
| **基础设施层** | 通用技术服务 | 加密解密、日志、数据库连接 |

---

## 3. 核心数据流

### 3.1 实时行情数据流

```
                    ┌─────────────┐
                    │   Exchange  │
                    │  (Binance)  │
                    └──────┬──────┘
                           │ WebSocket
                           ▼
                    ┌─────────────┐
                    │MarketClient │
                    │  (Rust)     │
                    └──────┬──────┘
                           │ 解析
                           ▼
                    ┌─────────────┐
                    │ EventBus    │
                    │ (广播通道)   │
                    └──────┬──────┘
                           │
           ┌───────────────┼───────────────┐
           ▼               ▼               ▼
    ┌───────────┐  ┌───────────┐  ┌───────────┐
    │  Strategy │  │   Cache   │  │  Frontend │
    │  Engine   │  │  (SQLite) │  │   (Vue)   │
    └───────────┘  └───────────┘  └───────────┘
```

### 3.2 策略执行数据流

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   User      │────▶│ Strategy    │────▶│  Exchange   │
│  (手动启动)  │     │   Engine    │     │  (下单)     │
└─────────────┘     └──────┬──────┘     └─────────────┘
                           │
                           ▼ (接收行情)
                    ┌─────────────┐
                    │ EventBus    │
                    │  (MarketEvent)
                    └─────────────┘
                           │
                           ▼ (策略逻辑)
                    ┌─────────────┐
                    │  Strategy   │
                    │   Script    │
                    │ (JavaScript)│
                    └──────┬──────┘
                           │
                           ▼ (生成信号)
                    ┌─────────────┐
                    │   Signal    │
                    │  (buy/sell) │
                    └──────┬──────┘
                           │
                           ▼
                    ┌─────────────┐
                    │ RiskCheck   │
                    │  (风控检查)  │
                    └──────┬──────┘
                           │ 通过
                           ▼
                    ┌─────────────┐
                    │   Exchange  │
                    │  (执行订单)  │
                    └─────────────┘
```

### 3.3 回测数据流

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   User      │────▶│ Backtest    │────▶│ Report      │
│ (发起回测)   │     │   Engine    │     │  (报告)     │
└─────────────┘     └──────┬──────┘     └─────────────┘
                           │
                           ▼ (加载历史数据)
                    ┌─────────────┐
                    │ DataFeed    │
                    │  (SQLite)   │
                    └──────┬──────┘
                           │
                           ▼ (逐根K线回放)
                    ┌─────────────┐
                    │  Strategy   │
                    │   Script    │
                    └──────┬──────┘
                           │
                           ▼ (模拟成交)
                    ┌─────────────┐
                    │ Simulated   │
                    │  Exchange   │
                    │ (模拟交易所) │
                    └──────┬──────┘
                           │
                           ▼ (记录每笔交易)
                    ┌─────────────┐
                    │  Trades     │
                    │  Log        │
                    └─────────────┘
                           │
                           ▼ (计算指标)
                    ┌─────────────┐
                    │  Metrics    │
                    │ (夏普/回撤)  │
                    └─────────────┘
```

---

## 4. 核心模块设计

### 4.1 模块关系图

```
┌─────────────────────────────────────────────────────────────────┐
│                        模块依赖关系                              │
└─────────────────────────────────────────────────────────────────┘

    ┌──────────┐       ┌──────────┐       ┌──────────┐
    │  Frontend │◄─────►│ Commands │◄─────►│ Services │
    │   (Vue)   │  IPC  │ (Tauri)  │       │  (Rust)  │
    └──────────┘       └─────┬────┘       └─────┬────┘
                            │                   │
                            │         ┌─────────┴─────────┐
                            │         ▼                   ▼
                            │    ┌──────────┐        ┌──────────┐
                            │    │ Exchange │        │ Strategy │
                            │    │  Trait   │        │  Engine  │
                            │    └──────────┘        └──────────┘
                            │         │                   │
                            │         └─────────┬─────────┘
                            │                   ▼
                            │            ┌──────────┐
                            │            │ EventBus │
                            │            └─────┬────┘
                            │                  │
                            ▼                  ▼
                       ┌─────────────────────────────┐
                       │       Repository           │
                       │      (SQLite)              │
                       └─────────────────────────────┘
```

### 4.2 Exchange Trait（交易所抽象）

**设计目的**：统一不同交易所的API差异，方便扩展新交易所。

```rust
// core/trade/exchange/trait.rs

#[derive(Debug, Clone, PartialEq)]
pub enum ExchangeName {
    Binance,
    OKX,
    Bybit,
}

/// 交易所统一接口
#[async_trait]
pub trait Exchange: Send + Sync {
    // ========== 元数据 ==========
    fn name(&self) -> ExchangeName;
    fn is_connected(&self) -> bool;

    // ========== 连接管理 ==========
    async fn connect(&self) -> Result<()>;
    async fn disconnect(&self) -> Result<()>;

    // ========== 行情数据 (REST) ==========
    async fn get_ticker(&self, symbol: &str) -> Result<Ticker>;
    async fn get_klines(
        &self,
        symbol: &str,
        interval: Interval,
        limit: usize,
    ) -> Result<Vec<Kline>>;

    // ========== 行情数据 (WebSocket订阅) ==========
    async fn subscribe_ticker(&self, symbols: Vec<String>) -> Result<()>;
    async fn subscribe_kline(
        &self,
        symbols: Vec<String>,
        interval: Interval,
    ) -> Result<()>;

    // ========== 账户数据 ==========
    async fn get_balance(&self) -> Result<Vec<Balance>>;
    async fn get_positions(&self) -> Result<Vec<Position>>;
    async fn get_open_orders(&self, symbol: Option<&str>) -> Result<Vec<Order>>;

    // ========== 交易操作 ==========
    async fn place_order(&self, request: &OrderRequest) -> Result<Order>;
    async fn cancel_order(&self, symbol: &str, order_id: &str) -> Result<()>;
    async fn cancel_all_orders(&self, symbol: &str) -> Result<()>;

    // ========== 事件流 ==========
    /// 订阅实时行情事件流
    fn ticker_stream(&self) -> BroadcastStream<Ticker>;
    fn kline_stream(&self) -> BroadcastStream<Kline>;

    /// 订阅订单更新事件流
    fn order_stream(&self) -> BroadcastStream<Order>;

    /// 订阅持仓更新事件流
    fn position_stream(&self) -> BroadcastStream<Position>;
}
```

**实现示例**：

```rust
// core/trade/exchange/binance.rs

pub struct BinanceExchange {
    config: ExchangeConfig,
    ws_client: Arc<Mutex<Option<WsClient>>>,
    event_bus: EventBus,
}

#[async_trait]
impl Exchange for BinanceExchange {
    fn name(&self) -> ExchangeName {
        ExchangeName::Binance
    }

    async fn get_ticker(&self, symbol: &str) -> Result<Ticker> {
        // 调用Binance API
        let url = format!("{}/ticker/price?symbol={}", BASE_URL, symbol);
        let response = reqwest::get(url).await?.json().await?;
        Ok(response)
    }

    // ... 其他方法实现
}
```

### 4.3 Strategy Engine（策略引擎）

**核心概念**：

| 概念 | 说明 |
|------|------|
| `Strategy` | 策略模板（代码+参数定义） |
| `StrategyInstance` | 策略实例（配置好的运行单元） |
| `StrategyContext` | 策略执行上下文（提供API给脚本） |

```rust
// core/strategy/engine.rs

pub struct StrategyEngine {
    /// 运行中的策略实例
    instances: Arc<RwLock<HashMap<InstanceId, RunningInstance>>>,

    /// 交易所管理器
    exchanges: Arc<ExchangeManager>,

    /// 事件总线
    event_bus: EventBus,
}

impl StrategyEngine {
    /// 启动一个策略实例
    pub async fn start_instance(
        &self,
        config: StrategyConfig,
    ) -> Result<InstanceId> {
        // 1. 创建实例ID
        let instance_id = Uuid::new_v4().to_string();

        // 2. 加载交易所
        let exchange = self.exchanges.get(&config.exchange_id)?;

        // 3. 创建运行时实例
        let instance = RunningInstance::new(
            instance_id.clone(),
            config,
            exchange.clone(),
            self.event_bus.clone(),
        );

        // 4. 启动策略循环
        let instance_clone = instance.clone();
        tokio::spawn(async move {
            instance_clone.run().await;
        });

        // 5. 注册到引擎
        self.instances.write().await.insert(instance_id.clone(), instance);

        Ok(instance_id)
    }

    /// 停止策略实例
    pub async fn stop_instance(&self, instance_id: &InstanceId) -> Result<()> {
        let mut instances = self.instances.write().await;
        if let Some(instance) = instances.remove(instance_id) {
            instance.stop().await;
        }
        Ok(())
    }
}

/// 运行中的策略实例
pub struct RunningInstance {
    id: InstanceId,
    config: StrategyConfig,
    exchange: Arc<dyn Exchange>,
    event_bus: EventBus,
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
}

impl RunningInstance {
    fn new(
        id: String,
        config: StrategyConfig,
        exchange: Arc<dyn Exchange>,
        event_bus: EventBus,
    ) -> Self {
        Self {
            id,
            config,
            exchange,
            event_bus,
            shutdown_tx: None,
        }
    }

    /// 策略主循环
    async fn run(mut self) {
        // 1. 订阅必要的事件流
        let mut kline_stream = self.exchange.kline_stream();

        // 2. 初始化策略脚本
        let mut strategy = StrategyScript::new(
            &self.config.code,
            &self.config.params,
        );

        // 3. 执行策略初始化回调
        strategy.on_init().await;

        // 4. 进入事件循环
        loop {
            tokio::select! {
                // 接收K线事件
                Some(event) = kline_stream.recv() => {
                    if let Some(signal) = strategy.on_bar(&event).await {
                        self.execute_signal(signal).await;
                    }
                }

                // 接收停止信号
                _ = self.wait_for_shutdown() => {
                    break;
                }
            }
        }

        // 5. 清理
        strategy.on_stop().await;
    }

    async fn execute_signal(&self, signal: Signal) {
        // 1. 风控检查
        if !self.check_risk(&signal).await {
            return;
        }

        // 2. 执行订单
        match signal.side {
            SignalSide::Buy => {
                self.exchange.place_order(&signal.into()).await.ok();
            }
            SignalSide::Sell => {
                self.exchange.place_order(&signal.into()).await.ok();
            }
        }
    }
}
```

### 4.4 Backtest Engine（回测引擎）

```rust
// core/backtest/engine.rs

pub struct BacktestEngine {
    datafeed: HistoricalDatafeed,
    broker: Broker,
    exchange: SimulatedExchange,
}

impl BacktestEngine {
    pub async fn run(
        &mut self,
        strategy_code: &str,
        params: &BacktestParams,
    ) -> Result<BacktestReport> {
        // 1. 加载历史数据
        let klines = self.datafeed.load_klines(
            &params.symbol,
            &params.interval,
            params.start_time,
            params.end_time,
        ).await?;

        // 2. 初始化策略脚本
        let mut strategy = StrategyScript::new(strategy_code, &params.strategy_params);

        // 3. 逐根K线回放
        strategy.on_init().await;

        for kline in klines {
            // 更新模拟交易所状态
            self.exchange.update_kline(kline.clone());

            // 执行策略逻辑
            if let Some(signal) = strategy.on_bar(&kline).await {
                // 模拟成交
                self.broker.execute(signal, &mut self.exchange).await;
            }

            // 更新持仓和盈亏
            self.exchange.update_positions();
        }

        // 4. 平仓所有持仓
        self.broker.close_all_positions(&mut self.exchange).await;

        // 5. 生成报告
        let report = self.generate_report()?;

        strategy.on_stop().await;

        Ok(report)
    }
}
```

---

## 5. 前端架构

### 5.1 目录结构

```
src/
├── main.ts                 # 应用入口
├── App.vue                 # 根组件
├── router/
│   └── index.ts            # 路由配置
├── store/
│   ├── index.ts            # Pinia入口
│   ├── user.ts             # 用户状态
│   ├── market.ts           # 行情状态
│   ├── strategy.ts         # 策略状态
│   └── trade.ts            # 交易状态
├── views/
│   ├── Dashboard.vue       # 仪表盘
│   ├── MarketView.vue      # 行情页面
│   ├── StrategyList.vue    # 策略列表
│   ├── StrategyEditor.vue  # 策略编辑器
│   ├── BacktestView.vue    # 回测页面
│   ├── TradeConsole.vue    # 交易控制台
│   └── Settings.vue        # 设置页面
├── components/
│   ├── KlineChart.vue      # K线图组件
│   ├── OrderBook.vue       # 订单簿组件
│   ├── PositionList.vue    # 持仓列表
│   └── StrategyEditor.vue  # 代码编辑器
├── api/
│   └── tauri.ts            # Tauri命令封装
├── types/
│   └── index.ts            # TypeScript类型定义
└── utils/
    ├── format.ts           # 格式化工具
    └── validation.ts       # 验证工具
```

### 5.2 状态管理

```typescript
// store/market.ts
import { defineStore } from 'pinia';

export const useMarketStore = defineStore('market', {
  state: () => ({
    // 当前选中的交易对
    currentSymbol: 'BTCUSDT',
    // 当前K线周期
    currentInterval: '1h',
    // 实时行情数据
    tickers: new Map<string, Ticker>(),
    // K线数据缓存
    klines: new Map<string, Kline[]>(),
    // WebSocket连接状态
    wsConnected: false,
  }),

  actions: {
    // 订阅行情
    async subscribeTicker(symbols: string[]) {
      await invoke('subscribe_ticker', { symbols });
    },

    // 获取K线数据
    async loadKlines(symbol: string, interval: string, limit: number) {
      const klines = await invoke<Kline[]>('get_klines', {
        symbol,
        interval,
        limit,
      });
      this.klines.set(`${symbol}_${interval}`, klines);
    },

    // 监听实时行情更新
    onKlineUpdate(kline: Kline) {
      const key = `${kline.symbol}_${kline.interval}`;
      const data = this.klines.get(key) || [];
      data.push(kline);
      this.klines.set(key, data);
    },
  },
});
```

---

## 6. 数据库设计概要

### 6.1 表清单

| 表名 | 说明 | 核心字段 |
|------|------|----------|
| `users` | 用户表 | id, username, password_hash, role_id |
| `roles` | 角色表 | id, name, permissions (JSON) |
| `exchanges` | 交易所配置（加密） | id, user_id, exchange_name, api_key_encrypted |
| `strategies` | 策略模板 | id, user_id, name, code, parameters (JSON) |
| `strategy_instances` | 策略实例 | id, strategy_id, exchange_id, symbol, timeframe, status |
| `backtests` | 回测记录 | id, strategy_id, params (JSON), sharpe_ratio, max_drawdown |
| `orders` | 订单记录 | id, user_id, exchange_id, symbol, side, status |
| `positions` | 持仓记录 | id, user_id, exchange_id, symbol, side, quantity, pnl |
| `trades` | 成交记录 | id, order_id, price, quantity, pnl |
| `klines` | K线历史数据 | exchange_name, symbol, timeframe, timestamp, ohlc |
| `risk_rules` | 风控规则 | id, user_id, rule_type, threshold_value, action |
| `risk_alerts` | 风控预警 | id, rule_id, severity, status, message |
| `audit_logs` | 操作审计 | id, user_id, operation_type, resource_id, before/after (JSON) |

### 6.2 ER关系图

```
┌─────────┐     ┌─────────┐     ┌─────────┐
│  users  │────▶│  roles  │     │exchanges│
└────┬────┘     └─────────┘     └─────────┘
     │                                  │
     │                                  │
     ▼                                  ▼
┌─────────────┐                 ┌─────────────┐
│  strategies │                 │  orders     │
└──────┬──────┘                 └──────┬──────┘
       │                                │
       ▼                                ▼
┌───────────────┐               ┌─────────────┐
│strategy_instances│             │  trades     │
└───────┬───────┘               └─────────────┘
        │
        ▼
┌─────────────┐
│  backtests  │
└─────────────┘
```

---

## 7. 安全设计

### 7.1 加密策略

| 数据 | 加密方式 | 存储位置 |
|------|----------|----------|
| 用户密码 | argon2 (cost=12) | 数据库哈希存储 |
| API密钥 | AES-256-GCM | 数据库加密存储 |
| 策略代码 | AES-256-GCM（可选） | 数据库/文件 |

### 7.2 密钥派生流程

```
用户登录密码
      │
      ▼ argon2 哈希
  用户密码哈希
      │
      ▼ 存储到数据库
   users.password_hash

      │
      ▼ (登录时)
  验证密码哈希
      │
      ▼ 通过
  派生主密钥 (argon2)
      │
      ▼
  加密/解密 API密钥
```

### 7.3 操作审计

所有敏感操作必须记录审计日志：

```rust
pub struct AuditLog {
    pub id: String,
    pub user_id: String,
    pub operation: Operation,
    pub resource_type: ResourceType,
    pub resource_id: Option<String>,
    pub before: Option<Json>,
    pub after: Option<Json>,
    pub result: Result<(), Error>,
    pub timestamp: i64,
}
```

---

## 8. 开发阶段规划

### Phase 1: 基础框架（2周）
- [ ] Tauri 项目搭建
- [ ] 数据库表结构创建
- [ ] 用户登录/权限系统
- [ ] 基础UI框架

### Phase 2: 行情模块（2周）
- [ ] Binance API 集成
- [ ] WebSocket 行情订阅
- [ ] K线图表组件
- [ ] 行情数据缓存

### Phase 3: 策略开发（2周）
- [ ] 策略编辑器（Monaco）
- [ ] 策略引擎核心
- [ ] 简单回测功能
- [ ] 回测报告展示

### Phase 4: 实盘交易（2周）
- [ ] 订单管理系统
- [ ] 持仓管理
- [ ] 策略实盘运行
- [ ] 交易控制台

### Phase 5: 风控系统（1周）
- [ ] 风控规则配置
- [ ] 实时风险监控
- [ ] 预警通知

### Phase 6: 优化完善（1周）
- [ ] 性能优化
- [ ] 错误处理
- [ ] 单元测试
- [ ] 文档完善
