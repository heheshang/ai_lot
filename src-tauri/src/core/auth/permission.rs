/// 权限定义
/// 定义系统中的所有权限常量

// ========== 用户管理 ==========
pub const PERM_USER_READ: &str = "user:read";
pub const PERM_USER_WRITE: &str = "user:write";

// ========== 策略管理 ==========
pub const PERM_STRATEGY_READ: &str = "strategy:read";
pub const PERM_STRATEGY_WRITE: &str = "strategy:write";
pub const PERM_STRATEGY_DELETE: &str = "strategy:delete";

// ========== 回测执行 ==========
pub const PERM_BACKTEST_EXECUTE: &str = "backtest:execute";

// ========== 交易执行 ==========
pub const PERM_TRADE_EXECUTE: &str = "trade:execute";

// ========== 市场数据 ==========
pub const PERM_MARKET_READ: &str = "market:read";

// ========== 持仓管理 ==========
pub const PERM_POSITION_READ: &str = "position:read";

// ========== 订单管理 ==========
pub const PERM_ORDER_READ: &str = "order:read";
pub const PERM_ORDER_WRITE: &str = "order:write";

// ========== 风险管理 ==========
pub const PERM_RISK_READ: &str = "risk:read";
pub const PERM_RISK_WRITE: &str = "risk:write";

// ========== 审计日志 ==========
pub const PERM_AUDIT_READ: &str = "audit:read";

// ========== 超级权限 ==========
pub const PERM_ALL: &str = "*";
