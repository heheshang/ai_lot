/**
 * 统一行情数据类型定义
 * Unified Market Data Types
 *
 * 用于屏蔽不同交易所的数据格式差异
 * Abstracts away differences between exchange data formats
 */

// ============== 交易所原始数据类型 ==============
// Exchange Raw Data Types

/**
 * Binance 原始 Ticker 数据格式
 */
export interface BinanceTicker {
  symbol: string;
  priceChange: string;
  priceChangePercent: string;
  weightedAvgPrice: string;
  prevClosePrice: string;
  lastPrice: string;
  lastQty: string;
  bidPrice: string;
  bidQty: string;
  askPrice: string;
  askQty: string;
  openPrice: string;
  highPrice: string;
  lowPrice: string;
  volume: string;
  quoteVolume: string;
  openTime: number;
  closeTime: number;
  firstId: number;
  lastId: number;
  count: number;
}

/**
 * Binance 原始 K线数据格式
 */
export interface BinanceKline {
  openTime: number;
  open: string;
  high: string;
  low: string;
  close: string;
  volume: string;
  closeTime: number;
  quoteVolume: string;
  trades: number;
  takerBuyBaseVolume: string;
  takerBuyQuoteVolume: string;
  ignore: string;
}

/**
 * OKX 原始 Ticker 数据格式
 */
export interface OKXTicker {
  instId: string;
  last: string;
  lastSz: string;
  askPx: string;
  bidPx: string;
  open24h: string;
  high24h: string;
  low24h: string;
  volCcy24h: string;
  vol24h: string;
  ts: string;
}

/**
 * OKX 原始 K线数据格式
 */
export interface OKXKline {
  timestamp: string;
  open: string;
  high: string;
  low: string;
  close: string;
  volume: string;
  volCcy: string;
  volCcyQuote: string;
  confirm: string;
}

/**
 * Bybit 原始 Ticker 数据格式
 */
export interface BybitTicker {
  symbol: string;
  bid1Price: string;
  bid1Size: number;
  ask1Price: string;
  ask1Size: number;
  lastPrice: string;
  price24hPcnt: string;
  highPrice24h: string;
  lowPrice24h: string;
  turnover24h: string;
  volume24h: string;
}

/**
 * Bybit 原始 K线数据格式
 */
export interface BybitKline {
  id: number;
  symbol: string;
  start: number;
  end: number;
  open: string;
  high: string;
  low: string;
  close: string;
  volume: string;
  turnover: string;
  interval: string;
}

// ============== 统一数据模型 ==============
// Unified Data Models

/**
 * 统一的 Ticker 数据模型
 * Unified Ticker Model
 */
export interface UnifiedTicker {
  // 基础信息
  symbol: string;           // 交易对标识 (例如: BTCUSDT)
  exchange: string;         // 交易所标识
  timestamp: number;        // 数据时间戳 (毫秒)

  // 价格信息
  price: number;            // 最新价格
  open: number;             // 24小时开盘价
  high: number;             // 24小时最高价
  low: number;              // 24小时最低价

  // 变动信息
  priceChange: number;      // 价格变动量
  priceChangePercent: number; // 价格变动百分比

  // 成交信息
  volume: number;           // 成交量 (基础货币)
  quoteVolume: number;      // 成交额 (计价货币)

  // 订单簿信息 (可选)
  bidPrice?: number;        // 买一价
  bidQty?: number;          // 买一量
  askPrice?: number;        // 卖一价
  askQty?: number;          // 卖一量
}

/**
 * 统一的 K线数据模型
 * Unified Kline Model
 */
export interface UnifiedKline {
  // 基础信息
  symbol: string;           // 交易对标识
  exchange: string;         // 交易所标识
  timeframe: string;        // 时间周期 (1m, 5m, 1h, 1d 等)
  timestamp: number;        // K线开始时间戳 (毫秒)

  // OHLCV 数据
  open: number;             // 开盘价
  high: number;             // 最高价
  low: number;              // 最低价
  close: number;            // 收盘价
  volume: number;           // 成交量

  // 额外信息 (可选)
  quoteVolume?: number;     // 成交额
  trades?: number;          // 成交笔数
}

/**
 * 统一的深度数据模型
 * Unified Orderbook Depth Model
 */
export interface UnifiedDepth {
  symbol: string;
  exchange: string;
  timestamp: number;
  bids: [number, number][]; // [价格, 数量]
  asks: [number, number][]; // [价格, 数量]
}

/**
 * 统一的成交数据模型
 * Unified Trade Model
 */
export interface UnifiedTrade {
  symbol: string;
  exchange: string;
  timestamp: number;
  price: number;
  quantity: number;
  side: 'buy' | 'sell';
  tradeId?: string;
}

// ============== 转换器错误类型 ==============
// Converter Error Types

/**
 * 转换错误类型
 */
export enum ConverterErrorType {
  INVALID_DATA = 'invalid_data',
  MISSING_FIELD = 'missing_field',
  PARSE_ERROR = 'parse_error',
  UNSUPPORTED_EXCHANGE = 'unsupported_exchange',
  VALIDATION_FAILED = 'validation_failed',
}

/**
 * 转换结果类型
 */
export interface ConverterResult<T> {
  success: boolean;
  data?: T;
  error?: {
    type: ConverterErrorType;
    message: string;
    raw?: any;
  };
}

/**
 * 转换器配置
 */
export interface ConverterConfig {
  // 验证开关
  validateData: boolean;
  strictMode: boolean;

  // 数据清理
  removeNaN: boolean;
  removeInfinity: boolean;

  // 日志
  logErrors: boolean;
  logWarnings: boolean;
}

/**
 * 默认配置
 */
export const DEFAULT_CONVERTER_CONFIG: ConverterConfig = {
  validateData: true,
  strictMode: false,
  removeNaN: true,
  removeInfinity: true,
  logErrors: true,
  logWarnings: true,
};
