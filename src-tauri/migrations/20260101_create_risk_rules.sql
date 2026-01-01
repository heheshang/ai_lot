-- Risk Rules Table
-- Stores risk management rule configurations
-- This migration replaces the old risk_rules table with a new simplified structure

-- Drop risk_alerts table first (it has foreign key to risk_rules)
DROP TABLE IF EXISTS risk_alerts;

-- Drop old risk_rules table
DROP TABLE IF EXISTS risk_rules;

-- Create new risk_rules table with updated structure
CREATE TABLE risk_rules (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,                 -- Rule unique identifier (e.g., "position_limit")
    display_name TEXT NOT NULL,                -- Human-readable name
    description TEXT NOT NULL,                 -- Rule description
    rule_type TEXT NOT NULL,                   -- Rule type identifier
    enabled INTEGER NOT NULL DEFAULT 1,        -- 1 = enabled, 0 = disabled
    action TEXT NOT NULL,                      -- Action to take: "warning", "stop_strategy", "emergency_close"
    notify_methods TEXT NOT NULL,              -- JSON array: ["dingtalk", "email"]
    params_json TEXT NOT NULL,                 -- JSON object with rule parameters
    created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);

-- Create risk_alerts table (updated to remove user_id foreign key)
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
    created_at INTEGER NOT NULL
);

-- Create indexes for common queries
CREATE INDEX idx_risk_rules_name ON risk_rules(name);
CREATE INDEX idx_risk_rules_enabled ON risk_rules(enabled);
CREATE INDEX idx_risk_rules_type ON risk_rules(rule_type);
CREATE INDEX idx_risk_alerts_user_id ON risk_alerts(user_id);
CREATE INDEX idx_risk_alerts_status ON risk_alerts(status);

-- Insert default risk rules
INSERT OR IGNORE INTO risk_rules (name, display_name, description, rule_type, enabled, action, notify_methods, params_json) VALUES
    -- Position limit rule
    (
        'position_limit',
        '仓位限制规则',
        '限制单个仓位和总仓位的大小，控制单方向持仓比例',
        'position_limit',
        1,
        'warning',
        '["dingtalk"]',
        '{"max_position_value": 10000.0, "max_total_value": 50000.0, "max_direction_ratio": 0.7}'
    ),
    -- Drawdown limit rule
    (
        'drawdown_limit',
        '回撤限制规则',
        '监控权益回撤，当回撤超过阈值时触发保护动作',
        'drawdown_limit',
        1,
        'emergency_close',
        '["dingtalk", "email"]',
        '{"max_drawdown_pct": 15.0}'
    );

-- Create trigger to update updated_at timestamp
CREATE TRIGGER IF NOT EXISTS update_risk_rules_timestamp
    AFTER UPDATE ON risk_rules
    FOR EACH ROW
BEGIN
    UPDATE risk_rules SET updated_at = strftime('%s', 'now') WHERE id = NEW.id;
END;
