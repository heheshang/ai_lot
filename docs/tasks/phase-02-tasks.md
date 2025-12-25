# Phase 2: 行情数据模块 - 详细任务规范

## 目标

连接交易所，实现实时行情数据获取和展示。

---

## 任务列表

### P2-01: 定义 Exchange Trait

**估时**: 2h | **优先级**: P0 | **依赖**: P1-10

#### 实施步骤

1. 创建 `src-tauri/src/core/trade/types.rs`：
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticker {
    pub symbol: String,
    pub price: f64,
    pub price_change: f64,
    pub price_change_percent: f64,
    pub high_24h: f64,
    pub low_24h: f64,
    pub volume_24h: f64,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kline {
    pub symbol: String,
    pub timeframe: String,
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
    pub quote_volume: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub asset: String,
    pub free: f64,
    pub locked: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub symbol: String,
    pub side: String,
    pub size: f64,
    pub entry_price: f64,
    pub mark_price: f64,
    pub unrealized_pnl: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: String,
    pub client_order_id: Option<String>,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub price: Option<f64>,
    pub quantity: f64,
    pub filled_quantity: f64,
    pub avg_price: Option<f64>,
    pub status: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interval {
    OneMinute,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    FourHours,
    OneDay,
}

impl Interval {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OneMinute => "1m",
            Self::FiveMinutes => "5m",
            Self::FifteenMinutes => "15m",
            Self::ThirtyMinutes => "30m",
            Self::OneHour => "1h",
            Self::FourHours => "4h",
            Self::OneDay => "1d",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "1m" => Some(Self::OneMinute),
            "5m" => Some(Self::FiveMinutes),
            "15m" => Some(Self::FifteenMinutes),
            "30m" => Some(Self::ThirtyMinutes),
            "1h" => Some(Self::OneHour),
            "4h" => Some(Self::FourHours),
            "1d" => Some(Self::OneDay),
            _ => None,
        }
    }
}
```

2. 创建 `src-tauri/src/core/trade/exchange/trait.rs`：
```rust
use super::super::types::*;
use async_trait::async_trait;
use anyhow::Result;
use tokio::sync::broadcast;

/// 交易所名称
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    // ========== 事件流 ==========
    fn ticker_stream(&self) -> broadcast::Receiver<Ticker>;
    fn kline_stream(&self) -> broadcast::Receiver<Kline>;
}
```

#### 验收标准
- [ ] Exchange Trait 定义完整
- [ ] 所有类型定义与前端 TypeScript 类型一致
- [ ] 编译无错误

#### 产物
- `src-tauri/src/core/trade/types.rs`
- `src-tauri/src/core/trade/exchange/trait.rs`

---

### P2-02: 实现数据类型

**估时**: 1h | **优先级**: P0 | **依赖**: P2-01

#### 见 P2-01，类型已包含在 `core/trade/types.rs` 中

---

### P2-03: 实现 Binance REST API

**估时**: 3h | **优先级**: P0 | **依赖**: P2-01

#### 实施步骤

1. 创建 `src-tauri/src/core/trade/exchange/binance.rs`：
```rust
use super::super::types::*;
use super::trait::{Exchange, ExchangeName};
use super::super::event::MarketEvent;
use async_trait::async_trait;
use anyhow::{Result, anyhow};
use tokio::sync::{broadcast, Mutex, RwLock};
use reqwest::Client;
use serde_json::Value;

const REST_API_BASE: &str = "https://api.binance.com";
const WS_API_BASE: &str = "wss://stream.binance.com:9443/ws";

pub struct BinanceExchange {
    api_key: Option<String>,
    api_secret: Option<String>,
    client: Client,
    ticker_tx: broadcast::Sender<Ticker>,
    kline_tx: broadcast::Sender<Kline>,
    connection_state: Arc<RwLock<bool>>,
}

impl BinanceExchange {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        let (ticker_tx, _) = broadcast::channel(1000);
        let (kline_tx, _) = broadcast::channel(1000);

        Self {
            api_key,
            api_secret,
            client: Client::new(),
            ticker_tx,
            kline_tx,
            connection_state: Arc::new(RwLock::new(false)),
        }
    }

    async fn get(&self, path: &str) -> Result<Value> {
        let url = format!("{}{}", REST_API_BASE, path);
        let response = self.client.get(&url).send().await?;
        let json = response.json().await?;
        Ok(json)
    }
}

#[async_trait]
impl Exchange for BinanceExchange {
    fn name(&self) -> ExchangeName {
        ExchangeName::Binance
    }

    fn is_connected(&self) -> bool {
        *self.connection_state.try_read().unwrap_or(false)
    }

    async fn connect(&self) -> Result<()> {
        *self.connection_state.write().await = true;
        Ok(())
    }

    async fn disconnect(&self) -> Result<()> {
        *self.connection_state.write().await = false;
        Ok(())
    }

    async fn get_ticker(&self, symbol: &str) -> Result<Ticker> {
        let path = format!("/ticker/price?symbol={}", symbol.to_uppercase());
        let json = self.get(&path).await?;

        Ok(Ticker {
            symbol: symbol.to_uppercase(),
            price: json["price"].as_str().unwrap().parse()?,
            price_change: 0.0,
            price_change_percent: 0.0,
            high_24h: 0.0,
            low_24h: 0.0,
            volume_24h: 0.0,
            timestamp: chrono::Utc::now().timestamp_millis(),
        })
    }

    async fn get_klines(
        &self,
        symbol: &str,
        interval: Interval,
        limit: usize,
    ) -> Result<Vec<Kline>> {
        let path = format!(
            "/klines?symbol={}&interval={}&limit={}",
            symbol.to_uppercase(),
            interval.as_str(),
            limit
        );
        let json = self.get(&path).await?;

        let klines = json.as_array()
            .ok_or_else(|| anyhow!("Invalid response"))?
            .iter()
            .map(|item| -> Result<Kline> {
                Ok(Kline {
                    symbol: symbol.to_uppercase(),
                    timeframe: interval.as_str().to_string(),
                    timestamp: item[0].as_i64().unwrap(),
                    open: item[1].as_str().unwrap().parse()?,
                    high: item[2].as_str().unwrap().parse()?,
                    low: item[3].as_str().unwrap().parse()?,
                    close: item[4].as_str().unwrap().parse()?,
                    volume: item[5].as_str().unwrap().parse()?,
                    quote_volume: item[7].as_str().map(|s| s.parse().ok()).flatten(),
                })
            })
            .collect::<Result<Vec<Kline>>>()?;

        Ok(klines)
    }

    async fn subscribe_ticker(&self, symbols: Vec<String>) -> Result<()> {
        // WebSocket 订阅在下一个任务实现
        Ok(())
    }

    async fn subscribe_kline(
        &self,
        symbols: Vec<String>,
        interval: Interval,
    ) -> Result<()> {
        // WebSocket 订阅在下一个任务实现
        Ok(())
    }

    fn ticker_stream(&self) -> broadcast::Receiver<Ticker> {
        self.ticker_tx.subscribe()
    }

    fn kline_stream(&self) -> broadcast::Receiver<Kline> {
        self.kline_tx.subscribe()
    }
}
```

#### 验收标准
- [ ] 可通过 REST API 获取行情数据
- [ ] get_ticker 返回正确的 Ticker 数据
- [ ] get_klines 返回正确的 K线数组

#### 产物
- `src-tauri/src/core/trade/exchange/binance.rs`

---

### P2-04: 实现 Binance WebSocket

**估时**: 3h | **优先级**: P0 | **依赖**: P2-03

#### 实施步骤

1. 更新 `src-tauri/src/core/trade/exchange/binance.rs`，添加 WebSocket 支持：
```rust
use super::super::types::*;
use super::trait::{Exchange, ExchangeName};
use async_trait::async_trait;
use anyhow::{Result, anyhow};
use tokio::sync::{broadcast, RwLock};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{StreamExt, SinkExt};
use serde_json::Value;
use std::sync::Arc;

pub struct BinanceExchange {
    // ... 现有字段
    ws_task_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
}

impl BinanceExchange {
    async fn ws_loop(&self, stream: String) -> Result<()> {
        let url = format!("{}{}", WS_API_BASE, stream);
        let (ws_stream, _) = connect_async(&url).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();

        // 订阅消息
        let subscribe_msg = serde_json::json!({
            "method": "SUBSCRIBE",
            "params": ["btcusdt@ticker"],
            "id": 1
        });
        ws_sender.send(Message::Text(subscribe_msg.to_string())).await?;

        // 接收消息循环
        while let Some(msg) = ws_receiver.next().await {
            match msg? {
                Message::Text(text) => {
                    if let Ok(json) = serde_json::from_str::<Value>(&text) {
                        if let Some(event) = json.get("e").and_then(|e| e.as_str()) {
                            match event {
                                "24hrTicker" => {
                                    if let Ok(ticker) = self.parse_ticker(&json) {
                                        let _ = self.ticker_tx.send(ticker);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Message::Close(_) => break,
                _ => {}
            }
        }

        Ok(())
    }

    fn parse_ticker(&self, json: &Value) -> Result<Ticker> {
        Ok(Ticker {
            symbol: json["s"].as_str().unwrap().to_string(),
            price: json["c"].as_str().unwrap().parse()?,
            price_change: json["p"].as_str().unwrap().parse()?,
            price_change_percent: json["P"].as_str().unwrap().parse()?,
            high_24h: json["h"].as_str().unwrap().parse()?,
            low_24h: json["l"].as_str().unwrap().parse()?,
            volume_24h: json["v"].as_str().unwrap().parse()?,
            timestamp: json["E"].as_i64().unwrap(),
        })
    }

    async fn start_ws(&self) -> Result<()> {
        let tx = self.ticker_tx.clone();
        let handle = tokio::spawn(async move {
            // WebSocket 连接逻辑
            // ...
        });

        *self.ws_task_handle.lock().await = Some(handle);
        Ok(())
    }
}

// 更新 connect 方法启动 WebSocket
#[async_trait]
impl Exchange for BinanceExchange {
    // ... 其他方法

    async fn connect(&self) -> Result<()> {
        *self.connection_state.write().await = true;
        self.start_ws().await?;
        Ok(())
    }

    async fn subscribe_ticker(&self, symbols: Vec<String>) -> Result<()> {
        // 构建订阅流
        let streams: Vec<String> = symbols
            .iter()
            .map(|s| format!("{}@ticker", s.to_lowercase()))
            .collect();
        let stream = streams.join("/");
        self.ws_loop(&stream).await?;
        Ok(())
    }

    // ...
}
```

2. 更新 `Cargo.toml` 添加 WebSocket 依赖：
```toml
tokio-tungstenite = "0.24"
futures-util = "0.3"
```

#### 验收标准
- [ ] WebSocket 连接成功建立
- [ ] 可接收实时行情数据
- [ ] 行情数据通过 broadcast channel 发送

#### 产物
- 更新的 `src-tauri/src/core/trade/exchange/binance.rs`

---

### P2-05: 实现 EventBus

**估时**: 2h | **优先级**: P0 | **依赖**: P2-04

#### 实施步骤

1. 创建 `src-tauri/src/core/event.rs`：
```rust
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use super::trade::types::*;

/// 市场事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketEvent {
    Ticker(Ticker),
    Kline(Kline),
}

/// 交易事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TradeEvent {
    OrderPlaced(Order),
    OrderFilled(Order),
    OrderCanceled(Order),
    PositionUpdated(Position),
}

/// 策略事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StrategyEvent {
    StrategyStarted(String),
    StrategyStopped(String),
    SignalGenerated(Signal),
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signal {
    pub symbol: String,
    pub action: String,
    pub quantity: f64,
    pub price: Option<f64>,
}

/// 事件总线
#[derive(Clone)]
pub struct EventBus {
    market_tx: broadcast::Sender<MarketEvent>,
    trade_tx: broadcast::Sender<TradeEvent>,
    strategy_tx: broadcast::Sender<StrategyEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (market_tx, _) = broadcast::channel(1000);
        let (trade_tx, _) = broadcast::channel(1000);
        let (strategy_tx, _) = broadcast::channel(1000);

        Self {
            market_tx,
            trade_tx,
            strategy_tx,
        }
    }

    // ========== Market Events ==========
    pub fn publish_market(&self, event: MarketEvent) {
        let _ = self.market_tx.send(event);
    }

    pub fn subscribe_market(&self) -> broadcast::Receiver<MarketEvent> {
        self.market_tx.subscribe()
    }

    // ========== Trade Events ==========
    pub fn publish_trade(&self, event: TradeEvent) {
        let _ = self.trade_tx.send(event);
    }

    pub fn subscribe_trade(&self) -> broadcast::Receiver<TradeEvent> {
        self.trade_tx.subscribe()
    }

    // ========== Strategy Events ==========
    pub fn publish_strategy(&self, event: StrategyEvent) {
        let _ = self.strategy_tx.send(event);
    }

    pub fn subscribe_strategy(&self) -> broadcast::Receiver<StrategyEvent> {
        self.strategy_tx.subscribe()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
```

#### 验收标准
- [ ] EventBus 可正常创建
- [ ] 可订阅和发布事件
- [ ] 多个订阅者可同时接收事件

#### 产物
- `src-tauri/src/core/event.rs`

---

### P2-06: 实现 MarketService

**估时**: 2h | **优先级**: P0 | **依赖**: P2-05

#### 实施步骤

1. 创建 `src-tauri/src/services/market_service.rs`：
```rust
use crate::core::trade::exchange::{Exchange, ExchangeName, binance::BinanceExchange};
use crate::core::event::{EventBus, MarketEvent};
use crate::infrastructure::Database;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct MarketService {
    exchanges: Arc<RwLock<Vec<Arc<dyn Exchange>>>>,
    event_bus: Arc<EventBus>,
    db: Database,
}

impl MarketService {
    pub fn new(db: Database) -> Self {
        Self {
            exchanges: Arc::new(RwLock::new(Vec::new())),
            event_bus: Arc::new(EventBus::new()),
            db,
        }
    }

    /// 添加交易所
    pub async fn add_exchange(&self, exchange: Arc<dyn Exchange>) {
        let mut exchanges = self.exchanges.write().await;
        exchanges.push(exchange);
    }

    /// 获取交易所
    pub async fn get_exchange(&self, name: ExchangeName) -> Option<Arc<dyn Exchange>> {
        let exchanges = self.exchanges.read().await;
        exchanges.iter().find(|e| e.name() == name).cloned()
    }

    /// 订阅行情
    pub async fn subscribe_ticker(&self, symbols: Vec<String>) -> Result<()> {
        // 订阅所有已连接的交易所
        let exchanges = self.exchanges.read().await;
        for exchange in exchanges.iter() {
            exchange.subscribe_ticker(symbols.clone()).await?;
        }
        Ok(())
    }

    /// 获取K线数据
    pub async fn get_klines(
        &self,
        symbol: &str,
        interval: &str,
        limit: usize,
    ) -> Result<Vec<crate::core::trade::types::Kline>> {
        let exchange = self.get_exchange(ExchangeName::Binance)
            .ok_or_else(|| anyhow!("Exchange not found"))?;

        let interval = crate::core::trade::types::Interval::from_str(interval)
            .ok_or_else(|| anyhow!("Invalid interval"))?;

        exchange.get_klines(symbol, interval, limit).await
    }

    /// 保存K线到数据库
    pub async fn save_klines(&self, klines: &[crate::core::trade::types::Kline]) -> Result<()> {
        for kline in klines {
            sqlx::query(
                r#"
                INSERT OR REPLACE INTO klines
                (exchange_name, symbol, timeframe, timestamp, open, high, low, close, volume)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#
            )
            .bind("binance")
            .bind(&kline.symbol)
            .bind(&kline.timeframe)
            .bind(kline.timestamp)
            .bind(kline.open)
            .bind(kline.high)
            .bind(kline.low)
            .bind(kline.close)
            .bind(kline.volume)
            .execute(&self.db.pool)
            .await?;
        }
        Ok(())
    }
}
```

#### 验收标准
- [ ] 可添加交易所实例
- [ ] 可订阅行情
- [ ] K线数据可保存到数据库

#### 产物
- `src-tauri/src/services/market_service.rs`

---

### P2-07: 实现行情数据缓存

**估时**: 1h | **优先级**: P1 | **依赖**: P2-06

#### 实施步骤

在 `MarketService` 中实现缓存逻辑（已在 P2-06 中包含 `save_klines` 方法）。

#### 产物
见 P2-06

---

### P2-08: 实现 Tauri 行情命令

**估时**: 1h | **优先级**: P0 | **依赖**: P2-06

#### 实施步骤

1. 创建 `src-tauri/src/commands/market.rs`：
```rust
use crate::services::market_service::MarketService;
use crate::infrastructure::Database;
use tauri::State;

/// 订阅行情
#[tauri::command]
pub async fn market_subscribe_ticker(
    db: State<'_, Database>,
    symbols: Vec<String>,
) -> Result<(), String> {
    // 获取或创建 MarketService
    // 这里简化处理，实际应该从 app state 获取
    Ok(())
}

/// 获取K线数据
#[tauri::command]
pub async fn market_get_klines(
    db: State<'_, Database>,
    symbol: String,
    interval: String,
    limit: usize,
) -> Result<Vec<crate::core::trade::types::Kline>, String> {
    // 从数据库获取或从交易所API获取
    Ok(vec![])
}

/// 获取交易对列表
#[tauri::command]
pub async fn market_get_symbols(
    db: State<'_, Database>,
) -> Result<Vec<String>, String> {
    Ok(vec![
        "BTCUSDT".to_string(),
        "ETHUSDT".to_string(),
        "BNBUSDT".to_string(),
    ])
}
```

#### 验收标准
- [ ] 命令可被前端调用
- [ ] 返回数据格式正确

#### 产物
- `src-tauri/src/commands/market.rs`

---

### P2-09: 实现 MarketStore

**估时**: 1h | **优先级**: P0 | **依赖**: P2-08

#### 实施步骤

更新 `src/store/modules/market.ts`：
```typescript
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Ticker, Kline } from '@/types';
import * as api from '@/api/tauri';

export const useMarketStore = defineStore('market', () => {
  // State
  const currentSymbol = ref('BTCUSDT');
  const currentTimeframe = ref('1h');
  const tickers = ref<Map<string, Ticker>>(new Map());
  const klines = ref<Map<string, Kline[]>>(new Map());
  const wsConnected = ref(false);
  const symbols = ref<string[]>([]);

  // Getters
  const currentTicker = computed(() => tickers.value.get(currentSymbol.value));
  const currentKlines = computed(() => {
    const key = `${currentSymbol.value}_${currentTimeframe.value}`;
    return klines.value.get(key) || [];
  });

  // Actions
  async function loadSymbols() {
    try {
      const result = await api.marketApi.getSymbols();
      symbols.value = result;
    } catch (error) {
      console.error('Failed to load symbols:', error);
    }
  }

  async function loadKlines(symbol: string, interval: string, limit = 500) {
    try {
      const data = await api.marketApi.getKlines(symbol, interval, limit);
      const key = `${symbol}_${interval}`;
      klines.value.set(key, data);
    } catch (error) {
      console.error('Failed to load klines:', error);
    }
  }

  async function subscribeTicker(syms: string[]) {
    try {
      await api.marketApi.subscribeTicker(syms);
      wsConnected.value = true;
    } catch (error) {
      console.error('Failed to subscribe ticker:', error);
      wsConnected.value = false;
    }
  }

  function updateTicker(ticker: Ticker) {
    tickers.value.set(ticker.symbol, ticker);
  }

  function updateKline(kline: Kline) {
    const key = `${kline.symbol}_${kline.timeframe}`;
    const data = klines.value.get(key) || [];
    data.push(kline);
    // 保持最多 1000 根 K线
    if (data.length > 1000) {
      data.shift();
    }
    klines.value.set(key, data);
  }

  function setCurrentSymbol(symbol: string) {
    currentSymbol.value = symbol;
    // 加载新交易对的 K线
    loadKlines(symbol, currentTimeframe.value);
  }

  function setCurrentTimeframe(timeframe: string) {
    currentTimeframe.value = timeframe;
    // 重新加载 K线
    loadKlines(currentSymbol.value, timeframe);
  }

  return {
    // State
    currentSymbol,
    currentTimeframe,
    tickers,
    klines,
    wsConnected,
    symbols,
    // Getters
    currentTicker,
    currentKlines,
    // Actions
    loadSymbols,
    loadKlines,
    subscribeTicker,
    updateTicker,
    updateKline,
    setCurrentSymbol,
    setCurrentTimeframe,
  };
});
```

#### 验收标准
- [ ] Store 可正常管理行情状态
- [ ] 可加载 K线数据
- [ ] 可订阅行情

#### 产物
- 更新的 `src/store/modules/market.ts`

---

### P2-10 ~ P2-13

后续任务与前面类似，创建相应的组件和页面。

---

## Phase 2 验收标准

### 功能验收
- [ ] 可查看实时行情（价格、涨跌幅）
- [ ] 可查看K线图表（多周期切换）
- [ ] 行情数据实时更新
- [ ] WebSocket 断线可自动重连

### 技术验收
- [ ] REST API 调用正常
- [ ] WebSocket 连接稳定
- [ ] 事件总线正常工作
