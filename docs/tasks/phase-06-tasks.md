# Phase 6: 完善与优化 - 详细任务规范

## 目标

系统稳定、生产可用，包括错误处理、日志、配置、备份、性能优化、测试和文档。

---

## 任务概览

| ID | 任务 | 估时 |
|----|------|------|
| P6-01 | 错误处理统一 | 2h |
| P6-02 | 日志系统完善 | 1h |
| P6-03 | 配置文件管理 | 1h |
| P6-04 | 数据备份恢复 | 2h |
| P6-05 | 性能优化 | 3h |
| P6-06 | 单元测试 | 4h |
| P6-07 | 用户文档 | 3h |

---

## 核心任务详解

### P6-01: 错误处理统一

```rust
// src-tauri/src/core/error.rs

use thiserror::Error;

/// 应用统一错误类型
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Exchange error: {0}")]
    Exchange(String),

    #[error("Strategy error: {0}")]
    Strategy(String),

    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("Permission denied: {0}")]
    Permission(String),

    #[error("Invalid input: {0}")]
    Validation(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Risk limit triggered: {0}")]
    RiskLimit(String),
}

/// 将 AppError 转换为 Tauri 响应
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}

/// 结果类型别名
pub type AppResult<T> = Result<T, AppError>;
```

**Tauri 命令错误处理模板**：

```rust
// src-tauri/commands/mod.rs

use crate::core::error::AppError;

/// 统一的命令错误处理宏
macro_rules! command_handler {
    ($expr:expr) => {
        match $expr {
            Ok(result) => Ok(result),
            Err(e) => {
                log::error!("Command failed: {}", e);
                Err(format!("{}", e))
            }
        }
    };
}

#[tauri::command]
async fn some_command(param: String) -> Result<String, String> {
    command_handler!(async {
        // 业务逻辑
        Ok("success".to_string())
    }.await)
}
```

**前端错误处理**：

```typescript
// src/utils/error-handler.ts

export interface AppError {
  code: string;
  message: string;
  details?: any;
}

export class ErrorHandler {
  static handle(error: any): AppError {
    // Tauri 错误
    if (typeof error === 'string') {
      return {
        code: 'TAURI_ERROR',
        message: error,
      };
    }

    // 网络错误
    if (error?.response) {
      return {
        code: `HTTP_${error.response.status}`,
        message: error.response.data?.message || '网络请求失败',
        details: error.response.data,
      };
    }

    // 默认错误
    return {
      code: 'UNKNOWN_ERROR',
      message: error?.message || '未知错误',
      details: error,
    };
  }

  static show(error: AppError) {
    ElMessage.error({
      message: error.message,
      duration: 5000,
    });
  }
}

// API 调用包装
export async function callApi<T>(
  fn: () => Promise<T>
): Promise<T> {
  try {
    return await fn();
  } catch (error) {
    const appError = ErrorHandler.handle(error);
    ErrorHandler.show(appError);
    throw appError;
  }
}
```

### P6-02: 日志系统完善

```rust
// src-tauri/src/infrastructure/logging.rs

use tracing_subscriber::{
    fmt, prelude::*, registry, EnvFilter,
};
use tracing_appender::{rolling, non_blocking};
use std::path::PathBuf;

/// 初始化日志系统
pub fn init_log(app_dir: PathBuf) -> anyhow::Result<()> {
    let log_dir = app_dir.join("logs");
    std::fs::create_dir_all(&log_dir)?;

    // 文件日志 - 每天轮转
    let file_appender = rolling::daily(log_dir, "ai-lot.log");
    let (non_blocking_file, _guard) = non_blocking(file_appender);

    // 控制台日志
    let (non_blocking_console, _guard) = non_blocking(std::io::stdout());

    // 配置日志级别
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    // 组合日志输出
    registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_writer(non_blocking_file)
                .with_ansi(false)
                .with_target(true)
                .with_thread_ids(true)
        )
        .with(
            fmt::layer()
                .with_writer(non_blocking_console)
                .with_ansi(true)
        )
        .init();

    // 设置 panic 钩子
    std::panic::set_hook(Box::new(|panic_info| {
        log::error!("PANIC: {}", panic_info);
    }));

    Ok(())
}

/// 日志宏使用示例
/*
log::error!("Exchange connection failed: {}", e);
log::warn!("Order rejected: {}", order_id);
log::info!("Strategy started: {}", strategy_id);
log::debug!("Kline received: {:?}", kline);
*/
```

**日志配置文件**：`config/log.toml`

```toml
# 默认日志级别
default = "info"

# 模块特定级别
[modules]
"ai_lot::core::trade" = "debug"
"ai_lot::services::market" = "debug"
"sqlx" = "warn"
```

### P6-03: 配置文件管理

```rust
// src-tauri/src/infrastructure/config.rs

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// 应用设置
    pub app: AppSettings,
    /// 数据库配置
    pub database: DatabaseConfig,
    /// 风控配置
    pub risk: RiskConfig,
    /// 通知配置
    pub notifications: NotificationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// 界面语言
    pub language: String,
    /// 主题
    pub theme: String,
    /// 自动保存间隔（秒）
    pub auto_save_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// 数据库路径
    pub path: String,
    /// 备份间隔（小时）
    pub backup_interval_hours: u64,
    /// 备份保留天数
    pub backup_retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskConfig {
    /// 是否启用风控
    pub enabled: bool,
    /// 默认风控动作
    pub default_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// 钉钉 webhook
    pub dingtalk_webhook: Option<String>,
    /// 邮件 SMTP
    pub smtp_server: Option<String>,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>, // 加密存储
    pub notification_emails: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app: AppSettings {
                language: "zh-CN".to_string(),
                theme: "dark".to_string(),
                auto_save_interval: 60,
            },
            database: DatabaseConfig {
                path: "ai-lot.db".to_string(),
                backup_interval_hours: 24,
                backup_retention_days: 30,
            },
            risk: RiskConfig {
                enabled: true,
                default_action: "Notify".to_string(),
            },
            notifications: NotificationConfig {
                dingtalk_webhook: None,
                smtp_server: None,
                smtp_username: None,
                smtp_password: None,
                notification_emails: vec![],
            },
        }
    }
}

pub struct ConfigManager {
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new(app_dir: PathBuf) -> Self {
        let config_path = app_dir.join("config.toml");
        Self { config_path }
    }

    /// 加载配置
    pub async fn load(&self) -> anyhow::Result<AppConfig> {
        if !self.config_path.exists() {
            let default = AppConfig::default();
            self.save(&default).await?;
            return Ok(default);
        }

        let content = fs::read_to_string(&self.config_path).await?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// 保存配置
    pub async fn save(&self, config: &AppConfig) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(config)?;
        fs::write(&self.config_path, content).await?;
        Ok(())
    }

    /// 更新配置
    pub async fn update<F>(&self, updater: F) -> anyhow::Result<AppConfig>
    where
        F: FnOnce(&mut AppConfig),
    {
        let mut config = self.load().await?;
        updater(&mut config);
        self.save(&config).await?;
        Ok(config)
    }
}
```

**Tauri 命令**：

```rust
#[tauri::command]
async fn config_get() -> Result<AppConfig, String> {
    let config_manager = get_config_manager().await;
    command_handler!(config_manager.load().await)
}

#[tauri::command]
async fn config_update(updater: Json<AppConfigUpdater>) -> Result<AppConfig, String> {
    let config_manager = get_config_manager().await;
    command_handler!(config_manager.update(|cfg| updater.apply_to(cfg)).await)
}
```

### P6-04: 数据备份恢复

```rust
// src-tauri/src/services/backup_service.rs

use std::path::PathBuf;
use tokio::fs;

pub struct BackupService {
    db_path: PathBuf,
    backup_dir: PathBuf,
    retention_days: u32,
}

impl BackupService {
    pub fn new(db_path: PathBuf, backup_dir: PathBuf, retention_days: u32) -> Self {
        Self {
            db_path,
            backup_dir,
            retention_days,
        }
    }

    /// 创建备份
    pub async fn create_backup(&self) -> anyhow::Result<PathBuf> {
        fs::create_dir_all(&self.backup_dir).await?;

        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("backup_{}.db", timestamp);
        let backup_path = self.backup_dir.join(&backup_name);

        // 复制数据库文件
        fs::copy(&self.db_path, &backup_path).await?;

        // 压缩备份
        self.compress_backup(&backup_path).await?;

        log::info!("Backup created: {}", backup_path.display());
        Ok(backup_path)
    }

    /// 压缩备份
    async fn compress_backup(&self, path: &PathBuf) -> anyhow::Result<()> {
        let compressed_path = path.with_extension("db.gz");

        let input = fs::read(path).await?;
        let compressed = izip::compress(&input, izip::Compression::Best)?;

        fs::write(compressed_path, compressed).await?;
        fs::remove_file(path).await?; // 删除未压缩的文件

        Ok(())
    }

    /// 恢复备份
    pub async fn restore_backup(&self, backup_path: &PathBuf) -> anyhow::Result<()> {
        // 解压
        let compressed = fs::read(backup_path).await?;
        let decompressed = izip::decompress(&compressed)?;

        // 创建临时备份
        let temp_backup = self.db_path.with_extension("db.tmp");
        fs::copy(&self.db_path, &temp_backup).await?;

        // 写入恢复的数据库
        fs::write(&self.db_path, decompressed).await?;

        // 验证数据库完整性
        if let Err(e) = self.verify_database().await {
            fs::copy(&temp_backup, &self.db_path).await?;
            fs::remove_file(&temp_backup).await?;
            anyhow::bail!("Database verification failed: {}", e);
        }

        fs::remove_file(&temp_backup).await?;
        log::info!("Backup restored from: {}", backup_path.display());
        Ok(())
    }

    /// 验证数据库
    async fn verify_database(&self) -> anyhow::Result<()> {
        let pool = sqlx::SqlitePool::connect(&format!("sqlite://{}", self.db_path.display())).await?;

        // 执行完整性检查
        let result: (String,) = sqlx::query_as("PRAGMA integrity_check")
            .fetch_one(&pool)
            .await?;

        if result.0 != "ok" {
            anyhow::bail!("Database integrity check failed: {}", result.0);
        }

        Ok(())
    }

    /// 清理过期备份
    pub async fn cleanup_old_backups(&self) -> anyhow::Result<()> {
        let entries = fs::read_dir(&self.backup_dir).await?;
        let cutoff = chrono::Utc::now() - chrono::Duration::days(self.retention_days as i64);

        for entry in entries {
            let entry = entry?;
            let metadata = entry.metadata().await?;
            let modified = metadata.modified()?.into();
            let modified_chrono: chrono::DateTime<chrono::Utc> = modified.into();

            if modified_chrono < cutoff {
                fs::remove_file(entry.path()).await?;
                log::info!("Removed old backup: {}", entry.path().display());
            }
        }

        Ok(())
    }

    /// 启动自动备份任务
    pub async fn start_auto_backup(self: Arc<Self>, interval_hours: u64) {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(
                tokio::time::Duration::from_secs(interval_hours * 3600)
            );

            loop {
                interval.tick().await;
                if let Err(e) = self.create_backup().await {
                    log::error!("Auto backup failed: {}", e);
                }
                if let Err(e) = self.cleanup_old_backups().await {
                    log::error!("Backup cleanup failed: {}", e);
                }
            }
        });
    }
}
```

### P6-05: 性能优化

**数据库查询优化**：

```sql
-- 为常用查询添加索引
CREATE INDEX IF NOT EXISTS idx_klines_symbol_time
ON klines(symbol, timeframe, close_time DESC);

CREATE INDEX IF NOT EXISTS idx_orders_user_status
ON orders(user_id, status, created_at DESC);

CREATE INDEX IF NOT EXISTS idx_positions_instance
ON positions(instance_id, status);

-- 使用视图优化复杂查询
CREATE VIEW IF NOT EXISTS v_active_positions AS
SELECT
    p.*,
    s.name as strategy_name,
    s.user_id
FROM positions p
JOIN strategy_instances si ON p.instance_id = si.id
JOIN strategies s ON si.strategy_id = s.id
WHERE p.status = 'open';
```

**Rust 性能优化**：

```rust
// src-tauri/src/infrastructure/cache.rs

use std::time::Duration;
use moka::future::Cache;

/// 通用内存缓存
pub struct CacheManager<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    cache: Cache<K, V>,
}

impl<K, V> CacheManager<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new(max_capacity: u64, ttl: Duration) -> Self {
        Self {
            cache: Cache::builder()
                .max_capacity(max_capacity)
                .time_to_live(ttl)
                .build(),
        }
    }

    pub async fn get(&self, key: &K) -> Option<V> {
        self.cache.get(key).await
    }

    pub async fn insert(&self, key: K, value: V) {
        self.cache.insert(key, value).await;
    }

    pub async fn invalidate(&self, key: &K) {
        self.cache.invalidate(key).await;
    }

    pub async fn clear(&self) {
        self.cache.invalidate_all();
    }
}

// 使用示例
lazy_static! {
    static ref TICKER_CACHE: Arc<CacheManager<String, Ticker>> =
        CacheManager::new(1000, Duration::from_secs(5));
}
```

**前端性能优化**：

```typescript
// src/utils/performance.ts

// 防抖
export function debounce<T extends (...args: any[]) => any>(
  fn: T,
  delay: number
): (...args: Parameters<T>) => void {
  let timeout: ReturnType<typeof setTimeout> | null = null;
  return function(this: any, ...args: Parameters<T>) {
    if (timeout) clearTimeout(timeout);
    timeout = setTimeout(() => fn.apply(this, args), delay);
  };
}

// 节流
export function throttle<T extends (...args: any[]) => any>(
  fn: T,
  interval: number
): (...args: Parameters<T>) => void {
  let lastTime = 0;
  return function(this: any, ...args: Parameters<T>) {
    const now = Date.now();
    if (now - lastTime >= interval) {
      lastTime = now;
      fn.apply(this, args);
    }
  };
}

// 虚拟滚动优化
export function useVirtualList<T>(
  items: Ref<T[]>,
  itemHeight: number,
  containerHeight: number
) {
  const visibleCount = Math.ceil(containerHeight / itemHeight);
  const scrollTop = ref(0);

  const startIndex = computed(() =>
    Math.max(0, Math.floor(scrollTop.value / itemHeight) - 5)
  );
  const endIndex = computed(() =>
    Math.min(items.value.length, startIndex.value + visibleCount + 10)
  );

  const visibleItems = computed(() =>
    items.value.slice(startIndex.value, endIndex.value)
  );

  const offsetY = computed(() => startIndex.value * itemHeight);

  return {
    visibleItems,
    offsetY,
    scrollTop,
    totalHeight: items.value.length * itemHeight,
  };
}
```

### P6-06: 单元测试

```rust
// src-tauri/src/core/trade/order/tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_state_transition_pending_to_open() {
        let mut machine = OrderStateMachine::new();
        assert_eq!(machine.state(), &OrderState::Pending);

        assert!(machine.transition_to(OrderState::Open).is_ok());
        assert_eq!(machine.state(), &OrderState::Open);
    }

    #[test]
    fn test_order_state_transition_invalid() {
        let mut machine = OrderStateMachine::new();

        // 不能从 Pending 直接跳到 Filled
        assert!(machine.transition_to(OrderState::Filled).is_err());
        assert_eq!(machine.state(), &OrderState::Pending);
    }

    #[test]
    fn test_order_state_full_cycle() {
        let mut machine = OrderStateMachine::new();

        // Pending -> Open -> PartiallyFilled -> Filled
        assert!(machine.transition_to(OrderState::Open).is_ok());
        assert!(machine.transition_to(OrderState::PartiallyFilled).is_ok());
        assert!(machine.transition_to(OrderState::Filled).is_ok());

        assert_eq!(machine.state(), &OrderState::Filled);
    }
}

// src-tauri/src/core/risk/tests.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_position_limit_rule() {
        let rule = PositionLimitRule::new(1000.0, 5000.0, 0.8);

        let context = RiskContext {
            positions: vec![
                Position {
                    symbol: "BTCUSDT".to_string(),
                    quantity: 1.0,
                    entry_price: 50000.0,
                    ..Default::default()
                }
            ],
            ..Default::default()
        };

        // 50000 > 1000，应该触发
        assert!(rule.check(&context).await.unwrap());
    }

    #[tokio::test]
    async fn test_drawdown_limit_rule() {
        let rule = DrawdownLimitRule::new(10.0);

        let mut context = RiskContext {
            positions: vec![],
            balance: 9000.0,
            ..Default::default()
        };

        // 设置峰值为 10000
        rule.update_peak(&context.instance_id, 10000.0);

        // 回撤 = (10000 - 9000) / 10000 = 10%，刚好触发
        assert!(rule.check(&context).await.unwrap());
    }
}
```

### P6-07: 用户文档

创建 `README.md`：

```markdown
# AI-LOT 加密货币量化交易系统

## 简介

AI-LOT 是一个基于 Tauri + Vue 3 的桌面量化交易系统，支持：

- 多交易所统一接口
- JavaScript 策略编写
- 历史回测
- 实盘交易
- 风控系统

## 快速开始

### 环境要求

- Rust 1.70+
- Bun (前端包管理器)

### 安装

\`\`\`bash
# 克隆项目
git clone https://github.com/your-org/ai-lot.git
cd ai-lot

# 安装依赖
bun install
cd src-tauri
cargo fetch
cd ..

# 启动开发服务器
bun run tauri dev
\`\`\`

### 配置交易所

1. 打开应用，登录系统
2. 进入"交易所管理"页面
3. 添加交易所 API 密钥
4. 测试连接

### 编写策略

1. 进入"策略管理"页面
2. 点击"新建策略"
3. 使用 Monaco Editor 编写 JavaScript 代码

\`\`\`javascript
// 策略示例
function onInit() {
  log.info("策略初始化");
}

function onBar(kline) {
  // 简单的均线策略
  const ma5 = ma(klines, 5);
  const ma20 = ma(klines, 20);

  if (ma5 > ma20 && position.size === 0) {
    return {
      action: 'buy',
      quantity: 0.1,
    };
  }

  return null;
}
\`\`\`

### 回测

1. 选择策略
2. 配置回测参数（时间范围、初始资金等）
3. 点击"开始回测"
4. 查看回测报告

### 实盘运行

1. 选择策略
2. 配置运行参数（交易对、时间周期等）
3. 点击"启动策略"
4. 在交易控制台监控运行

## 目录结构

\`\`\`
ai-lot/
├── src/              # Vue 前端
│   ├── views/        # 页面组件
│   ├── components/   # 通用组件
│   ├── store/        # Pinia 状态管理
│   └── api/          # API 封装
├── src-tauri/        # Rust 后端
│   ├── commands/     # Tauri 命令
│   ├── core/         # 核心业务逻辑
│   ├── services/     # 服务层
│   └── infrastructure/ # 基础设施
└── docs/             # 文档
\`\`\`

## 安全注意事项

1. **API 密钥加密**：所有 API 密钥使用 AES-256-GCM 加密存储
2. **风控规则**：建议启用仓位限制和回撤限制
3. **小资金测试**：实盘前先用小资金测试
4. **定期备份**：启用自动备份功能

## 常见问题

### Q: 连接交易所失败？
A: 检查 API 密钥是否正确，网络是否畅通。

### Q: 策略不执行交易？
A: 检查策略代码是否返回信号，查看日志确认。

### Q: 如何停止策略？
A: 在交易控制台点击"停止"按钮。

## 技术支持

- GitHub Issues: https://github.com/your-org/ai-lot/issues
- 文档: https://docs.ai-lot.com

## 许可证

MIT License
```

创建 `docs/user-guide.md`：

```markdown
# AI-LOT 用户指南

## 1. 首次使用

### 1.1 登录系统

默认管理员账号：
- 用户名: admin
- 密码: admin123

首次登录后请立即修改密码！

### 1.2 配置交易所

1. 点击左侧菜单"交易所"
2. 点击"添加交易所"
3. 填写信息：
   - 交易所: Binance
   - API Key: 从交易所获取
   - Secret Key: 从交易所获取
4. 点击"测试连接"确认
5. 点击"保存"

> 注意：API 密钥加密存储，请妥善保管

## 2. 策略开发

### 2.1 策略结构

\`\`\`javascript
// 必需：策略初始化
function onInit() {
  // 设置参数
  setParameter('fastPeriod', 5);
  setParameter('slowPeriod', 20);
}

// 必需：K线回调
function onBar(kline) {
  // kline 结构：
  // {
  //   symbol: "BTCUSDT",
  //   openTime: 1234567890000,
  //   open: 50000,
  //   high: 50100,
  //   low: 49900,
  //   close: 50050,
  //   volume: 123.45
  // }

  // 返回交易信号
  return {
    action: 'buy',  // 或 'sell'
    quantity: 0.1,
    price: 50000,   // 可选，不填则为市价
  };
}

// 可选：策略停止
function onStop() {
  // 清理资源
}
\`\`\`

### 2.2 可用 API

- `log.info(message)` - 记录信息
- `log.warn(message)` - 记录警告
- `log.error(message)` - 记录错误
- `ma(period)` - 计算均线
- `position` - 当前持仓信息
- `account` - 账户信息

## 3. 回测

### 3.1 配置回测

| 参数 | 说明 |
|------|------|
| 交易对 | BTCUSDT、ETHUSDT 等 |
| 时间周期 | 1m、5m、15m、1h、4h、1d |
| 开始时间 | 回测起始时间 |
| 结束时间 | 回测结束时间 |
| 初始资金 | 起始资金量 (USDT) |
| 手续费率 | 交易手续费 |

### 3.2 查看报告

回测完成后可查看：
- 总收益率
- 年化收益
- 最大回撤
- 夏普比率
- 胜率
- 盈亏比
- 资金曲线

## 4. 实盘交易

### 4.1 启动策略

1. 选择策略和配置
2. 点击"启动策略"
3. 系统自动：
   - 连接交易所 WebSocket
   - 订阅行情
   - 执行策略逻辑
   - 下单交易

### 4.2 监控运行

在交易控制台可查看：
- 运行状态
- 当前持仓
- 活跃订单
- 实时盈亏

### 4.3 手动交易

也可以手动下单：
1. 选择交易对
2. 选择方向（买入/卖出）
3. 选择类型（市价/限价）
4. 输入数量/价格
5. 点击"下单"

## 5. 风控管理

### 5.1 配置规则

建议启用：
- **仓位限制**: 单个交易对最大持仓
- **回撤限制**: 达到回撤后自动平仓
- **紧急停止**: 一键停止所有策略

### 5.2 设置通知

- 钉钉: 填写 webhook URL
- 邮件: 配置 SMTP 信息
- 接收风控预警和重要通知

## 6. 数据管理

### 6.1 自动备份

系统默认每 24 小时自动备份数据库。

### 6.2 手动备份

在设置页面可以：
- 立即创建备份
- 从备份恢复
- 清理过期备份
```

---

## Phase 6 验收标准

### 功能验收
- [ ] 错误提示友好
- [ ] 日志完整可查
- [ ] 配置可读写
- [ ] 备份恢复正常
- [ ] 无明显性能问题

### 技术验收
- [ ] 单元测试覆盖率 > 60%
- [ ] 关键路径有测试
- [ ] 无内存泄漏
- [ ] 无明显瓶颈
- [ ] 文档完整
