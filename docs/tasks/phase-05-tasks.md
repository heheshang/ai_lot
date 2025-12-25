# Phase 5: 风控系统 - 详细任务规范

## 目标

实现风险监控、预警通知和紧急止损功能。

---

## 任务概览

| ID | 任务 | 估时 |
|----|------|------|
| P5-01 | 定义 RiskRule Trait | 1h |
| P5-02 | 实现仓位限制规则 | 1h |
| P5-03 | 实现回撤限制规则 | 1h |
| P5-04 | 实现风控监控 | 2h |
| P5-05 | 实现预警记录 | 1h |
| P5-06 | 实现钉钉通知 | 1h |
| P5-07 | 实现邮件通知 | 1h |
| P5-08 | 实现紧急止损功能 | 1h |
| P5-09 | 实现风控规则配置页面 | 2h |
| P5-10 | 实现风险监控仪表盘 | 2h |
| P5-11 | 实现预警历史页面 | 1h |

---

## 核心任务详解

### P5-01: 定义 RiskRule Trait

```rust
// src-tauri/src/core/risk/rule.rs

use crate::core::trade::types::*;
use anyhow::Result;

/// 风控规则 trait
#[async_trait]
pub trait RiskRule: Send + Sync {
    /// 规则名称
    fn name(&self) -> &str;

    /// 规则描述
    fn description(&self) -> &str;

    /// 检查是否触发风控
    async fn check(&self, context: &RiskContext) -> Result<bool>;

    /// 获取规则配置
    fn config(&self) -> &RiskRuleConfig;

    /// 更新规则配置
    fn update_config(&mut self, config: RiskRuleConfig) -> Result<()>;
}

/// 风控检查上下文
#[derive(Clone)]
pub struct RiskContext {
    /// 当前持仓
    pub positions: Vec<Position>,
    /// 活跃订单
    pub orders: Vec<Order>,
    /// 账户余额
    pub balance: f64,
    /// 今日盈亏
    pub today_pnl: f64,
    /// 策略实例ID
    pub instance_id: String,
}

/// 风控规则配置
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RiskRuleConfig {
    /// 规则是否启用
    pub enabled: bool,
    /// 触发动作
    pub action: RiskAction,
    /// 通知方式
    pub notify_methods: Vec<String>,
}

/// 风控触发后的动作
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum RiskAction {
    /// 仅记录
    LogOnly,
    /// 发送通知
    Notify,
    /// 暂停策略
    PauseStrategy,
    /// 平仓止损
    ClosePositions,
    /// 紧急停止所有策略
    EmergencyStop,
}
```

### P5-02: 实现仓位限制规则

```rust
// src-tauri/src/core/risk/position_limit.rs

use crate::core::risk::rule::*;
use anyhow::Result;

/// 仓位限制规则
pub struct PositionLimitRule {
    config: RiskRuleConfig,
    /// 单个交易对最大持仓价值
    max_position_value: f64,
    /// 总持仓价值限制
    max_total_value: f64,
    /// 单个方向最大持仓比例
    max_direction_ratio: f64,
}

impl PositionLimitRule {
    pub fn new(
        max_position_value: f64,
        max_total_value: f64,
        max_direction_ratio: f64,
    ) -> Self {
        Self {
            config: RiskRuleConfig {
                enabled: true,
                action: RiskAction::Notify,
                notify_methods: vec!["dingtalk".to_string()],
            },
            max_position_value,
            max_total_value,
            max_direction_ratio,
        }
    }

    /// 计算当前持仓总价值
    fn calculate_position_value(&self, positions: &[Position]) -> f64 {
        positions
            .iter()
            .map(|p| p.quantity * p.entry_price)
            .sum()
    }

    /// 计算多头持仓比例
    fn calculate_long_ratio(&self, positions: &[Position]) -> f64 {
        let total_value = self.calculate_position_value(positions);
        if total_value == 0.0 {
            return 0.0;
        }

        let long_value = positions
            .iter()
            .filter(|p| p.side == "long")
            .map(|p| p.quantity * p.entry_price)
            .sum::<f64>();

        long_value / total_value
    }
}

#[async_trait]
impl RiskRule for PositionLimitRule {
    fn name(&self) -> &str {
        "position_limit"
    }

    fn description(&self) -> &str {
        "限制单个交易对和总持仓价值"
    }

    async fn check(&self, context: &RiskContext) -> Result<bool> {
        // 检查单个交易对持仓
        for position in &context.positions {
            let value = position.quantity * position.entry_price;
            if value > self.max_position_value {
                log::warn!(
                    "Position limit exceeded: {} = {} > {}",
                    position.symbol,
                    value,
                    self.max_position_value
                );
                return Ok(true);
            }
        }

        // 检查总持仓
        let total_value = self.calculate_position_value(&context.positions);
        if total_value > self.max_total_value {
            log::warn!(
                "Total position limit exceeded: {} > {}",
                total_value,
                self.max_total_value
            );
            return Ok(true);
        }

        // 检查方向比例
        let long_ratio = self.calculate_long_ratio(&context.positions);
        if long_ratio > self.max_direction_ratio
            || long_ratio < (1.0 - self.max_direction_ratio)
        {
            log::warn!("Direction ratio imbalance: {}", long_ratio);
            return Ok(true);
        }

        Ok(false)
    }

    fn config(&self) -> &RiskRuleConfig {
        &self.config
    }

    fn update_config(&mut self, config: RiskRuleConfig) -> Result<()> {
        self.config = config;
        Ok(())
    }
}
```

### P5-03: 实现回撤限制规则

```rust
// src-tauri/src/core/risk/drawdown_limit.rs

use crate::core::risk::rule::*;
use std::collections::HashMap;

/// 回撤限制规则
pub struct DrawdownLimitRule {
    config: RiskRuleConfig,
    /// 最大回撤百分比
    max_drawdown_pct: f64,
    /// 记录每个策略的峰值权益
    peak_equity: HashMap<String, f64>,
}

impl DrawdownLimitRule {
    pub fn new(max_drawdown_pct: f64) -> Self {
        Self {
            config: RiskRuleConfig {
                enabled: true,
                action: RiskAction::ClosePositions,
                notify_methods: vec!["dingtalk".to_string(), "email".to_string()],
            },
            max_drawdown_pct,
            peak_equity: HashMap::new(),
        }
    }

    /// 计算当前回撤百分比
    fn calculate_drawdown(&self, current_equity: f64, peak: f64) -> f64 {
        if peak == 0.0 {
            return 0.0;
        }
        ((peak - current_equity) / peak) * 100.0
    }

    /// 更新峰值权益
    fn update_peak(&mut self, instance_id: &str, current_equity: f64) {
        let peak = self
            .peak_equity
            .entry(instance_id.to_string())
            .or_insert(current_equity);
        *peak = (*peak).max(current_equity);
    }
}

#[async_trait]
impl RiskRule for DrawdownLimitRule {
    fn name(&self) -> &str {
        "drawdown_limit"
    }

    fn description(&self) -> &str {
        "限制最大回撤百分比，超过后触发止损"
    }

    async fn check(&self, context: &RiskContext) -> Result<bool> {
        // 计算当前权益
        let position_value: f64 = context
            .positions
            .iter()
            .map(|p| p.quantity * p.entry_price + p.unrealized_pnl)
            .sum();
        let current_equity = context.balance + position_value;

        // 获取峰值
        let peak = self
            .peak_equity
            .get(&context.instance_id)
            .copied()
            .unwrap_or(current_equity);

        // 计算回撤
        let drawdown = self.calculate_drawdown(current_equity, peak);

        if drawdown > self.max_drawdown_pct {
            log::warn!(
                "Drawdown limit exceeded: {:.2}% > {:.2}%",
                drawdown,
                self.max_drawdown_pct
            );
            return Ok(true);
        }

        Ok(false)
    }

    fn config(&self) -> &RiskRuleConfig {
        &self.config
    }

    fn update_config(&mut self, config: RiskRuleConfig) -> Result<()> {
        self.config = config;
        Ok(())
    }
}
```

### P5-04: 实现风控监控

```rust
// src-tauri/src/core/risk/monitor.rs

use crate::core::risk::rule::*;
use crate::core::trade::types::*;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};

pub struct RiskMonitor {
    rules: Arc<RwLock<Vec<Box<dyn RiskRule>>>>,
    trade_service: Arc<TradeService>,
    notification_service: Arc<NotificationService>,
}

impl RiskMonitor {
    pub fn new(
        trade_service: Arc<TradeService>,
        notification_service: Arc<NotificationService>,
    ) -> Self {
        Self {
            rules: Arc::new(RwLock::new(Vec::new())),
            trade_service,
            notification_service,
        }
    }

    /// 添加风控规则
    pub async fn add_rule(&self, rule: Box<dyn RiskRule>) {
        let mut rules = self.rules.write().await;
        rules.push(rule);
    }

    /// 启动风控监控
    pub async fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            let mut timer = interval(Duration::from_secs(10));

            loop {
                timer.tick().await;

                if let Err(e) = self.check_all_rules().await {
                    log::error!("Risk check failed: {}", e);
                }
            }
        });
    }

    /// 检查所有规则
    async fn check_all_rules(&self) -> Result<()> {
        // 获取当前状态
        let positions = self.trade_service.get_positions().await?;
        let orders = self.trade_service.get_active_orders().await?;
        let balance = self.trade_service.get_account_balance().await?;

        // 构建上下文
        let context = RiskContext {
            positions: positions.clone(),
            orders,
            balance,
            today_pnl: 0.0, // TODO: 从数据库计算
            instance_id: "default".to_string(),
        };

        // 检查所有规则
        let rules = self.rules.read().await;
        for rule in rules.iter() {
            if !rule.config().enabled {
                continue;
            }

            match rule.check(&context).await {
                Ok(triggered) => {
                    if triggered {
                        self.handle_rule_trigger(rule, &context).await?;
                    }
                }
                Err(e) => {
                    log::error!("Rule {} check failed: {}", rule.name(), e);
                }
            }
        }

        Ok(())
    }

    /// 处理规则触发
    async fn handle_rule_trigger(
        &self,
        rule: &Box<dyn RiskRule>,
        context: &RiskContext,
    ) -> Result<()> {
        let config = rule.config();

        // 记录预警
        self.record_alert(rule, context).await?;

        // 发送通知
        for method in &config.notify_methods {
            self.send_notification(method, rule, context).await?;
        }

        // 执行动作
        match &config.action {
            RiskAction::LogOnly => {}
            RiskAction::Notify => {
                // 已在上面处理
            }
            RiskAction::PauseStrategy => {
                self.pause_strategy(&context.instance_id).await?;
            }
            RiskAction::ClosePositions => {
                self.close_all_positions(&context.instance_id).await?;
            }
            RiskAction::EmergencyStop => {
                self.emergency_stop().await?;
            }
        }

        Ok(())
    }

    /// 记录预警到数据库
    async fn record_alert(
        &self,
        rule: &Box<dyn RiskRule>,
        context: &RiskContext,
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO risk_alerts (rule_name, instance_id, message, action, created_at)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(rule.name())
        .bind(&context.instance_id)
        .bind(format!("Rule '{}' triggered", rule.name()))
        .bind(format!("{:?}", rule.config().action))
        .bind(chrono::Utc::now().timestamp())
        .execute(&self.trade_service.db.pool)
        .await?;

        Ok(())
    }

    /// 发送通知
    async fn send_notification(
        &self,
        method: &str,
        rule: &Box<dyn RiskRule>,
        context: &RiskContext,
    ) -> Result<()> {
        let message = format!(
            "风控触发: {}\n策略: {}\n动作: {:?}",
            rule.name(),
            context.instance_id,
            rule.config().action
        );

        match method {
            "dingtalk" => {
                self.notification_service.send_dingtalk(&message).await?;
            }
            "email" => {
                self.notification_service.send_email("风控预警", &message).await?;
            }
            _ => {
                log::warn!("Unknown notification method: {}", method);
            }
        }

        Ok(())
    }

    /// 暂停策略
    async fn pause_strategy(&self, instance_id: &str) -> Result<()> {
        log::warn!("Pausing strategy due to risk rule: {}", instance_id);
        // 调用策略引擎停止实例
        Ok(())
    }

    /// 平仓所有持仓
    async fn close_all_positions(&self, instance_id: &str) -> Result<()> {
        log::warn!("Closing all positions for strategy: {}", instance_id);
        // 调用交易服务平仓
        Ok(())
    }

    /// 紧急停止
    async fn emergency_stop(&self) -> Result<()> {
        log::error!("EMERGENCY STOP triggered!");
        // 停止所有策略并平仓
        Ok(())
    }
}
```

### P5-05: 实现预警记录

```sql
-- 创建预警表（如果不存在）
CREATE TABLE IF NOT EXISTS risk_alerts (
    id TEXT PRIMARY KEY,
    rule_name TEXT NOT NULL,
    instance_id TEXT NOT NULL,
    message TEXT NOT NULL,
    action TEXT NOT NULL,
    resolved BOOLEAN DEFAULT FALSE,
    created_at INTEGER NOT NULL,
    resolved_at INTEGER
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_risk_alerts_instance
ON risk_alerts(instance_id);

CREATE INDEX IF NOT EXISTS idx_risk_alerts_created
ON risk_alerts(created_at DESC);
```

### P5-06: 实现钉钉通知

```rust
// src-tauri/src/infrastructure/notification/dingtalk.rs

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct DingTalkNotifier {
    webhook_url: String,
    client: Client,
}

#[derive(Serialize)]
struct DingTalkMessage {
    msgtype: String,
    text: DingTalkText,
}

#[derive(Serialize)]
struct DingTalkText {
    content: String,
}

impl DingTalkNotifier {
    pub fn new(webhook_url: String) -> Self {
        Self {
            webhook_url,
            client: Client::new(),
        }
    }

    pub async fn send(&self, message: &str) -> Result<()> {
        let payload = DingTalkMessage {
            msgtype: "text".to_string(),
            text: DingTalkText {
                content: format!("[AI-LOT 风控预警]\n{}", message),
            },
        };

        let resp = self
            .client
            .post(&self.webhook_url)
            .json(&payload)
            .send()
            .await?;

        if resp.status().is_success() {
            Ok(())
        } else {
            anyhow::bail!("DingTalk notification failed: {}", resp.status())
        }
    }
}
```

### P5-07: 实现邮件通知

```rust
// src-tauri/src/infrastructure/notification/email.rs

use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

#[derive(Clone)]
pub struct EmailNotifier {
    smtp_server: String,
    smtp_username: String,
    smtp_password: String,
    from_email: String,
    to_emails: Vec<String>,
}

impl EmailNotifier {
    pub fn new(
        smtp_server: String,
        smtp_username: String,
        smtp_password: String,
        from_email: String,
        to_emails: Vec<String>,
    ) -> Self {
        Self {
            smtp_server,
            smtp_username,
            smtp_password,
            from_email,
            to_emails,
        }
    }

    pub async fn send(&self, subject: &str, body: &str) -> Result<()> {
        let credentials = Credentials::new(
            self.smtp_username.clone(),
            self.smtp_password.clone(),
        );

        let mailer = SmtpTransport::relay(&self.smtp_server)?
            .credentials(credentials)
            .build();

        for to_email in &self.to_emails {
            let email = Message::builder()
                .from(self.from_email.parse()?)
                .to(to_email.parse()?)
                .subject(subject)
                .header(ContentType::TEXT_PLAIN)
                .body(body.to_string())?;

            mailer.send(&email)?;
        }

        Ok(())
    }
}
```

### P5-08: 实现紧急止损功能

```rust
// src-tauri/src/services/emergency_service.rs

use crate::core::trade::types::*;
use anyhow::Result;

pub struct EmergencyService {
    trade_service: Arc<TradeService>,
    strategy_engine: Arc<StrategyEngine>,
}

impl EmergencyService {
    pub async fn emergency_stop_all(&self) -> Result<()> {
        log::error!("!!! EMERGENCY STOP INITIATED !!!");

        // 1. 停止所有策略实例
        self.strategy_engine.stop_all().await?;

        // 2. 撤销所有挂单
        self.cancel_all_orders().await?;

        // 3. 平仓所有持仓
        self.close_all_positions().await?;

        // 4. 发送紧急通知
        self.send_emergency_alert().await?;

        log::error!("!!! EMERGENCY STOP COMPLETED !!!");
        Ok(())
    }

    async fn cancel_all_orders(&self) -> Result<()> {
        let orders = self.trade_service.get_active_orders().await?;

        for order in orders {
            if let Err(e) = self.trade_service.cancel_order(&order.id).await {
                log::error!("Failed to cancel order {}: {}", order.id, e);
            }
        }

        Ok(())
    }

    async fn close_all_positions(&self) -> Result<()> {
        let positions = self.trade_service.get_positions().await?;

        for position in positions {
            // 创建市价平仓单
            let close_request = OrderRequest {
                symbol: position.symbol.clone(),
                side: if position.side == "long" {
                    OrderSide::Sell
                } else {
                    OrderSide::Buy
                },
                order_type: OrderType::Market,
                quantity: position.quantity,
                price: None,
            };

            if let Err(e) = self.trade_service.place_order(close_request).await {
                log::error!("Failed to close position {}: {}", position.symbol, e);
            }
        }

        Ok(())
    }

    async fn send_emergency_alert(&self) -> Result<()> {
        // 发送最高级别紧急通知
        Ok(())
    }
}
```

### P5-09 ~ P5-11: 风控前端

创建 `src/views/Risk/RiskRules.vue`：

```vue
<template>
  <div class="risk-rules">
    <el-card>
      <template #header>
        <div class="header">
          <h2>风控规则配置</h2>
          <el-button type="primary" @click="handleSave">保存配置</el-button>
        </div>
      </template>

      <el-table :data="rules" border>
        <el-table-column prop="name" label="规则名称" width="150" />
        <el-table-column prop="description" label="说明" />

        <el-table-column label="启用" width="80">
          <template #default="{ row }">
            <el-switch v-model="row.enabled" />
          </template>
        </el-table-column>

        <el-table-column label="触发动作" width="150">
          <template #default="{ row }">
            <el-select v-model="row.action">
              <el-option label="仅记录" value="LogOnly" />
              <el-option label="发送通知" value="Notify" />
              <el-option label="暂停策略" value="PauseStrategy" />
              <el-option label="平仓止损" value="ClosePositions" />
              <el-option label="紧急停止" value="EmergencyStop" />
            </el-select>
          </template>
        </el-table-column>

        <el-table-column label="通知方式" width="200">
          <template #default="{ row }">
            <el-checkbox-group v-model="row.notifyMethods">
              <el-checkbox label="dingtalk">钉钉</el-checkbox>
              <el-checkbox label="email">邮件</el-checkbox>
            </el-checkbox-group>
          </template>
        </el-table-column>

        <el-table-column label="参数配置" width="300">
          <template #default="{ row }">
            <div v-if="row.name === 'position_limit'">
              <el-form-item label="单持仓限制:">
                <el-input-number v-model="row.maxPositionValue" :min="0" />
              </el-form-item>
              <el-form-item label="总持仓限制:">
                <el-input-number v-model="row.maxTotalValue" :min="0" />
              </el-form-item>
            </div>
            <div v-if="row.name === 'drawdown_limit'">
              <el-form-item label="最大回撤(%):">
                <el-input-number v-model="row.maxDrawdownPct" :min="0" :max="100" />
              </el-form-item>
            </div>
          </template>
        </el-table-column>
      </el-table>
    </el-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { ElMessage } from 'element-plus';

const rules = ref([
  {
    name: 'position_limit',
    description: '限制单个交易对和总持仓价值',
    enabled: true,
    action: 'Notify',
    notifyMethods: ['dingtalk'],
    maxPositionValue: 1000,
    maxTotalValue: 5000,
  },
  {
    name: 'drawdown_limit',
    description: '限制最大回撤百分比',
    enabled: true,
    action: 'ClosePositions',
    notifyMethods: ['dingtalk', 'email'],
    maxDrawdownPct: 10,
  },
]);

async function handleSave() {
  try {
    // 调用保存 API
    ElMessage.success('配置已保存');
  } catch (error) {
    ElMessage.error('保存失败：' + (error as Error).message);
  }
}
</script>
```

创建 `src/views/Risk/RiskMonitor.vue`：

```vue
<template>
  <div class="risk-monitor">
    <el-row :gutter="20">
      <!-- 风险指标 -->
      <el-col :span="6" v-for="metric in metrics" :key="metric.label">
        <el-card>
          <statistic
            :title="metric.label"
            :value="metric.value"
            :unit="metric.unit"
            :trend="metric.trend"
          />
        </el-card>
      </el-col>
    </el-row>

    <!-- 紧急操作 -->
    <el-card header="紧急操作" style="margin-top: 20px;">
      <el-button type="danger" @click="handleEmergencyStop">
        <el-icon><Warning /></el-icon>
        紧急停止所有策略
      </el-button>
      <el-button type="warning" @click="handleCloseAll">
        平仓所有持仓
      </el-button>
    </el-card>
  </div>
</template>
```

---

## Phase 5 验收标准

### 功能验收
- [ ] 风控规则可配置
- [ ] 规则触发后正确执行动作
- [ ] 通知可正常发送
- [ ] 紧急止损功能可用

### 技术验收
- [ ] 规则检查及时（10秒内）
- [ ] 通知发送成功
- [ ] 数据库记录完整
