# AI-LOT 数据库设计文档

## 1. 数据库概述

### 1.1 选型说明

| 项目 | 选型 | 理由 |
|------|------|------|
| 数据库 | SQLite | 嵌入式、零配置、适合桌面应用 |
| Rust库 | sqlx | 编译时SQL检查、异步支持 |
| 加密 | SQLCipher | 数据库文件加密（可选） |

### 1.2 文件结构

```
data/
├── ai-lot.db              # 主数据库
├── ai-lot.db-shm         # 共享内存（WAL模式）
├── ai-lot.db-wal         # 预写日志
└── backups/              # 自动备份目录
    ├── ai-lot-20241224.db
    └── ai-lot-20241223.db
```

### 1.3 设计原则

1. **最小化持久化**：只在必要时持久化，运行时状态保存在内存
2. **加密敏感数据**：API密钥等必须加密存储
3. **索引优化**：为高频查询添加索引
4. **级联删除**：合理设置外键约束

---

## 2. 表结构设计

### 2.1 核心实体关系

```
┌──────────┐     ┌──────────┐     ┌──────────┐
│  users   │────▶│  roles   │     │exchanges │
└─────┬────┘     └──────────┘     └─────┬────┘
     │                                 │
     │                                 │
     ▼                                 ▼
┌─────────────┐                 ┌─────────────┐
│  strategies │                 │   orders    │
└─────┬───────┘                 └─────┬───────┘
      │                               │
      ▼                               ▼
┌───────────────────┐         ┌─────────────┐
│strategy_instances│         │   trades    │
└─────┬─────────────┘         └─────────────┘
      │
      ▼
┌─────────────┐
│  backtests  │
└─────────────┘
```

### 2.2 用户与权限

#### users（用户表）

```sql
CREATE TABLE users (
    -- 主键
    id              TEXT PRIMARY KEY,

    -- 基本信息
    username        TEXT UNIQUE NOT NULL,
    password_hash   TEXT NOT NULL,
    display_name    TEXT,

    -- 角色与状态
    role_id         TEXT NOT NULL,
    status          TEXT NOT NULL DEFAULT 'active',

    -- 安全相关
    salt            TEXT NOT NULL,
    failed_attempts INTEGER DEFAULT 0,
    locked_until    INTEGER,

    -- 时间戳
    created_at      INTEGER NOT NULL,
    updated_at      INTEGER NOT NULL,

    FOREIGN KEY (role_id) REFERENCES roles(id)
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_role_id ON users(role_id);
```

#### roles（角色表）

```sql
CREATE TABLE roles (
    id          TEXT PRIMARY KEY,
    name        TEXT UNIQUE NOT NULL,
    description TEXT,

    -- 权限列表（JSON数组）
    permissions TEXT NOT NULL,

    created_at  INTEGER NOT NULL
);

-- 预置角色
INSERT INTO roles (id, name, description, permissions, created_at) VALUES
('role_admin', '管理员', '全部权限', '["*"]', 0),
('role_developer', '策略开发者', '策略编写、回测', '["strategy:read","strategy:write","backtest:execute","market:read"]', 0),
('role_trader', '交易员', '执行实盘交易', '["trade:execute","market:read","position:read","order:read","order:write"]', 0),
('role_auditor', '审计员', '只读查看日志', '["audit:read","trade:read","position:read"]', 0);
```

#### audit_logs（审计日志表）

```sql
CREATE TABLE audit_logs (
    id              TEXT PRIMARY KEY,
    user_id         TEXT NOT NULL,
    username        TEXT NOT NULL,

    -- 操作信息
    operation_type  TEXT NOT NULL,
    resource_type   TEXT NOT NULL,
    resource_id     TEXT,

    -- 变更数据（JSON）
    before_data     TEXT,
    after_data      TEXT,

    -- 操作结果
    result          TEXT NOT NULL,
    error_message   TEXT,

    -- 请求信息
    ip_address      TEXT,
    user_agent      TEXT,

    -- 时间戳
    timestamp       INTEGER NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_timestamp ON audit_logs(timestamp DESC);
```

### 2.3 交易所配置

#### exchanges（交易所配置表）

```sql
CREATE TABLE exchanges (
    id                  TEXT PRIMARY KEY,
    user_id             TEXT NOT NULL,

    -- 交易所信息
    exchange_name       TEXT NOT NULL,
    display_name        TEXT NOT NULL,

    -- API凭证（加密存储）
    api_key_encrypted   TEXT NOT NULL,
    api_secret_encrypted TEXT NOT NULL,
    passphrase_encrypted TEXT,

    -- 环境配置
    is_testnet          INTEGER DEFAULT 0,

    -- 状态（用于UI显示）
    status              TEXT DEFAULT 'inactive',

    -- 时间戳
    created_at          INTEGER NOT NULL,
    updated_at          INTEGER NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_exchanges_user_id ON exchanges(user_id);
```

**注**：`status` 字段仅用于UI展示，实际连接状态保存在内存中。

### 2.4 策略相关

#### strategies（策略模板表）

```sql
CREATE TABLE strategies (
    id              TEXT PRIMARY KEY,
    user_id         TEXT NOT NULL,

    -- 基本信息
    name            TEXT NOT NULL,
    description     TEXT,

    -- 策略代码
    code            TEXT NOT NULL,
    language        TEXT NOT NULL,

    -- 参数配置（JSON）
    parameters      TEXT,

    -- 分类与标签
    category        TEXT,
    tags            TEXT,

    -- 版本管理
    version         INTEGER DEFAULT 1,
    parent_id       TEXT,

    -- 状态
    status          TEXT DEFAULT 'draft',

    -- 时间戳
    created_at      INTEGER NOT NULL,
    updated_at      INTEGER NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES strategies(id)
);

CREATE INDEX idx_strategies_user_id ON strategies(user_id);
CREATE INDEX idx_strategies_status ON strategies(status);
```

#### strategy_instances（策略实例表）

```sql
CREATE TABLE strategy_instances (
    id              TEXT PRIMARY KEY,
    strategy_id     TEXT NOT NULL,
    user_id         TEXT NOT NULL,

    -- 实例配置
    name            TEXT NOT NULL,

    -- 运行参数（JSON）
    parameters      TEXT NOT NULL,

    -- 交易配置
    exchange_id     TEXT NOT NULL,
    symbol          TEXT NOT NULL,
    timeframe       TEXT NOT NULL,

    -- 运行模式
    mode            TEXT NOT NULL,

    -- 状态
    status          TEXT DEFAULT 'stopped',
    error_message   TEXT,

    -- 统计数据
    start_time      INTEGER,
    stop_time       INTEGER,
    total_trades    INTEGER DEFAULT 0,
    total_pnl       REAL DEFAULT 0,
    max_drawdown    REAL DEFAULT 0,

    created_at      INTEGER NOT NULL,
    updated_at      INTEGER NOT NULL,

    FOREIGN KEY (strategy_id) REFERENCES strategies(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (exchange_id) REFERENCES exchanges(id)
);

CREATE INDEX idx_strategy_instances_strategy_id ON strategy_instances(strategy_id);
CREATE INDEX idx_strategy_instances_user_id ON strategy_instances(user_id);
CREATE INDEX idx_strategy_instances_status ON strategy_instances(status);
```

### 2.5 回测相关

#### backtests（回测记录表）

```sql
CREATE TABLE backtests (
    id              TEXT PRIMARY KEY,
    strategy_id     TEXT NOT NULL,
    user_id         TEXT NOT NULL,

    -- 回测配置（JSON）
    parameters      TEXT NOT NULL,
    symbol          TEXT NOT NULL,
    timeframe       TEXT NOT NULL,
    start_time      INTEGER NOT NULL,
    end_time        INTEGER NOT NULL,

    -- 回测设置
    initial_balance REAL NOT NULL,
    commission_rate REAL DEFAULT 0.001,
    slippage        REAL DEFAULT 0,

    -- 性能指标
    total_return    REAL,
    sharpe_ratio    REAL,
    max_drawdown    REAL,
    win_rate        REAL,

    -- 交易统计
    total_trades    INTEGER,
    winning_trades  INTEGER,
    losing_trades   INTEGER,

    -- 状态
    status          TEXT DEFAULT 'pending',
    error_message   TEXT,

    -- 详细数据（JSON）
    trades_data     TEXT,
    equity_curve    TEXT,

    created_at      INTEGER NOT NULL,
    completed_at    INTEGER,

    FOREIGN KEY (strategy_id) REFERENCES strategies(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_backtests_strategy_id ON backtests(strategy_id);
CREATE INDEX idx_backtests_user_id ON backtests(user_id);
CREATE INDEX idx_backtests_created_at ON backtests(created_at DESC);
```

### 2.6 交易相关

#### orders（订单表）

```sql
CREATE TABLE orders (
    id                  TEXT PRIMARY KEY,
    user_id             TEXT NOT NULL,
    exchange_id         TEXT NOT NULL,
    strategy_instance_id TEXT,

    -- 订单ID
    exchange_order_id   TEXT UNIQUE,
    client_order_id     TEXT UNIQUE,

    -- 订单信息
    symbol              TEXT NOT NULL,
    side                TEXT NOT NULL,
    order_type          TEXT NOT NULL,

    -- 价格和数量
    price               REAL,
    quantity            REAL NOT NULL,
    filled_quantity     REAL DEFAULT 0,
    avg_price           REAL,

    -- 状态
    status              TEXT NOT NULL,

    -- 时间
    created_at          INTEGER NOT NULL,
    updated_at          INTEGER NOT NULL,
    filled_at           INTEGER,

    -- 手续费
    commission          REAL DEFAULT 0,
    commission_asset    TEXT,

    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (exchange_id) REFERENCES exchanges(id),
    FOREIGN KEY (strategy_instance_id) REFERENCES strategy_instances(id)
);

CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_exchange_id ON orders(exchange_id);
CREATE INDEX idx_orders_strategy_instance_id ON orders(strategy_instance_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_created_at ON orders(created_at DESC);
```

#### positions（持仓表）

```sql
CREATE TABLE positions (
    id              TEXT PRIMARY KEY,
    user_id         TEXT NOT NULL,
    exchange_id     TEXT NOT NULL,
    strategy_instance_id TEXT,

    -- 持仓信息
    symbol          TEXT NOT NULL,
    side            TEXT NOT NULL,

    -- 数量和价格
    quantity        REAL NOT NULL,
    entry_price     REAL NOT NULL,
    current_price   REAL,

    -- 盈亏
    unrealized_pnl  REAL DEFAULT 0,
    realized_pnl    REAL DEFAULT 0,

    -- 时间
    opened_at       INTEGER NOT NULL,
    updated_at      INTEGER NOT NULL,
    closed_at       INTEGER,

    -- 状态
    status          TEXT DEFAULT 'open',

    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (exchange_id) REFERENCES exchanges(id),
    FOREIGN KEY (strategy_instance_id) REFERENCES strategy_instances(id)
);

CREATE INDEX idx_positions_user_id ON positions(user_id);
CREATE INDEX idx_positions_exchange_id ON positions(exchange_id);
CREATE INDEX idx_positions_strategy_instance_id ON positions(strategy_instance_id);
CREATE INDEX idx_positions_status ON positions(status);
```

#### trades（成交记录表）

```sql
CREATE TABLE trades (
    id              TEXT PRIMARY KEY,
    user_id         TEXT NOT NULL,
    exchange_id     TEXT NOT NULL,
    order_id        TEXT NOT NULL,

    -- 成交信息
    exchange_trade_id TEXT UNIQUE,
    symbol          TEXT NOT NULL,
    side            TEXT NOT NULL,

    -- 价格和数量
    price           REAL NOT NULL,
    quantity        REAL NOT NULL,

    -- 手续费
    commission      REAL DEFAULT 0,
    commission_asset TEXT,

    -- 盈亏
    pnl             REAL,

    -- 时间
    timestamp       INTEGER NOT NULL,
    created_at      INTEGER NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (exchange_id) REFERENCES exchanges(id),
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE
);

CREATE INDEX idx_trades_user_id ON trades(user_id);
CREATE INDEX idx_trades_exchange_id ON trades(exchange_id);
CREATE INDEX idx_trades_order_id ON trades(order_id);
CREATE INDEX idx_trades_timestamp ON trades(timestamp DESC);
```

### 2.7 行情数据

#### klines（K线历史数据表）

```sql
CREATE TABLE klines (
    exchange_name   TEXT NOT NULL,
    symbol          TEXT NOT NULL,
    timeframe       TEXT NOT NULL,
    timestamp       INTEGER NOT NULL,

    -- OHLCV数据
    open            REAL NOT NULL,
    high            REAL NOT NULL,
    low             REAL NOT NULL,
    close           REAL NOT NULL,
    volume          REAL NOT NULL,

    PRIMARY KEY (exchange_name, symbol, timeframe, timestamp)
);

CREATE INDEX idx_klines_symbol_timeframe ON klines(symbol, timeframe);
CREATE INDEX idx_klines_timestamp ON klines(timestamp DESC);
```

### 2.8 风控相关

#### risk_rules（风控规则表）

```sql
CREATE TABLE risk_rules (
    id              TEXT PRIMARY KEY,
    user_id         TEXT NOT NULL,

    -- 规则信息
    name            TEXT NOT NULL,
    description     TEXT,

    -- 规则类型
    rule_type       TEXT NOT NULL,

    -- 规则配置（JSON）
    config          TEXT NOT NULL,

    -- 阈值和动作
    threshold_value REAL NOT NULL,
    action          TEXT NOT NULL,

    -- 应用范围
    apply_to        TEXT,
    target_id       TEXT,

    -- 状态
    is_enabled      INTEGER DEFAULT 1,

    -- 统计
    trigger_count   INTEGER DEFAULT 0,
    last_triggered_at INTEGER,

    created_at      INTEGER NOT NULL,
    updated_at      INTEGER NOT NULL,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_risk_rules_user_id ON risk_rules(user_id);
CREATE INDEX idx_risk_rules_is_enabled ON risk_rules(is_enabled);
```

#### risk_alerts（风控预警历史表）

```sql
CREATE TABLE risk_alerts (
    id              TEXT PRIMARY KEY,
    rule_id         TEXT NOT NULL,
    user_id         TEXT NOT NULL,

    -- 预警信息
    severity        TEXT NOT NULL,
    title           TEXT NOT NULL,
    message         TEXT NOT NULL,

    -- 相关数据
    strategy_instance_id TEXT,
    symbol          TEXT,

    -- 当前值
    current_value   REAL NOT NULL,
    threshold_value REAL NOT NULL,

    -- 处理状态
    status          TEXT DEFAULT 'active',
    handled_by      TEXT,
    handled_at      INTEGER,

    created_at      INTEGER NOT NULL,

    FOREIGN KEY (rule_id) REFERENCES risk_rules(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX idx_risk_alerts_user_id ON risk_alerts(user_id);
CREATE INDEX idx_risk_alerts_status ON risk_alerts(status);
```

---

## 3. 数据访问层

### 3.1 Repository Trait 定义

```rust
#[async_trait]
pub trait Repository<T, ID> {
    async fn find_by_id(&self, id: ID) -> Result<Option<T>>;
    async fn find_all(&self) -> Result<Vec<T>>;
    async fn insert(&self, entity: &T) -> Result<()>;
    async fn update(&self, entity: &T) -> Result<()>;
    async fn delete(&self, id: ID) -> Result<()>;
}

// 用户仓库
#[async_trait]
pub trait UserRepository: Repository<User, String> {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
}

// 策略仓库
#[async_trait]
pub trait StrategyRepository: Repository<Strategy, String> {
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<Strategy>>;
}
```

### 3.2 实现示例

```rust
pub struct SqliteUserRepository {
    pool: SqlitePool,
}

#[async_trait]
impl UserRepository for SqliteUserRepository {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    async fn insert(&self, entity: &User) -> Result<()> {
        sqlx::query(
            "INSERT INTO users (id, username, password_hash, display_name, role_id, status, salt, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&entity.id)
        .bind(&entity.username)
        .bind(&entity.password_hash)
        .bind(&entity.display_name)
        .bind(&entity.role_id)
        .bind(&entity.status)
        .bind(&entity.salt)
        .bind(&entity.created_at)
        .bind(&entity.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
```

---

## 4. 备份与恢复

### 4.1 备份策略

```rust
pub struct BackupManager {
    db_path: String,
    backup_dir: String,
}

impl BackupManager {
    pub async fn create_backup(&self) -> Result<String> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("ai-lot-{}.db", timestamp);
        let backup_path = Path::new(&self.backup_dir).join(&backup_name);

        fs::create_dir_all(&self.backup_dir)?;
        fs::copy(&self.db_path, &backup_path)?;

        Ok(backup_path.to_string_lossy().to_string())
    }

    // 保留最近N个备份
    pub async fn cleanup_old_backups(&self, keep_count: usize) -> Result<()> {
        let mut backups = fs::read_dir(&self.backup_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map(|ext| ext == "db").unwrap_or(false))
            .collect::<Vec<_>>();

        backups.sort_by_key(|e| e.metadata().ok()?.modified().ok());

        for backup in backups.iter().rev().skip(keep_count) {
            fs::remove_file(backup.path())?;
        }

        Ok(())
    }
}
```

---

## 5. 数据库迁移

### 5.1 迁移文件结构

```
src-tauri/migrations/
├── 001_initial_schema.sql
├── 002_add_encryption.sql
├── 003_add_risk_management.sql
└── ...
```

### 5.2 迁移管理

```rust
pub struct MigrationManager {
    pool: SqlitePool,
}

impl MigrationManager {
    pub async fn run_migrations(&self) -> Result<()> {
        sqlx::query("PRAGMA foreign_keys = ON")
            .execute(&self.pool)
            .await?;

        sqlx::migrate!("./migrations").run(&self.pool).await?;

        Ok(())
    }
}
```
