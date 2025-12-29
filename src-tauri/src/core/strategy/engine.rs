use crate::core::event::{EventBus, MarketEvent};
use crate::core::strategy::ScriptExecutor;
use crate::core::trade::exchange::Exchange;
use crate::core::trade::types::*;
use crate::core::risk::rule::{RiskContext, RiskRule};
use crate::models::CreateInstanceRequest;
use crate::repository::StrategyInstanceRepository;
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
    Paused,
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
    exchange: Arc<dyn Exchange>,
    user_id: String,
    instance_repo: Arc<StrategyInstanceRepository>,
    shutdown_tx: Option<broadcast::Sender<()>>,
    paused: Arc<std::sync::atomic::AtomicBool>,
    history: HashMap<String, Vec<Kline>>, // symbol -> klines
    /// 风控规则列表
    risk_rules: Vec<Box<dyn RiskRule>>,
    /// 最大单笔订单金额限制
    max_order_amount: f64,
    /// 最大日内交易次数
    max_daily_trades: usize,
    /// 今日已交易次数
    daily_trade_count: Arc<std::sync::atomic::AtomicUsize>,
    /// 账户余额缓存
    cached_balance: Arc<RwLock<Option<f64>>>,
    /// 持仓缓存
    cached_positions: Arc<RwLock<Vec<Position>>>,
}

impl RunningInstance {
    /// 创建新的运行实例
    pub fn new(
        id: String,
        config: StrategyConfig,
        event_bus: Arc<EventBus>,
        exchange: Arc<dyn Exchange>,
        user_id: String,
        instance_repo: Arc<StrategyInstanceRepository>,
        risk_rules: Vec<Box<dyn RiskRule>>,
    ) -> Result<Self> {
        let executor = ScriptExecutor::new()?;
        Ok(Self {
            id,
            config,
            executor,
            event_bus,
            exchange,
            user_id,
            instance_repo,
            shutdown_tx: None,
            paused: Arc::new(std::sync::atomic::AtomicBool::new(false)),
            history: HashMap::new(),
            // 默认风控配置
            risk_rules,
            max_order_amount: 1000.0,  // 默认最大单笔订单金额
            max_daily_trades: 100,      // 默认最大日内交易次数
            daily_trade_count: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            cached_balance: Arc::new(RwLock::new(None)),
            cached_positions: Arc::new(RwLock::new(Vec::new())),
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

            // 更新数据库状态为 error
            let error_msg = format!("onInit failed: {}", e);
            if let Err(db_err) = self.instance_repo.update_status(&self.id, "error", Some(&error_msg)).await {
                log::error!("Failed to update instance {} error status: {}", self.id, db_err);
            }

            return Err(e);
        }

        log::info!("Strategy {} initialized successfully", self.id);

        // 策略主循环
        loop {
            // 检查是否暂停，如果暂停则等待一段时间后继续
            if self.paused.load(std::sync::atomic::Ordering::SeqCst) {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                continue;
            }

            tokio::select! {
                // 接收行情事件
                result = kline_stream.recv() => {
                    match result {
                        Ok(event) => {
                            if let MarketEvent::Kline(kline) = event {
                                // 再次检查暂停状态，防止在等待期间收到信号后暂停
                                if !self.paused.load(std::sync::atomic::Ordering::SeqCst) {
                                    if let Err(e) = self.on_kline(kline).await {
                                        log::error!("Error processing kline: {}", e);
                                        // 更新数据库状态为 error
                                        let error_msg = format!("Kline processing error: {}", e);
                                        if let Err(db_err) = self.instance_repo.update_status(&self.id, "error", Some(&error_msg)).await {
                                            log::error!("Failed to update instance {} error status: {}", self.id, db_err);
                                        }
                                    }
                                }
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

            // 更新数据库状态为 error
            let error_msg = format!("onStop failed: {}", e);
            if let Err(db_err) = self.instance_repo.update_status(&self.id, "error", Some(&error_msg)).await {
                log::error!("Failed to update instance {} error status: {}", self.id, db_err);
            }
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
            .or_default();
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
                self.event_bus.publish_signal(signal.clone());

                // 执行交易信号
                if let Err(e) = self.execute_signal(signal).await {
                    log::error!("Failed to execute signal: {}", e);
                }
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

    /// 执行交易信号
    async fn execute_signal(&self, signal: crate::core::event::Signal) -> Result<()> {
        log::info!(
            "Executing signal: {} {} {} @ {:?}",
            signal.action,
            signal.symbol,
            signal.quantity,
            signal.price
        );

        // 风控检查
        if !self.check_risk(&signal).await {
            log::warn!("Risk check failed for signal, ignoring: {:?}", signal);
            self.event_bus
                .publish_strategy_error(format!("Risk check failed: {:?}", signal));
            return Ok(());
        }

        // 将信号转换为订单请求
        let side = signal.action.parse::<OrderSide>().map_err(|_| {
            anyhow::anyhow!("Invalid order side: {}", signal.action)
        })?;

        let order_request = OrderRequest {
            symbol: signal.symbol.clone(),
            side,
            order_type: OrderType::Market,
            price: signal.price,
            stop_price: None,
            quantity: signal.quantity,
            client_order_id: Some(format!("{}-{}", self.id, uuid::Uuid::new_v4())),
            time_in_force: Some(TimeInForce::IOC),
        };

        // 执行订单
        match self.exchange.place_order(&order_request).await {
            Ok(order) => {
                log::info!("Order placed successfully: {}", order.id);
                self.event_bus.publish_order_placed(order);
            }
            Err(e) => {
                log::error!("Failed to place order: {}", e);
                self.event_bus
                    .publish_strategy_error(format!("Order failed: {}", e));
            }
        }

        Ok(())
    }

    /// 风控检查
    async fn check_risk(&self, signal: &crate::core::event::Signal) -> bool {
        // ========== 基础参数检查 ==========

        // 1. 数量必须为正数
        if signal.quantity <= 0.0 {
            log::warn!("[{}] Invalid quantity: {}", self.id, signal.quantity);
            return false;
        }

        // 2. 价格必须合理（如果提供）
        if let Some(price) = signal.price {
            if price <= 0.0 {
                log::warn!("[{}] Invalid price: {}", self.id, price);
                return false;
            }
        }

        // ========== 账户余额检查 ==========

        // 刷新账户余额缓存
        if let Ok(balances) = self.exchange.get_balance().await {
            let total_balance: f64 = balances.iter().map(|b| b.total).sum();
            *self.cached_balance.write().await = Some(total_balance);
        }

        // 检查余额是否足够
        let order_value = signal.quantity * signal.price.unwrap_or(0.0);
        if let Some(balance) = *self.cached_balance.read().await {
            if order_value > balance {
                log::warn!(
                    "[{}] Insufficient balance: order_value={}, balance={}",
                    self.id, order_value, balance
                );
                return false;
            }
        }

        // ========== 单笔订单金额限制 ==========

        if order_value > self.max_order_amount {
            log::warn!(
                "[{}] Order amount exceeds limit: order_value={}, max={}",
                self.id, order_value, self.max_order_amount
            );
            return false;
        }

        // ========== 日内交易次数限制 ==========

        let current_count = self.daily_trade_count.load(std::sync::atomic::Ordering::Relaxed);
        if current_count >= self.max_daily_trades {
            log::warn!(
                "[{}] Daily trade limit reached: {}/{}",
                self.id, current_count, self.max_daily_trades
            );
            return false;
        }

        // ========== 持仓限制检查 ==========

        // 刷新持仓缓存
        if let Ok(positions) = self.exchange.get_positions().await {
            *self.cached_positions.write().await = positions;
        }

        // 检查单个持仓限制
        let positions = self.cached_positions.read().await;
        let symbol_position = positions.iter()
            .find(|p| p.symbol == signal.symbol)
            .map(|p| p.quantity * p.entry_price)
            .unwrap_or(0.0);

        // 计算新订单后的持仓价值
        let new_position_value = symbol_position + order_value;

        // 单个交易对最大持仓限制（默认为账户余额的50%）
        let max_single_position = self.cached_balance.read().await
            .map(|b| b * 0.5)
            .unwrap_or(5000.0);

        if new_position_value > max_single_position {
            log::warn!(
                "[{}] Single position limit exceeded: symbol={}, new_value={}, max={}",
                self.id, signal.symbol, new_position_value, max_single_position
            );
            return false;
        }

        // 总持仓限制（默认为账户余额的90%）
        let total_position_value: f64 = positions.iter()
            .map(|p| p.quantity * p.entry_price)
            .sum::<f64>() + order_value;

        let max_total_position = self.cached_balance.read().await
            .map(|b| b * 0.9)
            .unwrap_or(9000.0);

        if total_position_value > max_total_position {
            log::warn!(
                "[{}] Total position limit exceeded: total_value={}, max={}",
                self.id, total_position_value, max_total_position
            );
            return false;
        }

        // ========== 风险规则检查 ==========

        // 构建风险上下文
        let risk_context = RiskContext {
            positions: positions.clone(),
            orders: Vec::new(), // 可从 exchange 获取
            balance: self.cached_balance.read().await.unwrap_or(0.0),
            today_pnl: 0.0, // 可从数据库计算
            instance_id: self.id.clone(),
        };

        // 检查所有启用的风控规则
        for rule in &self.risk_rules {
            if !rule.config().enabled {
                continue;
            }

            match rule.check(&risk_context).await {
                Ok(triggered) => {
                    if triggered {
                        log::warn!(
                            "[{}] Risk rule '{}' triggered, rejecting signal",
                            self.id, rule.name()
                        );
                        return false;
                    }
                }
                Err(e) => {
                    log::error!("[{}] Risk rule '{}' check error: {}", self.id, rule.name(), e);
                    // 风控检查出错时，拒绝订单以保安全
                    return false;
                }
            }
        }

        // ========== 所有检查通过 ==========

        log::debug!(
            "[{}] Risk check passed: symbol={}, quantity={}, price={:?}",
            self.id, signal.symbol, signal.quantity, signal.price
        );
        true
    }

    /// 停止策略实例
    pub fn stop(&self) {
        if let Some(ref tx) = self.shutdown_tx {
            let _ = tx.send(());
        }
    }

    /// 暂停策略实例
    pub fn pause(&self) {
        self.paused.store(true, std::sync::atomic::Ordering::SeqCst);
        log::info!("Strategy instance {} paused", self.id);
    }

    /// 恢复策略实例
    pub fn resume(&self) {
        self.paused.store(false, std::sync::atomic::Ordering::SeqCst);
        log::info!("Strategy instance {} resumed", self.id);
    }

    /// 获取实例信息
    pub fn info(&self) -> InstanceInfo {
        let status = if self.paused.load(std::sync::atomic::Ordering::SeqCst) {
            InstanceStatus::Paused
        } else {
            InstanceStatus::Running
        };

        InstanceInfo {
            id: self.id.clone(),
            name: self.config.name.clone(),
            status,
            symbols: self.config.symbols.clone(),
            timeframes: self.config.timeframes.clone(),
        }
    }
}

/// 策略引擎
pub struct StrategyEngine {
    instances: Arc<RwLock<HashMap<String, Arc<RwLock<RunningInstance>>>>>,
    event_bus: Arc<EventBus>,
    exchange: Arc<dyn Exchange>,
    instance_repo: Arc<StrategyInstanceRepository>,
}

impl StrategyEngine {
    /// 创建新的策略引擎
    pub fn new(
        event_bus: Arc<EventBus>,
        exchange: Arc<dyn Exchange>,
        instance_repo: Arc<StrategyInstanceRepository>,
    ) -> Self {
        Self {
            instances: Arc::new(RwLock::new(HashMap::new())),
            event_bus,
            exchange,
            instance_repo,
        }
    }

    /// 启动策略实例
    pub async fn start_instance(&self, config: StrategyConfig, user_id: String, exchange_id: Option<String>) -> Result<String> {
        let id = config.id.clone().unwrap_or_else(|| {
            uuid::Uuid::new_v4().to_string()
        });

        // 检查是否已存在
        let instances = self.instances.read().await;
        if instances.contains_key(&id) {
            return Err(anyhow::anyhow!("Strategy instance {} already exists", id));
        }
        drop(instances);

        log::info!("Starting strategy instance: {} ({}) for user: {}", id, config.name, user_id);

        // 构建数据库请求
        let strategy_id = id.clone();
        let name = config.name.clone();
        let parameters = config.parameters.clone();
        // 使用传入的 exchange_id，如果没有提供则返回错误
        let exchange_id = exchange_id.ok_or_else(|| {
            anyhow::anyhow!("exchange_id is required but not provided")
        })?;
        let symbol = config.symbols.first().cloned().unwrap_or_else(|| {
            log::warn!("Strategy {} has no symbols, using default", id);
            "BTCUSDT".to_string()
        });
        let timeframe = config.timeframes.first().cloned().unwrap_or_else(|| {
            log::warn!("Strategy {} has no timeframes, using default", id);
            "1h".to_string()
        });
        let mode = "paper".to_string();

        let create_req = CreateInstanceRequest {
            strategy_id,
            user_id: user_id.clone(),
            name,
            parameters,
            exchange_id,
            symbol,
            timeframe,
            mode,
        };

        // 在数据库中创建实例记录
        let db_instance = match self.instance_repo.create(create_req).await {
            Ok(instance) => instance,
            Err(e) => {
                log::error!("Failed to create strategy instance in database: {}", e);
                return Err(e);
            }
        };

        let instance_id = db_instance.id.clone();
        log::info!("Created strategy instance record in database: {}", instance_id);

        // 更新状态为 running
        if let Err(e) = self.instance_repo.update_status(&instance_id, "running", None).await {
            log::error!("Failed to update strategy instance status: {}", e);
            // 清理已创建的记录
            let _ = self.instance_repo.delete(&instance_id).await;
            return Err(e);
        }

        log::info!("Updated strategy instance {} status to running", instance_id);

        // 创建默认风控规则
        let risk_rules: Vec<Box<dyn RiskRule>> = vec![
            // TODO: 从配置加载风控规则
        ];

        // 创建运行实例
        let instance = RunningInstance::new(
            instance_id.clone(),
            config,
            self.event_bus.clone(),
            self.exchange.clone(),
            user_id,
            self.instance_repo.clone(),
            risk_rules,
        )?;

        // 启动策略循环
        let instance_ref = Arc::new(RwLock::new(instance));
        let instance_clone = instance_ref.clone();
        let instance_id_clone = instance_id.clone();
        let repo_clone = self.instance_repo.clone();

        tokio::spawn(async move {
            let mut instance = instance_clone.write().await;
            if let Err(e) = instance.run().await {
                log::error!("Strategy instance {} error: {}", instance_id_clone, e);
                // 更新数据库状态为 error
                let error_msg = e.to_string();
                if let Err(db_err) = repo_clone.update_status(&instance_id_clone, "error", Some(&error_msg)).await {
                    log::error!("Failed to update instance {} error status: {}", instance_id_clone, db_err);
                }
            }
        });

        // 保存实例到内存
        let mut instances = self.instances.write().await;
        instances.insert(instance_id.clone(), instance_ref);

        Ok(instance_id)
    }

    /// 停止策略实例
    pub async fn stop_instance(&self, id: &str) -> Result<()> {
        log::info!("Stopping strategy instance: {}", id);

        let mut instances = self.instances.write().await;
        if let Some(instance) = instances.remove(id) {
            let instance = instance.read().await;
            instance.stop();
            drop(instance);
        } else {
            return Err(anyhow::anyhow!("Strategy instance {} not found", id));
        }
        drop(instances);

        // 更新数据库状态为 stopped
        self.instance_repo
            .update_status(id, "stopped", None)
            .await
            .map_err(|e| {
                log::error!("Failed to update instance {} stopped status: {}", id, e);
                e
            })?;

        log::info!("Updated strategy instance {} status to stopped", id);
        Ok(())
    }

    /// 暂停策略实例
    pub async fn pause_instance(&self, id: &str) -> Result<()> {
        log::info!("Pausing strategy instance: {}", id);

        let instances = self.instances.read().await;
        if let Some(instance) = instances.get(id) {
            let instance = instance.read().await;
            instance.pause();
            drop(instance);
        } else {
            return Err(anyhow::anyhow!("Strategy instance {} not found", id));
        }
        drop(instances);

        // 更新数据库状态为 paused
        self.instance_repo
            .update_status(id, "paused", None)
            .await
            .map_err(|e| {
                log::error!("Failed to update instance {} paused status: {}", id, e);
                e
            })?;

        log::info!("Updated strategy instance {} status to paused", id);
        Ok(())
    }

    /// 恢复策略实例
    pub async fn resume_instance(&self, id: &str) -> Result<()> {
        log::info!("Resuming strategy instance: {}", id);

        let instances = self.instances.read().await;
        if let Some(instance) = instances.get(id) {
            let instance = instance.read().await;
            instance.resume();
            drop(instance);
        } else {
            return Err(anyhow::anyhow!("Strategy instance {} not found", id));
        }
        drop(instances);

        // 更新数据库状态为 running
        self.instance_repo
            .update_status(id, "running", None)
            .await
            .map_err(|e| {
                log::error!("Failed to update instance {} running status: {}", id, e);
                e
            })?;

        log::info!("Updated strategy instance {} status to running", id);
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

    /// 停止所有运行中的策略实例
    ///
    /// This method is used for emergency stop functionality to halt all strategies.
    /// It continues stopping instances even if individual instances fail to stop.
    pub async fn stop_all(&self) -> Result<usize> {
        log::warn!("Stopping all strategy instances");

        let mut instances = self.instances.write().await;
        let count = instances.len();
        let instance_ids: Vec<String> = instances.keys().cloned().collect();

        // Stop each instance
        let mut stopped_count = 0;
        for id in instance_ids {
            if let Some(instance) = instances.remove(&id) {
                let instance = instance.read().await;
                instance.stop();
                drop(instance);

                // Update database status
                if let Err(e) = self.instance_repo.update_status(&id, "stopped", None).await {
                    log::error!("Failed to update instance {} stopped status: {}", id, e);
                } else {
                    stopped_count += 1;
                }
            }
        }

        drop(instances);

        log::warn!("Stopped {}/{} strategy instances", stopped_count, count);
        Ok(stopped_count)
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
