/**
 * 时间常量 (毫秒)
 */
export const TIME = {
  SECOND: 1000,
  MINUTE: 60 * 1000,
  HOUR: 60 * 60 * 1000,
  DAY: 24 * 60 * 60 * 1000,
  WEEK: 7 * 24 * 60 * 60 * 1000,
} as const

/**
 * API 超时配置 (毫秒)
 */
export const API_TIMEOUTS = {
  DEFAULT: 30000,
  SHORT: 5000,
  RETRY_DELAY: 1000,
  POLLING_INTERVAL: 5000,
} as const

/**
 * 轮询间隔配置 (毫秒)
 */
export const POLLING = {
  INSTANCE_REFRESH: 5000,
  MARKET_UPDATE: 5000,
  ACCOUNT_DATA: 5000,
  RISK_MONITOR: 5000,
} as const

/**
 * 数据限制常量
 */
export const LIMITS = {
  MAX_TRADES: 1000,
  MAX_KLINES: 1000,
  MAX_ORDERS: 1000,
  MAX_POSITIONS: 100,
  AUDIT_LOGS_LIMIT: 1000,
  DEFAULT_PAGE_SIZE: 20,
  STRATEGY_PAGE_SIZES: [12, 24, 48, 96] as const,
} as const

/**
 * 风险阈值常量 (百分比)
 */
export const RISK_THRESHOLDS = {
  DANGER_POSITION_RATIO: 80,
  WARNING_POSITION_RATIO: 60,
  DANGER_DAILY_LOSS: -500,
  WARNING_DAILY_LOSS: -200,
  EXCELLENT_SHARPE_RATIO: 5,
} as const

/**
 * 仓位风险等级阈值
 */
export const POSITION_RISK = {
  HIGH: 80,
  MEDIUM: 60,
  LOW: 40,
} as const

/**
 * 价格精度配置
 */
export const PRICE_PRECISION = {
  HIGH: 10000, // 4位小数
  MEDIUM: 1000, // 3位小数
  LOW: 100, // 2位小数
} as const

/**
 * 回测配置常量
 */
export const BACKTEST = {
  DEFAULT_INITIAL_CAPITAL: 10000,
  DEFAULT_FEE_RATE: 0.1, // 0.1%
  DEFAULT_SLIPPAGE: 0.05, // 0.05%
  DEFAULT_MAX_POSITIONS: 5,
  DEFAULT_MAX_POSITION_RATIO: 0.5, // 50%
  DEFAULT_STOP_LOSS_RATIO: 0.1, // 10%
  MAX_DATA_POINTS: 10000,
} as const

/**
 * 交易所配置
 */
export const EXCHANGE = {
  BALANCE_PRECISION: 8,
  QUANTITY_PRECISION: 4,
  PRICE_PRECISION: 4,
} as const

/**
 * 文件上传限制
 */
export const UPLOAD = {
  AVATAR_MAX_SIZE_MB: 2,
  AVATAR_MAX_SIZE_BYTES: 2 * 1024 * 1024,
  STRATEGY_MAX_SIZE_KB: 1024,
} as const

/**
 * 动画持续时间 (毫秒)
 */
export const ANIMATION = {
  FAST: 150,
  NORMAL: 300,
  SLOW: 500,
  EXTRA_SLOW: 800,
} as const

/**
 * 数字格式化配置
 */
export const FORMAT = {
  CURRENCY_DECIMALS: 2,
  PERCENTAGE_DECIMALS: 2,
  PRICE_DECIMALS: 4,
  QUANTITY_DECIMALS: 4,
  PNL_DECIMALS: 2,
} as const

/**
 * 侧边栏配置
 */
export const LAYOUT = {
  SIDEBAR_WIDTH: 240,
  SIDEBAR_COLLAPSED_WIDTH: 64,
  HEADER_HEIGHT: 64,
  MOBILE_DRAWER_WIDTH: 280,
} as const

/**
 * 缓存配置
 */
export const CACHE = {
  TTL_SHORT: 30 * 1000, // 30秒
  TTL_MEDIUM: 60 * 1000, // 1分钟
  TTL_LONG: 5 * 60 * 1000, // 5分钟
  MAX_CACHE_SIZE: 100,
} as const

/**
 * 重试配置
 */
export const RETRY = {
  MAX_ATTEMPTS: 3,
  DELAY_BASE: 1000,
  DELAY_MAX: 5000,
  BACKOFF_MULTIPLIER: 2,
} as const
