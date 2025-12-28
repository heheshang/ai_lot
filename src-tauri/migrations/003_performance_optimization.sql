-- Migration 003: Performance Optimization (P6-05)
-- Adds indexes for common queries and materialized views

-- ============== Performance Indexes ==============

-- Index for klines lookup by symbol and timeframe with time ordering
CREATE INDEX IF NOT EXISTS idx_klines_symbol_time
ON klines(symbol, timeframe, timestamp DESC);

-- Index for orders by user, status, and created_at (common dashboard query)
CREATE INDEX IF NOT EXISTS idx_orders_user_status
ON orders(user_id, status, created_at DESC);

-- Index for positions by strategy instance and status
CREATE INDEX IF NOT EXISTS idx_positions_instance
ON positions(strategy_instance_id, status);

-- Index for risk alerts by user, status, and created_at
CREATE INDEX IF NOT EXISTS idx_alerts_user_status
ON risk_alerts(user_id, status, created_at DESC);

-- Index for trades by user and timestamp
CREATE INDEX IF NOT EXISTS idx_trades_user_timestamp
ON trades(user_id, timestamp DESC);

-- ============== Performance Views ==============

-- View for active positions with strategy info
CREATE VIEW IF NOT EXISTS v_active_positions AS
SELECT
    p.id,
    p.user_id,
    p.exchange_id,
    p.strategy_instance_id,
    p.symbol,
    p.side,
    p.quantity,
    p.entry_price,
    p.current_price,
    p.unrealized_pnl,
    p.realized_pnl,
    p.opened_at,
    p.updated_at,
    p.status,
    si.name as strategy_name,
    si.parameters as strategy_parameters,
    e.exchange_name,
    e.display_name as exchange_display_name,
    e.is_testnet
FROM positions p
LEFT JOIN strategy_instances si ON p.strategy_instance_id = si.id
LEFT JOIN exchanges e ON p.exchange_id = e.id
WHERE p.status = 'open' AND p.quantity > 0;

-- View for active orders with strategy and exchange info
CREATE VIEW IF NOT EXISTS v_active_orders AS
SELECT
    o.id,
    o.user_id,
    o.exchange_id,
    o.strategy_instance_id,
    o.symbol,
    o.side,
    o.order_type,
    o.price,
    o.quantity,
    o.filled_quantity,
    o.avg_price,
    o.status,
    o.commission,
    o.created_at,
    o.updated_at,
    o.filled_at,
    o.exchange_order_id,
    o.client_order_id,
    si.name as strategy_name,
    si.parameters as strategy_parameters,
    e.exchange_name,
    e.display_name as exchange_display_name,
    e.is_testnet
FROM orders o
LEFT JOIN strategy_instances si ON o.strategy_instance_id = si.id
LEFT JOIN exchanges e ON o.exchange_id = e.id
WHERE o.status IN ('open', 'partially_filled');

-- View for recent trades with performance metrics
CREATE VIEW IF NOT EXISTS v_recent_trades AS
SELECT
    t.id,
    t.user_id,
    t.exchange_id,
    t.order_id,
    t.symbol,
    t.side,
    t.price,
    t.quantity,
    t.commission,
    t.commission_asset,
    t.pnl,
    t.timestamp,
    t.created_at,
    o.exchange_order_id,
    si.name as strategy_name
FROM trades t
LEFT JOIN orders o ON t.order_id = o.id
LEFT JOIN strategy_instances si ON o.strategy_instance_id = si.id
ORDER BY t.timestamp DESC;

-- View for risk monitoring dashboard
CREATE VIEW IF NOT EXISTS v_risk_dashboard AS
SELECT
    ra.id,
    ra.rule_id,
    ra.user_id,
    ra.severity,
    ra.title,
    ra.message,
    ra.strategy_instance_id,
    ra.symbol,
    ra.current_value,
    ra.threshold_value,
    ra.status,
    ra.created_at,
    ra.handled_by,
    ra.handled_at,
    rr.name as rule_name,
    rr.rule_type,
    rr.action as rule_action,
    si.name as strategy_name
FROM risk_alerts ra
LEFT JOIN risk_rules rr ON ra.rule_id = rr.id
LEFT JOIN strategy_instances si ON ra.strategy_instance_id = si.id
WHERE ra.status = 'active'
ORDER BY ra.created_at DESC;

-- View for strategy instance performance summary
CREATE VIEW IF NOT EXISTS v_strategy_performance AS
SELECT
    si.id,
    si.strategy_id,
    si.user_id,
    si.name,
    si.symbol,
    si.timeframe,
    si.mode,
    si.status,
    si.total_trades,
    si.total_pnl,
    si.max_drawdown,
    si.start_time,
    si.stop_time,
    si.created_at,
    si.updated_at,
    s.name as strategy_base_name,
    s.category as strategy_category,
    COUNT(DISTINCT o.id) as order_count,
    COUNT(DISTINCT p.id) as position_count,
    SUM(CASE WHEN p.status = 'open' THEN 1 ELSE 0 END) as open_positions
FROM strategy_instances si
LEFT JOIN strategies s ON si.strategy_id = s.id
LEFT JOIN orders o ON si.id = o.strategy_instance_id
LEFT JOIN positions p ON si.id = p.strategy_instance_id
GROUP BY si.id
ORDER BY si.created_at DESC;
