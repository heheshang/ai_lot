// ============== 通用类型 ==============
export type Nullable<T> = T | null;
export type Optional<T> = T | undefined;

// ============== 用户类型 ==============
export enum UserRole {
  ADMIN = 'admin',
  DEVELOPER = 'developer',
  TRADER = 'trader',
  AUDITOR = 'auditor',
}

export enum UserStatus {
  ACTIVE = 'active',
  DISABLED = 'disabled',
  LOCKED = 'locked',
}

export interface User {
  id: string;
  username: string;
  displayName?: string;
  roleId: string;
  roleName: string;
  status: UserStatus;
  createdAt: number;
  updatedAt: number;
}

// ============== 认证类型 ==============
export interface LoginRequest {
  username: string;
  password: string;
}

export interface LoginResponse {
  user: User;
  token: string;
}

// ============== 交易所类型 ==============
export enum ExchangeName {
  BINANCE = 'binance',
  OKX = 'okx',
  BYBIT = 'bybit',
}

export interface ExchangeConfig {
  id: string;
  exchangeName: ExchangeName;
  displayName: string;
  isTestnet: boolean;
  status: 'active' | 'inactive' | 'error';
  createdAt: number;
}

// ============== 行情类型 ==============
export interface Ticker {
  symbol: string;
  price: number;
  priceChange: number;
  priceChangePercent: number;
  high24h: number;
  low24h: number;
  volume24h: number;
  timestamp: number;
}

export interface Kline {
  symbol: string;
  timeframe: string;
  timestamp: number;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

// ============== 策略类型 ==============
export interface StrategyParameter {
  name: string;
  type: 'number' | 'string' | 'boolean' | 'select';
  default: any;
  min?: number;
  max?: number;
  step?: number;
  options?: { label: string; value: any }[];
  description?: string;
}

export interface Strategy {
  id: string;
  userId: string;
  name: string;
  description?: string;
  code: string;
  language: 'javascript' | 'typescript';
  parameters: StrategyParameter[];
  parameterValues: Record<string, any>;
  category?: string;
  tags: string[];
  version: number;
  status: 'draft' | 'testing' | 'active' | 'archived';
  createdAt: number;
  updatedAt: number;
}

export interface StrategyInstance {
  id: string;
  strategyId: string;
  strategyName: string;
  exchangeId: string;
  symbol: string;
  timeframe: string;
  mode: 'backtest' | 'paper_trading' | 'live';
  status: 'stopped' | 'running' | 'paused' | 'error';
  parameters: Record<string, any>;
  startTime?: number;
  stopTime?: number;
  totalTrades: number;
  totalPnl: number;
  maxDrawdown: number;
  createdAt: number;
}

// ============== 交易类型 ==============
export enum OrderSide {
  BUY = 'buy',
  SELL = 'sell',
}

export enum OrderType {
  MARKET = 'market',
  LIMIT = 'limit',
  STOP_LIMIT = 'stop_limit',
  STOP_MARKET = 'stop_market',
}

export enum OrderStatus {
  PENDING = 'pending',
  OPEN = 'open',
  PARTIALLY_FILLED = 'partially_filled',
  FILLED = 'filled',
  CANCELED = 'canceled',
  REJECTED = 'rejected',
}

export interface Order {
  id: string;
  exchangeOrderId?: string;
  clientOrderId?: string;
  symbol: string;
  side: OrderSide;
  type: OrderType;
  price?: number;
  quantity: number;
  filledQuantity: number;
  avgPrice?: number;
  status: OrderStatus;
  commission: number;
  createdAt: number;
  filledAt?: number;
}

export interface Position {
  id: string;
  symbol: string;
  side: 'long' | 'short';
  quantity: number;
  entryPrice: number;
  currentPrice?: number;
  unrealizedPnl: number;
  realizedPnl: number;
  openedAt: number;
}

// ============== 回测类型 ==============
export interface BacktestParams {
  strategyId: string;
  symbol: string;
  timeframe: string;
  startTime: number;
  endTime: number;
  initialBalance: number;
  commissionRate: number;
  slippage: number;
  strategyParams: Record<string, any>;
}

export interface BacktestReport {
  id: string;
  strategyId: string;
  params: BacktestParams;
  totalReturn: number;
  sharpeRatio: number;
  maxDrawdown: number;
  winRate: number;
  totalTrades: number;
  winningTrades: number;
  losingTrades: number;
  trades: any[];
  equityCurve: number[];
  status: 'pending' | 'running' | 'completed' | 'failed';
}

// ============== 风控类型 ==============
export enum RiskRuleType {
  POSITION_LIMIT = 'position_limit',
  DRAWDOWN_LIMIT = 'drawdown_limit',
  LOSS_LIMIT = 'loss_limit',
}

export enum RiskAction {
  ALERT = 'alert',
  CLOSE_POSITION = 'close_position',
  STOP_STRATEGY = 'stop_strategy',
}

export interface RiskRule {
  id: string;
  name: string;
  description?: string;
  ruleType: RiskRuleType;
  config: Record<string, any>;
  thresholdValue: number;
  action: RiskAction;
  isEnabled: boolean;
}

export interface RiskAlert {
  id: string;
  ruleId: string;
  severity: 'info' | 'warning' | 'critical';
  title: string;
  message: string;
  currentValue: number;
  thresholdValue: number;
  status: 'active' | 'acknowledged' | 'resolved';
  createdAt: number;
}

// ============== API 响应类型 ==============
export interface ApiResponse<T = any> {
  success: boolean;
  data?: T;
  error?: string;
  code?: number;
}

// ============== Tauri 命令类型 ==============
export interface TauriCommand {
  cmd: string;
  payload?: any;
}
