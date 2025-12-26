use crate::core::event::{EventBus, MarketEvent};
use crate::core::strategy::ScriptExecutor;
use crate::core::trade::types::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

/// 策略配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrategyConfig {
    /// 策略ID（可选，为空时自动生成）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 策略名称
    pub name: String,
    /// 策略代码
    pub code: String,
    /// 策略参数
    #[serde(default)]
    pub parameters: serde_json::Value,
    /// 订阅的交易对
    pub symbols: Vec<String>,
    /// 订阅的周期
    pub timeframes: Vec<String>,
}

/// 运行中的策略实例状态
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstanceStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Error(String),
}

/// 策略实例信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceInfo {
    pub id: String,
    pub name: String,
    pub status: InstanceStatus,
    pub symbols: Vec<String>,
    pub timeframes: Vec<String>,
}

/// 运行中的策略实例
struct RunningInstance {
    id: String,
    config: StrategyConfig,
    executor: ScriptExecutor,
    event_bus: Arc<EventBus>,
    shutdown_tx: Option<broadcast::Sender<()>>,
    history: HashMap<String, Vec<Kline>>, // symbol -> klines
}

impl RunningInstance {
    /// 创建新的运行实例
    pub fn new(
        id: String,
        config: StrategyConfig,
        event_bus: Arc<EventBus>,
    ) -> Result<Self> {
        let executor = ScriptExecutor::new()?;
        Ok(Self {
            id,
            config,
            executor,
            event_bus,
            shutdown_tx: None,
            history: HashMap::new(),
        })
    }

    /// 启动策略运行循环
    pub async fn run(&mut self) -> Result<()> {
        log::info!("Starting strategy instance: {}", self.id);

        // 订阅市场事件
        let mut kline_stream = self.event_bus.subscribe_market();
        let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        // 发布策略启动事件
        self.event_bus
            .publish_strategy_started(self.id.clone());

        // 执行策略初始化
        if let Err(e) = self.executor.on_init(&self.config.code, &self.config.parameters) {
            log::error!("Strategy onInit failed: {}", e);
            self.event_bus
                .publish_strategy_error(format!("{}: onInit failed: {}", self.id, e));
            return Err(e);
        }

        log::info!("Strategy {} initialized successfully", self.id);

        // 策略主循环
        loop {
            tokio::select! {
                // 接收行情事件
                result = kline_stream.recv() => {
                    match result {
                        Ok(event) => {
                            if let MarketEvent::Kline(kline) = event {
                                self.on_kline(kline).await?;
                            }
                        }
                        Err(e) => {
                            log::warn!("Market event stream error: {}", e);
                            // Stream closed, exit loop
                            break;
                        }
                    }
                }

                // 接收停止信号
                result = shutdown_rx.recv() => {
                    match result {
                        Ok(()) => {
                            log::info!("Strategy {} received shutdown signal", self.id);
                            break;
                        }
                        Err(e) => {
                            log::warn!("Strategy {} shutdown receiver error: {}", self.id, e);
                            break;
                        }
                    }
                }
            }
        }

        // 执行策略清理
        if let Err(e) = self.executor.on_stop(&self.config.code) {
            log::error!("Strategy onStop failed: {}", e);
            self.event_bus
                .publish_strategy_error(format!("{}: onStop failed: {}", self.id, e));
        }

        // 发布策略停止事件
        self.event_bus
            .publish_strategy_stopped(self.id.clone());

        log::info!("Strategy instance {} stopped", self.id);
        Ok(())
    }

    /// 处理K线数据
    async fn on_kline(&mut self, kline: Kline) -> Result<()> {
        // 检查是否订阅了该交易对和周期
        let symbol_match = self.config.symbols.is_empty()
            || self.config.symbols.contains(&kline.symbol);
        let timeframe_match = self.config.timeframes.is_empty()
            || self.config.timeframes.contains(&kline.timeframe);

        if !symbol_match || !timeframe_match {
            return Ok(());
        }

        // 更新历史数据
        let history = self
            .history
            .entry(kline.symbol.clone())
            .or_insert_with(Vec::new);
        history.push(kline.clone());

        // 限制历史数据长度（保留最近1000根）
        if history.len() > 1000 {
            history.drain(0..history.len() - 1000);
        }

        // 执行策略 onBar 回调
        let params = &self.config.parameters;
        let history_slice = history.as_slice();

        match self
            .executor
            .on_bar(&self.config.code, &kline, params, history_slice)
        {
            Ok(Some(signal)) => {
                log::info!(
                    "Strategy {} generated signal: {} {} @ {}",
                    self.id,
                    signal.action,
                    signal.symbol,
                    signal.price.unwrap_or(0.0)
                );

                // 发布信号事件
                self.event_bus.publish_signal(signal);
            }
            Ok(None) => {
                // 策略不返回信号
            }
            Err(e) => {
                log::error!("Strategy onBar error: {}", e);
                self.event_bus
                    .publish_strategy_error(format!("{}: onBar error: {}", self.id, e));
            }
        }

        Ok(())
    }

    /// 停止策略实例
    pub fn stop(&self) {
        if let Some(ref tx) = self.shutdown_tx {
            let _ = tx.send(());
        }
    }

    /// 获取实例信息
    pub fn info(&self) -> InstanceInfo {
        InstanceInfo {
            id: self.id.clone(),
            name: self.config.name.clone(),
            status: InstanceStatus::Running,
            symbols: self.config.symbols.clone(),
            timeframes: self.config.timeframes.clone(),
        }
    }
}

/// 策略引擎
pub struct StrategyEngine {
    instances: Arc<RwLock<HashMap<String, Arc<RwLock<RunningInstance>>>>>,
    event_bus: Arc<EventBus>,
}

impl StrategyEngine {
    /// 创建新的策略引擎
    pub fn new(event_bus: Arc<EventBus>) -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            event_bus,
        }
    }

    /// 启动策略实例
    pub async fn start_instance(&self, config: StrategyConfig) -> Result<String> {
        let id = config.id.clone().unwrap_or_else(|| {
            uuid::Uuid::new_v4().to_string()
        });

        // 检查是否已存在
        let instances = self.instances.read().await;
        if instances.contains_key(&id) {
            return Err(anyhow::anyhow!("Strategy instance {} already exists", id));
        }
        drop(instances);

        log::info!("Starting strategy instance: {} ({})", id, config.name);

        // 创建实例
        let instance = RunningInstance::new(
            id.clone(),
            config,
            self.event_bus.clone(),
        )?;

        // 启动策略循环
        let instance_ref = Arc::new(RwLock::new(instance));
        let instance_clone = instance_ref.clone();

        tokio::spawn(async move {
            let mut instance = instance_clone.write().await;
            if let Err(e) = instance.run().await {
                log::error!("Strategy instance {} error: {}", instance.id, e);
            }
        });

        // 保存实例
        let mut instances = self.instances.write().await;
        instances.insert(id.clone(), instance_ref);

        Ok(id)
    }

    /// 停止策略实例
    pub async fn stop_instance(&self, id: &str) -> Result<()> {
        log::info!("Stopping strategy instance: {}", id);

        let mut instances = self.instances.write().await;
        if let Some(instance) = instances.remove(id) {
            let instance = instance.read().await;
            instance.stop();
        } else {
            return Err(anyhow::anyhow!("Strategy instance {} not found", id));
        }

        Ok(())
    }

    /// 获取所有实例信息
    pub async fn list_instances(&self) -> Vec<InstanceInfo> {
        let instances = self.instances.read().await;
        let mut infos = Vec::new();

        for instance in instances.values() {
            let instance = instance.read().await;
            infos.push(instance.info());
        }

        infos
    }

    /// 获取单个实例信息
    pub async fn get_instance(&self, id: &str) -> Option<InstanceInfo> {
        let instances = self.instances.read().await;
        if let Some(instance) = instances.get(id) {
            let instance = instance.read().await;
            Some(instance.info())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_config_serialization() {
        let config = StrategyConfig {
            id: None,
            name: "Test Strategy".to_string(),
            code: "function onBar(context, kline) { return null; }".to_string(),
            parameters: serde_json::json!({"param1": "value1"}),
            symbols: vec!["BTCUSDT".to_string()],
            timeframes: vec!["1h".to_string()],
        };

        let json = serde_json::to_string(&config).unwrap();
        let decoded: StrategyConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded.name, config.name);
        assert_eq!(decoded.code, config.code);
        assert_eq!(decoded.symbols, config.symbols);
    }

    #[test]
    fn test_instance_status_serialization() {
        let status = InstanceStatus::Error("test error".to_string());
        let json = serde_json::to_string(&status).unwrap();
        let decoded: InstanceStatus = serde_json::from_str(&json).unwrap();

        match decoded {
            InstanceStatus::Error(msg) => assert_eq!(msg, "test error"),
            _ => panic!("Expected Error status"),
        }
    }
}
