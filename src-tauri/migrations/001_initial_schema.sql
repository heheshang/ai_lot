-- ============== 用户与权限 ==============
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    display_name TEXT,
    role_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    salt TEXT NOT NULL,
    failed_attempts INTEGER DEFAULT 0,
    locked_until INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE TABLE roles (
    id TEXT PRIMARY KEY,
    name TEXT UNIQUE NOT NULL,
    description TEXT,
    permissions TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_role_id ON users(role_id);

-- 预置角色
INSERT INTO roles (id, name, description, permissions, created_at) VALUES
('role_admin', '管理员', '全部权限', '["*"]', 0),
('role_developer', '策略开发者', '策略编写、回测', '["strategy:read","strategy:write","backtest:execute","market:read"]', 0),
('role_trader', '交易员', '执行实盘交易', '["trade:execute","market:read","position:read","order:read","order:write"]', 0),
('role_auditor', '审计员', '只读查看日志', '["audit:read","trade:read","position:read"]', 0);

-- 预置管理员账户 (密码: admin123)
INSERT INTO users (id, username, password_hash, display_name, role_id, status, salt, created_at, updated_at) VALUES
('u_admin', 'admin', '$argon2id$v=19$m=19456,t=2,p=1$FiaZKETKHXAqjkGOIsyacg$D9tEkJ/JnJOxUXurmdUHdhPiQfUQJnpp3BwQtbNeu8g', '系统管理员', 'role_admin', 'active', '', 0, 0);

-- ============== 交易所配置 ==============
CREATE TABLE exchanges (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    exchange_name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    api_key_encrypted TEXT NOT NULL,
    api_secret_encrypted TEXT NOT NULL,
    passphrase_encrypted TEXT,
    is_testnet INTEGER DEFAULT 0,
    status TEXT DEFAULT 'inactive',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_exchanges_user_id ON exchanges(user_id);

-- ============== 策略相关 ==============
CREATE TABLE strategies (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    code TEXT NOT NULL,
    language TEXT NOT NULL,
    parameters TEXT,
    category TEXT,
    tags TEXT,
    version INTEGER DEFAULT 1,
    parent_id TEXT,
    status TEXT DEFAULT 'draft',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (parent_id) REFERENCES strategies(id)
);

CREATE TABLE strategy_instances (
    id TEXT PRIMARY KEY,
    strategy_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    parameters TEXT NOT NULL,
    exchange_id TEXT NOT NULL,
    symbol TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    mode TEXT NOT NULL,
    status TEXT DEFAULT 'stopped',
    error_message TEXT,
    start_time INTEGER,
    stop_time INTEGER,
    total_trades INTEGER DEFAULT 0,
    total_pnl REAL DEFAULT 0,
    max_drawdown REAL DEFAULT 0,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (strategy_id) REFERENCES strategies(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (exchange_id) REFERENCES exchanges(id)
);

CREATE INDEX idx_strategies_user_id ON strategies(user_id);
CREATE INDEX idx_strategies_status ON strategies(status);
CREATE INDEX idx_strategy_instances_strategy_id ON strategy_instances(strategy_id);
CREATE INDEX idx_strategy_instances_user_id ON strategy_instances(user_id);
CREATE INDEX idx_strategy_instances_status ON strategy_instances(status);

-- ============== 交易相关 ==============
CREATE TABLE orders (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    exchange_id TEXT NOT NULL,
    strategy_instance_id TEXT,
    exchange_order_id TEXT UNIQUE,
    client_order_id TEXT UNIQUE,
    symbol TEXT NOT NULL,
    side TEXT NOT NULL,
    order_type TEXT NOT NULL,
    price REAL,
    quantity REAL NOT NULL,
    filled_quantity REAL DEFAULT 0,
    avg_price REAL,
    status TEXT NOT NULL,
    commission REAL DEFAULT 0,
    commission_asset TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    filled_at INTEGER,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (exchange_id) REFERENCES exchanges(id),
    FOREIGN KEY (strategy_instance_id) REFERENCES strategy_instances(id)
);

CREATE TABLE positions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    exchange_id TEXT NOT NULL,
    strategy_instance_id TEXT,
    symbol TEXT NOT NULL,
    side TEXT NOT NULL,
    quantity REAL NOT NULL,
    entry_price REAL NOT NULL,
    current_price REAL,
    unrealized_pnl REAL DEFAULT 0,
    realized_pnl REAL DEFAULT 0,
    opened_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    closed_at INTEGER,
    status TEXT DEFAULT 'open',
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (exchange_id) REFERENCES exchanges(id),
    FOREIGN KEY (strategy_instance_id) REFERENCES strategy_instances(id)
);

CREATE TABLE trades (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    exchange_id TEXT NOT NULL,
    order_id TEXT NOT NULL,
    exchange_trade_id TEXT UNIQUE,
    symbol TEXT NOT NULL,
    side TEXT NOT NULL,
    price REAL NOT NULL,
    quantity REAL NOT NULL,
    commission REAL DEFAULT 0,
    commission_asset TEXT,
    pnl REAL,
    timestamp INTEGER NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (exchange_id) REFERENCES exchanges(id),
    FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE
);

CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_exchange_id ON orders(exchange_id);
CREATE INDEX idx_orders_strategy_instance_id ON orders(strategy_instance_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_created_at ON orders(created_at DESC);
CREATE INDEX idx_positions_user_id ON positions(user_id);
CREATE INDEX idx_positions_exchange_id ON positions(exchange_id);
CREATE INDEX idx_positions_strategy_instance_id ON positions(strategy_instance_id);
CREATE INDEX idx_positions_status ON positions(status);
CREATE INDEX idx_trades_user_id ON trades(user_id);
CREATE INDEX idx_trades_exchange_id ON trades(exchange_id);
CREATE INDEX idx_trades_order_id ON trades(order_id);
CREATE INDEX idx_trades_timestamp ON trades(timestamp DESC);

-- ============== 行情数据 ==============
CREATE TABLE klines (
    exchange_name TEXT NOT NULL,
    symbol TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    timestamp INTEGER NOT NULL,
    open REAL NOT NULL,
    high REAL NOT NULL,
    low REAL NOT NULL,
    close REAL NOT NULL,
    volume REAL NOT NULL,
    PRIMARY KEY (exchange_name, symbol, timeframe, timestamp)
);

CREATE INDEX idx_klines_symbol_timeframe ON klines(symbol, timeframe);
CREATE INDEX idx_klines_timestamp ON klines(timestamp DESC);

-- ============== 回测相关 ==============
CREATE TABLE backtests (
    id TEXT PRIMARY KEY,
    strategy_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    parameters TEXT NOT NULL,
    symbol TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    start_time INTEGER NOT NULL,
    end_time INTEGER NOT NULL,
    initial_balance REAL NOT NULL,
    commission_rate REAL DEFAULT 0.001,
    slippage REAL DEFAULT 0,
    total_return REAL,
    sharpe_ratio REAL,
    max_drawdown REAL,
    win_rate REAL,
    total_trades INTEGER,
    winning_trades INTEGER,
    losing_trades INTEGER,
    status TEXT DEFAULT 'pending',
    error_message TEXT,
    trades_data TEXT,
    equity_curve TEXT,
    created_at INTEGER NOT NULL,
    completed_at INTEGER,
    FOREIGN KEY (strategy_id) REFERENCES strategies(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX idx_backtests_strategy_id ON backtests(strategy_id);
CREATE INDEX idx_backtests_user_id ON backtests(user_id);
CREATE INDEX idx_backtests_created_at ON backtests(created_at DESC);

-- ============== 风控相关 ==============
CREATE TABLE risk_rules (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    rule_type TEXT NOT NULL,
    config TEXT NOT NULL,
    threshold_value REAL NOT NULL,
    action TEXT NOT NULL,
    apply_to TEXT,
    target_id TEXT,
    is_enabled INTEGER DEFAULT 1,
    trigger_count INTEGER DEFAULT 0,
    last_triggered_at INTEGER,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE risk_alerts (
    id TEXT PRIMARY KEY,
    rule_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    severity TEXT NOT NULL,
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    strategy_instance_id TEXT,
    symbol TEXT,
    current_value REAL NOT NULL,
    threshold_value REAL NOT NULL,
    status TEXT DEFAULT 'active',
    handled_by TEXT,
    handled_at INTEGER,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (rule_id) REFERENCES risk_rules(id),
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX idx_risk_rules_user_id ON risk_rules(user_id);
CREATE INDEX idx_risk_rules_is_enabled ON risk_rules(is_enabled);
CREATE INDEX idx_risk_alerts_user_id ON risk_alerts(user_id);
CREATE INDEX idx_risk_alerts_status ON risk_alerts(status);

-- ============== 审计日志 ==============
CREATE TABLE audit_logs (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    username TEXT NOT NULL,
    operation_type TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id TEXT,
    before_data TEXT,
    after_data TEXT,
    result TEXT NOT NULL,
    error_message TEXT,
    ip_address TEXT,
    user_agent TEXT,
    timestamp INTEGER NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_timestamp ON audit_logs(timestamp DESC);
