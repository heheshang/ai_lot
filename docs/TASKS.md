# AI-LOT 量化交易系统 - 任务规划文档

> 版本: 0.1.0
> 更新时间: 2025-12-29
> 状态: 进行中

---

## 📋 任务总览

| 阶段 | 任务 | 优先级 | 预估工时 | 状态 |
|------|------|--------|----------|------|
| Phase 1 | 技术指标库实现 | P0 | 4h | 🔄 进行中 |
| Phase 1 | 策略调试工具 | P0 | 3h | ⏳ 待开始 |
| Phase 1 | 回测报告增强 | P1 | 5h | ⏳ 待开始 |
| Phase 1 | 风控规则扩展 | P1 | 4h | ⏳ 待开始 |
| Phase 2 | 参数优化模块 | P2 | 8h | ⏳ 待开始 |
| Phase 2 | 数据质量监控 | P2 | 4h | ⏳ 待开始 |
| Phase 2 | 通知系统完善 | P2 | 3h | ⏳ 待开始 |

---

## 🎯 Phase 1: 核心功能完善 (1-2周)

### Task 1.1: 技术指标库实现

**目标**: 为策略脚本提供常用的技术分析指标

**实现位置**: `src-tauri/src/core/strategy/indicators.rs`

**功能清单**:

#### 1.1.1 趋势指标
- [x] SMA (简单移动平均)
- [x] EMA (指数移动平均)
- [x] WMA (加权移动平均)
- [ ] VWAP (成交量加权平均价)

#### 1.1.2 动量指标
- [x] RSI (相对强弱指标)
- [x] MACD (异同移动平均线)
- [ ] Stochastic (随机指标)
- [ ] CCI (顺势指标)

#### 1.1.3 波动率指标
- [x] Bollinger Bands (布林带)
- [x] ATR (平均真实波幅)
- [ ] Keltner Channels

#### 1.1.4 成交量指标
- [ ] OBV (能量潮)
- [ ] Volume MA (成交量移动平均)
- [ ] Volume Profile

**API 设计**:

```rust
// src-tauri/src/core/strategy/indicators.rs

/// 技术指标计算器
pub struct IndicatorCalculator {
    data: Vec<Kline>,
}

impl IndicatorCalculator {
    // 趋势指标
    pub fn sma(&self, period: usize) -> Vec<Option<f64>>;
    pub fn ema(&self, period: usize) -> Vec<Option<f64>>;
    pub fn wma(&self, period: usize) -> Vec<Option<f64>>;

    // 动量指标
    pub fn rsi(&self, period: usize) -> Vec<Option<f64>>;
    pub fn macd(&self, fast: usize, slow: usize, signal: usize) -> MacdResult;

    // 波动率指标
    pub fn bollinger_bands(&self, period: usize, std_dev: f64) -> BollingerBandsResult;
    pub fn atr(&self, period: usize) -> Vec<Option<f64>>;

    // 成交量指标
    pub fn obv(&self) -> Vec<Option<f64>>;
}
```

**JavaScript 绑定**:

```javascript
// 策略脚本中可调用
const indicators = context.indicators;

// 趋势指标
const sma20 = indicators.sma(20);
const ema12 = indicators.ema(12);

// 动量指标
const rsi14 = indicators.rsi(14);
const macd = indicators.macd(12, 26, 9);

// 波动率指标
const bb = indicators.bollingerBands(20, 2);
const atr = indicators.atr(14);
```

---

### Task 1.2: 策略调试工具

**目标**: 提供策略开发调试能力

**实现位置**:
- `src-tauri/src/core/strategy/debug.rs`
- `src-tauri/src/commands/strategy_debug.rs`

**功能清单**:

#### 1.2.1 控制台日志
- [x] `console.log()` 输出到前端
- [x] 日志级别控制 (debug/info/warn/error)
- [x] 日志持久化

#### 1.2.2 变量监控
- [ ] 实时变量查看
- [ ] 变量历史记录
- [ ] 变量变化监听

#### 1.2.3 性能分析
- [ ] 策略执行时间统计
- [ ] 函数调用次数统计
- [ ] 内存使用分析

**API 设计**:

```rust
pub struct DebugContext {
    logs: Arc<RwLock<Vec<DebugLog>>>,
    variables: Arc<RwLock<HashMap<String, DebugVariable>>>,
    metrics: Arc<RwLock<PerformanceMetrics>>,
}

pub struct DebugLog {
    pub level: LogLevel,
    pub message: String,
    pub timestamp: i64,
    pub line: Option<u32>,
}

pub struct PerformanceMetrics {
    pub execution_times: HashMap<String, Vec<Duration>>,
    pub call_counts: HashMap<String, usize>,
}
```

---

### Task 1.3: 回测报告增强

**目标**: 提供详细的回测分析报告

**实现位置**: `src-tauri/src/services/backtest_service.rs`

**新增指标**:

#### 1.3.1 收益指标
- [x] 总收益率
- [x] 年化收益率
- [ ] 夏普比率
- [ ] 索提诺比率
- [ ] 卡尔玛比率

#### 1.3.2 风险指标
- [x] 最大回撤
- [x] 平均回撤
- [ ] 回撤持续时间
- [ ] VaR (风险价值)

#### 1.3.3 交易统计
- [x] 总交易次数
- [x] 胜率
- [x] 盈亏比
- [ ] 平均盈利/亏损
- [ ] 最大单笔盈利/亏损

#### 1.3.4 高级分析
- [ ] 月度收益统计
- [ ] 交易时段分析
- [ ] 持仓时间分析
- [ ] 滑点统计

**数据结构**:

```rust
pub struct BacktestReport {
    // 基础信息
    pub id: String,
    pub strategy_id: String,
    pub symbol: String,
    pub timeframe: String,
    pub start_time: i64,
    pub end_time: i64,

    // 收益指标
    pub total_return: f64,
    pub annualized_return: f64,
    pub sharpe_ratio: Option<f64>,
    pub sortino_ratio: Option<f64>,

    // 风险指标
    pub max_drawdown: f64,
    pub avg_drawdown: f64,
    pub max_drawdown_duration: i64,

    // 交易统计
    pub total_trades: usize,
    pub winning_trades: usize,
    pub losing_trades: usize,
    pub win_rate: f64,
    pub avg_win: f64,
    pub avg_loss: f64,
    pub profit_factor: f64,

    // 详细数据
    pub trades: Vec<TradeDetail>,
    pub equity_curve: Vec<EquityPoint>,
    pub drawdown_curve: Vec<DrawdownPoint>,
    pub monthly_returns: Vec<MonthlyReturn>,
}
```

---

### Task 1.4: 风控规则扩展

**目标**: 添加更多风控规则类型

**实现位置**: `src-tauri/src/core/risk/`

**新增规则**:

#### 1.4.1 日内亏损限制
- [ ] DailyLossLimitRule
- [ ] 当日内亏损超过阈值时触发
- [ ] 可配置重置时间

#### 1.4.2 连续亏损限制
- [ ] ConsecutiveLossLimitRule
- [ ] 连续 N 次亏损后触发
- [ ] 可配置冷却期

#### 1.4.3 波动率限制
- [ ] VolatilityLimitRule
- [ ] 市场波动率过高时停止交易
- [ ] 基于 ATR 计算

#### 1.4.4 敞口限制
- [ ] ExposureLimitRule
- [ ] 总敞口不超过账户净值的 X%
- [ ] 单方向敞口限制

**规则接口**:

```rust
pub trait RiskRule: Send + Sync {
    fn name(&self) -> &str;
    fn config(&self) -> &RiskRuleConfig;

    async fn check(&self, context: &RiskContext) -> Result<bool, Error>;
    async fn on_triggered(&self, context: &RiskContext) -> Result<RiskAction, Error>;
}
```

---

## 🎯 Phase 2: 高级功能 (2-3周)

### Task 2.1: 参数优化模块

**目标**: 实现策略参数自动优化

**实现位置**: `src-tauri/src/services/optimizer.rs`

**优化算法**:
- [ ] 网格搜索 (Grid Search)
- [ ] 随机搜索 (Random Search)
- [ ] 贝叶斯优化
- [ ] 遗传算法

**API 设计**:

```rust
pub struct Optimizer {
    algorithm: OptimizationAlgorithm,
    objective: OptimizationObjective,
}

pub enum OptimizationAlgorithm {
    Grid { params: Vec<ParamRange> },
    Random { iterations: usize },
    Bayesian { iterations: usize },
    Genetic { population: usize, generations: usize },
}

pub enum OptimizationObjective {
    MaximizeReturn,
    MaximizeSharpe,
    MinimizeDrawdown,
    Custom { fn: Box<dyn Fn(&BacktestReport) -> f64> },
}
```

---

### Task 2.2: 数据质量监控

**目标**: 监控行情数据质量

**实现位置**: `src-tauri/src/services/data_quality.rs`

**监控指标**:
- [ ] 连接状态
- [ ] 延迟统计
- [ ] 消息频率
- [ ] 数据缺口检测
- [ ] 重复数据检测
- [ ] 异常值检测

---

### Task 2.3: 通知系统完善

**目标**: 完善多渠道通知

**实现位置**: `src-tauri/src/infrastructure/notification/`

**通知渠道**:
- [x] 钉钉机器人
- [x] 邮件通知
- [ ] Telegram 机器人
- [ ] 企业微信机器人
- [ ] Webhook 通知

---

## 📁 文件结构

```
src-tauri/src/
├── core/
│   └── strategy/
│       ├── mod.rs
│       ├── engine.rs          # 策略引擎
│       ├── script.rs          # 脚本执行器
│       ├── indicators.rs      # 🆕 技术指标
│       └── debug.rs           # 🆕 调试工具
├── core/
│   └── risk/
│       ├── mod.rs
│       ├── rule.rs            # 风控规则接口
│       ├── position_limit.rs  # 仓位限制
│       ├── drawdown_limit.rs  # 回撤限制
│       ├── daily_loss.rs      # 🆕 日内亏损限制
│       ├── consecutive_loss.rs # 🆕 连续亏损限制
│       ├── volatility_limit.rs # 🆕 波动率限制
│       └── exposure_limit.rs  # 🆕 敞口限制
├── services/
│   ├── mod.rs
│   ├── market_service.rs
│   ├── trade_service.rs
│   ├── backtest_service.rs    # 🔄 回测服务增强
│   ├── optimizer.rs           # 🆕 参数优化
│   └── data_quality.rs        # 🆕 数据质量监控
├── commands/
│   ├── mod.rs
│   ├── strategy.rs
│   ├── strategy_debug.rs      # 🆕 调试命令
│   └── optimizer.rs           # 🆕 优化命令
└── types/
    └── backtest.rs            # 🆕 回测类型定义
```

---

## 🔧 开发规范

### 代码风格
- Rust: 使用 `rustfmt` 格式化
- 使用 `clippy` 进行代码检查
- 遵循 Rust 命名规范

### 测试要求
- 每个新模块需要单元测试
- 覆盖率目标: 80%+
- 集成测试覆盖关键流程

### 文档要求
- 所有公开 API 需要文档注释
- 复杂逻辑需要详细注释
- 示例代码需要注释说明

---

## 📊 进度跟踪

### 当前阶段: Phase 1

| 任务 | 负责人 | 状态 | 完成度 | 备注 |
|------|--------|------|--------|------|
| 技术指标库 | - | 🔄 进行中 | 0% | 开始实现 |
| 策略调试工具 | - | ⏳ 待开始 | 0% | 依赖指标库 |
| 回测报告增强 | - | ⏳ 待开始 | 0% | 需要设计报告格式 |
| 风控规则扩展 | - | ⏳ 待开始 | 0% | 需要定义规则接口 |

---

## 🚀 快速开始

### 本地开发
```bash
# 前端开发
npm run dev

# 后端开发
cargo run

# 类型检查
npm run type-check

# 构建
npm run build
```

### 测试
```bash
# Rust 测试
cargo test

# 前端测试 (待配置)
npm run test
```

---

## 📝 变更日志

### 2025-12-29
- 创建任务规划文档
- 开始 Phase 1 开发
- 实现技术指标库
