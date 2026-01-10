-- Migration 005: Additional Performance Indexes (2026-01-10)
-- Adds missing indexes for query optimization

-- ============== Orders Indexes ==============

-- Composite index for orders by user, symbol, and status
-- Used when filtering orders by symbol within a user's account
CREATE INDEX IF NOT EXISTS idx_orders_user_symbol_status
ON orders(user_id, symbol, status, created_at DESC);

-- Index for orders by exchange and symbol (exchange-specific queries)
CREATE INDEX IF NOT EXISTS idx_orders_exchange_symbol
ON orders(exchange_id, symbol, status, created_at DESC);

-- ============== Positions Indexes ==============

-- Composite index for positions by user and symbol
CREATE INDEX IF NOT EXISTS idx_positions_user_symbol
ON positions(user_id, symbol, status, updated_at DESC);

-- Index for positions by exchange
CREATE INDEX IF NOT EXISTS idx_positions_exchange
ON positions(exchange_id, status, updated_at DESC);

-- ============== Strategies Indexes ==============

-- Composite index for strategies by user and category
CREATE INDEX IF NOT EXISTS idx_strategies_user_category
ON strategies(user_id, category, created_at DESC);

-- Index for strategies by user and status
CREATE INDEX IF NOT EXISTS idx_strategies_user_status
ON strategies(user_id, status, created_at DESC);

-- ============== Strategy Instances Indexes ==============

-- Composite index for strategy instances by user and status
CREATE INDEX IF NOT EXISTS idx_strategy_instances_user_status
ON strategy_instances(user_id, status, created_at DESC);

-- Index for strategy instances by strategy_id
CREATE INDEX IF NOT EXISTS idx_strategy_instances_strategy
ON strategy_instances(strategy_id, status, created_at DESC);

-- Index for running instances (frequently queried)
CREATE INDEX IF NOT EXISTS idx_strategy_instances_running
ON strategy_instances(status, start_time DESC)
WHERE status = 'running';

-- ============== Risk Alerts Indexes ==============

-- Composite index for alerts by user, severity, and status
CREATE INDEX IF NOT EXISTS idx_risk_alerts_user_severity
ON risk_alerts(user_id, severity, status, created_at DESC);

-- Index for alerts by strategy instance
CREATE INDEX IF NOT EXISTS idx_risk_alerts_instance
ON risk_alerts(strategy_instance_id, status, created_at DESC);

-- ============== Exchanges Indexes ==============

-- Index for exchanges by user and status
CREATE INDEX IF NOT EXISTS idx_exchanges_user_status
ON exchanges(user_id, status, created_at DESC);

-- ============== Audit Logs Indexes ==============

-- Composite index for audit logs by user and event type
CREATE INDEX IF NOT EXISTS idx_audit_logs_user_event
ON audit_logs(user_id, event_type, timestamp DESC);

-- Index for audit logs by timestamp (time-range queries)
CREATE INDEX IF NOT EXISTS idx_audit_logs_timestamp
ON audit_logs(timestamp DESC);

-- ============== Backtests Indexes ==============

-- Index for backtests by user and status
CREATE INDEX IF NOT EXISTS idx_backtests_user_status
ON backtests(user_id, status, created_at DESC);

-- Index for backtests by strategy_id
CREATE INDEX IF NOT EXISTS idx_backtests_strategy
ON backtests(strategy_id, created_at DESC);
